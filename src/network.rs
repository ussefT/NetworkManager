use std::collections::BTreeMap;
use sysinfo::Networks;

// AI do it my order

// Get interface
pub fn get_interface() -> Vec<String> {
    let networks = Networks::new_with_refreshed_list();

    networks.iter().map(|(name, _)| name.to_string()).collect()
}

pub fn network(networks: &mut Networks) -> BTreeMap<String, BTreeMap<String, String>> {
    // BTreeMap keeps interface names and statistic keys in sorted order.
    let mut hash: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();

    networks.refresh(true);

    for (name, data) in networks.iter() {
        let mut interface: BTreeMap<String, String> = BTreeMap::new();

        let download = data.received();
        let upload = data.transmitted();
        let download_mb = download as f64 / 1_000_000.0;
        let upload_mb = upload as f64 / 1_000_000.0;

        // Download size live.
        interface.insert("DS".to_string(), format!("{download_mb:.2}"));

        // Upload size live.
        interface.insert("US".to_string(), format!("{upload_mb:.2}"));

        // Speed download in MB/s.
        interface.insert("SD".to_string(), format!("{download_mb:.2}"));

        // Speed upload in MB/s.
        interface.insert("SU".to_string(), format!("{upload_mb:.2}"));

        // All size download.
        interface.insert("AD".to_string(), data.total_received().to_string());

        // All size upload.
        interface.insert("AU".to_string(), data.total_transmitted().to_string());

        hash.insert(name.to_string(), interface);
    }

    hash
}
