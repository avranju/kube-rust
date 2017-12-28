/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// BatchV1JobCondition : JobCondition describes current state of a job.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchV1JobCondition {
  /// Last time the condition was checked.
  #[serde(rename = "lastProbeTime")]
  last_probe_time: Option<::models::MetaV1Time>,
  /// Last time the condition transit from one status to another.
  #[serde(rename = "lastTransitionTime")]
  last_transition_time: Option<::models::MetaV1Time>,
  /// Human readable message indicating details about last transition.
  #[serde(rename = "message")]
  message: Option<String>,
  /// (brief) reason for the condition's last transition.
  #[serde(rename = "reason")]
  reason: Option<String>,
  /// Status of the condition, one of True, False, Unknown.
  #[serde(rename = "status")]
  status: String,
  /// Type of job condition, Complete or Failed.
  #[serde(rename = "type")]
  _type: String
}

impl BatchV1JobCondition {
  /// JobCondition describes current state of a job.
  pub fn new(status: String, _type: String) -> BatchV1JobCondition {
    BatchV1JobCondition {
      last_probe_time: None,
      last_transition_time: None,
      message: None,
      reason: None,
      status: status,
      _type: _type
    }
  }

  pub fn set_last_probe_time(&mut self, last_probe_time: ::models::MetaV1Time) {
    self.last_probe_time = Some(last_probe_time);
  }

  pub fn with_last_probe_time(mut self, last_probe_time: ::models::MetaV1Time) -> BatchV1JobCondition {
    self.last_probe_time = Some(last_probe_time);
    self
  }

  pub fn last_probe_time(&self) -> Option<&::models::MetaV1Time> {
    self.last_probe_time.as_ref()
  }

  pub fn reset_last_probe_time(&mut self) {
    self.last_probe_time = None;
  }

  pub fn set_last_transition_time(&mut self, last_transition_time: ::models::MetaV1Time) {
    self.last_transition_time = Some(last_transition_time);
  }

  pub fn with_last_transition_time(mut self, last_transition_time: ::models::MetaV1Time) -> BatchV1JobCondition {
    self.last_transition_time = Some(last_transition_time);
    self
  }

  pub fn last_transition_time(&self) -> Option<&::models::MetaV1Time> {
    self.last_transition_time.as_ref()
  }

  pub fn reset_last_transition_time(&mut self) {
    self.last_transition_time = None;
  }

  pub fn set_message(&mut self, message: String) {
    self.message = Some(message);
  }

  pub fn with_message(mut self, message: String) -> BatchV1JobCondition {
    self.message = Some(message);
    self
  }

  pub fn message(&self) -> Option<&String> {
    self.message.as_ref()
  }

  pub fn reset_message(&mut self) {
    self.message = None;
  }

  pub fn set_reason(&mut self, reason: String) {
    self.reason = Some(reason);
  }

  pub fn with_reason(mut self, reason: String) -> BatchV1JobCondition {
    self.reason = Some(reason);
    self
  }

  pub fn reason(&self) -> Option<&String> {
    self.reason.as_ref()
  }

  pub fn reset_reason(&mut self) {
    self.reason = None;
  }

  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }

  pub fn with_status(mut self, status: String) -> BatchV1JobCondition {
    self.status = status;
    self
  }

  pub fn status(&self) -> &String {
    &self.status
  }


  pub fn set__type(&mut self, _type: String) {
    self._type = _type;
  }

  pub fn with__type(mut self, _type: String) -> BatchV1JobCondition {
    self._type = _type;
    self
  }

  pub fn _type(&self) -> &String {
    &self._type
  }


}



