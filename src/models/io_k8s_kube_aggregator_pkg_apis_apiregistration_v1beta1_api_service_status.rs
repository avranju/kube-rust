/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1beta1ApiServiceStatus : APIServiceStatus contains derived information about an API server

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1beta1ApiServiceStatus {
  /// Current service state of apiService.
  #[serde(rename = "conditions")]
  conditions: Option<Vec<::models::V1beta1ApiServiceCondition>>
}

impl V1beta1ApiServiceStatus {
  /// APIServiceStatus contains derived information about an API server
  pub fn new() -> V1beta1ApiServiceStatus {
    V1beta1ApiServiceStatus {
      conditions: None
    }
  }

  pub fn set_conditions(&mut self, conditions: Vec<::models::V1beta1ApiServiceCondition>) {
    self.conditions = Some(conditions);
  }

  pub fn with_conditions(mut self, conditions: Vec<::models::V1beta1ApiServiceCondition>) -> V1beta1ApiServiceStatus {
    self.conditions = Some(conditions);
    self
  }

  pub fn conditions(&self) -> Option<&Vec<::models::V1beta1ApiServiceCondition>> {
    self.conditions.as_ref()
  }

  pub fn reset_conditions(&mut self) {
    self.conditions = None;
  }

}



