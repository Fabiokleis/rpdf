use fltk::{
    prelude::*,
    app::{self, Sender, Scheme}, 
    group::{Pack, Flex, FlexType}, menu::{Choice, MenuFlag, SysMenuBar},
    enums::{Color, Event, Key, Shortcut, FrameType, self},
    button,
    window::Window,
    image::{self, SharedImage},
    dialog::{self, NativeFileChooser, NativeFileChooserType, HelpDialog}, frame::{Frame, self}
};
use crate::utils::{W_WIDTH, W_HEIGHT, Message, FileOperations, Themes, PdfSizes, IMAGE_WIDTH, IMAGE_HEIGTH};

pub struct MyMenu {
    menu: SysMenuBar,
}

impl MyMenu {
    pub fn new() -> Self {
        let mut menu = SysMenuBar::default().with_size(800, 35);
        menu.set_frame(FrameType::FlatBox);
        menu.set_color(Color::from_rgb(235, 235, 235));

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

pub struct MyButton {
    btn: button::Button,
}

pub struct ButtonSection {
    flex: Flex,
}

pub struct PreviewSection {
    flex: Flex,
}

impl MyButton {
    pub fn new<F: FnMut(&mut button::Button) + 'static>(parent: &mut Flex, label: String, w: i32, cb: F) -> MyButton {
        let mut btn = button::Button::default()
            .with_label(label.as_str());
        
        btn.set_callback(cb);
        parent.set_size(&btn, w);
        MyButton { 
            btn,
        }
    }
}

impl ButtonSection {
    pub fn new(parent: &mut Flex, w: i32, pad: i32, margin: i32) -> Self {
        let flex = Flex::default().row();
        parent.set_size(&flex, w);
        parent.set_pad(pad);
        parent.set_margin(margin);
        
        ButtonSection {
            flex,
        }
    }

    pub fn end(&mut self) {
        self.flex.end();
    }

    pub fn create_button<F: FnMut(&mut button::Button) + 'static>(&mut self, label: String, w: i32, cb: F) {
        MyButton::new(&mut self.flex, label, w, cb);
    }
}

impl PreviewSection {
    pub fn new(parent: &mut Flex, w: i32, pad: i32, margin: i32) -> Self {
        let mut flex = Flex::default()
            .with_label("Preview Section")
            .row();
        flex.set_frame(FrameType::FlatBox);
        flex.set_color(Color::from_rgb(235, 235, 235));
        
        parent.set_size(&flex, w);
        parent.set_pad(pad);
        parent.set_margin(margin);
        
        PreviewSection {
            flex,
        }
    }

    pub fn end(&mut self) {
        self.flex.end();
    }

    pub fn flex(&mut self) -> &mut Flex {
        &mut self.flex
    }

    pub fn add_image(&mut self, path: String, w: i32, h: i32, pad: i32, margin: i32) {
        ImageItem::new(&mut self.flex, path, w, h, pad, margin);
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
        
        frame.draw(move |f| {
            img.scale(f.w(), f.h(), true, true);
            img.draw(f.x(), f.y(), f.w(), f.h());
        });

        parent.set_size(&frame, w);
        parent.set_pad(pad);
        parent.set_margin(margin);

        ImageItem {
            frame,
        }
    }
}
