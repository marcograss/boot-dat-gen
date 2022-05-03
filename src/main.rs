use clap::{Arg, Command};
use std::path::Path;
use tx_custom_boot::generate_boot_dat;

fn main() {
    let cmd = Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .arg(Arg::new("payload").takes_value(true).required(true).help("The payload to be converted to boot.dat"))
    .arg(Arg::new("output").takes_value(true).required(false).help("The output file"));

    let matches = cmd.get_matches();

    let payload = matches.value_of("payload").unwrap();
    let output = matches.value_of("output").unwrap_or("boot.dat");

    // Check if the input file exists
    if !Path::new(payload).exists() {
        eprintln!("Input file does not exist.");
        return;
    }

    // Check if the output file exists
    if Path::new(output).exists() {
        eprintln!("Output file already exists, please remove it first.");
        return;
    }

    // Read the input file
    let payload = std::fs::read(payload).expect("cannot read the input file");
    let generated = generate_boot_dat(&payload);

    // Write the output file
    std::fs::write(output, generated).expect("cannot write the output file");
}
