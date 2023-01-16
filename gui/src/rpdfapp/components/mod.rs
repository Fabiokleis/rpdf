use fltk::{
    prelude::*,
    app::{self, Sender, Scheme}, 
    group::{Pack, Flex, FlexType}, menu::{Choice, MenuFlag, SysMenuBar},
    enums::{Color, Event, Key, Shortcut, FrameType, self, Align},
    button,
    draw,
    window::Window,
    image::{self, SharedImage},
    dialog::{self, NativeFileChooser, NativeFileChooserType, HelpDialog}, frame::{Frame, self}, widget::Widget
};
use crate::utils::{W_WIDTH, W_HEIGHT, Message, FileOperations, Themes, PdfSizes, IMAGE_WIDTH, IMAGE_HEIGTH};

pub struct MyMenu {
    menu: SysMenuBar,
}

impl MyMenu {
    pub fn new() -> Self {
        let mut menu = SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);
        menu.set_color(Color::from_rgb(200, 200, 200));

        MyMenu {
            menu
        }
    }

    pub fn add_emit(&mut self, name: String, shortcut: Shortcut, flag: MenuFlag, sender: &Sender<Message>, message: Message) {
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

pub struct MyDropDownList<M: MenuExt> {
    dd_list: M,
}

impl<M: MenuExt> MyDropDownList<M> {
    pub fn new(parent: &mut Flex, label: String, choices: String, choice_message: Message, sender: &Sender<Message>, menu: M) -> Self {
        let mut dd_list = menu.with_label(label.as_str());
        choices.split("|").for_each(|opt| {
            dd_list.add_choice(opt);
            let variant: Message = match choice_message {
                Message::Theme(_) => Themes::get_variant(opt.to_string()),
                Message::FileOperation(_) => FileOperations::get_variant(opt.to_string()),
                Message::PdfSize(_) => PdfSizes::get_variant(opt.to_string()),
                _ => Message::None
            };
            dd_list.add_emit(opt, Shortcut::None, MenuFlag::Normal, sender.clone(), variant);
            parent.set_size(&dd_list, 120);
        });
        //parent.set_size(&dd_list, 80);
        MyDropDownList { 
            dd_list,
        }
    }
}

#[derive(Clone)]
pub struct InputButton {
    btn: button::Button,
    image_paths: Vec<String>,
}

impl InputButton {
    pub fn new(parent: &mut Flex, label: String, w: i32) -> InputButton {
        let btn = button::Button::default()
            .with_label(label.as_str());
        parent.add(&btn);
        parent.set_size(&btn, w);
        InputButton { 
            btn,
            image_paths: vec![],
        }
    }

    pub fn emit(&mut self, sender: Sender<Message>, message: Message) {
        self.btn.emit(sender, message);
    }

    pub fn get_paths(&self) -> &Vec<String> {
        &self.image_paths        
    }

    pub fn add_path(&mut self, path: String) {
        self.image_paths.push(path);
    }

    pub fn clean_images(&mut self) -> bool {
        if self.image_paths.is_empty() {
            return false;
        }
        self.image_paths.clear();
        true
    }

    pub fn send(&mut self) {
        self.btn.do_callback();
    } 
}

pub struct ButtonSection {
    flex: Flex,
}

impl ButtonSection {
    pub fn new(parent: &mut Flex, w: i32, pad: i32, margin: i32) -> Self {
        let mut flex = Flex::default().row();
        flex.set_pad(pad);
        flex.set_margin(margin);
        flex.set_frame(FrameType::FlatBox);
        flex.set_color(Color::from_rgb(200, 200, 200));
        parent.set_margin(margin);
        parent.add(&flex);
        parent.set_size(&flex, w);
       
        ButtonSection {
            flex,
        }
    }

    pub fn end(&mut self) {
        self.flex.end();
    }

    pub fn create_button(&mut self, label: String, w: i32) -> button::Button {
        let b = button::Button::default()
            .with_pos(self.flex.x(), self.flex.y())
            .with_size(w, self.flex.h())
            .with_label(label.as_str());

        self.flex.set_size(&b, w);
        b
    }

    pub fn create_input_button(&mut self, label: String, w: i32) -> InputButton {
        InputButton::new(&mut self.flex, label, w)
    }

    pub fn flex(&mut self) -> &mut Flex {
        &mut self.flex
    }
}

pub struct PreviewSection {
    flex: Flex,
}

impl PreviewSection {
    pub fn new(parent: &mut Flex, w: i32, pad: i32, margin: i32) -> Self {
        let mut flex = Flex::default()
            .with_label("Preview Section")
            .row();
        
        flex.set_frame(FrameType::FlatBox);
        flex.set_color(Color::from_rgb(230, 230, 240));

        flex.set_pad(pad);
        parent.set_margin(margin);

        parent.add(&flex);
        parent.set_size(&flex, w);

        PreviewSection {
            flex,
        }
    }

    pub fn redraw(&mut self) {
        self.flex.redraw();
    }

    pub fn begin(&mut self) {
        self.flex.begin();
    }

    pub fn end(&mut self) {
        self.flex.end();
    }

    pub fn flex(&mut self) -> &mut Flex {
        &mut self.flex
    }

    pub fn add_image(&mut self, path: String, w: i32, h: i32, pad: i32, margin: i32) -> ImageItem {
        ImageItem::new(&mut self.flex, path, w, h, pad, margin)
    }
    
}

type ImageWrapp = Option<SharedImage>;

#[derive(Clone)]
pub struct ImageItem {
    frame: Frame,
}

impl ImageItem {
    pub fn new(parent: &mut Flex, path: String, w: i32, h: i32, pad: i32, margin: i32) -> Self {
        let mut frame = Frame::default().with_size(w, h); 
        let mut img = SharedImage::load(path).unwrap();
        frame.set_align(Align::Bottom | Align::Inside);
       
        frame.draw(move |f| {
            img.scale(w, h, true, true);
            img.draw(f.x(), f.y(), w, h);
        });
           

        parent.add(&frame);

        ImageItem {
            frame,
        }
    }
}
