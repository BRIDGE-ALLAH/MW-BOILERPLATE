use std::env;
use std::process::{Command, ExitCode};
fn main() -> ExitCode {
 let mut args = env::args().skip(1);
 let task = args.next().unwrap_or_else(|| "help".into());
 let extra: Vec<String> = args.collect();
 let (program, mut cmd): (&str, Vec<String>) = match task.as_str() {
  "dev" => ("dx", vec!["serve".into(), "--package".into(), "mw-web".into()]),
  "build" => ("dx", vec!["build".into(), "--release".into(), "--package".into(), "mw-web".into()]),
  "check" => ("cargo", vec!["check".into(), "--workspace".into()]),
  "test" => ("cargo", vec!["test".into(), "--workspace".into()]),
  "lint" => ("cargo", vec!["clippy".into(), "--workspace".into(), "--all-targets".into(), "--".into(), "-D".into(), "warnings".into()]),
  "fmt" => ("cargo", vec!["fmt".into(), "--all".into()]),
  "help" | "--help" | "-h" => { println!("cargo dev [dx args]  Start Dioxus dev server with hot reload\ncargo build-web       Build optimized web app\ncargo check-all       Check workspace\ncargo test-all        Run tests\ncargo lint            Run Clippy\ncargo fmt             Format Rust sources"); return ExitCode::SUCCESS; },
  _ => { eprintln!("Unknown task: {task}"); return ExitCode::from(2); }
 };
 cmd.extend(extra);
 match Command::new(program).args(cmd).status() {
  Ok(s) if s.success() => ExitCode::SUCCESS,
  Ok(s) => ExitCode::from(s.code().unwrap_or(1) as u8),
  Err(e) => { eprintln!("Could not start {program}: {e}"); if program == "dx" { eprintln!("Install with: cargo install dioxus-cli --locked"); } ExitCode::from(1) }
 }
}
