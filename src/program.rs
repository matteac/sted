use crate::buffer::Buffer;

#[derive(Debug)]
pub struct Program {
    buffers: Vec<Buffer>,
    pub current_buffer_id: Option<u16>,
}

#[derive(Debug)]
pub enum Identifier {
    Id(u16),
    Path(String),
}

#[allow(dead_code)]
impl Program {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
            current_buffer_id: None,
        }
    }
    pub fn open(&mut self, filepath: impl Into<String> + Copy) -> Result<(), std::io::Error> {
        for buffer in &self.buffers {
            if buffer.filepath == filepath.into() {
                self.current_buffer_id = Some(buffer.id);
                return Ok(());
            }
        }
        let buffer = match Buffer::create(self.create_id(), filepath) {
            Ok(b) => b,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not read file",
                ))
            }
        };
        self.current_buffer_id = Some(buffer.id);
        self.buffers.push(buffer);
        Ok(())
    }
    pub fn close(&mut self, identifier: Identifier) {
        match identifier {
            Identifier::Id(id) => {
                self.buffers.retain(|b| b.id != id);
            }
            Identifier::Path(path) => {
                self.buffers.retain(|b| b.filepath != path);
            }
        }
        self.current_buffer_id = None;
    }
    pub fn get_buffer(&mut self, identifier: Identifier) -> Option<&mut Buffer> {
        match identifier {
            Identifier::Id(id) => self.buffers.iter_mut().find(|b| b.id == id),
            Identifier::Path(path) => self.buffers.iter_mut().find(|b| b.filepath == path),
        }
    }
    pub fn list_buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }
    pub fn focus(&mut self, identifier: Identifier) {
        self.current_buffer_id = match self.get_buffer(identifier) {
            Some(b) => Some(b.id),
            None => {
                eprintln!("\x1b[31mBuffer not found\x1b[0m");
                self.current_buffer_id
            }
        };
    }
    pub fn insert(&mut self, text: impl Into<String> + Copy, line_idx: isize) {
        match self.current_buffer_id {
            Some(id) => match self.get_buffer(Identifier::Id(id)) {
                Some(b) => {
                    b.insert(text, line_idx);
                }
                None => {
                    eprintln!("\x1b[31mBuffer {} not found\x1b[0m", id);
                }
            },
            None => {
                eprintln!("\x1b[31mNo buffer focused\x1b[0m");
            }
        }
    }
    pub fn save(&mut self) {
        match self.current_buffer_id {
            Some(id) => match self.get_buffer(Identifier::Id(id)) {
                Some(b) => match b.save() {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("\x1b[31mCould not save buffer {}\x1b[0m", id);
                    }
                },
                None => {
                    eprintln!("\x1b[31mBuffer {} not found\x1b[0m", id);
                }
            },
            None => {
                eprintln!("\x1b[31mNo buffer focused\x1b[0m");
            }
        }
    }
    pub fn save_all(&mut self) {
        for b in self.buffers.iter_mut() {
            match b.save() {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("\x1b[31mCould not save buffer {}\x1b[0m", b.id);
                }
            }
        }
    }
    pub fn print(&mut self) {
        match self.current_buffer_id {
            Some(id) => match self.get_buffer(Identifier::Id(id)) {
                Some(b) => {
                    b.print();
                }
                None => {
                    eprintln!("\x1b[31mBuffer {} not found\x1b[0m", id);
                }
            },
            None => {
                eprintln!("\x1b[31mNo buffer focused\x1b[0m");
            }
        }
    }
    fn create_id(&self) -> u16 {
        for id in 0..std::u16::MAX {
            if !self.buffers.iter().any(|b| b.id == id) {
                return id;
            }
        }
        0
    }
}
