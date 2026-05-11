use super::emitter::WatEmitter;
use super::runtime_fn::RuntimeFn;

impl WatEmitter<'_> {
    /// Dispatch Date, Math, Promise, and Task domain runtime functions.
    pub(super) fn emit_dispatch_date(&mut self, f: RuntimeFn, wat: &mut String) {
        match f {
            RuntimeFn::DateNew => self.emit_date_new(wat),
            RuntimeFn::DateNewLive => self.emit_date_new_live(wat),
            RuntimeFn::DateNow => self.emit_date_now(wat),
            RuntimeFn::DateEpochMsNowNumber => self.emit_date_epoch_ms_now_number(wat),
            RuntimeFn::DateGetTime => self.emit_date_get_time(wat),
            RuntimeFn::DateToString => self.emit_date_to_string(wat),
            RuntimeFn::DateGetLocalTimeField => self.emit_date_get_local_time_field(wat),
            RuntimeFn::DateToISOString => self.emit_date_to_iso_string(wat),
            RuntimeFn::DateGetTimezoneOffset => self.emit_date_get_timezone_offset(wat),
            RuntimeFn::DateGetUtcMilliseconds => self.emit_date_get_utc_milliseconds(wat),
            RuntimeFn::DateGetUtcSeconds => self.emit_date_get_utc_seconds(wat),
            RuntimeFn::DateGetUtcMinutes => self.emit_date_get_utc_minutes(wat),
            RuntimeFn::DateGetUtcHours => self.emit_date_get_utc_hours(wat),
            RuntimeFn::DateGetUtcDay => self.emit_date_get_utc_day(wat),
            RuntimeFn::DateGetUtcDate => self.emit_date_get_utc_date(wat),
            RuntimeFn::DateGetUtcMonth => self.emit_date_get_utc_month(wat),
            RuntimeFn::DateGetUtcFullYear => self.emit_date_get_utc_full_year(wat),
            RuntimeFn::MathFloor => self.emit_math_floor(wat),
            RuntimeFn::MathCeil => self.emit_math_ceil(wat),
            RuntimeFn::MathRound => self.emit_math_round(wat),
            RuntimeFn::MathAbs => self.emit_math_abs(wat),
            RuntimeFn::MathMax => self.emit_math_max(wat),
            RuntimeFn::MathMin => self.emit_math_min(wat),
            RuntimeFn::MathPow => self.emit_math_pow(wat),
            RuntimeFn::MathRandom => self.emit_math_random(wat),
            RuntimeFn::MathTrunc => self.emit_math_trunc(wat),
            RuntimeFn::MathSign => self.emit_math_sign(wat),
            RuntimeFn::PromiseConstructor => self.emit_promise_constructor(wat),
            RuntimeFn::PromiseResolve => self.emit_promise_resolve(wat),
            RuntimeFn::PromiseReject => self.emit_promise_reject(wat),
            RuntimeFn::PromiseThen => self.emit_promise_then(wat),
            RuntimeFn::PromiseCatch => self.emit_promise_catch(wat),
            RuntimeFn::PromiseAll => self.emit_promise_all(wat),
            RuntimeFn::PromiseRace => self.emit_promise_race(wat),
            RuntimeFn::TaskPoll => self.emit_task_poll(wat),
            RuntimeFn::TaskResult => self.emit_task_result(wat),
            RuntimeFn::TaskDrop => self.emit_task_drop(wat),
            _ => unreachable!("non-date/math/promise/task RuntimeFn routed to date dispatch"),
        }
    }
}
