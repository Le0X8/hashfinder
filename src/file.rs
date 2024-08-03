use std::io::{Read, Seek};

// Parts of this helper struct are taken from @acridotheres/core.

pub struct File {
    file: std::fs::File,
    pos: u64,
}

impl File {
    pub fn new(input: &String) -> Self {
        Self {
            file: std::fs::OpenOptions::new()
                .read(true)
                .open(input)
                .unwrap(),
            pos: 0,
        }
    }

    pub fn seek(&mut self, pos: &u64) {
        self.file.seek(std::io::SeekFrom::Start(*pos)).unwrap();
        self.pos = *pos;
    }

    pub fn rewind(&mut self) {
        self.seek(&0);
    }

    pub fn read<'a>(&mut self, buf: &'a mut [u8]) -> &'a mut [u8] {
        let _ = self.file.read_exact(buf);
        self.pos += buf.len() as u64;
        buf
    }

    pub fn get_position(&self) -> u64 {
        self.pos
    }

    pub fn get_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }
}

