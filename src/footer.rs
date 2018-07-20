use super::JpngError;

use byteorder::{LittleEndian, ReadBytesExt};
use failure::Error;
use std::io::Cursor;
use std::mem;
use std::ops::Range;

#[derive(Copy, Clone, Debug)]
pub struct JpngFooter {
    pub image_size: u32,
    pub mask_size: u32,
    footer_size: u16,
    pub major_version: u8,
    pub minor_version: u8,
    identifier: u32,
}

const JPNG_IDENTIFIER: u32 = 0x4A504E47;

impl JpngFooter {
    pub fn new(data: &[u8]) -> Result<Self, Error> {
        //  Fail if the length of the data given doesn't match
        //  the footer's expected size.
        ensure!(data.len() == mem::size_of::<JpngFooter>(), 
            JpngError::InvalidFooterLen);
        
        let mut reader = Cursor::new(data);

        let footer = Self {
            image_size: reader.read_u32::<LittleEndian>()?,
            mask_size: reader.read_u32::<LittleEndian>()?,
            footer_size: reader.read_u16::<LittleEndian>()?,
            major_version: reader.read_u8()?,
            minor_version: reader.read_u8()?,
            identifier: reader.read_u32::<LittleEndian>()?,
        };

        //  Ensure that the identifier is correct before finishing.
        ensure!(footer.identifier == JPNG_IDENTIFIER, JpngError::InvalidImage);

        Ok(footer)
    }

    /// Gives a default representation of a JpngFooter.
    /// 
    /// Footer size is automatically set to the size of the struct,
    /// the major_version is set to 1 since it is the minimum version,
    /// and the identifier is set to its expected value.
    pub fn default() -> Self {
        Self {
            image_size: 0,
            mask_size: 0,
            footer_size: mem::size_of::<JpngFooter>() as u16,
            major_version: 1,
            minor_version: 0,
            identifier: JPNG_IDENTIFIER,
        }
    }

    /// Gives a string representation of the version as declared by
    /// the footer. For example, "1.0".
    pub fn version(&self) -> String {
        format!("{}.{}", self.major_version, self.minor_version)
    }

    /// Gives the range that the image component is in.
    pub fn image_range(&self) -> Range<usize> {
        Range {
            start: 0,
            end: self.image_size as usize,
        }
    }

    /// Gives the range that the mask component is in.
    pub fn mask_range(&self) -> Range<usize> {
        Range {
            start: self.image_size as usize,
            end: (self.image_size + self.mask_size) as usize,
        }
    }
}