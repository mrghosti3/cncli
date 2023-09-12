use std::fs;
use std::io::{BufReader, Write};

use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;
use libgcode::gen_fn as gc;

mod gen;
mod setup;
mod util;

fn main() {
    let args = setup::Args::parse();
    dbg!(&args);
    let config = args.get_config();
    let mut output = args.open_output().expect("Could not open output");

    let dwg = {
        let mut file = BufReader::new(fs::File::open(args.dxf).expect("Invalid file"));
        Drawing::load(&mut file).expect("Invalid dxf format")
    };

    gen::setup_gcode(&config, &mut output).expect("Could not write to output");

    for ent in dwg.entities() {
        eprintln!("found ent on layer: {}", ent.common.layer);

        match ent.specific {
            EntityType::Line(ref line) => {
                let (x1, y1, _) = line.p1.tuple();
                let (x2, y2, _) = line.p2.tuple();
                eprintln!("Line {{ ({x1}, {y1}) , ({x2}, {y2}) }}");

                writeln!(
                    &mut output,
                    "; Path 0\n; Rapid to initial position\n{}",
                    gc::move_xy(x1, y1)
                )
                .unwrap();

                // Make sure it is retracted.
                writeln!(&mut output, "{}", gc::move_z(0.0)).unwrap();
            }
            _ => todo!("Not yet implemented handle"),
        }
    }
}
