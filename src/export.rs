use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub start: u32,
    pub length: String,
    pub pitch: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub track: String,
    pub notes: Vec<Note>,
}

impl Output {
    pub fn new(track: String) -> Self {
        Self {
            track,
            notes: Vec::new(),
        }
    }

    pub fn add_note(&mut self, start: u32, length: String, pitch: String) {
        self.notes.push(Note {
            start,
            length,
            pitch,
        });
    }

    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let yaml = self.to_yaml().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })?;

        let mut file = File::create(path)?;
        file.write_all(yaml.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_creation() {
        let mut output = Output::new("Bass".to_string());
        output.add_note(0, "quarter".to_string(), "E2".to_string());
        output.add_note(480, "eighth".to_string(), "F#2".to_string());

        assert_eq!(output.track, "Bass");
        assert_eq!(output.notes.len(), 2);
        assert_eq!(output.notes[0].pitch, "E2");
    }

    #[test]
    fn test_to_yaml() {
        let mut output = Output::new("Bass".to_string());
        output.add_note(0, "quarter".to_string(), "E2".to_string());

        let yaml = output.to_yaml().unwrap();
        assert!(yaml.contains("track: Bass"));
        assert!(yaml.contains("start: 0"));
        assert!(yaml.contains("pitch: E2"));
    }
}
