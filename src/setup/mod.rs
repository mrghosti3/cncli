use std::{fs, io};

use clap::Parser;

use crate::util::IoWrap;

#[derive(Debug, Parser)]
#[command(name = "cnc cli")]
#[command(author = "Ghosti3 <mysteriousghst@gmail.com>")]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    // Output file path
    #[arg(short, long)]
    #[arg(default_value = "-")]
    output: String,

    // TODO: Input cnc config file path
    // #[arg(short = 'c', long = "config")]
    // #[arg(default_value = "-")]
    // config: String,

    // Input dxf file path
    pub dxf: String,
}

impl Args {

    /// Opens and buffered writer to output file (`std::io::Stdout`/`std::fs::File`).
    ///
    /// ```rust
    /// let args = setup::Args::parse();
    /// let mut output = args.open_output().expect("Could not open output");
    /// ````
    /// TODO: Bench test with dynamic dispatch vs enum.
    pub(crate) fn open_output(&self) -> io::Result<io::BufWriter<impl io::Write>> {
        match self.output.as_str() {
            "-" => Ok(io::BufWriter::new(IoWrap::Stdout(io::stdout()))),
            path => Ok(io::BufWriter::new(IoWrap::File(fs::File::create(path)?))),
        }
    }
}

pub mod conf;
