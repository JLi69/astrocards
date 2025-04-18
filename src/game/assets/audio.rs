use super::open_file;
use crate::impfile;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, source::Buffered};
use std::{collections::HashMap, fs::File, io::BufReader};

type Sfx = Buffered<Decoder<BufReader<File>>>;

pub struct SfxPlayer {
    sources: HashMap<String, Sfx>,
    stream: Option<(OutputStream, OutputStreamHandle)>,
    muted: bool,
}

fn sfx_from_file(path: &str) -> Result<Sfx, String> {
    let file = BufReader::new(open_file(path)?);
    let decoder = Decoder::new(file).map_err(|e| e.to_string())?;
    Ok(decoder.buffered())
}

impl SfxPlayer {
    pub fn init() -> Self {
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => Self {
                sources: HashMap::new(),
                stream: Some((stream, stream_handle)),
                muted: false,
            },
            Err(msg) => {
                eprintln!("{msg}");
                Self {
                    sources: HashMap::new(),
                    stream: None,
                    muted: false,
                }
            }
        }
    }

    pub fn play(&self, id: &str) {
        if self.muted {
            return;
        }

        if let Some((_, stream_handle)) = &self.stream {
            let src = match self.sources.get(id) {
                Some(s) => s.clone(),
                None => {
                    eprintln!("No sfx id found: {id}");
                    return;
                }
            };
            let res = stream_handle.play_raw(src.convert_samples());
            if let Err(msg) = res {
                eprintln!("{msg}");
            }
        }
    }

    //Pass in the path to the impfile containining the audio metadata
    pub fn load_audio(&mut self, audio_impfile_path: &str) {
        let audio = impfile::find_impfile(audio_impfile_path);
        for entry in audio {
            let id = entry.get_name();
            let path = entry.get_var("path");

            if path.is_empty() {
                continue;
            }

            match sfx_from_file(&path) {
                Ok(sfx) => {
                    self.sources.insert(id, sfx);
                }
                Err(msg) => {
                    eprintln!("Failed to open file: {path}");
                    eprintln!("{msg}");
                }
            }
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted
    }

    pub fn muted(&self) -> bool {
        self.muted
    }
}
