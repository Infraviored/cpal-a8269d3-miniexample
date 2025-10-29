// audio.rs
use rodio::{OutputStream, Sink};
use std::io::Cursor;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Embed the sound file as bytes
const SOUND_BYTES: &[u8] = include_bytes!("../sounds/start.wav");

// Global audio stream that persists for the lifetime of the application
// We store Arc<Mutex<...>> so multiple threads can access it
static AUDIO_STREAM: Lazy<Arc<Mutex<Option<OutputStream>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});

/// Play a sound asynchronously on the default output device.
/// This function returns immediately, spawning a background thread for playback.
pub fn play_sound() {
    let sound_bytes = SOUND_BYTES;
    let volume = 0.75;
    
    // Initialize stream on first call and get mixer
    let stream_lock = AUDIO_STREAM.lock().unwrap();
    
    // Initialize if needed
    let mixer = if stream_lock.is_none() {
        drop(stream_lock);
        let mut stream_lock = AUDIO_STREAM.lock().unwrap();
        
        log::info!("Initializing audio stream");
        match rodio::OutputStreamBuilder::open_default_stream() {
            Ok(stream) => {
                log::info!("Audio stream initialized successfully");
                let mixer = stream.mixer().clone();
                *stream_lock = Some(stream);
                mixer
            }
            Err(e) => {
                log::error!("Failed to initialize audio stream: {}", e);
                return;
            }
        }
    } else {
        // Get mixer and clone it so we can use it in the thread
        let mixer = stream_lock.as_ref().unwrap().mixer().clone();
        mixer
    };
    
    // Now we can spawn thread with the cloned mixer
    std::thread::spawn(move || {
        log::info!("Playing sound on persistent output stream");
        
        // Create a sink to play the sound
        let sink = Sink::connect_new(&mixer);
        sink.set_volume(volume);
        
        // Decode and play the sound
        let cursor = Cursor::new(sound_bytes);
        if let Ok(source) = rodio::Decoder::new(cursor) {
            sink.append(source);
            log::info!("Sound playback started");
            sink.sleep_until_end();
            log::info!("Sound playback finished");
        } else {
            log::error!("Failed to decode sound");
        }
    });
}
