use std::process;
use structopt::StructOpt;
#[macro_use]
extern crate log;


#[derive(StructOpt, Debug)]
/// Rustacean mastermind implementation (https://rosettacode.org/wiki/Mastermind).
///
/// Creates a secret code of colors that you should guess. Users can select
/// the length of the code, the number of available colors and the max number
/// of guesses. For each guess, the program will print the result:
///
///   - `X`: correct color and position
///   - `O`: correct color
struct Opt {
    /// Number of different colors to use
    #[structopt(short = "n", long = "ncolors", default_value = "4")]
    colors: u8,
    /// Length of the code to break
    #[structopt(short, long, default_value = "4")]
    length: u32,
    /// Max number of guesses
    #[structopt(short, long, default_value = "20")]
    guesses: u32,
    /// Do not allow repeated colors in the code
    #[structopt(short, long)]
    unique: bool
}


fn main() {
    env_logger::init();
    let opt = Opt::from_args();
    println!("{:?}", opt);
    info!("Starting the game!");
    if let Err(e) = mastermind::run(opt.guesses, opt.length,opt.colors, opt.unique) {
        eprintln!("{}", e);
        eprintln!("Sorry, but you lost!");
        process::exit(2);
    } else {
        println!("Congratulations, you won!");
    }
}
