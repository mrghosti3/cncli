pub mod conf;

use std::{fs, io, string};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cnc cli")]
#[command(author = "Ghosti3 <mysteriousghst@gmail.com>")]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Args {
    // Output file path
    #[arg(short, long)]
    #[arg(default_value = "-")]
    pub output: String,
    // Input cnc config file path
    #[arg(short = 'c', long = "config")]
    #[arg(default_value = "-")]
    config: String,
    // Input dxf file path
    pub dxf: String,
}

// impl Args {
//     fn get_config(&self) -> Result<Config, ConfigError> {
//         let in_cnf: ConfigInput = self.in_cnf.as_str().try_into()?;
//         let mut cnf: Vec<u8> = Vec::new();
//         in_cnf.read_to_end(&mut cnf);
//         let cnf = String::from_utf8(cnf)?;
//
//         // TODO: Implement deserialize from JSON to ConfigInput!
//     }
// }

enum ConfigError {
    Io(io::Error),
    Utf8(string::FromUtf8Error),
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<string::FromUtf8Error> for ConfigError {
    fn from(value: string::FromUtf8Error) -> Self {
        Self::Utf8(value)
    }
}

enum ConfigInput {
    Stdin(io::Stdin),
    File(fs::File),
}

impl TryFrom<&str> for ConfigInput {
    type Error = io::Error;

    fn try_from(value: &str) -> io::Result<Self> {
        match value {
            "-" => Ok(Self::Stdin(io::stdin())),
            file => Ok(Self::File(fs::File::open(file)?)),
        }
    }
}

impl io::Read for ConfigInput {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Stdin(file) => file.read(buf),
            Self::File(file) => file.read(buf),
        }
    }
}
