/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Event : Event is a report of an event somewhere in the cluster.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1Event {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// The number of times this event has occurred.
  #[serde(rename = "count")]
  count: Option<i32>,
  /// The time at which the event was first recorded. (Time of server receipt is in TypeMeta.)
  #[serde(rename = "firstTimestamp")]
  first_timestamp: Option<::models::MetaV1Time>,
  /// The object that this event is about.
  #[serde(rename = "involvedObject")]
  involved_object: ::models::V1ObjectReference,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// The time at which the most recent occurrence of this event was recorded.
  #[serde(rename = "lastTimestamp")]
  last_timestamp: Option<::models::MetaV1Time>,
  /// A human-readable description of the status of this operation.
  #[serde(rename = "message")]
  message: Option<String>,
  /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
  #[serde(rename = "metadata")]
  metadata: ::models::MetaV1ObjectMeta,
  /// This should be a short, machine understandable string that gives the reason for the transition into the object's current status.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// The component reporting this event. Should be a short machine understandable string.
  #[serde(rename = "source")]
  source: Option<::models::V1EventSource>,
  /// Type of this event (Normal, Warning), new types could be added in the future
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1Event {
  /// Event is a report of an event somewhere in the cluster.
  pub fn new(involved_object: ::models::V1ObjectReference, metadata: ::models::MetaV1ObjectMeta) -> V1Event {
    V1Event {
      api_version: None,
      count: None,
      first_timestamp: None,
      involved_object: involved_object,
      kind: None,
      last_timestamp: None,
      message: None,
      metadata: metadata,
      reason: None,
      source: None,
      _type: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> V1Event {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_count(&mut self, count: i32) {
    self.count = Some(count);
  }

  pub fn with_count(mut self, count: i32) -> V1Event {
    self.count = Some(count);
    self
  }

  pub fn count(&self) -> Option<&i32> {
    self.count.as_ref()
  }

  pub fn reset_count(&mut self) {
    self.count = None;
  }

  pub fn set_first_timestamp(&mut self, first_timestamp: ::models::MetaV1Time) {
    self.first_timestamp = Some(first_timestamp);
  }

  pub fn with_first_timestamp(mut self, first_timestamp: ::models::MetaV1Time) -> V1Event {
    self.first_timestamp = Some(first_timestamp);
    self
  }

  pub fn first_timestamp(&self) -> Option<&::models::MetaV1Time> {
    self.first_timestamp.as_ref()
  }

  pub fn reset_first_timestamp(&mut self) {
    self.first_timestamp = None;
  }

  pub fn set_involved_object(&mut self, involved_object: ::models::V1ObjectReference) {
    self.involved_object = involved_object;
  }

  pub fn with_involved_object(mut self, involved_object: ::models::V1ObjectReference) -> V1Event {
    self.involved_object = involved_object;
    self
  }

  pub fn involved_object(&self) -> &::models::V1ObjectReference {
    &self.involved_object
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> V1Event {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_last_timestamp(&mut self, last_timestamp: ::models::MetaV1Time) {
    self.last_timestamp = Some(last_timestamp);
  }

  pub fn with_last_timestamp(mut self, last_timestamp: ::models::MetaV1Time) -> V1Event {
    self.last_timestamp = Some(last_timestamp);
    self
  }

  pub fn last_timestamp(&self) -> Option<&::models::MetaV1Time> {
    self.last_timestamp.as_ref()
  }

  pub fn reset_last_timestamp(&mut self) {
    self.last_timestamp = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> V1Event {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::MetaV1ObjectMeta) {
    self.metadata = metadata;
  }

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> V1Event {
    self.metadata = metadata;
    self
  }

  pub fn metadata(&self) -> &::models::MetaV1ObjectMeta {
    &self.metadata
  }


  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> V1Event {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_source(&mut self, source: ::models::V1EventSource) {
    self.source = Some(source);
  }

  pub fn with_source(mut self, source: ::models::V1EventSource) -> V1Event {
    self.source = Some(source);
    self
  }

  pub fn source(&self) -> Option<&::models::V1EventSource> {
    self.source.as_ref()
  }

  pub fn reset_source(&mut self) {
    self.source = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1Event {
    self._type = Some(_type);
    self
  }

  pub fn _type(&self) -> Option<&String> {
    self._type.as_ref()
  }

  pub fn reset__type(&mut self) {
    self._type = None;
  }

}



