/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MetaV1WatchEvent : Event represents a single event to a watched resource.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaV1WatchEvent {
  /// Object is:  * If Type is Added or Modified: the new state of the object.  * If Type is Deleted: the state of the object immediately before deletion.  * If Type is Error: *Status is recommended; other types may make sense    depending on context.
  #[serde(rename = "object")]
  object: ::models::RuntimeRawExtension,
  #[serde(rename = "type")]
  _type: String
}

impl MetaV1WatchEvent {
  /// Event represents a single event to a watched resource.
  pub fn new(object: ::models::RuntimeRawExtension, _type: String) -> MetaV1WatchEvent {
    MetaV1WatchEvent {
      object: object,
      _type: _type
    }
  }

  pub fn set_object(&mut self, object: ::models::RuntimeRawExtension) {
    self.object = object;
  }

  pub fn with_object(mut self, object: ::models::RuntimeRawExtension) -> MetaV1WatchEvent {
    self.object = object;
    self
  }

  pub fn object(&self) -> &::models::RuntimeRawExtension {
    &self.object
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> MetaV1WatchEvent {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}


