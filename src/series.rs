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

    pub fn to_file(&self) -> Result<()> {
        let mut output: String = "".to_string();

        for file in &self.patch_files {
            output.push_str(format!("{}\n", file.display()).as_str());
        }

        std::fs::write(&self.filepath, output)?;

        Ok(())
    }

    pub fn from_paths(&mut self, paths: &[std::path::PathBuf]) -> Result<()> {
        self.patch_files = paths.to_owned();
        Ok(())
    }

    pub fn from_text(&mut self, text: &str) -> Result<()> {
        for line in text.split('\n') {
            // Remove everything past a comment character
            let mut_line = everything_before(line, "#").trim().to_string();

            if !mut_line.is_empty() {
                self.patch_files.push(std::path::PathBuf::from(mut_line));
            }
        }

        Ok(())
    }

    pub fn from_file(&mut self) -> Result<()> {
        let lines = std::fs::read_to_string(&self.filepath)?;

        self.from_text(lines.as_str())
    }

    pub fn patch_files(&self) -> &Vec<std::path::PathBuf> {
        &self.patch_files
    }
}
