use std::process::ExitCode;

use nook::repl::Repl;

fn main() -> ExitCode {
    if let Err(e) = Repl::new(std::io::stdin().lock(), std::io::stdout().lock()).run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
