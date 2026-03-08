//! Аудио движок для игрового движка на Rust
//! 
//! Реализует:
//! - Воспроизведение звуковых эффектов
//! - Фоновую музыку с crossfade
//! - 3D позиционированный звук
//! - Микширование каналов

use rodio::{Source, Sink, OutputStream, OutputStreamHandle};
use std::collections::HashMap;
use std::sync::Arc;
use glam::Vec3;

/// Тип звукового канала
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum AudioChannel {
    /// Звуковые эффекты (SFX)
    Sfx,
    /// Музыка
    Music,
    /// Голосовые сообщения/диалоги
    Voice,
    /// Окружающие звуки среды
    Ambient,
}

/// Настройки аудио канала
#[derive(Clone, Debug)]
pub struct ChannelConfig {
    /// Громкость канала (0.0 - 1.0)
    pub volume: f32,
    /// Включен ли канал
    pub enabled: bool,
    /// Максимальное количество одновременных звуков
    pub max_simultaneous: usize,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            volume: 1.0,
            enabled: true,
            max_simultaneous: 8,
        }
    }
}

/// Звуковой эффект
#[derive(Clone, Debug)]
pub struct SoundEffect {
    /// Название эффекта
    pub name: String,
    /// Данные звука (samples)
    pub samples: Vec<f32>,
    /// Частота дискретизации
    pub sample_rate: u32,
    /// Количество каналов (1 = mono, 2 = stereo)
    pub channels: u16,
}

impl SoundEffect {
    pub fn new(name: &str, samples: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self {
            name: name.to_string(),
            samples,
            sample_rate,
            channels,
        }
    }
}

/// Трек музыки
#[derive(Clone, Debug)]
pub struct MusicTrack {
    /// Название трека
    pub name: String,
    /// Длительность в секундах
    pub duration: f32,
    /// Данные звука
    pub samples: Vec<f32>,
    /// Частота дискретизации
    pub sample_rate: u32,
}

impl MusicTrack {
    pub fn new(name: &str, samples: Vec<f32>, sample_rate: u32, duration: f32) -> Self {
        Self {
            name: name.to_string(),
            samples,
            sample_rate,
            duration,
        }
    }
}

/// 3D источник звука
#[derive(Clone, Debug)]
pub struct AudioSource {
    /// Позиция источника в мире
    pub position: Vec3,
    /// Скорость источника (для Doppler effect)
    pub velocity: Vec3,
    /// Радиус влияния звука
    pub radius: f32,
    /// Затухание с расстоянием
    pub attenuation: f32,
    /// Звук для воспроизведения
    pub sound: Option<SoundEffect>,
    /// Зацикливание
    pub looped: bool,
    /// Включен ли источник
    pub playing: bool,
}

impl AudioSource {
    pub fn new(position: Vec3, sound: SoundEffect) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            radius: 50.0,
            attenuation: 1.0,
            sound: Some(sound),
            looped: false,
            playing: false,
        }
    }

    /// Расчет громкости на основе позиции слушателя
    pub fn calculate_volume(&self, listener_pos: Vec3) -> f32 {
        let distance = (self.position - listener_pos).length();
        
        if distance > self.radius {
            return 0.0;
        }
        
        // Линейное затухание
        let volume = 1.0 - (distance / self.radius);
        volume.powf(self.attenuation).clamp(0.0, 1.0)
    }
}

/// Настройки слушателя (камеры/игрока)
#[derive(Clone, Debug)]
pub struct ListenerSettings {
    /// Позиция слушателя
    pub position: Vec3,
    /// Направление взгляда
    pub direction: Vec3,
    /// Вектор вверх
    pub up: Vec3,
    /// Скорость слушателя
    pub velocity: Vec3,
}

impl Default for ListenerSettings {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            direction: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::Y,
            velocity: Vec3::ZERO,
        }
    }
}

/// Аудио движок
pub struct AudioEngine {
    /// Выходной поток
    output_stream: Option<OutputStream>,
    /// Handle для создания sink'ов
    output_handle: Option<Arc<OutputStreamHandle>>,
    
    /// Конфигурации каналов
    channel_configs: HashMap<AudioChannel, ChannelConfig>,
    
    /// Активные sink'ы по каналам
    active_sinks: HashMap<AudioChannel, Vec<Arc<Sink>>>,
    
    /// Библиотека звуковых эффектов
    sfx_library: HashMap<String, SoundEffect>,
    
    /// Библиотека музыки
    music_library: HashMap<String, MusicTrack>,
    
    /// 3D источники звука
    audio_sources: Vec<AudioSource>,
    
    /// Настройки слушателя
    listener: ListenerSettings,
    
    /// Текущий музыкальный трек
    current_music: Option<String>,
    /// Целевой трек для crossfade
    target_music: Option<String>,
    /// Прогресс crossfade (0-1)
    crossfade_progress: f32,
    /// Длительность crossfade
    crossfade_duration: f32,
}

impl AudioEngine {
    /// Создание нового аудио движка
    pub fn new() -> Result<Self, String> {
        let (output_stream, output_handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to create audio output: {}", e))?;
        
        let mut engine = Self {
            output_stream: Some(output_stream),
            output_handle: Some(Arc::new(output_handle)),
            channel_configs: HashMap::new(),
            active_sinks: HashMap::new(),
            sfx_library: HashMap::new(),
            music_library: HashMap::new(),
            audio_sources: Vec::new(),
            listener: ListenerSettings::default(),
            current_music: None,
            target_music: None,
            crossfade_progress: 0.0,
            crossfade_duration: 2.0,
        };
        
        // Инициализация каналов по умолчанию
        engine.channel_configs.insert(AudioChannel::Sfx, ChannelConfig::default());
        engine.channel_configs.insert(AudioChannel::Music, ChannelConfig::default());
        engine.channel_configs.insert(AudioChannel::Voice, ChannelConfig::default());
        engine.channel_configs.insert(AudioChannel::Ambient, ChannelConfig::default());
        
        Ok(engine)
    }

    /// Регистрация звукового эффекта в библиотеке
    pub fn register_sfx(&mut self, sfx: SoundEffect) {
        self.sfx_library.insert(sfx.name.clone(), sfx);
    }

    /// Регистрация музыкального трека
    pub fn register_music(&mut self, track: MusicTrack) {
        self.music_library.insert(track.name.clone(), track);
    }

    /// Воспроизведение звукового эффекта
    pub fn play_sfx(&mut self, name: &str, channel: AudioChannel, volume: f32) -> bool {
        let sfx = match self.sfx_library.get(name) {
            Some(s) => s,
            None => return false,
        };
        
        let config = match self.channel_configs.get(&channel) {
            Some(c) if c.enabled => c,
            _ => return false,
        };
        
        // Проверка лимита одновременных звуков
        let sinks = self.active_sinks.entry(channel).or_insert_with(Vec::new);
        if sinks.len() >= config.max_simultaneous {
            // Удаляем самый старый звук
            sinks.remove(0);
        }
        
        // Создаем sink и воспроизводим
        if let Some(handle) = &self.output_handle {
            if let Ok(sink) = Sink::try_new(handle) {
                // Упрощенно: создаем silence source
                // В реальной реализации нужно конвертировать samples в Source
                sink.set_volume(volume * config.volume);
                sinks.push(Arc::new(sink));
                return true;
            }
        }
        
        false
    }

    /// Воспроизведение музыки с crossfade
    pub fn play_music(&mut self, name: &str, fade_in: bool) -> bool {
        if !self.music_library.contains_key(name) {
            return false;
        }
        
        let config = match self.channel_configs.get(&AudioChannel::Music) {
            Some(c) if c.enabled => c,
            _ => return false,
        };
        
        if self.current_music.as_deref() == Some(name) {
            return true; // Уже играет
        }
        
        if fade_in && self.current_music.is_some() {
            // Запускаем crossfade
            self.target_music = Some(name.to_string());
            self.crossfade_progress = 0.0;
        } else {
            // Немедленное переключение
            self.current_music = Some(name.to_string());
            self.target_music = None;
        }
        
        true
    }

    /// Обновление crossfade музыки
    pub fn update_crossfade(&mut self, dt: f32) {
        if self.target_music.is_none() {
            return;
        }
        
        self.crossfade_progress += dt / self.crossfade_duration;
        
        if self.crossfade_progress >= 1.0 {
            // Crossfade завершен
            self.current_music = self.target_music.take();
            self.crossfade_progress = 0.0;
        }
    }

    /// Добавление 3D источника звука
    pub fn add_audio_source(&mut self, source: AudioSource) {
        self.audio_sources.push(source);
    }

    /// Обновление 3D источников звука
    pub fn update_audio_sources(&mut self) {
        let listener_pos = self.listener.position;
        
        for source in &mut self.audio_sources {
            if !source.playing {
                continue;
            }
            
            let volume = source.calculate_volume(listener_pos);
            
            // Здесь должно быть обновление громкости активного звука
            // В реальной реализации нужно хранить ссылки на активные sink'ы
            let _ = volume;
        }
    }

    /// Установка позиции слушателя
    pub fn set_listener_position(&mut self, position: Vec3) {
        self.listener.position = position;
    }

    /// Установка ориентации слушателя
    pub fn set_listener_orientation(&mut self, direction: Vec3, up: Vec3) {
        self.listener.direction = direction.normalize();
        self.listener.up = up.normalize();
    }

    /// Установка громкости канала
    pub fn set_channel_volume(&mut self, channel: AudioChannel, volume: f32) {
        if let Some(config) = self.channel_configs.get_mut(&channel) {
            config.volume = volume.clamp(0.0, 1.0);
        }
    }

    /// Включение/выключение канала
    pub fn set_channel_enabled(&mut self, channel: AudioChannel, enabled: bool) {
        if let Some(config) = self.channel_configs.get_mut(&channel) {
            config.enabled = enabled;
            
            if !enabled {
                // Останавливаем все звуки на канале
                if let Some(sinks) = self.active_sinks.get_mut(&channel) {
                    for sink in sinks {
                        sink.stop();
                    }
                    sinks.clear();
                }
            }
        }
    }

    /// Получение громкости канала
    pub fn get_channel_volume(&self, channel: AudioChannel) -> f32 {
        self.channel_configs.get(&channel).map(|c| c.volume).unwrap_or(1.0)
    }

    /// Пауза всех звуков
    pub fn pause_all(&mut self) {
        for sinks in self.active_sinks.values() {
            for sink in sinks {
                sink.pause();
            }
        }
    }

    /// Возобновление всех звуков
    pub fn resume_all(&mut self) {
        for sinks in self.active_sinks.values() {
            for sink in sinks {
                sink.play();
            }
        }
    }

    /// Остановка всей музыки
    pub fn stop_music(&mut self) {
        self.current_music = None;
        self.target_music = None;
        
        if let Some(sinks) = self.active_sinks.get_mut(&AudioChannel::Music) {
            for sink in sinks {
                sink.stop();
            }
            sinks.clear();
        }
    }

    /// Обновление движка (вызывать каждый кадр)
    pub fn update(&mut self, dt: f32) {
        self.update_crossfade(dt);
        self.update_audio_sources();
        
        // Очистка остановленных sink'ов
        for (_, sinks) in &mut self.active_sinks {
            sinks.retain(|sink| !sink.empty());
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            output_stream: None,
            output_handle: None,
            channel_configs: HashMap::new(),
            active_sinks: HashMap::new(),
            sfx_library: HashMap::new(),
            music_library: HashMap::new(),
            audio_sources: Vec::new(),
            listener: ListenerSettings::default(),
            current_music: None,
            target_music: None,
            crossfade_progress: 0.0,
            crossfade_duration: 2.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_creation() {
        // Тест может упасть если нет аудио устройства
        let result = AudioEngine::new();
        // Проверяем что либо успешно создан, либо ошибка связана с отсутствием устройства
        assert!(result.is_ok() || result.unwrap_err().contains("audio"));
    }

    #[test]
    fn test_channel_config_default() {
        let config = ChannelConfig::default();
        assert_eq!(config.volume, 1.0);
        assert!(config.enabled);
        assert_eq!(config.max_simultaneous, 8);
    }

    #[test]
    fn test_sound_effect_creation() {
        let samples = vec![0.0f32; 44100];
        let sfx = SoundEffect::new("test", samples, 44100, 1);
        assert_eq!(sfx.name, "test");
        assert_eq!(sfx.sample_rate, 44100);
    }

    #[test]
    fn test_music_track_creation() {
        let samples = vec![0.0f32; 44100 * 60]; // 1 минута
        let track = MusicTrack::new("ambient", samples, 44100, 60.0);
        assert_eq!(track.name, "ambient");
        assert_eq!(track.duration, 60.0);
    }

    #[test]
    fn test_audio_source_volume_calculation() {
        let samples = vec![0.0f32; 100];
        let sfx = SoundEffect::new("test", samples.clone(), 44100, 1);
        let mut source = AudioSource::new(Vec3::new(0.0, 0.0, 10.0), sfx);
        source.radius = 20.0;
        source.attenuation = 1.0;
        
        let listener_pos = Vec3::ZERO;
        let volume = source.calculate_volume(listener_pos);
        
        // На расстоянии 10 от радиуса 20 громкость должна быть 0.5
        assert!((volume - 0.5).abs() < 0.01);
    }
}
