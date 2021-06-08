/// An abstraction around a series file
use anyhow::Result;

const SERIES_FILENAME: &str = "series";

pub struct Series {
    filepath: std::path::PathBuf,
    patch_files: Vec<std::path::PathBuf>,
}

impl Series {
    pub fn new(directory: &std::path::PathBuf, patch_files: Vec<std::path::PathBuf>) -> Self {
        let mut filepath = directory.clone();
        filepath.push(SERIES_FILENAME);
        Series {
            filepath,
            patch_files,
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

    pub fn from_file(&mut self) -> Result<()> {
        let lines = std::fs::read_to_string(&self.filepath)?;

        for line in lines.split('\n') {
            self.patch_files.push(std::path::PathBuf::from(line));
        }

        Ok(())
    }
}
