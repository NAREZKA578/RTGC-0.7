use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use parking_lot::Mutex;

pub struct AudioSystem {
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioSystem {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        let sink = Arc::new(Mutex::new(sink));

        Ok(Self {
            _stream: stream,
            sink,
        })
    }

    pub fn play_sound(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        
        let sink = self.sink.lock();
        sink.append(source);
        
        Ok(())
    }

    pub fn play_music(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        
        let sink = self.sink.lock();
        sink.append(source);
        sink.set_volume(0.5); // Music usually plays at lower volume
        
        Ok(())
    }

    pub fn stop_all_audio(&self) {
        let sink = self.sink.lock();
        sink.stop();
    }

    pub fn set_master_volume(&self, volume: f32) {
        let sink = self.sink.lock();
        sink.set_volume(volume);
    }
}