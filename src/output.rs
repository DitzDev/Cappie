use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;

pub trait Output: Send + Sync {
    fn write(&self, message: &str);
}

pub struct StdoutOutput;

impl Output for StdoutOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct StderrOutput;

impl Output for StderrOutput {
    fn write(&self, message: &str) {
        eprintln!("{}", message);
    }
}

pub struct FileOutput {
    path: String,
}

impl FileOutput {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().to_string(),
        }
    }
}

impl Output for FileOutput {
    fn write(&self, message: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path) 
        {
            let _ = writeln!(file, "{}", message);
        }
    }
}

pub struct MultiOutput {
    outputs: Vec<Box<dyn Output>>,
}

impl MultiOutput {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
        }
    }
    
    pub fn add_output(mut self, output: Box<dyn Output>) -> Self {
        self.outputs.push(output);
        self
    }
}

impl Output for MultiOutput {
    fn write(&self, message: &str) {
        for output in &self.outputs {
            output.write(message);
        }
    }
}