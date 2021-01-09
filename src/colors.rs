pub enum Colors {
    GridColor,
    DeadColor,
    AliveColor,
}


impl Colors {
    pub fn to_hex_string(&self) -> &str {
        match self {
            Colors::GridColor => { "#CCCCCC" }
            Colors::DeadColor => { "#FFFFFF" }
            Colors::AliveColor => { "#000000" }
        }
    }
}