use printpdf::*;
use image_crate::codecs::png::PngDecoder;
use std::{path::Path, fs::File, io::BufWriter};
use crate::conf;

#[derive(Debug, Clone)]
pub struct Convert {
    config: conf::Conf,
}

impl Convert {
    pub fn new(config: conf::Conf) -> Self {
        Convert { config }
    }

    pub fn save_to_pdf(self) -> Result<(), String> {
        let output_path = self.config.get_output_path();
        let (doc, page1, layer1) = PdfDocument::new(
            output_path.clone(), 
            Mm(247.0), Mm(210.0), "Layer 1"
        );
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let load_paths: Vec<String> = self.config.get_image_paths();
        let load_path: String = load_paths.get(0).unwrap().to_string();
        
        let mut image_file = File::open(Path::new(&load_path)).unwrap();
        let img = Image::try_from(PngDecoder::new(&mut image_file).unwrap()).unwrap();

        img.add_to_layer(current_layer.clone(), ImageTransform::default());
        doc.save(&mut BufWriter::new(File::create(output_path.clone()).unwrap())).unwrap();
        Ok(())
    }
}
