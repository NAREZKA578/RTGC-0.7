//! Audio engine based on cpal and symphonia for decoding

use std::sync::Arc;
use std::collections::HashMap;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, Stream};
use nalgebra::Vector3;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub doppler_factor: f32,
    pub listener_position: Vector3<f32>,
    pub listener_velocity: Vector3<f32>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            doppler_factor: 1.0,
            listener_position: Vector3::zeros(),
            listener_velocity: Vector3::zeros(),
        }
    }
}

/// Sound source handle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundHandle(u32);

/// Loaded sound data (decoded samples)
#[derive(Clone)]
pub struct LoadedSound {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Sound source parameters
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub volume: f32,
    pub pitch: f32,
    pub is_looping: bool,
    pub max_distance: f32,
    pub rolloff_factor: f32,
    pub sound_handle: Option<SoundHandle>,
}

impl Default for AudioSource {
    fn default() -> Self {
        Self {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            volume: 1.0,
            pitch: 1.0,
            is_looping: false,
            max_distance: 100.0,
            rolloff_factor: 1.0,
            sound_handle: None,
        }
    }
}

/// Internal audio stream data
struct AudioStreamData {
    stream: Stream,
    is_playing: bool,
    samples: Vec<f32>,
    sample_pos: usize,
    is_looping: bool,
}

/// Audio engine
pub struct AudioEngine {
    config: AudioConfig,
    sources: Vec<(SoundHandle, AudioSource, Option<AudioStreamData>)>,
    loaded_sounds: HashMap<SoundHandle, LoadedSound>,
    next_handle_id: u32,
    host: cpal::Host,
    device: cpal::Device,
    sample_rate: u32,
}

impl AudioEngine {
    /// Creates a new audio engine
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        
        let device = host
            .default_output_device()
            .ok_or_else(|| "No output device available".to_string())?;

        let default_config = device
            .default_output_config()
            .map_err(|e| format!("Failed to get default output config: {}", e))?;

        let sample_rate = default_config.sample_rate().0;

        Ok(Self {
            config: AudioConfig::default(),
            sources: Vec::new(),
            next_handle_id: 0,
            host,
            device,
            sample_rate,
        })
    }

    /// Creates a new audio engine with custom config
    pub fn with_config(config: AudioConfig) -> Result<Self, String> {
        let mut engine = Self::new()?;
        engine.config = config;
        Ok(engine)
    }

    /// Loads a sound from file (OGG/MP3/FLAC via symphonia)
    pub fn load_sound(&mut self, path: &str) -> Result<SoundHandle, String> {
        use std::fs::File;
        use std::io::BufReader;
        use symphonia::core::source::Source;
        
        // Open file
        let file = File::open(path)
            .map_err(|e| format!("Failed to open file {}: {}", path, e))?;
        let buf_reader = BufReader::new(file);
        
        // Create media source stream
        let mss = MediaSourceStream::new(Box::new(buf_reader));
        
        // Probe format
        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let registry = symphonia::default::get_codecs();
        let probe = symphonia::default::get_probe();
        
        let format = probe
            .read(&hint, mss, &format_opts)
            .map_err(|e| format!("Failed to probe format: {}", e))?
            .format;
        
        // Find first audio track
        let track_id = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .map(|t| t.id)
            .ok_or_else(|| "No audio track found".to_string())?;
        
        // Create decoder
        let track = format
            .tracks()
            .iter()
            .find(|t| t.id == track_id)
            .unwrap();
        
        let codec_params = &track.codec_params;
        let decoder = registry
            .make(codec_params)
            .map_err(|e| format!("Failed to create decoder: {}", e))?;
        
        // Decode all samples
        let mut samples = Vec::new();
        let sample_rate = codec_params.sample_rate.unwrap_or(44100);
        let channels = codec_params.channels.map(|c| c.count()).unwrap_or(2) as u16;
        
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() != track_id {
                continue;
            }
            
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    let spec = decoded.spec().unwrap();
                    let buf = decoded.buf();
                    
                    for frame in buf.iter() {
                        for sample in frame.iter() {
                            // Convert to f32 [-1.0, 1.0]
                            let normalized = *sample as f32 / i16::MAX as f32;
                            samples.push(normalized);
                        }
                    }
                }
                Err(_) => break,
            }
        }
        
        let handle = SoundHandle(self.next_handle_id);
        self.next_handle_id += 1;
        
        self.loaded_sounds.insert(handle, LoadedSound {
            samples,
            sample_rate,
            channels,
        });
        
        Ok(handle)
    }

    /// Plays a loaded sound and returns a source handle
    pub fn play_loaded_sound(&mut self, sound_handle: SoundHandle, source: AudioSource) -> SoundHandle {
        if !self.loaded_sounds.contains_key(&sound_handle) {
            return sound_handle; // Return original as error indicator
        }
        
        let handle = SoundHandle(self.next_handle_id);
        self.next_handle_id += 1;
        
        let mut source = source;
        source.sound_handle = Some(sound_handle);
        self.sources.push((handle, source, None));
        handle
    }

    /// Sets the pitch of a sound source (for engine RPM effect)
    pub fn set_pitch(&mut self, handle: SoundHandle, pitch: f32) {
        if let Some((_, source, _)) = self.sources.iter_mut().find(|(h, _, _)| *h == handle) {
            source.pitch = pitch.clamp(0.5, 2.0);
        }
    }

    /// Updates engine sound based on RPM
    pub fn update_engine_sound(
        &mut self,
        handle: SoundHandle,
        rpm: f32,
        max_rpm: f32
    ) {
        let pitch = 0.5 + (rpm / max_rpm) * 1.5; // pitch 0.5..2.0
        self.set_pitch(handle, pitch);
    }

    /// Plays a sound and returns a handle
    pub fn play_sound(&mut self, source: AudioSource) -> SoundHandle {
        let handle = SoundHandle(self.next_handle_id);
        self.next_handle_id += 1;
        self.sources.push((handle, source, None));
        handle
    }

    /// Stops a sound by handle
    pub fn stop_sound(&mut self, handle: SoundHandle) {
        if let Some(pos) = self.sources.iter().position(|(h, _, _)| *h == handle) {
            self.sources.remove(pos);
        }
    }

    /// Updates the position of a sound source
    pub fn set_source_position(&mut self, handle: SoundHandle, position: Vector3<f32>) {
        if let Some((_, source, _)) = self.sources.iter_mut().find(|(h, _, _)| *h == handle) {
            source.position = position;
        }
    }

    /// Updates the velocity of a sound source
    pub fn set_source_velocity(&mut self, handle: SoundHandle, velocity: Vector3<f32>) {
        if let Some((_, source, _)) = self.sources.iter_mut().find(|(h, _, _)| *h == handle) {
            source.velocity = velocity;
        }
    }

    /// Sets the volume of a sound source
    pub fn set_source_volume(&mut self, handle: SoundHandle, volume: f32) {
        if let Some((_, source, _)) = self.sources.iter_mut().find(|(h, _, _)| *h == handle) {
            source.volume = volume.clamp(0.0, 1.0);
        }
    }

    /// Sets whether a sound should loop
    pub fn set_source_looping(&mut self, handle: SoundHandle, looping: bool) {
        if let Some((_, source, _)) = self.sources.iter_mut().find(|(h, _, _)| *h == handle) {
            source.is_looping = looping;
        }
    }

    /// Updates the listener position
    pub fn set_listener_position(&mut self, position: Vector3<f32>) {
        self.config.listener_position = position;
    }

    /// Updates the listener velocity
    pub fn set_listener_velocity(&mut self, velocity: Vector3<f32>) {
        self.config.listener_velocity = velocity;
    }

    /// Sets the master volume
    pub fn set_master_volume(&mut self, volume: f32) {
        self.config.master_volume = volume.clamp(0.0, 1.0);
    }

    /// Calculates the attenuated volume based on distance and Doppler effect
    fn calculate_attenuation(&self, source: &AudioSource) -> f32 {
        let distance_vec = source.position - self.config.listener_position;
        let distance = distance_vec.norm();

        if distance > source.max_distance {
            return 0.0;
        }

        // Distance attenuation
        let attenuation = if distance < 1.0 {
            1.0
        } else {
            1.0 / (1.0 + source.rolloff_factor * (distance - 1.0))
        };

        // Doppler effect
        let relative_velocity = source.velocity - self.config.listener_velocity;
        let doppler_shift = if distance > 0.0 {
            let radial_velocity = distance_vec.dot(&relative_velocity) / distance;
            let speed_of_sound = 343.0; // m/s
            (speed_of_sound - radial_velocity * self.config.doppler_factor) / speed_of_sound
        } else {
            1.0
        };

        let doppler_factor = doppler_shift.clamp(0.5, 2.0);

        attenuation * source.volume * doppler_factor * self.config.master_volume
    }

    /// Updates all audio sources
    pub fn update(&mut self) {
        // Remove finished non-looping sounds
        self.sources.retain(|(_, source, stream_data)| {
            if let Some(data) = stream_data {
                if !data.is_playing && !source.is_looping {
                    return false;
                }
            }
            true
        });
    }

    /// Returns the number of active sound sources
    pub fn active_source_count(&self) -> usize {
        self.sources.len()
    }
}

impl Drop for AudioEngine {
    fn drop(&mut self) {
        // Stop all sounds
        self.sources.clear();
    }
}

/// Creates a default audio engine, returning None if unavailable
pub fn create_audio_engine() -> Option<AudioEngine> {
    AudioEngine::new().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_source_default() {
        let source = AudioSource::default();
        assert_eq!(source.volume, 1.0);
        assert_eq!(source.pitch, 1.0);
        assert!(!source.is_looping);
    }

    #[test]
    fn test_audio_config_default() {
        let config = AudioConfig::default();
        assert_eq!(config.master_volume, 1.0);
        assert_eq!(config.doppler_factor, 1.0);
    }
}
