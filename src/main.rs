//! ``chromatic``
//!
//! A toolkit to assist color scheme design in Vim.
//! Uses a TOML config to generate a vim color scheme.

use askama::Template;
use std::convert::TryFrom;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod scheme;
use scheme::*;

/// Command Line Input
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct ChromaticArguments {
    /// Chromatic Config File
    #[structopt(name = "INPUT")]
    input: PathBuf,

    /// Output File
    #[structopt(short = "o", long = "output")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Fetch arguments
    let args = ChromaticArguments::from_args();

    // Check Config
    if args.input.exists() {
        // Parse Config
        let config = ChromaticConfig::try_from(args.input)?;

        // Parse Template from Config
        let template = ChromaticTemplate::try_from(config)?;

        // Render File to Output
        let mut output = File::create(args.output)?;
        output.write_all(template.render()?.as_bytes())?;
    } else {
        println!("Cannot find input file: {:?}", args.input.as_os_str());
    }

    Ok(())
}
