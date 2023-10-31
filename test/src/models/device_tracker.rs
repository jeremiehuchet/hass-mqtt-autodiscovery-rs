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
pub struct DeviceTracker {
    /// A list of MQTT topics subscribed to receive availability (online/offline) updates. Must not be used together with `availability_topic`.
    #[serde(rename = "availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Vec<crate::models::AlarmControlPanelAvailabilityInner>>,
    /// When `availability` is configured, this controls the conditions needed to set the entity to `available`. Valid entries are `all`, `any`, and `latest`. If set to `all`, `payload_available` must be received on all configured availability topics before the entity is marked as online. If set to `any`, `payload_available` must be received on at least one configured availability topic before the entity is marked as online. If set to `latest`, the last `payload_available` or `payload_not_available` received on any configured availability topic controls the availability. (Default: `latest)`
    #[serde(rename = "availability_mode", skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract device's availability from the `availability_topic`. To determine the devices's availability result of this template will be compared to `payload_available` and `payload_not_available`.
    #[serde(rename = "availability_template", skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    /// The MQTT topic subscribed to receive availability (online/offline) updates. Must not be used together with `availability`.
    #[serde(rename = "availability_topic", skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::BinarySensorDevice>>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attributes_template", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary message containing device tracker attributes. This topic can be used to set the location of the device tracker under the following conditions: - If the attributes in the JSON message include `longitude`, `latitude`, and `gps_accuracy` (optional).  - If the device tracker is within a configured [zone](/integrations/zone/).  If these conditions are met, it is not required to configure `state_topic`.   Be aware that any location message received at `state_topic`  overrides the location received via `json_attributes_topic` until a message configured with `payload_reset` is received at `state_topic`. For a more generic usage example of the `json_attributes_topic`, refer to the [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attributes_topic", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// The name of the MQTT device_tracker.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload value that represents the 'home' state for the device. (Default: `home)`
    #[serde(rename = "payload_home", skip_serializing_if = "Option::is_none")]
    pub payload_home: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// The payload value that represents the 'not_home' state for the device. (Default: `not_home)`
    #[serde(rename = "payload_not_home", skip_serializing_if = "Option::is_none")]
    pub payload_not_home: Option<String>,
    /// The payload value that will have the device's location automatically derived from Home Assistant's zones. (Default: `None)`
    #[serde(rename = "payload_reset", skip_serializing_if = "Option::is_none")]
    pub payload_reset: Option<String>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// Attribute of a device tracker that affects state when being used to track a [person](/integrations/person/). Valid options are `gps`, `router`, `bluetooth`, or `bluetooth_le`.
    #[serde(rename = "source_type", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The MQTT topic subscribed to receive device tracker state changes. The states defined in `state_topic` override the location states defined by the `json_attributes_topic`. This state override is turned inactive if the `state_topic` receives a message containing `payload_reset`. The `state_topic` can only be omitted if `json_attributes_topic` is used.
    #[serde(rename = "state_topic", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    /// An ID that uniquely identifies this device_tracker. If two device_trackers have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) that returns a device tracker state.
    #[serde(rename = "value_template", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl DeviceTracker {
    pub fn new() -> DeviceTracker {
        DeviceTracker {
            availability: None,
            availability_mode: None,
            availability_template: None,
            availability_topic: None,
            device: None,
            json_attributes_template: None,
            json_attributes_topic: None,
            name: None,
            object_id: None,
            payload_available: None,
            payload_home: None,
            payload_not_available: None,
            payload_not_home: None,
            payload_reset: None,
            qos: None,
            source_type: None,
            state_topic: None,
            unique_id: None,
            value_template: None,
        }
    }
}


