use crate::conf;
use printpdf::*;

use std::convert::From;
use std::convert::TryFrom;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct Convert {
    config: conf::Conf,
}

impl Convert {
    pub fn new(config: conf::Conf) -> Self {
        Convert { config }
    }

    pub fn save_to_pdf(self) -> Result<(), String> {
        Ok(())
    }
}
