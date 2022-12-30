extern crate fltk;
use fltk::{
    prelude::*,
    app::{self, Sender, Scheme}, 
    group::Pack, menu::{Choice, MenuFlag},
    enums::{Event, Key, Shortcut},
    window::Window,
    image::SharedImage, button,
    dialog::{NativeFileChooser, NativeFileChooserType}
};

/// Supported actions
#[derive(Clone, Copy)]
enum Message {
    Theme(Themes),
    FileOperation(FileOperations),
    PdfSize(PdfSizes),
    Quit,
    None,
}

#[derive(Clone, Copy)]
enum FileOperations {
    Upload,
    Convert,
    Save,
}

impl FileOperations {
    pub fn get_variant(e_v: String) -> Message {
        match e_v.as_str() {
            "Upload" => Message::FileOperation(FileOperations::Upload),
            "Convert" => Message::FileOperation(FileOperations::Convert),
            "Save" => Message::FileOperation(FileOperations::Save),
            _ => Message::None,
        }
    }
}

#[derive(Clone, Copy)]
enum Themes {
    Gtk,
    Plastic,
    Gleam,
    Oxy,
}

impl Themes {
    pub fn get_variant(e_v: String) -> Message {
        match e_v.as_str() {
            "Gtk" => Message::Theme(Themes::Gtk),
            "Plastic" => Message::Theme(Themes::Plastic),
            "Gleam" => Message::Theme(Themes::Gleam),
            "Oxy" => Message::Theme(Themes::Oxy),
            _ => Message::None,
        }
    }
}

#[derive(Clone, Copy)]
enum PdfSizes {
    A2,
    A3,
    A4,
    A5,
    ImgSize,
}

impl PdfSizes {
    pub fn get_variant(e_v: String) -> Message {
        match e_v.as_str() {
            "A2" => Message::PdfSize(PdfSizes::A2),
            "A3" => Message::PdfSize(PdfSizes::A3),
            "A4" => Message::PdfSize(PdfSizes::A4),
            "A5" => Message::PdfSize(PdfSizes::A5),
            "ImgSize" => Message::PdfSize(PdfSizes::ImgSize),
            _ => Message::None,
        }
    }
}

pub struct RpdfApp {
    app: app::App,
    main_win: Window,
    r: app::Receiver<Message>,
    imgs: Option<SharedImage>,
    themes_dd: MyDropDownList,
    pdf_sizes_dd: MyDropDownList,
    input_btn: MyButton,
}

struct MyDropDownList {
    dd_list: Choice,
}

struct MyButton {
    btn: button::Button,
}

impl MyButton {
    fn new<F: FnMut(&mut button::Button) + 'static>(label: String, cb: F) -> MyButton {
        let mut btn = button::Button::default()
            .with_size(0, 30)
            .with_label(label.as_str());
        btn.set_callback(cb);
        MyButton { btn }
    }
}

impl MyDropDownList {
    fn new(label: String, choices: String, choice_message: Message, sender: &Sender<Message>) -> Self {
        let mut dd_list = Choice::default().with_size(0, 30).with_label(label.as_str());
        choices.split("|").for_each(|opt| {
            dd_list.add_choice(opt);
            let variant: Message = match choice_message {
                Message::Theme(_) => { Themes::get_variant(opt.to_string()) },
                Message::FileOperation(_) => { FileOperations::get_variant(opt.to_string()) },
                Message::PdfSize(_) => { PdfSizes::get_variant(opt.to_string()) },
                _ => { Message::None }
            };
            dd_list.add_emit(opt, Shortcut::None, MenuFlag::Normal, sender.clone(), variant);
        });
        MyDropDownList { 
            dd_list
        }
    }
}

impl RpdfApp {
    fn new() -> Self {
        let app = app::App::default().with_scheme(Scheme::Oxy);
        let (s, r) = app::channel::<Message>();
        let mut main_win = Window::default()
            .with_size(640, 480)
            .center_screen()
            .with_label("Rpdf");

        let mut pack = Pack::new(100, 45, 150, 450 - 45, "");
        pack.set_spacing(10);
        // available themes
        let themes_dd = MyDropDownList::new(
            "App Theme".to_string(), 
            "Gtk|Plastic|Gleam|Oxy".to_string(),
            Message::Theme(Themes::Gtk),
            &s
        );

        // available pdf sizes
        let pdf_sizes_dd = MyDropDownList::new(
            "PDF size".to_string(),
            "A2|A3|A4|A5|ImgSize".to_string(),
            Message::PdfSize(PdfSizes::A4),
            &s
        );
       
        let input_btn = MyButton::new("@fileopen  Upload image".to_string(), 
            move |bt| {
                bt.emit(s, Message::FileOperation(FileOperations::Upload));
            }
        );

        pack.end();
        main_win.make_resizable(true);
        main_win.end();
        main_win.show();

        main_win.set_callback(move |_| {
            if (app::event() == Event::Close) || (app::event_key() == Key::Escape) {
                s.send(Message::Quit);
            }
        });


        let imgs = None;
        RpdfApp {
            app,
            main_win,
            r,
            imgs,
            themes_dd,
            pdf_sizes_dd,
            input_btn,
        }
    }

    pub fn launch(&mut self) {
        while self.app.wait() {
            if let Some(st) = self.r.recv() {
                match st {
                    Message::Theme(th) => match th {
                        Themes::Gtk => {
                            self.app.set_scheme(Scheme::Gtk);
                        },
                        Themes::Plastic => {
                            self.app.set_scheme(Scheme::Plastic);
                        },
                        Themes::Gleam => {
                            self.app.set_scheme(Scheme::Gleam);
                        },
                        Themes::Oxy => {
                            self.app.set_scheme(Scheme::Oxy);
                        },
                    },
                    Message::FileOperation(fopt) => match fopt {
                        FileOperations::Upload => {
                            println!("upload file call back!");
                            let mut dialog = NativeFileChooser::new(NativeFileChooserType::BrowseFile);
                            dialog.show();
                            println!("{:#?}", dialog.filename());
                        },
                        FileOperations::Convert => {
                            println!("convert file call back!");
                        },
                        FileOperations::Save => {
                            println!("save file call back!");
                        },
                    },
                    Message::PdfSize(ps) => match ps {
                        PdfSizes::A2 => {

                        },
                        PdfSizes::A3 => {
                        },
                        PdfSizes::A4 => {

                        },
                        PdfSizes::A5 => {

                        },
                        PdfSizes::ImgSize => {

                        }
                    },
                    Message::Quit => {
                        self.app.quit();
                    },
                    Message::None => {}
                }
            }
        }
    }

}

fn main() {
    let mut app = RpdfApp::new();
    app.launch();
}
