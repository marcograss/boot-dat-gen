use anyhow::anyhow;
use clap::{Arg, Command};
use std::path::Path;
use tx_custom_boot::generate_boot_dat;

fn main() -> anyhow::Result<()> {
    let cmd = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::new("payload")
                .required(true)
                .help("The payload to be converted to boot.dat"),
        )
        .arg(
            Arg::new("output")
                .required(false)
                .help("The output file")
                .default_value("boot.dat"),
        );

    let matches = cmd.get_matches();

    let payload = matches.get_one::<String>("payload").unwrap();
    let output = matches.get_one::<String>("output").unwrap();

    // Check if the input file exists
    if !Path::new(payload).exists() {
        return Err(anyhow!("Input file does not exist."));
    }

    // Check if the output file exists
    if Path::new(output).exists() {
        return Err(anyhow!(
            "Output file already exists, please remove it first."
        ));
    }

    // Read the input file
    let payload = std::fs::read(payload)?;
    let generated = generate_boot_dat(&payload)?;

    // Write the output file
    std::fs::write(output, generated)?;
    Ok(())
}
