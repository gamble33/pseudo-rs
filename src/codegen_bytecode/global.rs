use super::Generator;

impl Generator<'_> {
    pub fn declare_global(&mut self, name: String,) {
        self.globals.insert(name, self.globals.len());
    }

    pub fn resolve_global(&self, name: &str) -> usize {
        *self.globals.get(name).unwrap()
    }
}
