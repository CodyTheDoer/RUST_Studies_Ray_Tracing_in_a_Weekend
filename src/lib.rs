mod rtvec3;

pub use rtvec3::*;

use std::fs::File;
use std::io::{self, Write};

pub fn write_color(
    pixel: RtVec3,
    file: &mut File,
) -> std::io::Result<()> {
    // Pixel Algo
    let r: f32 = pixel.x();
    let g: f32 = pixel.y();
    let b: f32 = pixel.z();

    let ir: u32 = (255.999 * r) as u32;
    let ig: u32 = (255.999 * g) as u32;
    let ib: u32 = (255.999 * b) as u32;

    let pixel_triplets = format!("{} {} {} \n",ir , ig, ib);
    file.write_all(pixel_triplets.as_bytes())?;
    
    Ok(())
}
