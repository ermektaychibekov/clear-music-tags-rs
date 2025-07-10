// src/tag_processor.rs
use crate::config::Config;
use id3::{Content, Tag, TagLike, Version};
use log::{info, warn};
use std::path::Path;

pub struct TagProcessor {
    config: Config,
}

impl TagProcessor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn process_file(&self, file_path: &Path) {
        let tag = match Tag::read_from_path(file_path) {
            Ok(t) => t,
            Err(e) if e.kind == id3::ErrorKind::NoTag => return,
            Err(e) => {
                warn!("Error reading {}: {}", file_path.display(), e);
                return;
            }
        };

        let mut frames_to_remove = Vec::new();
        for frame in tag.frames() {
            if let Some(text) = self.extract_frame_text(frame.content()) {
                if self.config.remove_strings.iter().any(|s| s == text.trim()) {
                    frames_to_remove.push(frame.id().to_string());
                }
            }
        }

        if frames_to_remove.is_empty() {
            return;
        }

        let mut new_tag = Tag::new();
        for frame in tag.frames() {
            if !frames_to_remove.contains(&frame.id().to_string()) {
                new_tag.add_frame(frame.clone());
            }
        }

        if let Err(e) = new_tag.write_to_path(file_path, Version::Id3v24) {
            warn!("Error writing {}: {}", file_path.display(), e);
        } else {
            info!("Processed: {}", file_path.display());
        }
    }

    fn extract_frame_text<'a>(&self, content: &'a Content) -> Option<&'a str> {
        match content {
            Content::Text(s) => Some(s),
            Content::Comment(c) => Some(&c.text),
            Content::UnsynchronisedText(uslt) => Some(&uslt.text),
            _ => None,
        }
    }
}
