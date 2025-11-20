use std::{
    fmt::{self},
    path::PathBuf,
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
    pub fn try_call(&self) -> CommandResults {
        println!("{self:#}");

        let mut cmd = std::process::Command::new(self.command.clone());
        cmd.args(self.args.clone());
        if let Some(dir) = &self.cd {
            cmd.current_dir(dir);
        }

        if let ExecutionMode::PrintOutput { live: true } = self.execution_mode {
            let s = cmd
                .spawn()
                .unwrap_or_else(|_| panic!("command: {} failed to start", self.command.red()))
                .wait()
                .expect("failed to wait on child");
            CommandResults {
                stdout: "".into(),
                stderr: "".into(),
                success: s.success(),
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
}
