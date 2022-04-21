use clap::{App, Arg};

fn main() {
    let matches = App::new("A cli tool written in Rust.")
        .version("1.0.0")
        .help_message("Prints the help message.")
        .arg(Arg::with_name("INPUT").help("Sets the input file to use"))
        .get_matches();

    println!("Get: {:?}", matches);
}
