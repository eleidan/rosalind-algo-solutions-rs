extern crate rustc_serialize;
extern crate docopt;
extern crate rosalind_algo;

use docopt::Docopt;
use std::env;
use std::process;

mod cmd;
mod util;

const USAGE: &'static str ="
Rosalind Algorithmic Heights Solutions

Usage:
    rosalind-algo-solutions <command> [<args>...]
    rosalind-algo-solutions [options]

Options:
    -h, --help    Display this message
    --version     Print version info and exit

Commands:
    bins    Provide binary search
    fibo    Compute Fibonacci number
";

macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
    });
}

#[derive(RustcDecodable)]
struct Args {
    arg_command: Option<Command>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                    .and_then(|docopt| docopt.options_first(true)
                                             .version(Some(util::version()))
                                             .decode())
                    .unwrap_or_else(|e| e.exit());

    match args.arg_command {
        None => { print!("{}", USAGE) },
        Some(cmd) => {
            match cmd.run() {
                Ok(()) => process::exit(0),
                Err(CliError::Flag(err)) => err.exit(),
                Err(CliError::Other(msg)) => {
                    werr!("{}", msg);
                    process::exit(1);
                }
            }
        }
    }
}

#[derive(Debug, RustcDecodable)]
enum Command {
    Bins,
    Fibo,
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().map(|v| v.to_owned()).collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;
        match self {
            Command::Fibo => cmd::fibo::run(argv),
            Command::Bins => cmd::bins::run(argv),
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(docopt::Error),
    Other(String),
}

impl From<docopt::Error> for CliError {
    fn from(err: docopt::Error) -> CliError {
        CliError::Flag(err)
    }
}
