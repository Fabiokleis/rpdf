use convert::{Convert, conf::Conf};
use fltk::{
    prelude::*,
    frame::{self, Frame},
    app::{self, Sender, Scheme}, 
    group::{Pack, Flex, FlexType}, menu::{Choice, MenuFlag, SysMenuBar},
    enums::{Color, Event, Key, Shortcut, FrameType},
    window::Window,
    image::SharedImage, button,
    dialog::{self, NativeFileChooser, NativeFileChooserType, HelpDialog}
};

use std::{rc::Rc, cell::RefCell};
use crate::utils::{W_WIDTH, W_HEIGHT, P_HEIGHT, IMAGE_WIDTH, IMAGE_HEIGTH, IMAGE_PAD, IMAGE_MARGIN, Message, FileOperations, Themes, PdfSizes};

mod components;
use components::{MyMenu, MyDropDownList, InputButton, PreviewSection, ButtonSection};

fn main_menu(sys_menu: &mut MyMenu, s: &Sender<Message>) {
    sys_menu.add_emit("&Theme/Gtk".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Gtk));
    sys_menu.add_emit("&Theme/Plastic".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Plastic));
    sys_menu.add_emit("&Theme/Gleam".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Gleam));
    sys_menu.add_emit("&Theme/Oxy".to_string(), Shortcut::None, MenuFlag::Normal, &s, Message::Theme(Themes::Oxy));

    // actions
    sys_menu.add_emit("&About".to_string(), Shortcut::from_char('a'), MenuFlag::Normal, &s, Message::About);
    sys_menu.add_emit("&Help".to_string(), Shortcut::from_char('h'), MenuFlag::Normal, &s, Message::Help);
    sys_menu.add_emit("&Quit".to_string(), Shortcut::from_char('q'), MenuFlag::Normal, &s, Message::Quit);
}


//fn preview_panel(p_section: &mut PreviewSection, s: &Sender<Message>) {
//    MyDropDownList::new(
//        parent,
//        "PDF size".to_string(),
//        "A2|A3|A4|A5|Image size".to_string(),
//        Message::PdfSize(PdfSizes::A4),
//        &s,
//        Choice::default()
//    );
//}

pub fn center() -> (i32, i32) {
    (
        (app::screen_size().0 / 2.0) as i32,
        (app::screen_size().1 / 2.0) as i32,
    )
}


pub struct RpdfApp {
    app: app::App,
    main_win: Window,
    r: app::Receiver<Message>,
    sys_menu: MyMenu,
    app_flex: Flex,
    p_section: PreviewSection,
    b_section: ButtonSection,
    input_button: InputButton,
}

impl RpdfApp {
    pub fn new() -> Self {
        let app = app::App::default().with_scheme(Scheme::Gtk);
        let (s, r) = app::channel::<Message>();
        let mut main_win = Window::default()
            .with_size(W_WIDTH, W_HEIGHT)
            .center_screen()
            .with_label("Rpdf");

        main_win.set_color(Color::from_rgb(255, 255, 255));

        let mut sys_menu = MyMenu::new();
        main_menu(&mut sys_menu, &s);
        let mut app_flex = Flex::new(0, 45, W_WIDTH, W_HEIGHT - 45, "").column();

        let mut p_section = PreviewSection::new(&mut app_flex, P_HEIGHT, 0, 10);
        
        p_section.end();
        
        let mut b_section = ButtonSection::new(&mut app_flex, 50, 10, 10);
        let mut input_button = b_section.create_input_button("@fileopen Open image".to_string(), 140);
        input_button.emit(s, Message::FileOperation(FileOperations::Upload));
        let mut convert_button = b_section.create_button("@filenew Convert and save images".to_string(), 200);
        convert_button.emit(s, Message::FileOperation(FileOperations::ConvertAndSave));
        convert_button.deactivate();

        b_section.end();


        app_flex.end();
        
        main_win.make_resizable(true);
        //main_win.size_range(W_WIDTH, W_HEIGHT, 0, 0);
        main_win.end();
        main_win.show();

        main_win.set_callback(move |_| {
            if (app::event() == Event::Close) || (app::event_key() == Key::Escape) {
                s.send(Message::Quit);
            }
        });

        RpdfApp {
            app,
            main_win,
            r,
            sys_menu,
            app_flex,
            p_section,
            b_section,
            input_button,
        }
    }

    fn open_files_dialog(&mut self) -> Result<bool, String> {
        let mut dialog = NativeFileChooser::new(NativeFileChooserType::BrowseMultiFile);
        dialog.set_filter("*.{png,jpg}");
        dialog.show();
        let file_names: Vec<String> = dialog.filenames().iter().map(|p| p.to_string_lossy().to_string()).collect();
        if file_names.is_empty() {
            dialog::message_title("Choose file");
            dialog::alert(center().0 - 200, center().1 - 100, "Please choose a file!");
            return Ok(false);
        }
        
        for path in file_names.iter() {
            if ! self.input_button.get_paths().contains(&path.clone()) {
                self.input_button.add_path(path.clone());
                self.p_section.begin();
                self.p_section.add_image(path.to_string(), IMAGE_WIDTH, IMAGE_HEIGTH, IMAGE_PAD, IMAGE_MARGIN);
                self.p_section.end();
                self.p_section.redraw();
            }
        }

        Ok(true)
    }

    fn convert_and_save(&mut self) -> Result<bool, String> {
        let mut dlg = dialog::FileDialog::new(dialog::FileDialogType::BrowseSaveFile);
        dlg.set_option(dialog::FileDialogOptions::SaveAsConfirm);
        dlg.show();
        let out = dlg.filename().to_string_lossy().to_string();
        if out.is_empty() {
            dialog::message_title("Choose save file");
            dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!");
            return Ok(false);
        }

        self.p_section.flex().deactivate();
        self.b_section.flex().deactivate();
        let images = self.input_button.get_paths().clone();
        let config = Conf::from_images(images, out);
        let cvrt = Convert::new(config);
        self.clean_preview_section()?;
        cvrt.save_to_pdf()?;

        self.p_section.flex().activate();
        self.b_section.flex().activate();
        self.b_section.flex().child(1).unwrap().deactivate();
        Ok(true)
    }

    fn clean_preview_section(&mut self) -> Result<bool, String> {
        if ! self.input_button.clean_images() {
            dialog::alert(center().0 - 200, center().1 - 100, "No image in preview section!");
            return Ok(false);
        }
        self.p_section.flex().clear();
        
        Ok(true)
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
                            if self.open_files_dialog().unwrap() {
                                self.b_section.flex().child(1).unwrap().activate();
                            }
                        },
                        FileOperations::ConvertAndSave => {
                            self.convert_and_save();
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
                        dialog::message_title("About");
                        dialog::message(center().0 - 200, center().1 - 100, "About dialog");
                    },
                    Message::Help => {
                        dialog::message_title("Help");
                        dialog::message(center().0 - 200, center().1 - 100, "About dialog");
                    },
                    Message::Quit => {
                        if self.input_button.get_paths().is_empty() {
                            self.app.quit();
                        } else {
                            match dialog::choice2_default("Would you like to convert and save", "No", "Yes", "Cancel") {
                                Some(1) => self.b_section.flex().child(1).unwrap().do_callback(),
                                Some(0) => self.app.quit(),
                                Some(2) => {},
                                None => {},
                                _ => {}
                            }
                             
                        }
                    },
                    Message::None => todo!() 
                }
            }
        }
    }
}
