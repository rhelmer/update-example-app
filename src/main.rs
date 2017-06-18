#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct ResultMessage {
    update_type: String,
    download_path: String,
}

fn main() {
    println!("\n-----------------------------------------------------\n");
    println!("This is an example app to exercise the update client!\n");
    println!("Checking for updates...\n");

    let output = Command::new("/tmp/update_client")
        .output()
        .expect("failed to execute process");

    let json_str = String::from_utf8_lossy(&output.stdout);
    let result_message: ResultMessage = serde_json::from_str(&json_str).unwrap();

    println!("{} update is ready to apply at {}",
        result_message.update_type,
        result_message.download_path);
}