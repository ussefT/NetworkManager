use std::collections::BTreeMap;
use sysinfo::Networks;

// AI do it my order

// Get interface
pub fn get_interface() -> Vec<String> {
    let networks = Networks::new_with_refreshed_list();

    networks.iter().map(|(name, _)| name.to_string()).collect()
}

fn check_speed(n: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = n as f64;
    let mut i = 0;

    while size >= 1024.0 && i < units.len() - 1 {
        size /= 1024.0;
        i += 1;
    }

    if i == 0 {
        format!("{n} {}", units[i])
    } else {
        format!("{size:.2} {}", units[i])
    }
}
pub fn network(networks: &mut Networks) -> BTreeMap<String, BTreeMap<String, String>> {
    // BTreeMap keeps interface names and statistic keys in sorted order.
    let mut hash: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

    networks.refresh(true);

    for (name, data) in networks.iter() {
        let mut interface: BTreeMap<String, String> = BTreeMap::new();

        let download = data.received();
        let upload = data.transmitted();

        // let download_mb = download as f64 / 1_000_000.0;
        // let upload_mb = upload as f64 / 1_000_000.0;

        // Download size live.
        interface.insert("DS".to_string(), check_speed(download));

        // Upload size live.
        interface.insert("US".to_string(), check_speed(upload));

        // Speed download in KB/s , MB/s, GB/s.
        interface.insert("SD".to_string(), check_speed(download));

        // Speed upload in KB/s, MB/s, GB/s.
        interface.insert("SU".to_string(), check_speed(upload));

        // All size download.
        interface.insert("AD".to_string(), data.total_received().to_string());

        // All size upload.
        interface.insert("AU".to_string(), data.total_transmitted().to_string());

        hash.insert(name.to_string(), interface);
    }

    hash
}
