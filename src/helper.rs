use std::process::Command;

pub fn curl_get(url: &str) -> Vec<u8> {

    let output = Command::new("curl")
        .arg(url)
        .output()
        .expect("failed to execute curl");

    output.stdout
}

fn make_header(status_line: &[u8]) -> Vec<u8> {
    let ok_status = b"HTTP/1.1 200 OK\r\n";
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
