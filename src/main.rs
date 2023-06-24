use std::env;

use dxf::entities::EntityType;
use dxf::Drawing;

fn main() {
    let file_name = env::args().last().expect("File not specified");
    let dwg = Drawing::load_file(file_name).expect("Invalid file");

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
