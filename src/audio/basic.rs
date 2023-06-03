use crate::Engine;
use kira::manager::AudioManagerSettings;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};

#[derive(Clone)]
pub struct AudioSource {
    volume: f64,
    playback_rate: f64,
    panning: f64,
    path: String,
}

pub struct AudioSourceBuilder {
    volume: f64,
    playback_rate: f64,
    panning: f64,
    path: String,
}

impl AudioSourceBuilder {
    pub fn new() -> Self {
        AudioSourceBuilder {
            volume: 1.0,
            playback_rate: 1.0,
            panning: 0.5,
            path: "".to_string(),
        }
    }
    pub fn volume(mut self, volume: f64) -> Self {
        self.volume = volume;
        self
    }
    pub fn rate(mut self, rate: f64) -> Self {
        self.playback_rate = rate;
        self
    }
    pub fn pan(mut self, panning: f64) -> Self {
        self.panning = panning;
        self
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = path.to_string();
        self
    }
    pub fn build(self) -> AudioSource {
        AudioSource {
            volume: self.volume,
            playback_rate: self.playback_rate,
            panning: self.panning,
            path: self.path,
        }
    }
}

impl Engine {
    pub fn play_audio(&mut self, source: AudioSource) {
        // let settings = StaticSoundSettings::default()
        //     .volume(source.volume)
        //     .panning(source.panning)
        //     .playback_rate(source.playback_rate);
        // let sound_data = StaticSoundData::from_file(source.path, settings).unwrap();
        // self.audio_manager.play(sound_data).unwrap();
    }
}
