/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1HttpHeader : HTTPHeader describes a custom header to be used in HTTP probes

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1HttpHeader {
  /// The header field name
  #[serde(rename = "name")]
  name: String,
  /// The header field value
  #[serde(rename = "value")]
  value: String
}

impl V1HttpHeader {
  /// HTTPHeader describes a custom header to be used in HTTP probes
  pub fn new(name: String, value: String) -> V1HttpHeader {
    V1HttpHeader {
      name: name,
      value: value
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1HttpHeader {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_value(&mut self, value: String) {
    self.value = value;
  }

  pub fn with_value(mut self, value: String) -> V1HttpHeader {
    self.value = value;
    self
  }

  pub fn value(&self) -> &String {
    &self.value
  }


}



