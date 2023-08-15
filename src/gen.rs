use crate::setup::conf;
use std::io::{self, Write};

/// Writes preperation instructions into the gcode output.
pub fn setup_gcode(
    config: &conf::Config,
    output: &mut io::BufWriter<impl io::Write>,
) -> io::Result<()> {
    use gcode::{consts::Constants, move_z};

    writeln!(output, "%")?;

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
    output.write_all(b"G64 P0.1 Q0.02\n")?;


    // TODO: add to gen_code crate:
    writeln!(output, "M4 S{}", config.plunge_rate)?;

    // TODO: add to gen_code crate:
    // this gcode instruction sets analog output
    writeln!(output, "M68 E0 Q80\n")?;

    writeln!(
        output,
        ";
; Operation:    0
; Type:         Mill Cut
; Paths:        1
; Direction:    Conventional
; Rapid Z:      0
; Start Z:      0
; End Z:        -1
; Pass Depth:   1
; Plunge rate:  {} mm/min
; Cut rate:     {} mm/min
;
",
        config.plunge_rate,
        config.cut_rate
    )
    .unwrap();

    writeln!(output, "; Retract\n{}", move_z(0f32))?;

    Ok(())
}
