/*
 * Persistent WAMR runner: reads JSONL jobs from stdin, runs wasm modules,
 * writes JSONL results to stdout. WAMR runtime is initialized once.
 *
 * Build:
 *   gcc -o ts2wasm-iwasm-runner main.c \
 *       -I/path/to/wasm-micro-runtime/core/iwasm/include \
 *       -I/path/to/wasm-micro-runtime/core/shared \
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

#include "bh_platform.h"
#include "wasm_export.h"

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

        /* Capture exception info when execution fails */
        if (exec_ret != 0) {
            const char *exc = wasm_runtime_get_exception(inst);
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

        const char *status = "ok";
        if (exec_ret != 0 && (!cap_out || !cap_out[0]))
            status = "error";

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
