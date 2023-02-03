#![allow(non_snake_case)]

use crate::raw::{ICWWiFiClient, INSString, ICWInterface, INSArray, NSString, INSError, INSSet, ICWNetwork, ICWChannel};

/// "A wrapper around the entire Wi-Fi subsystem that you use to access interfaces and set up event
/// notifications."
pub struct CWWiFiClient {
    raw: crate::raw::CWWiFiClient
}

impl CWWiFiClient {
    /// "The shared Wi-Fi client object."
    pub fn sharedWiFiClient() -> CWWiFiClient {
        unsafe { CWWiFiClient { raw: crate::raw::CWWiFiClient::sharedWiFiClient() } }
    }
    /// "Initializes a Wi-Fi client object."
    /// You probably want to use [CWWiFiClient::sharedWiFiClient].
    pub fn init() -> CWWiFiClient {
        unsafe {
            CWWiFiClient {
                raw: crate::raw::CWWiFiClient(
                    crate::raw::CWWiFiClient::init(
                        &crate::raw::CWWiFiClient::alloc()
                    )
                )
            }
        }
    }
    /// "Returns the default Wi-Fi interface."
    pub fn interface(&self) -> CWInterface {
        unsafe { CWInterface { raw: self.raw.interface() } }
    }
    /// "Returns the Wi-Fi interface with the given name."
    /// TODO: Test what happens when an invalid interface name is passed.
    pub fn interface_with_name(&self, name: &str) -> CWInterface {
        let out_name = &format!("{}\0", name);
        unsafe { CWInterface { raw: self.raw.interfaceWithName_(crate::raw::NSString(crate::raw::NSString::stringWithUTF8String(out_name.as_bytes()))) } }
    }
    /// "Returns all available Wi-Fi interfaces."
    pub fn interfaces(&self) -> Vec<CWInterface> {
        let mut final_interfaces = vec![];
        unsafe {
            let interfaces = self.raw.interfaces();
            let arr_len = <crate::raw::NSArray as INSArray<crate::raw::CWInterface>>::count(&interfaces);
            for i in 0..arr_len {
                final_interfaces.push(CWInterface { raw: crate::raw::CWInterface(<crate::raw::NSArray as INSArray<crate::raw::CWInterface>>::objectAtIndex_(&interfaces, i)) } );
            }
        }
        return final_interfaces;
    }
    /// "Returns the list of the names of available Wi-Fi interfaces."
    pub fn interfaceNames(&self) -> Vec<String> {
        let mut final_interface_names = vec![];
        unsafe {
            let interface_names = crate::raw::CWWiFiClient::interfaceNames();
            let arr_len = <crate::raw::NSArray as INSArray<crate::raw::NSString>>::count(&interface_names);
            for i in 0..arr_len {
                let nsstring = crate::raw::NSString(<crate::raw::NSArray as INSArray<crate::raw::NSString>>::objectAtIndex_(&interface_names, i));
                let cstring = std::ffi::CStr::from_ptr(nsstring.UTF8String());
                let new_utf8 = cstring.to_str().unwrap();
                let safe_utf8 = String::from(new_utf8);
                final_interface_names.push(safe_utf8);
            }
        }
        return final_interface_names;
    }
}

/// "Encapsulates an IEEE 802.11 interface."
pub struct CWInterface {
    raw: crate::raw::CWInterface
}

impl CWInterface {
    /// "Scans for networks."
    /// 
    /// Scanning more than once every 10 seconds leads to an error.
    pub fn scanForNetworksWithName(&self, name: Option<String>) -> Result<Vec<CWNetwork>, ()> {
        let mut final_networks = vec![];
        unsafe {
            if let Some(name) = name {
                let modified_name = &format!("{}\0", name);
                let network_name = crate::raw::NSString(crate::raw::NSString::stringWithUTF8String(modified_name.as_bytes()));
                let potential_error = &mut crate::raw::NSError::alloc() as *mut crate::raw::NSError;
                let networks = self.raw.scanForNetworksWithName_error_(network_name, potential_error);
                if potential_error.as_ref().unwrap().code() != 0 {
                    // TODO: proper error codes!
                    println!("ERROR CODE #{}", potential_error.as_ref().unwrap().code());
                    return Err(());
                }
                let networks_nsarr = <crate::raw::NSSet as INSSet<crate::raw::CWNetwork>>::allObjects(&networks);
                let arr_len = <crate::raw::NSArray as INSArray<crate::raw::CWNetwork>>::count(&networks_nsarr);
                for i in 0..arr_len {
                    let instance = crate::raw::CWNetwork(<crate::raw::NSArray as INSArray<crate::raw::CWNetwork>>::objectAtIndex_(&networks_nsarr, i));
                    final_networks.push(CWNetwork { raw: instance });
                }
            }
            else {
                let potential_error = &mut crate::raw::NSError::alloc() as *mut crate::raw::NSError;
                let networks = self.raw.scanForNetworks_error_(potential_error);
                if potential_error.as_ref().unwrap().code() != 0 {
                    // TODO: proper error codes!
                    println!("ERROR CODE #{}", potential_error.as_ref().unwrap().code());
                    return Err(());
                }
                let networks_nsarr = <crate::raw::NSSet as INSSet<crate::raw::CWNetwork>>::allObjects(&networks);
                let arr_len = <crate::raw::NSArray as INSArray<crate::raw::CWNetwork>>::count(&networks_nsarr);
                for i in 0..arr_len {
                    let instance = crate::raw::CWNetwork(<crate::raw::NSArray as INSArray<crate::raw::CWNetwork>>::objectAtIndex_(&networks_nsarr, i));
                    final_networks.push(CWNetwork { raw: instance });
                }
            }
        }
        return Ok(final_networks);
    }
    /// "Disassociates from the current network."
    pub fn disassociate(&self) {
        unsafe { self.raw.disassociate() }
    }
}

/// "Encapsulates an IEEE 802.11 network, providing read-only accessors to various properties of the
/// network."
pub struct CWNetwork {
    raw: crate::raw::CWNetwork
}

impl CWNetwork {
    /// "Method for determining which security types a network supports."
    pub fn supportsSecurity(&self, security: CWSecurity) -> bool {
        unsafe { return self.raw.supportsSecurity_(security as i64); }
    }
    /// "Method for determining which PHY modes a network supports."
    pub fn supportsPHYMode(&self, mode: CWPHYMode) -> bool {
        unsafe { return self.raw.supportsPHYMode_(mode as i64); }
    }
    /// "The beacon interval (ms) for the network."
    pub fn beaconInterval(&self) -> i64 {
        unsafe { return self.raw.beaconInterval(); }
    }
    /// "The basic service set identifier (BSSID) for the network."
    /// 
    /// This value is not typically returned. Getting it to work is finicky. Try googling 'bssid
    /// CoreWLAN macOS' and pray.
    /// 
    /// Further notes:
    /// 
    /// Afaik if the following are true this should return a valid value:
    /// - CoreLocation::CLLocationManager::requestAlwaysAuthorization()
    /// - Executable is signed
    /// 
    /// I've been unable to test/reproduce this.
    pub fn bssid(&self) -> Option<String> {
        unsafe {
            // SAFTEY: This block checks if a string or null pointer was returned manually. It's not
            // clean, it's not the best soulution. It works. This shouldn't cause issues, but if it
            // can, or you have a better soulution, please open an issue immediately.
            // attempt to grab the bssid string
            let nsstring = self.raw.bssid();
            // get a pointer to our theoretical string
            let raw_val = &nsstring as *const NSString;
            // convert it to a pointer to a u8 (this is the danger)
            let to_u8 = raw_val as *const u8;
            // check if the value stored there = 0
            if to_u8.as_ref().unwrap() == &0 {
                // then there's not a string!
                return None;
            }
            // otherwise there is a string, ignore the pointer nonsense and carry on
            let cstring = std::ffi::CStr::from_ptr(nsstring.UTF8String());
            return Some(cstring.to_str().unwrap().clone().to_string());
        }
    }
    /// "The country code (ISO/IEC 3166-1:1997) for the network."
    /// 
    /// Requesting this information also requires location services permissions. See
    /// [CWNetwork::bssid] for how you might get this information.
    pub fn countryCode(&self) -> Option<String> {
        unsafe {
            // SAFTEY: This block checks if a string or null pointer was returned manually. It's not
            // clean, it's not the best soulution. It works. This shouldn't cause issues, but if it
            // can, or you have a better soulution, please open an issue immediately.
            let nsstring = self.raw.countryCode();
            // get a pointer to our theoretical string
            let raw_val = &nsstring as *const NSString;
            // convert it to a pointer to a u8 (this is the danger)
            let to_u8 = raw_val as *const u8;
            // check if the value stored there = 0
            if to_u8.as_ref().unwrap() == &0 {
                // then there's not a string!
                return None;
            }
            let cstring = std::ffi::CStr::from_ptr(nsstring.UTF8String());
            let new_utf8 = cstring.to_str().unwrap();
            return Some(String::from(new_utf8));
        }
    }
    /// "The network is an IBSS network."
    /// 
    /// IBSS networks are essentially peer-to-peer networks.
    pub fn ibss(&self) -> bool {
        unsafe { return self.raw.ibss(); }
    }
    /// "The aggregate noise measurement (dBm) for the network."
    pub fn noiseMeasurement(&self) -> i64 {
        unsafe { return self.raw.noiseMeasurement(); }
    }
    /// "The aggregate received signal strength indication (RSSI) measurement (dBm) for the
    /// network."
    pub fn rssiValue(&self) -> i64 {
        unsafe { return self.raw.rssiValue(); }
    }
    /// "The service set identifier (SSID) for the network."
    pub fn ssid(&self) -> String {
        unsafe {
            let nsstring = self.raw.ssid();
            let cstring = std::ffi::CStr::from_ptr(nsstring.UTF8String());
            let new_utf8 = cstring.to_str().unwrap();
            // SAFTEY: There is no promise on the lifetime of the returned NSString, so we create a
            // new string using the data while we know it's good.
            return String::from(new_utf8);
        }
    }
    /// "The channel for the network."
    pub fn wlanChannel(&self) -> CWChannel {
        let (number, width, band);
        unsafe {
            let raw_channel = self.raw.wlanChannel();
            number = raw_channel.channelNumber();
            // The OS has "Unknown" values for both these enums, and so should never return an
            // invalid value to us.
            width = CWChannelWidth::try_from_i64(raw_channel.channelWidth()).unwrap();
            band = CWChannelBand::try_from_i64(raw_channel.channelBand()).unwrap();
        }
        return CWChannel { number, width, band };
    }

}

#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// "CoreWLAN security types."
pub enum CWSecurity {
    /// "Open System authentication."
    None = 0,
    /// "WEP security."
    WEP = 1,
    /// "WPA Personal authentication."
    WPAPersonal = 2,
    /// "WPA/WPA2 Personal authentication."
    WPAPersonalMixed = 3,
    /// "WPA2 Personal authentication."
    WPA2Personal = 4,
    /// "Personal authentication."
    Personal = 5,
    /// "Dynamic WEP security."
    DynamicWEP = 6,
    /// "WPA Enterprise authentication."
    WPAEnterprise = 7,
    /// "WPA/WPA2 Enterprise authentication."
    WPAEnterpriseMixed = 8,
    /// "WPA2 Enterprise authentication."
    WPA2Enterprise = 9,
    /// "Enterprise authentication."
    Enterprise = 10,
    /// "WPA3 Personal authentication."
    WPA3Personal = 11,
    /// "WPA3 Enterprise authentication."
    WPA3Enterprise = 12,
    /// "WPA3 Transition (WPA3/WPA2 Personal) authentication."
    WPA3Transition = 13,
    /// "Unknown security type."
    Unknown = 9223372036854775807,
}

#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// "CoreWLAN physical layer modes."
pub enum CWPHYMode {
    /// "No specified mode."
    None = 0,
    /// "IEEE 802.11a PHY."
    M11a = 1,
    /// "IEEE 802.11b PHY."
    M11b = 2,
    /// "IEEE 802.11g PHY."
    M11g = 3,
    /// "IEEE 802.11n PHY."
    M11n = 4,
    /// "IEEE 802.11ac PHY."
    M11ac = 5,
    // Labeled weird in Apple's documentation, maybe normal, maybe not?
    M11ax = 6,
}

pub struct CWChannel {
    // ???
    pub number: i64,
    /// Specifies the width of this channel in MHz
    pub width: CWChannelWidth,
    /// Specifies the 2.4 or 5GHz band
    pub band: CWChannelBand,
}

#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// "The channel width."
pub enum CWChannelWidth {
    /// "Unknown channel width."
    Unknown = 0,
    /// "20MHz channel width."
    W20MHz = 1,
    /// "40MHz channel width."
    W40MHz = 2,
    /// "80MHz channel width."
    W80MHz = 3,
    /// "160MHz channel width."
    W160MHz = 4,
}

impl CWChannelWidth {
    pub fn try_from_i64(data: i64) -> Option<CWChannelWidth> {
        match data {
            x if x == Self::Unknown as i64 => Some(Self::Unknown),
            x if x == Self::W20MHz as i64 => Some(Self::W20MHz),
            x if x == Self::W40MHz as i64 => Some(Self::W40MHz),
            x if x == Self::W80MHz as i64 => Some(Self::W80MHz),
            x if x == Self::W160MHz as i64 => Some(Self::W160MHz),
            _ => None
        }
    }
}

#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// "The channel band."
pub enum CWChannelBand {
    /// "Unknown channel band."
    Unknown = 0,
    /// "2.4GHz channel band."
    B2GHz = 1,
    /// "5GHz channel band."
    B5GHz = 2,
}

impl CWChannelBand {
    pub fn try_from_i64(data: i64) -> Option<CWChannelBand> {
        match data {
            x if x == Self::Unknown as i64 => Some(Self::Unknown),
            x if x == Self::B2GHz as i64 => Some(Self::B2GHz),
            x if x == Self::B5GHz as i64 => Some(Self::B5GHz),
            _ => None
        }
    }
}
