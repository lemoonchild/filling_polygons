use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24;

pub fn write_bmp_file(
    file_path: &str,    // Path to the output BMP file
    buffer: &[u32],     // Framebuffer pixel data
    width: usize,       // Width of the image
    height: usize,      // Height of the image
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    writer.flush()?;
    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>, // Buffered writer for the file
    width: usize,               // Width of the image
    height: usize,              // Height of the image
) -> std::io::Result<()> {
    let row_size = (width * BMP_BITS_PER_PIXEL / 8 + 3) & !3;
    let pixel_data_size = row_size * height;
    let file_size = BMP_HEADER_SIZE + pixel_data_size;

    // BMP signature
    file.write_all(b"BM")?;

    // File size
    file.write_all(&(file_size as u32).to_le_bytes())?;

    // Reserved bytes
    file.write_all(&[0u8; 4])?;

    // Pixel data offset
    file.write_all(&(BMP_PIXEL_OFFSET as u32).to_le_bytes())?;

    // DIB header size
    file.write_all(&(40u32).to_le_bytes())?;

    // Image width
    file.write_all(&(width as u32).to_le_bytes())?;

    // Image height (negative to indicate a top-down DIB)
    file.write_all(&(height as i32).to_le_bytes())?;

    // Color planes (must be 1)
    file.write_all(&(1u16).to_le_bytes())?;

    // Bits per pixel
    file.write_all(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes())?;

    // Compression method (none)
    file.write_all(&(0u32).to_le_bytes())?;

    // Image size (can be 0 for uncompressed images)
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;

    // Horizontal resolution (pixels per meter)
    file.write_all(&(2835u32).to_le_bytes())?;

    // Vertical resolution (pixels per meter)
    file.write_all(&(2835u32).to_le_bytes())?;

    // Number of colors in the palette (0 means no palette)
    file.write_all(&(0u32).to_le_bytes())?;

    // Important colors (0 means all colors are important)
    file.write_all(&(0u32).to_le_bytes())?;

    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>, // Buffered writer for the file
    buffer: &[u32],             // Framebuffer pixel data
    width: usize,               // Width of the image
    height: usize,              // Height of the image
) -> std::io::Result<()> {
    let row_size = (width * 3 + 3) & !3; // 3 bytes per pixel, aligned to 4 bytes
    let padding_size = row_size - width * 3;
    let padding = vec![0u8; padding_size];

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let b = (pixel & 0x0000_00FF) as u8;
            let g = ((pixel & 0x0000_FF00) >> 8) as u8;
            let r = ((pixel & 0x00FF_0000) >> 16) as u8;
            file.write_all(&[b, g, r])?;
        }
        file.write_all(&padding)?;
    }

    Ok(())
}