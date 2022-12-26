#[derive(Debug, Clone, Default)]
pub struct Conf {
    num_pages: usize,
    image_paths: Vec<String>,
    output_path: String,
}


impl Conf {
    pub fn from_images(images: Vec<String>, out: String) -> Conf {
        Conf {
            num_pages: images.len(),
            image_paths: images,
            output_path: out,
        }
    }
}

