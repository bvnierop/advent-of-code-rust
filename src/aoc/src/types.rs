#[derive(Debug)]
pub struct Solver {
    pub year: u16,
    pub day: u8,
    pub level: u8,
    pub name: String,
    pub func: fn(&[&str]) -> String,
} 