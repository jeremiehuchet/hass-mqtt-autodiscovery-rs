/*
 * Data structures for Home Assistant MQTT discovery
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FanDevice {
    /// A link to the webpage that can manage the configuration of this device. Can be either an `http://`, `https://` or an internal `homeassistant://` URL.
    #[serde(rename = "configuration_url", skip_serializing_if = "Option::is_none")]
    pub configuration_url: Option<String>,
    #[serde(rename = "connections", skip_serializing_if = "Option::is_none")]
    pub connections: Option<Box<crate::models::BinarySensorDeviceConnections>>,
    /// The hardware version of the device.
    #[serde(rename = "hw_version", skip_serializing_if = "Option::is_none")]
    pub hw_version: Option<String>,
    #[serde(rename = "identifiers", skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Box<crate::models::ButtonDeviceIdentifiers>>,
    /// The manufacturer of the device.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The model of the device.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The name of the device.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Suggest an area if the device isn’t in one yet.
    #[serde(rename = "suggested_area", skip_serializing_if = "Option::is_none")]
    pub suggested_area: Option<String>,
    /// The firmware version of the device.
    #[serde(rename = "sw_version", skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// Identifier of a device that routes messages between this device and Home Assistant. Examples of such devices are hubs, or parent devices of a sub-device. This is used to show device topology in Home Assistant.
    #[serde(rename = "via_device", skip_serializing_if = "Option::is_none")]
    pub via_device: Option<String>,
}

impl FanDevice {
    pub fn new() -> FanDevice {
        FanDevice {
            configuration_url: None,
            connections: None,
            hw_version: None,
            identifiers: None,
            manufacturer: None,
            model: None,
            name: None,
            suggested_area: None,
            sw_version: None,
            via_device: None,
        }
    }
}


