/// An abstraction around a series file
use anyhow::Result;

use crate::string::*;

const SERIES_FILENAME: &str = "series";

pub struct Series {
    filepath: std::path::PathBuf,
    patch_files: Vec<std::path::PathBuf>,
}

impl Series {
    pub fn new(directory: &std::path::Path) -> Self {
        let mut filepath = directory.to_path_buf();
        filepath.push(SERIES_FILENAME);
        Series {
            filepath,
            patch_files: vec![],
        }
    }

    pub fn persist(&self) -> Result<()> {
        let mut output: String = "".to_string();

        for file in &self.patch_files {
            output.push_str(format!("{}\n", file.display()).as_str());
        }

        std::fs::write(&self.filepath, output)?;

        Ok(())
    }

    pub fn set_patch_files(&mut self, paths: &[std::path::PathBuf]) -> Result<()> {
        self.patch_files = paths.to_owned();
        Ok(())
    }

    pub fn populate_from_text(&mut self, text: &str) -> Result<()> {
        for line in text.split('\n') {
            // Remove everything past a comment character
            let mut_line = everything_before(line, "#").trim().to_string();

            if !mut_line.is_empty() {
                self.patch_files.push(std::path::PathBuf::from(mut_line));
            }
        }

        Ok(())
    }

    pub fn populate(&mut self) -> Result<()> {
        if let Ok(lines) = std::fs::read_to_string(&self.filepath) {
            self.populate_from_text(lines.as_str())
        } else {
            Err(anyhow::anyhow!(
                "unable to read series file: {:?}",
                self.filepath
            ))
        }
    }

    pub fn patch_files(&self) -> &Vec<std::path::PathBuf> {
        &self.patch_files
    }
}
