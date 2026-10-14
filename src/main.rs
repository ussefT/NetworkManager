mod network;
mod ui;
use std::io;
use std::{thread, time::Duration};
use sysinfo::Networks;

fn main() {

    ui::ui();

    println!("Welcome to the network monitor!");
    let mut input = String::new();
    let interface_name = network::get_interface();

    if interface_name.is_empty() {
        println!("No network interfaces found.");
        return;
    }

    for (index, name) in interface_name.iter().enumerate() {
        println!("{index} - {name}");
    }

    println!("Select an interface by index:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let index = match input.trim().parse::<usize>() {
        Ok(value) => value,
        Err(_) => {
            println!("Please enter a valid interface index.");
            return;
        }
    };

    let Some(selected_name) = interface_name.get(index) else {
        println!("Interface index {index} does not exist.");
        return;
    };

    println!("Selected interface: {selected_name}");
    let mut networks = Networks::new_with_refreshed_list();

    loop {
        thread::sleep(Duration::from_secs(1));
        let res = network::network(&mut networks);

        if let Some(value) = res.get(selected_name) {
            let download = value.get("DS").map_or("0", String::as_str);
            let upload = value.get("US").map_or("0", String::as_str);
            let download_speed = value.get("SD").map_or("0", String::as_str);
            let upload_speed = value.get("SU").map_or("0", String::as_str);

            println!(
                "{selected_name}: download {download} MB/s ({download_speed} bytes/s), \
                 upload {upload} MB/s ({upload_speed} bytes/s)"
            );
        } else {
            println!("No data available for {selected_name}");
            break;
        }
    }
}