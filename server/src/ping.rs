use std::process::Command;
use crate::response::ReturnResponse;

pub fn ping(url: &str) -> Result<ReturnResponse, ReturnResponse> {
    let command = format!("ping -c 5 {}", url);
    println!("{:?}", command);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("failed to execute process")
        };

        let output_str = match String::from_utf8(output.stdout) {
            Ok(some) => some,
            Err(_) => return Err(ReturnResponse {status: 100, message: "Couldn't ping url.".to_owned(), data: None})
        };

        if output_str == "".to_owned() {
            return Err(ReturnResponse { status: 300, message: "Couldn't get url data.".to_owned(), data: None })
        }

        Ok(ReturnResponse { status: 499, message: "Url is up and running".to_owned(), data: Some(output_str) })
}