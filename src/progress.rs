use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub current_chapter: usize,
    pub completed_chapters: Vec<usize>,
    pub company_surplus: f64,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            current_chapter: 1,
            completed_chapters: Vec::new(),
            company_surplus: 100_000.0,
        }
    }
}

impl Progress {
    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let data = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    fn path() -> Result<PathBuf> {
        let dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("actuary");
        Ok(dir.join("progress.json"))
    }

    pub fn is_unlocked(&self, chapter: usize) -> bool {
        chapter == 1 || self.completed_chapters.contains(&(chapter - 1))
    }

    pub fn mark_complete(&mut self, chapter: usize) {
        if !self.completed_chapters.contains(&chapter) {
            self.completed_chapters.push(chapter);
        }
    }
}
