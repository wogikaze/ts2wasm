/*
 * Persistent WAMR runner: reads JSONL jobs from stdin, runs wasm modules,
 * writes JSONL results to stdout. WAMR runtime is initialized once.
 *
 * Includes native function stubs for host imports that would otherwise
 * cause "unlinked import" RuntimeErrors.
 *
 * Build:
 *   gcc -o ts2wasm-iwasm-runner main.c \
 *       -I/path/to/wasm-micro-runtime/core/iwasm/include \
 *       -I/path/to/wasm-micro-runtime/core/shared/utils \
 *       -I/path/to/wasm-micro-runtime/core/shared/platform/linux \
 *       -I/path/to/wasm-micro-runtime/core/shared/platform/include \
 *       /path/to/wasm-micro-runtime/libiwasm.a \
 *       -lm -lpthread -ldl
 */

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <inttypes.h>
#include <unistd.h>
#include <time.h>
#include <signal.h>
#include <errno.h>

#include "bh_platform.h"
#include "wasm_export.h"
#include "lib_export.h"

static char global_heap_buf[16 * 1024 * 1024];

static int saved_stdout_fd = -1;
static int saved_stderr_fd = -1;
static int pipe_stdout[2] = {-1, -1};
static int pipe_stderr[2] = {-1, -1};

static void capture_start(void)
{
    pipe(pipe_stdout);
    pipe(pipe_stderr);
    saved_stdout_fd = dup(STDOUT_FILENO);
    saved_stderr_fd = dup(STDERR_FILENO);
    dup2(pipe_stdout[1], STDOUT_FILENO);
    dup2(pipe_stderr[1], STDERR_FILENO);
    close(pipe_stdout[1]);
    close(pipe_stderr[1]);
}

static char *capture_read(int fd, size_t *len)
{
    size_t cap = 4096, used = 0;
    char *buf = malloc(cap);
    while (1) {
        ssize_t n = read(fd, buf + used, cap - used - 1);
        if (n <= 0) break;
        used += n;
        if (used >= cap - 2) {
            cap *= 2;
            buf = realloc(buf, cap);
        }
    }
    buf[used] = '\0';
    *len = used;
    return buf;
}

static void capture_stop(char **stdout_out, char **stderr_out)
{
    close(STDOUT_FILENO);
    close(STDERR_FILENO);
    dup2(saved_stdout_fd, STDOUT_FILENO);
    dup2(saved_stderr_fd, STDERR_FILENO);
    close(saved_stdout_fd);
    close(saved_stderr_fd);
    size_t ol = 0, el = 0;
    *stdout_out = capture_read(pipe_stdout[0], &ol);
    *stderr_out = capture_read(pipe_stderr[0], &el);
    close(pipe_stdout[0]);
    close(pipe_stderr[0]);
}

static char *read_line(void)
{
    size_t cap = 4096, used = 0;
    char *buf = malloc(cap);
    while (1) {
        int c = fgetc(stdin);
        if (c == EOF) { if (!used) { free(buf); return NULL; } break; }
        if (c == '\n') break;
        buf[used++] = c;
        if (used >= cap - 2) { cap *= 2; buf = realloc(buf, cap); }
    }
    buf[used] = '\0';
    return buf;
}

static char *get_json_str(const char *json, const char *key)
{
    char search[64];
    snprintf(search, sizeof(search), "\"%s\"", key);
    const char *p = strstr(json, search);
    if (!p) return NULL;
    p = strchr(p, ':');
    if (!p) return NULL;
    p++; while (*p == ' ') p++;
    if (*p != '"') return NULL;
    p++;
    const char *end = strchr(p, '"');
    if (!end) return NULL;
    size_t len = end - p;
    char *v = malloc(len + 1);
    memcpy(v, p, len); v[len] = '\0';
    return v;
}

/* SIGALRM handler: just return (interrupts the blocking call) */
static void alarm_handler(int sig) { (void)sig; }

static int64_t get_json_int(const char *json, const char *key)
{
    char search[64];
    snprintf(search, sizeof(search), "\"%s\"", key);
    const char *p = strstr(json, search);
    if (!p) return 0;
    p = strchr(p, ':');
    if (!p) return 0;
    p++; while (*p == ' ') p++;
    return atoll(p);
}

static void print_json_str(FILE *f, const char *s)
{
    fputc('"', f);
    for (; *s; s++) {
        switch (*s) {
            case '"': fputs("\\\"", f); break;
            case '\\': fputs("\\\\", f); break;
            case '\n': fputs("\\n", f); break;
            default: fputc(*s, f); break;
        }
    }
    fputc('"', f);
}

/* ------------------------------------------------------------------ */
/* Native host functions                                               */
/* ------------------------------------------------------------------ */

/*
 * host.eval.direct — stub that returns undefined (tagged value 0).
 * Calling code sees undefined as the eval result and continues execution,
 * causing a test262 assertion failure instead of a RuntimeError crash.
 * WASM signature: (i32 source_tag, i32 env_tag) -> i32
 */
static uint32_t
host_eval_direct(wasm_exec_env_t exec_env, uint32_t source_tag, uint32_t env_tag)
{
    (void)exec_env;
    (void)source_tag;
    (void)env_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.eval.indirect — stub that returns undefined (tagged value 0).
 * WASM signature: (i32 source_tag, i32 env_tag) -> i32
 */
static uint32_t
host_eval_indirect(wasm_exec_env_t exec_env, uint32_t source_tag, uint32_t env_tag)
{
    (void)exec_env;
    (void)source_tag;
    (void)env_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.function.compile — stub that returns undefined (tagged value 0).
 * Called when the Function/AsyncFunction/GeneratorFunction constructor is
 * used with string arguments. A real implementation would parse and compile
 * the function body; returning undefined causes a test262 assertion failure
 * instead of a RuntimeError crash.
 * WASM signature: (i32 source_tag) -> i32
 */
static uint32_t
host_function_compile(wasm_exec_env_t exec_env, uint32_t source_tag)
{
    (void)exec_env;
    (void)source_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.function.call — stub that returns undefined (tagged value 0).
 * WASM signature: (i32 func_tag, i32 this_tag) -> i32
 */
static uint32_t
host_function_call(wasm_exec_env_t exec_env, uint32_t func_tag, uint32_t this_tag)
{
    (void)exec_env;
    (void)func_tag;
    (void)this_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.function.callMethod — stub that returns undefined (tagged value 0).
 * WASM signature: (i32 func_tag, i32 this_tag, i32 args_tag) -> i32
 */
static uint32_t
host_function_call_method(wasm_exec_env_t exec_env,
                          uint32_t func_tag, uint32_t this_tag, uint32_t args_tag)
{
    (void)exec_env;
    (void)func_tag;
    (void)this_tag;
    (void)args_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.function.construct — stub that returns undefined (tagged value 0).
 * WASM signature: (i32 func_tag, i32 args_tag) -> i32
 */
static uint32_t
host_function_construct(wasm_exec_env_t exec_env,
                        uint32_t func_tag, uint32_t args_tag)
{
    (void)exec_env;
    (void)func_tag;
    (void)args_tag;
    /* Return undefined (tagged value 0) */
    return 0;
}

/*
 * host.dateGetLocalTimeField — return a local time field from epoch ms.
 * WASM signature: (i32 epoch_ms, i32 field) -> i32
 *
 * field codes (from Date constructor):
 *   0 = year, 1 = month (0-based), 2 = day, 3 = hours,
 *   4 = minutes, 5 = seconds, 6 = ms
 */
static uint32_t
host_date_get_local_time_field(wasm_exec_env_t exec_env,
                                uint32_t epoch_ms, uint32_t field)
{
    (void)exec_env;
    time_t secs = (time_t)(epoch_ms / 1000);
    int ms_part = (int)(epoch_ms % 1000);
    if (ms_part < 0) { ms_part += 1000; secs -= 1; }

    struct tm result;
    if (!localtime_r(&secs, &result)) {
        return 0;
    }

    switch (field) {
        case 0: return result.tm_year + 1900;
        case 1: return result.tm_mon;
        case 2: return result.tm_mday;
        case 3: return result.tm_hour;
        case 4: return result.tm_min;
        case 5: return result.tm_sec;
        case 6: return (uint32_t)ms_part;
        default: return 0;
    }
}

/*
 * host.dateGetTimezoneOffset — return timezone offset in minutes.
 * WASM signature: (i32 epoch_ms) -> i32
 */
static uint32_t
host_date_get_timezone_offset(wasm_exec_env_t exec_env, uint32_t epoch_ms)
{
    (void)exec_env;
    time_t secs = (time_t)(epoch_ms / 1000);

    struct tm local_result;
    if (!localtime_r(&secs, &local_result)) {
        return 0;
    }

    /* Compute timezone offset by comparing local and UTC time.
     * mktime interprets tm as local time; we compare with gmtime. */
    struct tm utc_result;
    if (!gmtime_r(&secs, &utc_result)) {
        return 0;
    }

    /* Compute offset = local - UTC in minutes */
    time_t local_epoch = mktime(&local_result);
    time_t utc_epoch = timegm(&utc_result);

    int offset_minutes = (int)difftime(local_epoch, utc_epoch) / 60;
    return (uint32_t)(-offset_minutes); /* ECMAScript: positive = behind UTC */
}

/*
 * host.dateUTC — compute epoch ms from calendar fields (UTC).
 * WASM signature: (i32 year, i32 month, i32 date, i32 hours, i32 minutes, i32 seconds, i32 ms) -> i32
 */
static uint32_t
host_date_utc(wasm_exec_env_t exec_env,
              uint32_t year, uint32_t month, uint32_t day,
              uint32_t hours, uint32_t minutes, uint32_t seconds,
              uint32_t ms)
{
    (void)exec_env;
    struct tm tm_val;
    memset(&tm_val, 0, sizeof(tm_val));
    tm_val.tm_year = (int)year - 1900;
    tm_val.tm_mon = (int)month;
    tm_val.tm_mday = (int)day;
    tm_val.tm_hour = (int)hours;
    tm_val.tm_min = (int)minutes;
    tm_val.tm_sec = (int)seconds;
    tm_val.tm_isdst = 0;

    time_t epoch_secs = timegm(&tm_val);
    if (epoch_secs == (time_t)-1) {
        return 0;
    }
    return (uint32_t)((int64_t)epoch_secs * 1000 + (int64_t)ms);
}

/* Native symbol table for the "host" module.
 * import names in WASM use dot notation: host.eval.direct, host.dateGetTimezoneOffset, etc.
 */
static NativeSymbol host_native_symbols[] = {
    {
        .symbol = "eval.direct",
        .func_ptr = (void *)host_eval_direct,
        .signature = "(ii)i",
        .attachment = NULL,
    },
    {
        .symbol = "eval.indirect",
        .func_ptr = (void *)host_eval_indirect,
        .signature = "(ii)i",
        .attachment = NULL,
    },
    {
        .symbol = "function.compile",
        .func_ptr = (void *)host_function_compile,
        .signature = "(i)i",
        .attachment = NULL,
    },
    {
        .symbol = "function.call",
        .func_ptr = (void *)host_function_call,
        .signature = "(ii)i",
        .attachment = NULL,
    },
    {
        .symbol = "function.callMethod",
        .func_ptr = (void *)host_function_call_method,
        .signature = "(iii)i",
        .attachment = NULL,
    },
    {
        .symbol = "function.construct",
        .func_ptr = (void *)host_function_construct,
        .signature = "(ii)i",
        .attachment = NULL,
    },
    {
        .symbol = "dateGetTimezoneOffset",
        .func_ptr = (void *)host_date_get_timezone_offset,
        .signature = "(i)i",
        .attachment = NULL,
    },
    {
        .symbol = "dateGetLocalTimeField",
        .func_ptr = (void *)host_date_get_local_time_field,
        .signature = "(ii)i",
        .attachment = NULL,
    },
    {
        .symbol = "dateUTC",
        .func_ptr = (void *)host_date_utc,
        .signature = "(iiiiiii)i",
        .attachment = NULL,
    },
};

static int host_native_symbol_count =
    sizeof(host_native_symbols) / sizeof(host_native_symbols[0]);

/* ------------------------------------------------------------------ */

int main(int argc, char *argv[])
{
    RuntimeInitArgs init_args;
    memset(&init_args, 0, sizeof(init_args));
    init_args.mem_alloc_type = Alloc_With_Pool;
    init_args.mem_alloc_option.pool.heap_buf = global_heap_buf;
    init_args.mem_alloc_option.pool.heap_size = sizeof(global_heap_buf);
    init_args.running_mode = Mode_Interp;

    if (!wasm_runtime_full_init(&init_args)) {
        fprintf(stderr, "WAMR init failed\n");
        return 1;
    }
    bh_log_set_verbose_level(0);

    /* Register native host functions */
    if (!wasm_runtime_register_natives("host",
                                       host_native_symbols,
                                       (uint32_t)host_native_symbol_count)) {
        fprintf(stderr, "WARNING: failed to register host native symbols\n");
    }

    char *line;
    while ((line = read_line()) != NULL) {
        if (!strcmp(line, "exit") || !line[0]) { free(line); break; }

        char *wasm_path = get_json_str(line, "wasm_path");
        int64_t id_val = get_json_int(line, "id");
        uint32_t heap_size = (uint32_t)get_json_int(line, "heap_size");
        if (heap_size == 0) heap_size = 16 * 1024;

        if (!wasm_path) {
            printf("{\"id\":%" PRId64 ",\"status\":\"parse_error\"}\n", id_val);
            fflush(stdout); free(line); continue;
        }

        FILE *fp = fopen(wasm_path, "rb");
        if (!fp) {
            printf("{\"id\":%" PRId64 ",\"status\":\"error\",\"stderr\":\"cannot open\"}\n", id_val);
            fflush(stdout); free(wasm_path); free(line); continue;
        }
        fseek(fp, 0, SEEK_END);
        uint32_t size = ftell(fp);
        fseek(fp, 0, SEEK_SET);
        uint8_t *bytes = malloc(size);
        fread(bytes, 1, size, fp);
        fclose(fp);

        char err[128];
        wasm_module_t module = wasm_runtime_load(bytes, size, err, sizeof(err));
        if (!module) {
            printf("{\"id\":%" PRId64 ",\"status\":\"error\",\"stderr\":\"load: %s\"}\n", id_val, err);
            free(bytes); free(wasm_path); free(line); continue;
        }

        char *wasi_argv[] = { wasm_path, NULL };
        wasm_runtime_set_wasi_args(module, NULL, 0, NULL, 0, NULL, 0, wasi_argv, 1);

        wasm_module_inst_t inst = wasm_runtime_instantiate(
            module, 64 * 1024, heap_size, err, sizeof(err));
        if (!inst) {
            printf("{\"id\":%" PRId64 ",\"status\":\"error\",\"stderr\":\"instantiate: %s\"}\n", id_val, err);
            wasm_runtime_unload(module);
            free(bytes); free(wasm_path); free(line); continue;
        }

        /* Timeout from job, default 10s */
        uint32_t timeout_sec = (uint32_t)get_json_int(line, "timeout_ms");
        if (timeout_sec == 0) timeout_sec = 5000;
        timeout_sec = (timeout_sec + 999) / 1000;

        struct sigaction sa;
        memset(&sa, 0, sizeof(sa));
        sa.sa_handler = alarm_handler;
        sigaction(SIGALRM, &sa, NULL);
        alarm(timeout_sec);

        struct timespec t0, t1;
        clock_gettime(CLOCK_MONOTONIC, &t0);

        char *cap_out = NULL, *cap_err = NULL;
        capture_start();
        uint32_t exec_ret = wasm_application_execute_main(inst, 0, NULL);
        capture_stop(&cap_out, &cap_err);

        alarm(0); /* Disable alarm */

        clock_gettime(CLOCK_MONOTONIC, &t1);
        uint64_t dur = (t1.tv_sec - t0.tv_sec) * 1000000 +
                       (t1.tv_nsec - t0.tv_nsec) / 1000;
        dur = (dur + 500) / 1000; /* round to nearest ms */

        /* Capture exception info — check even when exec_ret==0 because
         * native functions that call wasm_runtime_set_exception may not
         * immediately trap execution. */
        const char *exc = wasm_runtime_get_exception(inst);
        int has_exception = (exc && exc[0]);
        if (exec_ret != 0 || has_exception) {
            if (exc && exc[0]) {
                /* Prepend exception to captured stderr */
                size_t exc_len = strlen(exc);
                if (cap_err) {
                    size_t old_len = strlen(cap_err);
                    cap_err = realloc(cap_err, exc_len + 3 + old_len + 1);
                    memmove(cap_err + exc_len + 2, cap_err, old_len + 1);
                    memcpy(cap_err, exc, exc_len);
                    cap_err[exc_len] = '\n';
                    cap_err[exc_len + 1] = '\n';
                } else {
                    cap_err = malloc(exc_len + 1);
                    memcpy(cap_err, exc, exc_len + 1);
                }
            }
        }

        const char *status;
        if (exec_ret == 0 && !has_exception) {
            status = "ok";
        } else {
            status = "error";
        }

        printf("{\"id\":%" PRId64 ",\"status\":\"%s\"", id_val, status);
        if (cap_out && cap_out[0]) {
            fputs(",\"stdout\":", stdout);
            print_json_str(stdout, cap_out);
        }
        if (cap_err && cap_err[0]) {
            fputs(",\"stderr\":", stdout);
            print_json_str(stdout, cap_err);
        }
        if (dur == 0) dur = 1; /* ensure non-zero for profiling */
        printf(",\"duration_ms\":%" PRIu64 "}\n", dur);
        fflush(stdout);

        free(cap_out); free(cap_err);
        wasm_runtime_deinstantiate(inst);
        wasm_runtime_unload(module);
        free(bytes); free(wasm_path); free(line);
    }

    wasm_runtime_destroy();
    return 0;
}
