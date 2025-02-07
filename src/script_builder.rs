use std::{fs::File, io::{self, Write}};

#[derive(Default)]
pub struct ScriptBuilder {
    script: Vec<u8>,
}

impl ScriptBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_instruction(&mut self, instruction: u8) {
        self.script.push(instruction);
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        self.script.extend_from_slice(bytes);
    }

    pub fn save(&self, at: &str) -> io::Result<()> {
        let mut file = File::create(at)?;
        file.write_all(&self.script)?;
        file.flush()
    }

    pub fn get_binary(&self) -> &[u8] {
        self.script.as_slice()
    }
}