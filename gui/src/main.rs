mod utils;
mod rpdfapp;
use rpdfapp::RpdfApp;

fn main() {
    let mut app = RpdfApp::new();
    app.launch();
}
