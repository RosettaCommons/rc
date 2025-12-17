use std::{
    fmt::{self},
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

use yansi::{Condition, Paint};

#[derive(Debug)]
enum ExecutionMode {
    Silent,
    PrintOutput { live: bool },
}

impl Default for ExecutionMode {
    fn default() -> Self {
        ExecutionMode::PrintOutput { live: false }
    }
}

#[derive(Default, Debug)]
pub struct Command {
    command: String,
    args: Vec<String>,
    message: Option<String>,
    cd: Option<PathBuf>,
    execution_mode: ExecutionMode,
}

#[derive(Debug)]
pub struct CommandResults {
    //command_line: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
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

#[allow(dead_code)]
impl Command {
    pub fn new(command: impl Into<String>) -> Command {
        Command {
            command: command.into(),
            ..Default::default()
        }
    }

    pub fn shell(command_line: impl Into<String>) -> Command {
        Command {
            command: "sh".into(),
            args: vec!["-c".into(), command_line.into()],
            ..Default::default()
        }
    }

    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = Some(message.into());
        self
    }

    pub fn args<I, T>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    pub fn arg(&mut self, arg: impl Into<String>) -> &mut Self {
        self.args.push(arg.into());
        self
    }

    pub fn cd(&mut self, path: impl AsRef<str>) -> &mut Self {
        let path = std::fs::canonicalize(path.as_ref()).unwrap();
        self.cd = Some(path);
        self
    }

    pub fn silent(&mut self) -> &mut Self {
        self.execution_mode = ExecutionMode::Silent;
        self
    }

    pub fn live(&mut self) -> &mut Self {
        self.execution_mode = ExecutionMode::PrintOutput { live: true };
        self
    }

    // maybe Result<CommandResults, std::io::Error> instead? ie Error::new(ErrorKind::Other, "something went wrong");
    // pub fn try_call(&self) -> CommandResults {
    //     println!("{self:#}");

    //     let mut cmd = std::process::Command::new(self.command.clone());
    //     cmd.args(self.args.clone());
    //     if let Some(dir) = &self.cd {
    //         cmd.current_dir(dir);
    //     }

    //     if let ExecutionMode::PrintOutput { live: true } = self.execution_mode {
    //         let s = cmd
    //             .spawn()
    //             .unwrap_or_else(|_| panic!("command: {} failed to start", self.command.red()))
    //             .wait()
    //             .expect("failed to wait on child");
    //         CommandResults {
    //             stdout: "".into(),
    //             stderr: "".into(),
    //             success: s.success(),
    //         }
    //     } else {
    //         let o = cmd.output().expect("failed to execute process");
    //         let r = CommandResults {
    //             stdout: String::from_utf8_lossy(&o.stdout).into(),
    //             stderr: String::from_utf8_lossy(&o.stderr).into(),
    //             success: o.status.success(),
    //         };
    //         if let ExecutionMode::PrintOutput { live: false } = self.execution_mode {
    //             println!("{}", r.stdout);
    //             eprintln!("{}", r.stderr);
    //         }
    //         r
    //     }
    // }

    /// Execute the command and capture both stdout and stderr while simultaneously printing them live if live is true
    pub fn try_call(&self) -> CommandResults {
        println!("{self:#}");

        let mut cmd = std::process::Command::new(self.command.clone());
        cmd.args(self.args.clone());
        if let Some(dir) = &self.cd {
            cmd.current_dir(dir);
        }

        if let ExecutionMode::PrintOutput { live: true } = self.execution_mode {
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let mut child = cmd
                .spawn()
                .unwrap_or_else(|_| panic!("command: {} failed to start", self.command.red()));

            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let stderr = child.stderr.take().expect("Failed to capture stderr");

            // Create channels to collect output
            let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
            let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

            fn ssd() {}
            // Spawn thread to read and print stdout
            let stdout_thread = thread::spawn(move || {
                let mut reader = stdout;
                let mut output = Vec::new();
                let mut buf = [0u8; 8192];

                let mut out = std::io::stdout().lock();

                loop {
                    let n = match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };

                    output.extend_from_slice(&buf[..n]);
                    let _ = out.write_all(&buf[..n]); // <-- bytes, not chars
                    let _ = out.flush();
                }

                let _ = stdout_tx.send(output);
            });

            // Spawn thread to read and print stderr
            let stderr_thread = thread::spawn(move || {
                let mut reader = stderr;
                let mut output = Vec::new();
                let mut buf = [0u8; 8192];

                let mut err = std::io::stderr().lock();

                loop {
                    let n = match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => n,
                        Err(_) => break,
                    };

                    output.extend_from_slice(&buf[..n]);
                    let _ = err.write_all(&buf[..n]);
                    let _ = err.flush();
                }

                let _ = stderr_tx.send(output);
            });

            // Wait for the child process to complete
            let status = child.wait().expect("Failed to wait on child process");

            // Wait for threads to finish and collect output
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
            let o = cmd.output().expect("failed to execute process");
            let r = CommandResults {
                stdout: String::from_utf8_lossy(&o.stdout).into(),
                stderr: String::from_utf8_lossy(&o.stderr).into(),
                success: o.status.success(),
            };
            if let ExecutionMode::PrintOutput { live: false } = self.execution_mode {
                println!("{}", r.stdout);
                eprintln!("{}", r.stderr);
            }
            r
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
        let mut cmd = Command::new("echo");
        cmd.args(["a", "b"]);
        cmd.args(["c".to_string(), "d".to_string()]);
        cmd.args(vec!["e", "f"]);
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
        let mut cmd = Command::new("echo");
        let result = cmd.arg("hi").try_call();
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
    fn test_live_mode() {
        let mut cmd = Command::shell(":");
        let result = cmd.live().try_call();
        assert!(result.success);
        // stdout/stderr are empty in live mode in current implementation
        assert_eq!(result.stdout, "");
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
        // fn strip_ansi_codes(s: &str) -> String {
        //     let mut result = String::with_capacity(s.len());
        //     let mut chars = s.chars().peekable();
        //     while let Some(c) = chars.next() {
        //         if c == '\x1b' && chars.peek() == Some(&'[') {
        //             chars.next(); // consume '['
        //             // Skip until a letter (ASCII A–Z or a–z)
        //             while let Some(&next) = chars.peek() {
        //                 if next.is_ascii_alphabetic() {
        //                     chars.next(); // consume final letter
        //                     break;
        //                 }
        //                 chars.next(); // consume part of escape
        //             }
        //             continue;
        //         }
        //         result.push(c);
        //     }
        //     result
        // }

        let mut cmd = Command::new("echo");
        cmd.arg("hello").arg("world");
        cmd.message("Running echo").cd(".");

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
}
