extern crate printpdf;
use printpdf::*;
use image_crate::codecs::{jpeg::JpegDecoder, png::PngDecoder};
use std::{path::Path, fs::File, io::BufWriter};
//use std::cmp::max;

const PPI_BY_DPI: f64 = 0.084666667;

#[allow(dead_code)]
pub mod conf;

#[derive(Debug, Clone)]
pub struct Convert {
    config: conf::Conf,
}

// TODO: Convert page size by setting a pdf size
//#[derive(Clone, Copy)]
//enum PdfSizes {
//    A2,
//    A3,
//    A4,
//    A5,
//    ImgSize,
//}

enum ImgExtension {
    Png,
    Jpg,
}



impl Convert {
    pub fn new(config: conf::Conf) -> Self {
        Convert { config }
    }

    pub fn save_to_pdf(self) -> Result<(), String> {
        let output_path = self.config.get_output_path();
        let doc = PdfDocument::empty(output_path.clone());
        let e_paths = verify_paths(&self.config.get_image_paths().clone());

        match e_paths {
            Some(vp) => {
                vp.iter().for_each(|p| {
                    let img_extension = get_img_extension(p.to_string());
                    match img_extension {
                        Some(ft) => {
                            let (img, px) = load_img(p.to_string(), ft);
                            let mm = px2mm(px.0, px.1);
                            let mut page_mm = (Mm(0.0), Mm(0.0));
                            let s_ratio = 1.0;

                            // TODO: Create a flag option to pdf size and center image on page 
                            //s_ratio = max(1, max((210.0 / mm.0) as i32, (297.0 / mm.1) as i32)) as f64;



                            // set to img size
                            page_mm.0 = Mm(mm.0);
                            page_mm.1 = Mm(mm.1);
                    
                            let (page, layer) = doc.add_page(page_mm.0 * s_ratio, page_mm.1 * s_ratio, "Layer");
                            let current_layer = doc.get_page(page).get_layer(layer);
                             
                            let img_transform = ImageTransform {
                                scale_x: Some(s_ratio),
                                scale_y: Some(s_ratio),
                                .. Default::default()
                            };
                            img.add_to_layer(current_layer.clone(), img_transform);

                        },
                        None => {}
                    }
                    
                });
                doc.save(&mut BufWriter::new(File::create(output_path.clone()).unwrap())).unwrap();
            },
            None => { 
                eprintln!("Not found any passed file path!");
                std::process::exit(1);
            }
        }
        Ok(())
    }
}

fn get_img_extension(path: String) -> Option<ImgExtension> {
    match Path::new(&path.clone()).extension() {
        Some(ft) => match ft.to_str() {
            Some("jpg") | Some("jpeg") => Some(ImgExtension::Jpg),
            Some("png") => Some(ImgExtension::Png),
            _ => {
                eprintln!("Image extension not implemented {}! Skipping", path.clone());
                None
            }
        },
        None => {
            eprintln!("File extension not found {}! Skipping", path.clone());
            None
        },
    }    
}

fn px2mm(x: f64, y: f64) -> (f64, f64) {
    (x * PPI_BY_DPI, y * PPI_BY_DPI)
}

fn load_img(path: String, ft: ImgExtension) -> (Image, (f64, f64)) {
    match ft {
        ImgExtension::Jpg => {
            let mut image_file = File::open(Path::new(&path)).unwrap();
            let img = Image::try_from(JpegDecoder::new(&mut image_file).unwrap()).unwrap();
            let px = (img.image.width.0 as f64, img.image.height.0 as f64);
            (img, px)
        },
        ImgExtension::Png => {
            let mut image_file = File::open(Path::new(&path)).unwrap();
            let mut img = Image::try_from(PngDecoder::new(&mut image_file).unwrap()).unwrap();
            let px = (img.image.width.0 as f64, img.image.height.0 as f64);
            img.image = remove_alpha_channel_from_image_x_object(img.image);
            (img, px)
        },
    }
}

pub fn verify_paths(paths: &Vec<String>) -> Option<Vec<String>>{
    let exists: Vec<String> = paths.iter().filter(|p| {
        Path::new(&p).exists()
    }).map(|p| p.to_string()).collect();
    
    if exists.len() > 0 {
        Some(exists)
    } else {
        None
    }
}

// https://github.com/fschutt/printpdf/issues/119#issuecomment-1319778952
// code to convert png to pdf by removing alpha channel
fn remove_alpha_channel_from_image_x_object(image_x_object: ImageXObject) -> ImageXObject {
    if !matches!(image_x_object.color_space, ColorSpace::Rgba) {
        return image_x_object;
    };
    let ImageXObject {
        color_space,
        image_data,
        ..
    } = image_x_object;

    let new_image_data = image_data
        .chunks(4)
        .map(|rgba| {
            let [red, green, blue, alpha]: [u8; 4] = rgba.try_into().ok().unwrap();
            let alpha = alpha as f64 / 255.0;
            let new_red = ((1.0 - alpha) * 255.0 + alpha * red as f64) as u8;
            let new_green = ((1.0 - alpha) * 255.0 + alpha * green as f64) as u8;
            let new_blue = ((1.0 - alpha) * 255.0 + alpha * blue as f64) as u8;
            return [new_red, new_green, new_blue];
        })
        .collect::<Vec<[u8; 3]>>()
        .concat();

    let new_color_space = match color_space {
        ColorSpace::Rgba => ColorSpace::Rgb,
        ColorSpace::GreyscaleAlpha => ColorSpace::Greyscale,
        other_type => other_type,
    };

    ImageXObject {
        color_space: new_color_space,
        image_data: new_image_data,
        ..image_x_object
    }
}
