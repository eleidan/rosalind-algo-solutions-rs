use CliResult;
use util;
use rosalind_algo::fibo::fibonacci2;

static USAGE: &'static str = "
Computes Fibonacci number

Usage:
    rosalind-algo-solutions fibo <input>
    rosalind-algo-solutions fibo [options]

Options:
    -h, --help             Display this message
";

#[derive(RustcDecodable)]
struct Args {
    arg_input: Option<usize>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = try!(util::get_args(USAGE, argv));

    match args.arg_input {
        Some(input) => {
            fibonacci2(input);
            println!("{}", fibonacci2(input));
        }
        None        => println!("{}", USAGE.trim())
    }

    Ok(())
}
