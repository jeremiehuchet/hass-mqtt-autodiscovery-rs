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
pub struct LawnMowerAvailabilityInner {
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// An MQTT topic subscribed to receive availability (online/offline) updates.
    #[serde(rename = "topic")]
    pub topic: String,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the device's availability from the `topic`. To determine the device's availability, the result of this template will be compared to `payload_available` and `payload_not_available`.
    #[serde(rename = "value_template", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl LawnMowerAvailabilityInner {
    pub fn new(topic: String) -> LawnMowerAvailabilityInner {
        LawnMowerAvailabilityInner {
            payload_available: None,
            payload_not_available: None,
            topic,
            value_template: None,
        }
    }
}


