pub const W_WIDTH: usize = 720;
pub const W_HEIGHT: usize = 480;
pub const IMAGE_WIDTH: usize = 100;
pub const IMAGE_HEIGTH: usize = 200;

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
