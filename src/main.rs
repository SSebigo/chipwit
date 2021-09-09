extern crate clap;
use clap::{App, Arg};

mod chipwit;
use chipwit::Chipwit;

mod fontset;
mod rom;

fn main() {
    let args = App::new("Chipwit")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .required(true)
                .takes_value(true)
                .help("Provide path to your ROM file"),
        )
        .get_matches();

    let path = args.value_of("path").expect("Provides path to ROM file");

    let mut chipwit: Chipwit = Chipwit::new();

    chipwit.load_rom(path);
    chipwit.load_font();

    println!("chipwit: {:?}", chipwit);
}
