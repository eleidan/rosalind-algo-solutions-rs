use CliResult;
use util;

static USAGE: &'static str = "
Provide binary search

Usage:
    rosalind-algo-solutions bins <input>
    rosalind-algo-solutions bins [options]

Options:
    -h, --help             Display this message
";

#[derive(RustcDecodable)]
struct Args {
    arg_input: Option<u64>,
}

pub fn run(argv: &[&str]) -> CliResult<()>  {
    let args: Args = try!(util::get_args(USAGE, argv));

    println!("Dispatching subcommand arguments:");
    println!("command   {:?}", args.arg_input);

    match args.arg_input {
        Some(input) => {
            println!("{}", input);
        }
        None        => unreachable!()
    }

    Ok(())

}
