use std::fs;
use std::io::BufReader;

use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;

mod setup;

fn main() {
    let args = setup::Args::parse();
    dbg!(&args);
    let dwg = {
        let mut file = BufReader::new(fs::File::open(args.dxf).expect("Invalid file"));
        Drawing::load(&mut file).expect("Invalid dxf format")
    };

    for ent in dwg.entities() {
        println!("found ent on layer: {}", ent.common.layer);

        match ent.specific {
            EntityType::Line(ref line) => {
                let (x1, y1, _) = line.p1.tuple();
                let (x2, y2, _) = line.p2.tuple();
                println!("Line {{ ({x1}, {y1}) , ({x2}, {y2}) }}")
            }
            _ => todo!("Not yet implemented handle")
        }
    }
}
