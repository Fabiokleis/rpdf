pub const W_WIDTH: i32 = 800;
pub const W_HEIGHT: i32 = 640;
pub const P_HEIGHT: i32 = 260;
pub const IMAGE_WIDTH: i32 = 250;
pub const IMAGE_HEIGTH: i32 = 140;
pub const IMAGE_PAD: i32 = 10;
pub const IMAGE_MARGIN: i32 = 10;

/// Supported actions
#[derive(Clone, Copy)]
pub enum Message {
    Theme(Themes),
    FileOperation(FileOperations),
    PdfSize(PdfSizes),
    About,
    Help,
    Quit,
    None,
}

#[derive(Clone, Copy)]
pub enum FileOperations {
    Upload,
    ConvertAndSave,
}

impl FileOperations {
    pub fn get_variant(e_v: String) -> Message {
        match e_v.as_str() {
            "Upload" => Message::FileOperation(FileOperations::Upload),
            "Convert" => Message::FileOperation(FileOperations::ConvertAndSave),
            _ => Message::None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Themes {
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
pub enum PdfSizes {
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
            "Image size" => Message::PdfSize(PdfSizes::ImgSize),
            _ => Message::None,
        }
    }
}
