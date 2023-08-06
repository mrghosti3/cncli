use std::fs;
use std::io::{BufReader, Write};

use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;

mod util;
mod setup;

fn main() {
    let args = setup::Args::parse();
    dbg!(&args);
    let mut output = args.open_output().expect("Could not open output");

    let dwg = {
        let mut file = BufReader::new(fs::File::open(args.dxf).expect("Invalid file"));
        Drawing::load(&mut file).expect("Invalid dxf format")
    };

    for ent in dwg.entities() {
        writeln!(&mut output, "found ent on layer: {}", ent.common.layer).unwrap();

        match ent.specific {
            EntityType::Line(ref line) => {
                let (x1, y1, _) = line.p1.tuple();
                let (x2, y2, _) = line.p2.tuple();
                writeln!(&mut output, "Line {{ ({x1}, {y1}) , ({x2}, {y2}) }}\n").unwrap();
            }
            _ => todo!("Not yet implemented handle"),
        }
    }
}
