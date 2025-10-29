use flexi_logger::{Logger, Duplicate};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}, HotKeyState};

mod audio;

fn main() {
    // Initialize logger
    Logger::try_with_str("info")
        .unwrap()
        .duplicate_to_stderr(Duplicate::Info)
        .start()
        .unwrap();
    
    log::info!("Mini Audio Client started");
    
    // Pre-initialize audio stream
    log::info!("Pre-initializing audio stream...");
    audio::play_sound(); // This will initialize the stream
    std::thread::sleep(std::time::Duration::from_millis(100)); // Small delay to ensure initialization
    
    // Create hotkey manager
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
    
    // Register hotkey: Ctrl+Shift+P
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyP);
    manager.register(hotkey).expect("Failed to register hotkey");
    log::info!("Hotkey registered: Ctrl+Shift+P");
    
    // Listen for hotkey events
    let receiver = GlobalHotKeyEvent::receiver();
    
    log::info!("Listening for hotkey events. Press Ctrl+Shift+P to play sound.");
    
    loop {
        if let Ok(event) = receiver.recv() {
            if event.state == HotKeyState::Pressed {
                log::info!("Hotkey pressed! Playing sound...");
                audio::play_sound();
            }
        }
    }
}
