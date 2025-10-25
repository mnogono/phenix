use crate::analysis_source::AnalysisSource;
use rayon::prelude::*;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Instant;

pub struct AnalysisSourceRepository {
    pub dir: String,
}

impl AnalysisSourceRepository {
    pub fn find_all(&self) -> Vec<AnalysisSource> {
        let start = Instant::now();
        let dir = Path::new(&self.dir);

        // Read directory entries once
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Failed to read directory '{}': {}", dir.display(), err);
                return Vec::new();
            }
        };

        // Collect all source.json paths
        let paths: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                // Quick check: if it's likely a directory (UUID folder)
                let path = entry.path();
                if path.is_dir() {
                    let source_path = path.join("source.json");
                    if source_path.exists() {
                        Some(source_path)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        println!("Found {} potential source files", paths.len());

        // Process files in parallel
        let sources: Vec<AnalysisSource> = paths
            .par_iter()
            .filter_map(|path| match File::open(path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    match serde_json::from_reader(reader) {
                        Ok(source) => Some(source),
                        Err(err) => {
                            eprintln!("Failed to parse JSON in '{}': {}", path.display(), err);
                            None
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read file '{}': {}", path.display(), err);
                    None
                }
            })
            .collect();

        let ms = start.elapsed().as_millis();
        let count = sources.len();
        println!("Found {} analysis sources in {} ms", count, ms);
        sources
    }
}
