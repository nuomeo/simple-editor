pub struct Buffer {
    data: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer { data: vec!["hello world".to_string(), "this is a test buffer".to_string()] }
    }
}

impl Buffer {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&self, index: usize) -> String{
        match self.data.get(index) {
            Option::Some(string) => {
                string.to_string()
            }
            _ => "hello, world".to_string() // TODO: handle this better
        }
    }
}