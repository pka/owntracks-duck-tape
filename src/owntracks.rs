use crate::db::{deserialize_dict_to_string, serialize_raw_json};
use serde::{Deserialize, Serialize};

/// OwnTracks JSON message
/// <https://owntracks.org/booklet/tech/json/>
#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::large_enum_variant)]
#[serde(tag = "_type")]
#[serde(rename_all = "lowercase")]
pub enum Message {
    Beacon,
    Card,
    Cmd,
    Configuration,
    Encrypted,
    Location(Location),
    Lwt,
    Request,
    Status,
    Steps,
    Transition,
    Waypoint,
    Waypoints,
}

/// OwnTracks location
#[derive(Serialize, Deserialize, Debug)]
pub struct OtLocation {
    /// Tracker ID used to display the initials of a user (iOS,Android/string/optional) required for http mode
    #[serde(default)] // Make optional regarding to spec
    pub tid: String,
    /// UNIX epoch timestamp in seconds of the location fix (iOS,Android/integer/epoch/required)
    #[serde(rename = "tst")]
    pub ts: i64,
    /// velocity (iOS,Android/integer/kmh/optional)
    #[serde(rename = "vel")]
    pub velocity: Option<u16>,
    /// latitude (iOS,Android/float/degree/required)
    pub lat: f32,
    /// longitude (iOS,Android/float/degree/required)
    pub lon: f32,
    /// Altitude measured above sea level (iOS,Android/integer/meters/optional)
    pub alt: Option<u16>,
    /// Accuracy of the reported location in meters without unit (iOS,Android/integer/meters/optional)
    #[serde(rename = "acc")]
    pub accuracy: Option<u32>,
    /// vertical accuracy of the alt element (iOS/integer/meters/optional)
    #[serde(rename = "vac")]
    pub v_accuracy: Option<i16>,
    /// Device battery level (iOS,Android/integer/percent/optional)
    #[serde(rename = "batt")]
    pub batt_level: Option<u8>,
    /// Battery Status 0=unknown, 1=unplugged, 2=charging, 3=full (iOS, Android)
    #[serde(rename = "bs")]
    pub batt_status: u8,
    /// Course over ground (iOS/integer/degree/optional)
    pub cog: Option<i16>,
    /// radius around the region when entering/leaving (iOS/integer/meters/optional)
    pub rad: Option<u32>,
    /// trigger for the location report (iOS,Android/string/optional)
    /// * `p`: ping issued randomly by background task (iOS,Android)
    /// * `c`: circular region enter/leave event (iOS,Android)
    /// * `C`: circular region enter/leave event for +follow regions (iOS)
    /// * `b`: beacon region enter/leave event (iOS)
    /// * `r`: response to a reportLocation cmd message (iOS,Android)
    /// * `u`: manual publish requested by the user (iOS,Android)
    /// * `t`: timer based publish in move move (iOS)
    /// * `v`: updated by Settings/Privacy/Locations Services/System Services/Frequent Locations monitoring (iOS)
    #[serde(rename = "t")]
    pub trigger: Option<String>,
    /// barometric pressure (iOS/float/kPa/optional/extended data)
    #[serde(rename = "p")]
    pub pressure: Option<f32>,
    /// point of interest name (iOS/string/optional)
    pub poi: Option<String>,
    /// Internet connectivity status (route to host) when the message is created (iOS,Android/string/optional/extended data)
    /// * `w`: phone is connected to a WiFi connection (iOS,Android)
    /// * `o`: phone is offline (iOS,Android)
    /// * `m`: mobile data (iOS,Android)
    #[serde(rename = "conn")]
    pub conn_status: Option<String>,
    /// name of the tag (iOS/string/optional)
    pub tag: Option<String>,
    /// (only in HTTP payloads) contains the original publish topic (e.g. owntracks/jane/phone). (iOS,Android >= 2.4,string)
    pub topic: Option<String>,
    /// contains a list of regions the device is currently in (e.g. ["Home","Garage"]). Might be empty. (iOS,Android/list of strings/optional)
    pub inregions: Option<String>,
    /// contains a list of region IDs the device is currently in (e.g. ["6da9cf","3defa7"]). Might be empty. (iOS,Android/list of strings/optional)
    pub inrids: Option<String>,
    /// unique name of the WLAN. (iOS,string/optional)
    #[serde(rename = "SSID")]
    pub ssid: Option<String>,
    /// identifies the access point. (iOS,string/optional)
    #[serde(rename = "BSSID")]
    pub bssid: Option<String>,
    /// identifies the time at which the message is constructed (if it differs from tst, which is the timestamp of the GPS fix) (iOS,Android/integer/epoch/optional)
    pub created_at: Option<i64>,
    /// identifies the monitoring mode at which the message is constructed (significant=1, move=2) (iOS/integer/optional)
    #[serde(rename = "m")]
    pub mmode: Option<u8>,
    /// random identifier to be used by consumers to correlate & distinguish send/return messages (Android/string)
    #[serde(rename = "_id")]
    pub msg_id: String,
}

/// OwnTracks compatible location with custom annotations
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    /// Tracker ID used to display the initials of a user (iOS,Android/string/optional) required for http mode
    #[serde(default)] // Make optional regarding to spec
    pub tid: String,
    /// UNIX epoch timestamp in seconds of the location fix (iOS,Android/integer/epoch/required)
    #[serde(rename = "tst")]
    pub ts: i64,
    /// velocity (iOS,Android/integer/kmh/optional)
    #[serde(rename = "vel")]
    pub velocity: Option<u16>,
    /// latitude (iOS,Android/float/degree/required)
    pub lat: f32,
    /// longitude (iOS,Android/float/degree/required)
    pub lon: f32,
    /// Altitude measured above sea level (iOS,Android/integer/meters/optional)
    pub alt: Option<u16>,
    /// Accuracy of the reported location in meters without unit (iOS,Android/integer/meters/optional)
    #[serde(rename = "acc")]
    pub accuracy: Option<u32>,
    /// vertical accuracy of the alt element (iOS/integer/meters/optional)
    #[serde(rename = "vac")]
    pub v_accuracy: Option<i16>,
    /// Course over ground (iOS/integer/degree/optional)
    pub cog: Option<i16>,
    /// Additional parameters
    #[serde(
        flatten,
        serialize_with = "serialize_raw_json",
        deserialize_with = "deserialize_dict_to_string"
    )]
    pub annotations: String,
}
