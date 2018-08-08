use std::process::Command;

pub fn curl_get(url: &str) -> Vec<u8> {

    let output = Command::new("curl")
        .arg(url)
        .output()
        .expect("failed to execute curl");

    output.stdout
}
