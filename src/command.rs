use crate::command::Success::{Exited, Running};
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{sleep, Duration, Instant};
use tracing::debug;

#[derive(PartialEq, Eq, Debug)]
enum Success {
    Running,
    Exited(i32),
    Killed,
}
/// Spawns the script detached from the current process. Since we are running servers we don't want
/// to kill the process when the parent dies. or kill it after this function returns. So if the
/// server exit's it returns the exit code. If the server is still running it returns Success::Running
async fn execute_and_monitor(path: &str) -> Success {
    let mut command = Command::new("sh");
    command.args(&["-c", &format!("nohup {} &", path)]);

    let mut child = match command.stdout(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(_) => return Success::Exited(-1),
    };

    let start = Instant::now();
    let timeout = Duration::from_secs(10);
    let mut reader = BufReader::new(child.stdout.take().unwrap());

    let mut line = String::new();
    while reader.read_line(&mut line).await.unwrap() != 0 {
        debug!("Output of {}: {}", path, line);
        if start.elapsed() > timeout {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    match child.try_wait() {
        Ok(Some(exit_status)) => match exit_status.code() {
            Some(code) => Success::Exited(code),
            None => Success::Killed,
        },
        Ok(None) => Running,
        Err(_) => Exited(-1),
    }
}
