extern crate fltk;
use fltk::{
    prelude::*,
    app::{self, Sender, Scheme}, 
    group::Pack, menu::{Choice, MenuFlag, SysMenuBar},
    enums::{Event, Key, Shortcut, FrameType},
    window::Window,
    image::SharedImage, button,
    dialog::{self, NativeFileChooser, NativeFileChooserType, HelpDialog}
};

/// Supported actions
#[derive(Clone, Copy)]
enum Message {
    Theme(Themes),
    FileOperation(FileOperations),
    PdfSize(PdfSizes),
    About,
    Help,
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
    sys_menu: MyMenu,
    imgs: Option<SharedImage>,
    pdf_sizes_dd: MyDropDownList<Choice>,
    input_btn: MyButton,
}


struct MyMenu {
    menu: SysMenuBar,
}

struct MyDropDownList<M: MenuExt> {
    dd_list: M,
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

impl<M: MenuExt> MyDropDownList<M> {
    fn new(label: String, choices: String, choice_message: Message, sender: &Sender<Message>, menu: M) -> Self {
        let mut dd_list = menu.with_size(0, 30).with_label(label.as_str());
        choices.split("|").for_each(|opt| {
            dd_list.add_choice(opt);
            let variant: Message = match choice_message {
                Message::Theme(_) => Themes::get_variant(opt.to_string()),
                Message::FileOperation(_) => FileOperations::get_variant(opt.to_string()),
                Message::PdfSize(_) => PdfSizes::get_variant(opt.to_string()),
                _ => Message::None
            };
            dd_list.add_emit(opt, Shortcut::None, MenuFlag::Normal, sender.clone(), variant);
        });
        MyDropDownList { 
            dd_list
        }
    }
}

impl MyMenu {
    fn new() -> Self {
        let mut menu = SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);
        MyMenu {
            menu
        }
    }

    fn add_emit(&mut self, name: String, shortcut: Shortcut, flag: MenuFlag, sender: &Sender<Message>, message: Message) {
        let variant: Message = match message {
            Message::Theme(_) => {
                let sufx = name.split("/").nth(1).unwrap();
                Themes::get_variant(sufx.to_string())
            },
            Message::About => Message::About,
            Message::Help => Message::Help,
            Message::Quit => Message::Quit,
            _ => Message::None,
        };
        self.menu.add_emit(name.as_str(), shortcut, flag, sender.clone(), variant);
    }
}

impl RpdfApp {
    fn new() -> Self {
        let app = app::App::default().with_scheme(Scheme::Base);
        let (s, r) = app::channel::<Message>();
        let mut main_win = Window::default()
            .with_size(720, 640)
            .center_screen()
            .with_label("Rpdf");

        let mut sys_menu = MyMenu::new();
        // available themes
        sys_menu.add_emit("&Theme/Gtk".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Gtk));
        sys_menu.add_emit("&Theme/Plastic".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Plastic));
        sys_menu.add_emit("&Theme/Gleam".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Gleam));
        sys_menu.add_emit("&Theme/Oxy".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Oxy));

        // actions
        sys_menu.add_emit("&About".to_string(), Shortcut::from_char('a'), MenuFlag::Normal, &s, Message::About);
        sys_menu.add_emit("&Help".to_string(), Shortcut::from_char('h'), MenuFlag::Normal, &s, Message::Help);
        sys_menu.add_emit("&Quit".to_string(), Shortcut::from_char('q'), MenuFlag::Normal, &s, Message::Quit);

        let mut pack = Pack::new(100, 100, 150, 450 - 45, "");
        pack.set_spacing(10);

        // available pdf sizes
        let pdf_sizes_dd = MyDropDownList::new(
            "PDF size".to_string(),
            "A2|A3|A4|A5|ImgSize".to_string(),
            Message::PdfSize(PdfSizes::A4),
            &s,
            Choice::default()
        );
       
        let input_btn = MyButton::new("@fileopen  Upload image".to_string(), 
            move |bt| {
                bt.emit(s, Message::FileOperation(FileOperations::Upload));
                let mut dialog = NativeFileChooser::new(NativeFileChooserType::BrowseMultiFile);
                dialog.show();
                println!("{:#?}", dialog.filenames());
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
            sys_menu,
            imgs,
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
                            todo!()
                        },
                        FileOperations::Convert => {
                            todo!()
                        },
                        FileOperations::Save => {
                            todo!()
                        },
                    },
                    Message::PdfSize(ps) => match ps {
                        PdfSizes::A2 => {
                            todo!()
                        },
                        PdfSizes::A3 => {
                            todo!()
                        },
                        PdfSizes::A4 => {
                            todo!()
                        },
                        PdfSizes::A5 => {
                            todo!()
                        },
                        PdfSizes::ImgSize => {
                            todo!()
                        }
                    },
                    Message::About => {
                        todo!()
                    },
                    Message::Help => {
                        todo!()
                    },
                    Message::Quit => {
                        self.app.quit();
                    },
                    Message::None => todo!() 
                }
            }
        }
    }
}

fn main() {
    let mut app = RpdfApp::new();
    app.launch();
}
