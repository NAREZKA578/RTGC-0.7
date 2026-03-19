//! Audio engine based on cpal

use std::sync::Arc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, Stream};
use nalgebra::Vector3;

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
#[derive(Debug, Clone, Copy)]
pub struct SoundHandle(u32);

/// Audio source parameters
#[derive(Debug, Clone)]
pub struct AudioSource {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub volume: f32,
    pub pitch: f32,
    pub is_looping: bool,
    pub max_distance: f32,
    pub rolloff_factor: f32,
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
        }
    }
}

/// Internal audio stream data
struct AudioStreamData {
    stream: Stream,
    is_playing: bool,
}

/// Audio engine
pub struct AudioEngine {
    config: AudioConfig,
    sources: Vec<(SoundHandle, AudioSource, Option<AudioStreamData>)>,
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
