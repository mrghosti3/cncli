#[derive(Debug)]
pub struct Config {
    pub plunge_rate: u32,
    pub cut_rate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            plunge_rate: 1000,
            cut_rate: 500,
        }
    }
}
