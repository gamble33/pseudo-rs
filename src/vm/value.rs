// pub type Value = i32;

#[derive(Clone, Copy)]
pub union Value {
    pub integer: i32,
    pub real: f64,
    pub boolean: bool,
    pub char: char,
}
