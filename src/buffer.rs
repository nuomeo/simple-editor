#[derive(Default)]
pub struct Buffer {
    data: Vec<String>,
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn load(filename: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(filename)?;
        let mut data = Vec::new();
        for line in contents.lines() {
            data.push(String::from(line));
        }
        Ok(Self { data })
    }
}