#![feature(cursor_remaining)]

mod pgs;

use anyhow::{anyhow, Context, Result};
use getopts::Options;

use std::fs;
use std::io::{stdout, Write};

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print usage information and exit");
    opts.optflag("i", "in-place", "Modify input file in-place");
    opts.optopt("o", "output", "Specify output file", "FILE");
    opts.optflag("v", "version", "Print version information and exit");

    let mut matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print!("{}", opts.usage("Usage: pgs-chroma [options...] <file>"));
        return Ok(());
    } else if matches.opt_present("v") {
        println!("pgs-chroma v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if matches.opt_present("i") && matches.opt_present("o") {
        return Err(anyhow!("Conflicting arguments: --ouput, --in-place"));
    }

    if matches.free.is_empty() {
        return Err(anyhow!("Input file not provided"));
    }

    let input = matches.free.swap_remove(0);

    let output: Option<String> = if matches.opt_present("i") {
        Some(input.clone())
    } else {
        matches.opt_str("o")
    };

    let mut data = fs::read(&input).with_context(|| format!("failed to read from '{}'", &input))?;

    pgs::convert(&mut data)?;

    if let Some(o) = output {
        fs::write(&o, &data).with_context(|| format!("failed to write to '{}'", &o))?;
    } else {
        stdout().write_all(&data)?;
    }

    Ok(())
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("pgs-chroma: {:#}", error);
            std::process::exit(1);
        }
    }
}
