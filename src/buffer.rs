use std::io::Error;

#[derive(Debug)]
pub struct Buffer {
    pub id: u16,
    pub filepath: String,
    pub data: String,
    pub modified: bool,
}

impl Buffer {
    /// Create a new buffer
    pub fn create(id: u16, filepath: impl Into<String> + Copy) -> Result<Buffer, Error> {
        match std::fs::read_to_string(filepath.into()) {
            Ok(data) => Ok(Buffer {
                id,
                filepath: filepath.into(),
                data,
                modified: false,
            }),
            Err(_) => Err(Error::new(std::io::ErrorKind::Other, "Could not read file")),
        }
    }

    /// Insert text at a specific line
    pub fn insert(&mut self, text: impl Into<String> + Copy, line_idx: isize) {
        let lines = self.data.lines().collect::<Vec<&str>>();
        let mut new_data = String::new();
        if line_idx == -1 {
            new_data.push_str(&text.into());
            new_data.push('\n');
            new_data.push_str(&self.data);
            self.data = new_data;
            self.modified = true;
            return;
        }

        if line_idx as usize >= lines.len() {
            let diff = line_idx as usize - lines.len();

            new_data.push_str(&self.data);
            for _ in 0..diff {
                new_data.push('\n');
            }
            new_data.push_str(&text.into());
            new_data.push('\n');
            self.data = new_data;
            self.modified = true;
            return;
        }

        for (i, line) in lines.iter().enumerate() {
            if i == line_idx as usize {
                new_data.push_str(&text.into());
                self.modified = true;
            } else {
                new_data.push_str(line);
            }
            new_data.push('\n');
        }
        new_data = new_data.trim().to_string();
        new_data.push('\n');
        self.data = new_data;
    }

    /// Save the buffer to disk
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if !self.modified {
            println!("Buffer {} is not modified", self.id);
            return Ok(());
        }
        match std::fs::write(&self.filepath, &self.data) {
            Ok(_) => {
                self.modified = false;
                Ok(())
            }
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write to file",
            )),
        }
    }
    /// Print the buffer
    pub fn print(&self) {
        for (idx, line) in self.data.lines().enumerate() {
            println!("{}│ {}", idx, line);
        }
    }
}
