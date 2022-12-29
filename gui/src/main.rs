extern crate fltk;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, };

#[derive(Copy, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Oxy);
    let mut wind = Window::default()
        .with_size(160, 200)
        .center_screen()
        .with_label("Counter");
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    wind.make_resizable(true);
    wind.end();
    wind.show();
    /* Event handling */
    let (s, r) = app::channel::<Message>();
    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        let label: i32 = frame.label().parse().unwrap();
        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }
    app.run().unwrap();
}
