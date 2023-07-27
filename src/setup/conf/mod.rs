pub mod machine;

#[derive(Debug)]
pub struct Config {
    enabled: bool,
    expanded: bool,
    filter_fill_color: Option<u8>,
    filter_stroke_color: Option<u8>,
    direction: &'static str,
    laser_power: u32,
    laser_power_range: { min: 0, max: 100 },
    laser_diameter: i32,
    tool_diameter: i32,
    line_distance: i32,
    line_angle: i32,
    margin: i32,
    passes: i32,
    cut_width: i32,
    tool_speed: u32,
    step_over: u32,
    pass_depth: u32,
    start_height: &'static str,
    mill_rapid_z: i32,
    mill_start_z: i32,
    mill_end_z: i32,
    segment_length: f32,
    tab_depth: i32,
    plunge_rate: i32,
    cut_rate: u32,
    over_scan: 0,
    tool_angle: 0,
    ramp: bool,
    use_a: bool,
    a_axis_diameter: 0,
    use_blower: bool,
    smoothing: bool,       // lw.raster-to-gcode: Smoothing the input image ?
    brightness: i16,          // lw.raster-to-gcode: Image brightness [-255 to +255]
    contrast: i16,            // lw.raster-to-gcode: Image contrast [-255 to +255]
    gamma: 0,               // lw.raster-to-gcode: Image gamma correction [0.01 to 7.99]
    grayscale: Graysale,      // lw.raster-to-gcode: Graysale algorithm [none, average, luma, luma-601, luma-709, luma-240, desaturation, decomposition-[min|max], [red|green|blue]-chanel]
    shades_of_gray: u16,      // lw.raster-to-gcode: Number of shades of gray [2-256]
    invert_color: bool,     // lw.raster-to-gcode
    trim_line: bool,         // lw.raster-to-gcode: Trim trailing white pixels
    join_pixel: bool,        // lw.raster-to-gcode: Join consecutive pixels with same intensity
    burn_white: bool,        // lw.raster-to-gcode: [true = G1 S0 | false = G0] on inner white pixels
    verbose_gcode: bool,    // lw.raster-to-gcode: Output verbose GCode (print each commands)
    diagonal: bool,        // lw.raster-to-gcode: Go diagonally (increase the distance between points)
    dithering: bool,       // lw.raster-to-gcode: Floyd Steinberg dithering
    lathe_tool_back_side: bool,
    lathe_rapid_to_diameter: 0,
    lathe_rapid_to_z: i32,
    lathe_start_z: i32,
    lathe_roughing_feed: 0,
    lathe_roughing_depth: 0,
    lathe_finish_feed: 0,
    lathe_finish_depth: 0,
    lathe_finish_extra_passes: 0,
    lathe_face: bool,
    lathe_face_end_diameter: 0,
    lathe_turns: [],
    _docs_visible: bool,
    // Hooks!
    // hookOperationStart: '',
    // hookOperationEnd: '',
    // hookPassStart: '',
    // hookPassEnd: ''
}

enum Graysale {
    None,
    Average,
    Luma,
    Luma601,
    Luma709,
    Luma240,
    Desaturation,
    DecompositionMin,
    DecompositionMax,
    RedChanel,
    GreenChanel,
    BlueChanel
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tool_speed: 1000,
            mill_end_z: -1,

            enabled: true,
            expanded: false,
            filter_fill_color: None,
            filter_stroke_color: None,
            direction: 'Conventional',
            laserPower: 100,
            laserPowerRange: { min: 0, max: 100 },
            laserDiameter: 0,
            toolDiameter: 0,
            lineDistance: 0,
            lineAngle: 0,
            margin: 0,
            passes: 1,
            cutWidth: 0,
            toolSpeed: 10000,
            stepOver: 40,
            passDepth: 1,
            startHeight: '',
            millRapidZ: 0,
            millStartZ: 0,
            millEndZ: -1,
            segmentLength: 0.1,
            tabDepth: 0,
            plungeRate: 1000,
            cutRate: 500,
            overScan: 0,
            toolAngle: 0,
            ramp: false,
            use_a: false,
            aAxisDiameter: 0,
            useBlower: false,
            smoothing: false,          // lw.raster-to-gcode: Smoothing the input image ?
            brightness: 0,             // lw.raster-to-gcode: Image brightness [-255 to +255]
            contrast: 0,               // lw.raster-to-gcode: Image contrast [-255 to +255]
            gamma: 0,                  // lw.raster-to-gcode: Image gamma correction [0.01 to 7.99]
            grayscale: Graysale::None, // lw.raster-to-gcode: Graysale algorithm [none, average, luma, luma-601, luma-709, luma-240, desaturation, decomposition-[min|max], [red|green|blue]-chanel]
            shades_of_gray: 256,       // lw.raster-to-gcode: Number of shades of gray [2-256]
            invertColor: false,        // lw.raster-to-gcode
            trim_line: true,           // lw.raster-to-gcode: Trim trailing white pixels
            join_pixel: true,          // lw.raster-to-gcode: Join consecutive pixels with same intensity
            burn_white: true,          // lw.raster-to-gcode: [true = G1 S0 | false = G0] on inner white pixels
            verbose_gcode: false,      // lw.raster-to-gcode: Output verbose GCode (print each commands)
            diagonal: false,           // lw.raster-to-gcode: Go diagonally (increase the distance between points)
            dithering: false,          // lw.raster-to-gcode: Floyd Steinberg dithering
            lathe_tool_back_side: false,
            lathe_rapid_to_diameter: 0,
            lathe_rapid_to_z: 0,
            lathe_start_z: 0,
            lathe_roughing_feed: 0,
            lathe_roughing_depth: 0,
            lathe_finish_feed: 0,
            lathe_finish_depth: 0,
            lathe_finish_extra_passes: 0,
            lathe_face: true,
            lathe_face_end_diameter: 0,
            latheTurns: [],
            _docs_visible: true,
            // Hooks!
            hookOperationStart: '',
            hookOperationEnd: '',
            hookPassStart: '',
            hookPassEnd: ''
        }
    }
}
