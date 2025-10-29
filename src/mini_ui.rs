use eframe::egui;
use flexi_logger::{Logger, Duplicate};

mod audio;

struct MiniAudioApp;

impl eframe::App for MiniAudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Mini Audio Playback Test");
                ui.add_space(20.0);
                
                if ui.button("Play Sound").clicked() {
                    log::info!("Play Sound button clicked");
                    audio::play_sound();
                }
                
                ui.add_space(20.0);
                ui.label("Click the button above to play a sound");
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    // Initialize logger
    Logger::try_with_str("info")
        .unwrap()
        .duplicate_to_stderr(Duplicate::Info)
        .start()
        .unwrap();
    
    log::info!("Mini Audio UI started");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 200.0])
            .with_title("Mini Audio Test"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Mini Audio Test",
        options,
        Box::new(|_cc| Ok(Box::new(MiniAudioApp))),
    )
}

