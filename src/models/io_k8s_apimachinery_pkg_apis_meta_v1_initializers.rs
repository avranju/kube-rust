/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MetaV1Initializers : Initializers tracks the progress of initialization.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaV1Initializers {
  /// Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients.
  #[serde(rename = "pending")]
  pending: Vec<::models::MetaV1Initializer>,
  /// If result is set with the Failure field, the object will be persisted to storage and then deleted, ensuring that other clients can observe the deletion.
  #[serde(rename = "result")]
  result: Option<::models::MetaV1Status>
}

impl MetaV1Initializers {
  /// Initializers tracks the progress of initialization.
  pub fn new(pending: Vec<::models::MetaV1Initializer>) -> MetaV1Initializers {
    MetaV1Initializers {
      pending: pending,
      result: None
    }
  }

  pub fn set_pending(&mut self, pending: Vec<::models::MetaV1Initializer>) {
    self.pending = pending;
  }

  pub fn with_pending(mut self, pending: Vec<::models::MetaV1Initializer>) -> MetaV1Initializers {
    self.pending = pending;
    self
  }

  pub fn pending(&self) -> &Vec<::models::MetaV1Initializer> {
    &self.pending
  }


  pub fn set_result(&mut self, result: ::models::MetaV1Status) {
    self.result = Some(result);
  }

  pub fn with_result(mut self, result: ::models::MetaV1Status) -> MetaV1Initializers {
    self.result = Some(result);
    self
  }

  pub fn result(&self) -> Option<&::models::MetaV1Status> {
    self.result.as_ref()
  }

  pub fn reset_result(&mut self) {
    self.result = None;
  }

}



