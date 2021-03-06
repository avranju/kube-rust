/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1ResourceQuotaStatus : ResourceQuotaStatus defines the enforced hard limits and observed use.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1ResourceQuotaStatus {
  /// Hard is the set of enforced hard limits for each named resource. More info: https://git.k8s.io/community/contributors/design-proposals/admission_control_resource_quota.md
  #[serde(rename = "hard")]
  hard: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// Used is the current observed total usage of the resource in the namespace.
  #[serde(rename = "used")]
  used: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>
}

impl V1ResourceQuotaStatus {
  /// ResourceQuotaStatus defines the enforced hard limits and observed use.
  pub fn new() -> V1ResourceQuotaStatus {
    V1ResourceQuotaStatus {
      hard: None,
      used: None
    }
  }

  pub fn set_hard(&mut self, hard: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.hard = Some(hard);
  }

  pub fn with_hard(mut self, hard: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1ResourceQuotaStatus {
    self.hard = Some(hard);
    self
  }

  pub fn hard(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.hard.as_ref()
  }

  pub fn reset_hard(&mut self) {
    self.hard = None;
  }

  pub fn set_used(&mut self, used: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.used = Some(used);
  }

  pub fn with_used(mut self, used: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1ResourceQuotaStatus {
    self.used = Some(used);
    self
  }

  pub fn used(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.used.as_ref()
  }

  pub fn reset_used(&mut self) {
    self.used = None;
  }

}



