#[derive(Debug, Clone, Default)]
pub struct Conf {
    image_paths: Vec<String>,
    output_path: String,
}


impl Conf {
    pub fn from_images(images: Vec<String>, out: String) -> Conf {
        Conf {
            image_paths: images,
            output_path: out,
        }
    }

    pub fn get_output_path(&self) -> String {
        self.output_path.clone()
    }

    pub fn get_image_paths(&self) -> Vec<String> {
        self.image_paths.clone()
    }
}

