/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ResourceFieldSelector : ResourceFieldSelector represents container resources (cpu, memory) and their output format

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1ResourceFieldSelector {
  /// Container name: required for volumes, optional for env vars
  #[serde(rename = "containerName")]
  container_name: Option<String>,
  /// Specifies the output format of the exposed resources, defaults to \"1\"
  #[serde(rename = "divisor")]
  divisor: Option<::models::ResourceQuantity>,
  /// Required: resource to select
  #[serde(rename = "resource")]
  resource: String
}

impl V1ResourceFieldSelector {
  /// ResourceFieldSelector represents container resources (cpu, memory) and their output format
  pub fn new(resource: String) -> V1ResourceFieldSelector {
    V1ResourceFieldSelector {
      container_name: None,
      divisor: None,
      resource: resource
    }
  }

  pub fn set_container_name(&mut self, container_name: String) {
    self.container_name = Some(container_name);
  }

  pub fn with_container_name(mut self, container_name: String) -> V1ResourceFieldSelector {
    self.container_name = Some(container_name);
    self
  }

  pub fn container_name(&self) -> Option<&String> {
    self.container_name.as_ref()
  }

  pub fn reset_container_name(&mut self) {
    self.container_name = None;
  }

  pub fn set_divisor(&mut self, divisor: ::models::ResourceQuantity) {
    self.divisor = Some(divisor);
  }

  pub fn with_divisor(mut self, divisor: ::models::ResourceQuantity) -> V1ResourceFieldSelector {
    self.divisor = Some(divisor);
    self
  }

  pub fn divisor(&self) -> Option<&::models::ResourceQuantity> {
    self.divisor.as_ref()
  }

  pub fn reset_divisor(&mut self) {
    self.divisor = None;
  }

  pub fn set_resource(&mut self, resource: String) {
    self.resource = resource;
  }

  pub fn with_resource(mut self, resource: String) -> V1ResourceFieldSelector {
    self.resource = resource;
    self
  }

  pub fn resource(&self) -> &String {
    &self.resource
  }


}



