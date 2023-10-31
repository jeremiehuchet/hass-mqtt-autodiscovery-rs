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
pub struct Light {
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
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract blue color from the state payload value. Expected result of the template is an integer from 0-255 range.
    #[serde(rename = "blue_template", skip_serializing_if = "Option::is_none")]
    pub blue_template: Option<String>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract brightness from the state payload value. Expected result of the template is an integer from 0-255 range.
    #[serde(rename = "brightness_template", skip_serializing_if = "Option::is_none")]
    pub brightness_template: Option<String>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract color temperature from the state payload value. Expected result of the template is an integer representing mired units.
    #[serde(rename = "color_temp_template", skip_serializing_if = "Option::is_none")]
    pub color_temp_template: Option<String>,
    /// The [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) for *off* state changes. Available variables: `state` and `transition`.
    #[serde(rename = "command_off_template")]
    pub command_off_template: String,
    /// The [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) for *on* state changes. Available variables: `state`, `brightness`, `color_temp`, `red`, `green`, `blue`, `flash`, `transition` and `effect`. Values `red`, `green`, `blue`, `brightness` are provided as integers from range 0-255. Value of `color_temp` is provided as integer representing mired units.
    #[serde(rename = "command_on_template")]
    pub command_on_template: String,
    /// The MQTT topic to publish commands to change the light’s state.
    #[serde(rename = "command_topic")]
    pub command_topic: String,
    #[serde(rename = "device", skip_serializing_if = "Option::is_none")]
    pub device: Option<Box<crate::models::LightDevice>>,
    /// Flag which defines if the entity should be enabled when first added. (Default: `true)`
    #[serde(rename = "enabled_by_default", skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    /// The encoding of the payloads received and published messages. Set to `\"\"` to disable decoding of incoming payload. (Default: `utf-8)`
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// The [category](https://developers.home-assistant.io/docs/core/entity#generic-properties) of the entity. (Default: `None)`
    #[serde(rename = "entity_category", skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
    #[serde(rename = "effect_list", skip_serializing_if = "Option::is_none")]
    pub effect_list: Option<Box<crate::models::ButtonDeviceIdentifiers>>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract effect from the state payload value.
    #[serde(rename = "effect_template", skip_serializing_if = "Option::is_none")]
    pub effect_template: Option<String>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract green color from the state payload value. Expected result of the template is an integer from 0-255 range.
    #[serde(rename = "green_template", skip_serializing_if = "Option::is_none")]
    pub green_template: Option<String>,
    /// Defines a [template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract the JSON dictionary from messages received on the `json_attributes_topic`. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-template-configuration) documentation.
    #[serde(rename = "json_attributes_template", skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    /// The MQTT topic subscribed to receive a JSON dictionary payload and then set as sensor attributes. Usage example can be found in [MQTT sensor](/integrations/sensor.mqtt/#json-attributes-topic-configuration) documentation.
    #[serde(rename = "json_attributes_topic", skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    /// The maximum color temperature in mireds.
    #[serde(rename = "max_mireds", skip_serializing_if = "Option::is_none")]
    pub max_mireds: Option<i32>,
    /// The minimum color temperature in mireds.
    #[serde(rename = "min_mireds", skip_serializing_if = "Option::is_none")]
    pub min_mireds: Option<i32>,
    /// The name of the light. (Default: `MQTT Template Light)`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Used instead of `name` for automatic generation of `entity_id`
    #[serde(rename = "object_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    /// Flag that defines if the light works in optimistic mode. (Default: ``true` if no state topic or state template is defined, else `false`.)`
    #[serde(rename = "optimistic", skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    /// The payload that represents the available state. (Default: `online)`
    #[serde(rename = "payload_available", skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    /// The payload that represents the unavailable state. (Default: `offline)`
    #[serde(rename = "payload_not_available", skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    /// The maximum QoS level to be used when receiving and publishing messages.
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<i32>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract red color from the state payload value. Expected result of the template is an integer from 0-255 range.
    #[serde(rename = "red_template", skip_serializing_if = "Option::is_none")]
    pub red_template: Option<String>,
    /// The schema to use. Must be `template` to select the template schema. (Default: `default)`
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// [Template](/docs/configuration/templating/#using-templates-with-the-mqtt-integration) to extract state from the state payload value.
    #[serde(rename = "state_template", skip_serializing_if = "Option::is_none")]
    pub state_template: Option<String>,
    /// The MQTT topic subscribed to receive state updates.
    #[serde(rename = "state_topic", skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    /// An ID that uniquely identifies this light. If two lights have the same unique ID, Home Assistant will raise an exception.
    #[serde(rename = "unique_id", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
}

impl Light {
    pub fn new(command_off_template: String, command_on_template: String, command_topic: String) -> Light {
        Light {
            availability: None,
            availability_mode: None,
            availability_template: None,
            availability_topic: None,
            blue_template: None,
            brightness_template: None,
            color_temp_template: None,
            command_off_template,
            command_on_template,
            command_topic,
            device: None,
            enabled_by_default: None,
            encoding: None,
            entity_category: None,
            effect_list: None,
            effect_template: None,
            green_template: None,
            json_attributes_template: None,
            json_attributes_topic: None,
            max_mireds: None,
            min_mireds: None,
            name: None,
            object_id: None,
            optimistic: None,
            payload_available: None,
            payload_not_available: None,
            qos: None,
            red_template: None,
            schema: None,
            state_template: None,
            state_topic: None,
            unique_id: None,
        }
    }
}


