use std::io::{BufReader, Write};
use std::{fs, io};

use clap::Parser;
use dxf::entities::EntityType;
use dxf::Drawing;

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

    setup_gcode(&config, &mut output).expect("Could not write to output");

    for ent in dwg.entities() {
        writeln!(&mut output, "found ent on layer: {}", ent.common.layer).unwrap();

        match ent.specific {
            EntityType::Line(ref line) => {
                let (x1, y1, _) = line.p1.tuple();
                let (x2, y2, _) = line.p2.tuple();
                writeln!(&mut output, "Line {{ ({x1}, {y1}) , ({x2}, {y2}) }}").unwrap();
            }
            _ => todo!("Not yet implemented handle"),
        }
    }
}

use setup::conf;

/// Writes preperation instructions into the gcode output.
fn setup_gcode(
    config: &conf::Config,
    output: &mut io::BufWriter<impl io::Write>,
) -> io::Result<()> {
    use gcode::consts::Constants;

    writeln!(
        output,
        "{}         ; Set units to mm",
        Constants::UseMillimeters.as_str()
    )?;
    writeln!(
        output,
        "{}         ; Absolute positioning",
        Constants::AbsolutePos.as_str()
    )?;

    // TODO: check/add to gen_code crate:
    output.write_all(b"G64P0.1Q0.02\n")?;
    // TODO: add to gen_code crate:
    output.write_all(b"M4 S1000\n")?;
    // TODO: add to gen_code crate:
    output.write_all(b"M68 E0 Q80\n")?;

    Ok(())
}
