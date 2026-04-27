use std::io;
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub(crate) const IWASM_TIMEOUT: Duration = Duration::from_secs(5);

pub(crate) struct IwasmRunResult {
    pub output: Output,
    pub timed_out: bool,
}

pub(crate) fn run_iwasm_with_timeout(cmd: &mut Command) -> io::Result<IwasmRunResult> {
    let child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    run_iwasm_child_with_timeout(child)
}

pub(crate) fn run_iwasm_child_with_timeout(mut child: Child) -> io::Result<IwasmRunResult> {
    let deadline = Instant::now() + IWASM_TIMEOUT;
    let mut timed_out = false;

    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            timed_out = true;
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }

    let output = child.wait_with_output()?;
    Ok(IwasmRunResult { output, timed_out })
}
