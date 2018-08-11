use std::process::Command;
use std::time::{UNIX_EPOCH, SystemTime};
use image::{ImageBuffer, ImageRgb8, Rgb, RgbImage};
use super::color_table::{Color, ColorTable};

pub fn curl_get(url: &str) -> Vec<u8> {

    let output = Command::new("curl")
        .arg(url)
        .output()
        .expect("failed to execute curl");

    output.stdout
}

fn make_header(status_line: &[u8]) -> Vec<u8> {
    let mut header: Vec<u8> = Vec::new();
    header.extend_from_slice(&status_line);
    header
}

pub fn make_ok_header() -> Vec<u8> {
    let ok_status = b"HTTP/1.1 200 OK\r\n";
    let header = make_header(ok_status);
    end_header(header)
}

pub fn end_header(header: Vec<u8>) -> Vec<u8> {
    let rn = b"\r\n";
    let mut new_header = header.clone(); 
    new_header.extend_from_slice(rn);
    new_header
}

pub fn make_not_found_header() -> Vec<u8> {
    let not_found_status = b"HTTP/1/1 404 NOT FOUND\r\n";
    let header = make_header(not_found_status);
    end_header(header)
}

pub fn make_json_header() -> Vec<u8> {
    let mut header = make_ok_header();
    let header_json = b"Content-Type:application/json;charset=UTF-8\r\n";
    header.extend_from_slice(header_json);

    end_header(header)
}

pub fn make_image_header(size: usize) -> Vec<u8> {
    let mut header = make_ok_header();
    let content_type = b"Content-Type:image/png\r\n";
    let content_length = format!("Content-Length:{}\r\n",size);
    header.extend_from_slice(content_length.as_bytes());
    header.extend_from_slice(content_type);
    end_header(header)
}

pub fn convert_to_bytes(img: &RgbImage) -> Vec<u8> {
    // assuming its 16 colors 1111
    let mut ct = ColorTable::new();

    let (width, height) = img.dimensions();

    // 0 3 6
    // 1 4 7
    // 2 5 8

    let mut byte: u8 = 0;
    let mut count = 0;
    let mut bytes: Vec<u8> = Vec::new();

    for x in 0..width {
        for y in 0..height{
            let c = img.get_pixel(x, y).data;
            let c: Color = Color::new(c[0], c[1], c[2]);
            let i = ct.push_if_unique(c);

            if count % 2 == 1 {
                let new_byte = i | byte << 4;
                bytes.push(new_byte);
            } else {
                byte = i;
            }
            count += 1;
        }
    }

    let mut overall = ct.output_file_header();
    overall.extend(bytes);
    overall
}

pub fn save_as_image(bytes: &[u8], width: u32, height: u32) {
    let mut imgbuf = RgbImage::new(width, height);
    // again assuming that this is 16 colors
    // color header will be 16 * 3 = 48 bytes
    let header_length = 16 * 3;

    let table_bytes = &bytes[0..header_length];
    let ct = ColorTable::from_file_header(table_bytes.to_vec());

    let pixel_bytes = &bytes[header_length..];
    // each pixel is saved like following
    // 00001111
    let mut count = 0;

    for b in pixel_bytes {
        let zero = ct.get(b >> 4).to_u8(); // left shift 4;
        let zero_pixel = Rgb(zero);
        let one = ct.get(b & 15).to_u8(); // only the the last 4; like as u4
        let one_pixel = Rgb(one);

        let (x, y) = index_to_pos(count * 2, height);
        imgbuf.put_pixel(x as u32, y as u32, zero_pixel) ;
        let (x, y) = index_to_pos(count * 2 + 1, height);
        imgbuf.put_pixel(x as u32, y as u32, one_pixel) ;

        count += 1;
    }

    ImageRgb8(imgbuf.clone()).save("current.png").unwrap();
    let temp_file_name = format!("archive_{}.png",SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    ImageRgb8(imgbuf).save(temp_file_name).unwrap();
    println!("saved image");
}

fn index_to_pos (i: usize, mod_value:u32) -> (usize, usize) {
    let index = i as u32;

    let x = (index / mod_value) as usize;// height
    let y = (index % mod_value) as usize; 

    (x, y)
}

pub fn log(new_line: String) {
    // IMPLEMENT log    
}

