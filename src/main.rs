use std::fs;
use std::io::{BufReader, BufWriter, Write};

use clap::Parser;
use dxf::entities::{EntityType, Line};
use dxf::Drawing;
use libgcode::{g_codes as gc, m_codes as mc, utils as uc};

mod gen;
mod setup;
mod util;

fn main() {
    let args = setup::Args::parse();
    let config = args.get_config();
    let mut output = args.open_output().expect("Could not open output");

    let dwg = {
        let mut file = BufReader::new(fs::File::open(args.dxf).expect("Invalid file"));
        Drawing::load(&mut file).expect("Invalid dxf format")
    };

    gen::setup_gcode(&config, &mut output).expect("Could not write to output");

    for (i, ent) in dwg.entities().enumerate() {
        eprintln!("found ent on layer: {}", ent.common.layer);

        writeln!(&mut output, "\n; Path {}", i).unwrap();
        match ent.specific {
            EntityType::Line(ref line) => line_to_gcode(&mut output, &line),
            _ => todo!("Not yet implemented handle"),
        }
    }

    writeln!(
        &mut output,
        "{}          ; Switch tool offEnd\n%",
        mc::stop_select_spindle(uc::SpindleSelect::Default)
    )
    .unwrap();
}

fn line_to_gcode(output: &mut BufWriter<impl Write>, line: &Line) {
    let (x1, y1, _) = line.p1.tuple();
    let (x2, y2, _) = line.p2.tuple();
    eprintln!("Line {{ ({x1}, {y1}) , ({x2}, {y2}) }}");

    writeln!(
        output,
        "; Rapid to initial position\n{}",
        gc::move_xy(x1, y1)
    )
    .unwrap();

    // Make sure it is retracted.
    writeln!(output, "{}", gc::move_z(0.0)).unwrap();

    writeln!(
        output,
        "; plunge\n{} {}",
        gc::move_z_feed(-1.0, 1000.0.into()),
        uc::SpindleSpeed::new(10000, uc::SpindleSelect::Default)
    )
    .unwrap();

    writeln!(
        output,
        "; cut\n{} {}",
        gc::move_xy_feed(&(x2, y2), 500.0.into()),
        uc::SpindleSpeed::new(10000, uc::SpindleSelect::Default)
    )
    .unwrap();

    // Make sure it is retracted.
    writeln!(output, "; Retract\n{}", gc::move_z(0.0)).unwrap();
}
