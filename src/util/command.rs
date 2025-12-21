use std::marker::PhantomData;
use std::sync::mpsc::Sender;
use std::{
    fmt::{self},
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

use yansi::{Condition, Paint};

#[derive(Debug, Clone)]
enum ExecutionMode {
    Silent,
    Live,
}

#[derive(Debug, Clone)]
pub struct Command {
    command: String,
    args: Vec<String>,
    message: Option<String>,
    cd: Option<PathBuf>,
    execution_mode: ExecutionMode,
}

#[derive(Debug, Clone)]
pub struct CommandResults {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

#[allow(dead_code)]
impl Command {
    pub fn new(command: impl Into<String>) -> Self {
        Command {
            command: command.into(),
            args: Vec::new(),
            cd: None,
            message: None,
            execution_mode: ExecutionMode::Silent,
        }
    }

    pub fn shell(command_line: impl Into<String>) -> Self {
        Self::new("sh").args(vec!["-c".into(), command_line.into()])
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn args<I, T>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn cd(mut self, path: impl AsRef<str>) -> Self {
        let path = std::fs::canonicalize(path.as_ref()).unwrap();
        self.cd = Some(path);
        self
    }

    pub fn silent(mut self) -> Self {
        self.execution_mode = ExecutionMode::Silent;
        self
    }

    pub fn live(mut self) -> Self {
        self.execution_mode = ExecutionMode::Live;
        self
    }

    fn build_process_command_and_log_details(&self) -> std::process::Command {
        println!("{self:#}");
        // if let Some(msg) = &self.message {
        //     println!("{}", msg);
        //     println!("{self:#}"); // only print exact command line if `message` is set
        // }

        let mut cmd = std::process::Command::new(&self.command);
        cmd.args(&self.args);

        if let Some(dir) = &self.cd {
            cmd.current_dir(dir);
        }
        cmd
    }

    fn spawn_pipe_thread<R, W>(
        mut reader: R,
        sink: W,
        tx: Sender<Vec<u8>>,
    ) -> thread::JoinHandle<()>
    where
        R: Read + Send + 'static,
        W: Write + Send + 'static,
    {
        thread::spawn(move || {
            let mut output = Vec::new();
            let mut buf = [0u8; 8192];
            let mut sink = sink;

            loop {
                let n = match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(_) => break,
                };

                output.extend_from_slice(&buf[..n]);
                let _ = sink.write_all(&buf[..n]);
            }

            let _ = sink.flush();
            let _ = tx.send(output);
        })
    }

    /// Execute the command and capture both stdout and stderr while simultaneously printing them live if live is true
    pub fn try_call(&self) -> CommandResults {
        // println!("{self:#}");

        let mut cmd = self.build_process_command_and_log_details();

        if let ExecutionMode::Live = self.execution_mode {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let mut child = cmd
                .spawn()
                .unwrap_or_else(|_| panic!("Command: {} failed to start", self.command.red()));

            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stderr = child.stderr.take().expect("Failed to capture stderr");

            let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
            let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

            let stdout_thread = Self::spawn_pipe_thread(stdout, std::io::stdout(), stdout_tx);
            let stderr_thread = Self::spawn_pipe_thread(stderr, std::io::stderr(), stderr_tx);

            let status = child.wait().expect("Failed to wait on child process");

            stdout_thread.join().expect("Failed to join stdout thread");
            stderr_thread.join().expect("Failed to join stderr thread");

            let stdout_bytes = stdout_rx.recv().unwrap_or_default();
            let stderr_bytes = stderr_rx.recv().unwrap_or_default();

            CommandResults {
                stdout: String::from_utf8_lossy(&stdout_bytes).into(),
                stderr: String::from_utf8_lossy(&stderr_bytes).into(),
                success: status.success(),
            }
        } else {
            let o = cmd.output().expect("Failed to execute process");
            CommandResults {
                stdout: String::from_utf8_lossy(&o.stdout).into(),
                stderr: String::from_utf8_lossy(&o.stderr).into(),
                success: o.status.success(),
            }
        }
    }

    pub fn call(&self) -> CommandResults {
        let r = self.try_call();
        if !r.success {
            if let ExecutionMode::Silent = self.execution_mode {
                println!("{}", r.stdout);
                eprintln!("{}", r.stderr);
            }
            panic!("{} {:?}", self.command, self.args);
        }
        r
    }

    /// Executes the command with inherited streams and returns a Result indicating success or failure.
    pub fn exec(&self) -> Result<(), std::io::Error> {
        // println!("{self:#}");

        let mut cmd = self.build_process_command_and_log_details();

        if matches!(self.execution_mode, ExecutionMode::Silent) {
            cmd.stdout(std::process::Stdio::null());
            cmd.stderr(std::process::Stdio::null());
        }

        cmd.spawn()
            .unwrap_or_else(|_| panic!("command: {} failed to start", self.command.red()))
            .wait()
            .expect("failed to wait on child")
            .success()
            .then_some(())
            .ok_or(std::io::Error::other(
                "command failed with non-zero exit code",
            ))
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let use_color = Condition::cached(f.alternate());
        //let use_color = Condition::cached(true);

        write!(
            f,
            "{}{}{} {}",
            self.message
                .as_deref()
                .map_or("".into(), |m| format!("{m}\n"))
                .green()
                .whenever(use_color),
            self.cd
                .as_ref()
                .map(|d| format!("cd {} && ", d.display()))
                .unwrap_or_default()
                .dim()
                .whenever(use_color),
            self.command.bright_white().whenever(use_color),
            self.args
                .iter()
                .map(|a| shell_escape::escape(a.into()))
                .collect::<Vec<_>>()
                .join(" "),
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_output() {
        let cmd = Command::shell("echo hello");
        let result = cmd.try_call();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "hello");
    }

    #[test]
    fn test_args() {
        let cmd = Command::new("echo")
            .args(["a", "b"])
            .args(["c".to_string(), "d".to_string()])
            .args(vec!["e", "f"]);
        let result = cmd.try_call();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "a b c d e f");
    }

    #[test]
    fn test_failure_command() {
        let result = Command::shell("exit 1").try_call();
        assert!(!result.success);
    }

    #[test]
    fn test_with_args() {
        let result = Command::new("echo").arg("hi").try_call();
        assert!(result.success);
        assert_eq!(result.stdout.trim(), "hi");
    }

    #[test]
    fn test_cd() {
        let path = "src";
        let expected_path = std::fs::canonicalize(path).unwrap();

        let result = Command::new("pwd").silent().cd(path).call();
        assert_eq!(result.stdout.trim(), expected_path.to_str().unwrap());
    }

    #[test]
    fn test_silent_mode() {
        let result = Command::shell("echo test").silent().try_call();
        assert_eq!(result.stdout.trim(), "test");
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_call_panic() {
        Command::new("ls")
            .arg("--there-is-no-such-options-rust-help")
            .call();
    }

    #[test]
    fn test_display_output() {
        let cmd = Command::new("echo")
            .arg("hello")
            .arg("world")
            .message("Running echo")
            .cd(".");

        let output = format!("{}", cmd); // plain formatting (no color)
        assert!(output.starts_with("Running echo"));
        assert!(output.contains("echo hello world"));
        assert!(output.contains("cd "));

        let alt_output = format!("{:#}", cmd); // alternate formatting (color on)

        assert!(!alt_output.starts_with("Running echo"));
        assert!(!alt_output.contains("echo hello world"));

        //let alt_output = strip_ansi_codes(&alt_output); // clean before checking
        assert!(alt_output.contains("Running echo"));
        assert!(alt_output.contains("echo"));
        assert!(alt_output.contains("hello world"));
    }

    #[test]
    fn test_call_live() {
        let result = Command::shell("echo stdout_test && echo stderr_test >&2")
            .live()
            .try_call();

        assert!(result.success);
        assert_eq!(result.stdout.trim(), "stdout_test");
        assert_eq!(result.stderr.trim(), "stderr_test");
    }

    #[test]
    fn test_call_live_multiline() {
        let result = Command::shell("echo line1 && echo line2 && echo line3")
            .live()
            .try_call();
        assert!(result.success);
        assert!(result.stdout.contains("line1"));
        assert!(result.stdout.contains("line2"));
        assert!(result.stdout.contains("line3"));
    }

    #[test]
    fn test_call_live_utf8() {
        // Test with various UTF-8 multi-byte characters
        let result = Command::shell("echo こんにちは世界 🌸 おちゃ café")
            .live()
            .try_call();

        assert!(result.success);
        assert!(result.stdout.contains("こんにちは世界"));
        assert!(result.stdout.contains("🌸"));
        assert!(result.stdout.contains("café"));
    }

    #[test]
    fn test_exec_success() {
        let result = Command::shell("echo test").exec();
        assert!(result.is_ok());
    }

    #[test]
    fn test_exec_failure() {
        let result = Command::shell("exit 1").exec();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "command failed with non-zero exit code"
        );
    }
}
