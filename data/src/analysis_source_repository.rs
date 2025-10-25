use crate::analysis_source::AnalysisSource;
use std::fs;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

pub struct AnalysisSourceRepository {
    pub dir: String,
}

impl AnalysisSourceRepository {
    pub fn find_all(&self) -> Vec<AnalysisSource> {
        let start = Instant::now();
        let dir = Path::new(&self.dir);
        let walker = WalkDir::new(dir).into_iter();

        let mut sources = Vec::new();

        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_name() == "source.json" {
                let path = entry.path();

                match fs::read_to_string(path) {
                    Ok(content) => match serde_json::from_str::<AnalysisSource>(&content) {
                        Ok(source) => sources.push(source),
                        Err(err) => {
                            eprintln!("Failed to parse JSON in '{}': {}", path.display(), err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to read file '{}': {}", path.display(), err);
                    }
                }
            }
        }
        let ms = start.elapsed().as_millis();
        let count = sources.len();
        println!("Find all {count} for {ms}, ms");
        sources
    }
}
