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
pub struct Event {
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
    pub device: Option<Box<crate::models::EventDevice>>,
    /// The [type/class](/integrations/event/#device-class) of the event to set the icon in the frontend. The `device_class` can be `null`. (Default: `None)`
    #[serde(rename = "device_class", skip_serializing_if = "Option::is_none")]
    pub device_class: Option<String>,
    /// Flag which defines if the entity should be enabled when first added. (Default: `true)`
    #[serde(rename = "enabled_by_default", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// The encoding of the published messages. (Default: `utf-8)`
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)`
    #[serde(rename = "entity_category", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
    /// A list of valid `event_type` strings.
    #[serde(rename = "event_types")]
    pub event_types: Vec<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attributes_template", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attributes_topic", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// The name to use when displaying this event. (Default: `MQTT Event)`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// The MQTT topic subscribed to receive JSON event payloads. The JSON payload should contain the `event_type` element. The event type should be one of the configured `event_types`. (Default: `None)`
    #[serde(rename = "state_topic")]
    pub state_topic: String,
    /// An ID that uniquely identifies this event entity. If two events have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the value and render it to a valid JSON event payload. If the template throws an error, the current state will be used instead.
    #[serde(rename = "value_template", skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

impl Event {
    pub fn new(event_types: Vec<String>, state_topic: String) -> Event {
        Event {
            availability: None,
            availability_mode: None,
            availability_template: None,
            availability_topic: None,
            device: None,
            device_class: None,
            enabled_by_default: None,
            encoding: None,
            entity_category: None,
            event_types,
            json_attributes_template: None,
            json_attributes_topic: None,
            name: None,
            object_id: None,
            payload_available: None,
            payload_not_available: None,
            qos: None,
            state_topic,
            unique_id: None,
            value_template: None,
        }
    }
}


