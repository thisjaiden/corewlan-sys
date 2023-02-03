use std::hint::black_box;
use crate::*;

#[test]
fn initialize_shared_client() {
    let shared_client = CWWiFiClient::sharedWiFiClient();
    black_box(shared_client);
}

#[test]
fn initialize_client() {
    let client = CWWiFiClient::init();
    black_box(client);
}

#[test]
fn get_default_interface() {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface();
    black_box(interface);
}

#[test]
fn get_named_interface() {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface_with_name("en0");
    black_box(interface);
}

#[test]
fn get_all_interfaces() {
    let client = CWWiFiClient::sharedWiFiClient();
    let interfaces = client.interfaces();
    black_box(interfaces);
}

#[test]
fn get_all_interface_names() {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface_names = client.interfaceNames();
    black_box(interface_names);
}

#[test]
#[ignore = "changes user hardware config"]
fn disassociate_from_network() {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface();
    interface.disassociate();
}

#[test]
fn get_networks() -> Result<(), ()> {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface();
    let networks = interface.scanForNetworksWithName(None)?;
    black_box(networks);
    Ok(())
}

#[test]
#[ignore = "requires nearby networks"]
fn get_network_properties() -> Result<(), ()> {
    let client = CWWiFiClient::sharedWiFiClient();
    let interface = client.interface();
    // This test fails if there are no networks found
    let networks = interface.scanForNetworksWithName(None)?;
    let network = &networks[0];
    black_box(network.beaconInterval());
    black_box(network.bssid());
    black_box(network.countryCode());
    black_box(network.ibss());
    black_box(network.noiseMeasurement());
    black_box(network.rssiValue());
    black_box(network.ssid());
    Ok(())
}
