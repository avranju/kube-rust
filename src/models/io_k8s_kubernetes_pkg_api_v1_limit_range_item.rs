/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1LimitRangeItem : LimitRangeItem defines a min/max usage limit for any resource that matches on kind.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1LimitRangeItem {
  /// Default resource requirement limit value by resource name if resource limit is omitted.
  #[serde(rename = "default")]
  default: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// DefaultRequest is the default resource requirement request value by resource name if resource request is omitted.
  #[serde(rename = "defaultRequest")]
  default_request: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// Max usage constraints on this kind by resource name.
  #[serde(rename = "max")]
  max: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource.
  #[serde(rename = "maxLimitRequestRatio")]
  max_limit_request_ratio: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// Min usage constraints on this kind by resource name.
  #[serde(rename = "min")]
  min: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// Type of resource that this limit applies to.
  #[serde(rename = "type")]
  _type: Option<String>
}

impl V1LimitRangeItem {
  /// LimitRangeItem defines a min/max usage limit for any resource that matches on kind.
  pub fn new() -> V1LimitRangeItem {
    V1LimitRangeItem {
      default: None,
      default_request: None,
      max: None,
      max_limit_request_ratio: None,
      min: None,
      _type: None
    }
  }

  pub fn set_default(&mut self, default: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.default = Some(default);
  }

  pub fn with_default(mut self, default: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1LimitRangeItem {
    self.default = Some(default);
    self
  }

  pub fn default(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.default.as_ref()
  }

  pub fn reset_default(&mut self) {
    self.default = None;
  }

  pub fn set_default_request(&mut self, default_request: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.default_request = Some(default_request);
  }

  pub fn with_default_request(mut self, default_request: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1LimitRangeItem {
    self.default_request = Some(default_request);
    self
  }

  pub fn default_request(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.default_request.as_ref()
  }

  pub fn reset_default_request(&mut self) {
    self.default_request = None;
  }

  pub fn set_max(&mut self, max: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.max = Some(max);
  }

  pub fn with_max(mut self, max: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1LimitRangeItem {
    self.max = Some(max);
    self
  }

  pub fn max(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.max.as_ref()
  }

  pub fn reset_max(&mut self) {
    self.max = None;
  }

  pub fn set_max_limit_request_ratio(&mut self, max_limit_request_ratio: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.max_limit_request_ratio = Some(max_limit_request_ratio);
  }

  pub fn with_max_limit_request_ratio(mut self, max_limit_request_ratio: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1LimitRangeItem {
    self.max_limit_request_ratio = Some(max_limit_request_ratio);
    self
  }

  pub fn max_limit_request_ratio(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.max_limit_request_ratio.as_ref()
  }

  pub fn reset_max_limit_request_ratio(&mut self) {
    self.max_limit_request_ratio = None;
  }

  pub fn set_min(&mut self, min: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.min = Some(min);
  }

  pub fn with_min(mut self, min: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1LimitRangeItem {
    self.min = Some(min);
    self
  }

  pub fn min(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.min.as_ref()
  }

  pub fn reset_min(&mut self) {
    self.min = None;
  }

  pub fn set__type(&mut self, _type: String) {
    self._type = Some(_type);
  }

  pub fn with__type(mut self, _type: String) -> V1LimitRangeItem {
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



