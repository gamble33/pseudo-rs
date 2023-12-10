use super::obj::Obj;

#[derive(Clone, Copy)]
pub union Value {
    pub integer: i64,
    pub real: f64,
    pub boolean: bool,
    pub char: char,
    pub obj: *mut Obj,
}

impl Value {
    pub fn print_all_possible(&self) {
        unsafe {
            println!("--- VALUE ---");
            println!("integer: {}", self.integer);
            println!("real: {}", self.real);
            println!("boolean: {}", self.boolean);
            println!("char: {}", self.char);
            println!("object: n/a");
            println!("--- ----- ---");
        }
    }
}
