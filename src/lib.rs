extern crate byteorder;
#[macro_use] extern crate failure;
extern crate image;

pub mod footer;
use self::footer::JpngFooter;

use failure::Error;
use image::DynamicImage;

use std::mem;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Fail)]
pub enum JpngError {
    #[fail(display = "Input is not a valid JPNG image.")]
    InvalidImage,
    #[fail(display = "Invalid footer length given.")]
    InvalidFooterLen,
}

#[derive(Clone)]
pub struct Jpng {
    image: DynamicImage,
    mask: DynamicImage,
    pub footer: JpngFooter,
}

impl Jpng {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Jpng, Error> {
        let mut contents = Vec::new();
        let footer_size = mem::size_of::<JpngFooter>();
        
        File::open(path)?.read_to_end(&mut contents)?;

        //  Fail early if the contents cannot hold a valid JpngFooter.
        ensure!(contents.len() > footer_size, JpngError::InvalidImage);
        
        //  Create the footer from the last bytes of the given file.
        let footer = JpngFooter::new(&contents[contents.len() - footer_size..])?;
        
        //  Using the footer information, load the JPG and PNG components.
        let image = image::load_from_memory(&contents[footer.image_range()])?;
        let mask = image::load_from_memory(&contents[footer.mask_range()])?;

        Ok(Self {
            image,
            mask,
            footer,
        })
    }

    /// Saves the JPNG image as a PNG with the combined image and mask.
    /// 
    /// The name passed to this method must be the basename without an
    /// extension. This is because .png is automatically appended to it
    /// in order to ensure it encodes the right format.
    pub fn save(&self, basename: &str) -> Result<(), Error> {
        //  Get luma values from mask
        let luma = self.mask.to_luma();
        //  Save changes to temporary image
        let mut combined = self.image.to_rgba();

        //  For each pixel in the JPNG image in RGBA format,
        //  set the alpha component to the luma value of the mask.
        self.image
            .to_rgba()
            .enumerate_pixels()
            .zip(luma.pixels())
            .map(|((x, y, old), luma)| {
                combined.put_pixel(x, y, 
                    image::Rgba { 
                        data: [old[0], old[1], old[2], luma[0]] 
                    });
            }).collect::<()>();

        //  Save the final masked output
        combined.save(&format!("{}.png", basename))?;

        Ok(())
    }

    /// Saves the JPEG image inside the JPNG.
    /// 
    /// The basename given will have .jpg appended to the end,
    /// and it will overwrite the destination if it already
    /// exists.
    pub fn save_image(&self, basename: &str) -> Result<(), Error> {
        self.image.save(&format!("{}.jpg", basename))?;
        Ok(())
    }

    /// Saves the PNG alpha mask inside the JPNG.
    /// 
    /// The basename given will have .png appended to the end,
    /// and it will overwrite the destination if it already
    /// exists.
    pub fn save_mask(&self, basename: &str) -> Result<(), Error> {
        self.mask.save(&format!("{}.png", basename))?;
        Ok(())
    }
}