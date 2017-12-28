/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Lifecycle : Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1Lifecycle {
  /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
  #[serde(rename = "postStart")]
  post_start: Option<::models::V1Handler>,
  /// PreStop is called immediately before a container is terminated. The container is terminated after the handler completes. The reason for termination is passed to the handler. Regardless of the outcome of the handler, the container is eventually terminated. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
  #[serde(rename = "preStop")]
  pre_stop: Option<::models::V1Handler>
}

impl V1Lifecycle {
  /// Lifecycle describes actions that the management system should take in response to container lifecycle events. For the PostStart and PreStop lifecycle handlers, management of the container blocks until the action is complete, unless the container process fails, in which case the handler is aborted.
  pub fn new() -> V1Lifecycle {
    V1Lifecycle {
      post_start: None,
      pre_stop: None
    }
  }

  pub fn set_post_start(&mut self, post_start: ::models::V1Handler) {
    self.post_start = Some(post_start);
  }

  pub fn with_post_start(mut self, post_start: ::models::V1Handler) -> V1Lifecycle {
    self.post_start = Some(post_start);
    self
  }

  pub fn post_start(&self) -> Option<&::models::V1Handler> {
    self.post_start.as_ref()
  }

  pub fn reset_post_start(&mut self) {
    self.post_start = None;
  }

  pub fn set_pre_stop(&mut self, pre_stop: ::models::V1Handler) {
    self.pre_stop = Some(pre_stop);
  }

  pub fn with_pre_stop(mut self, pre_stop: ::models::V1Handler) -> V1Lifecycle {
    self.pre_stop = Some(pre_stop);
    self
  }

  pub fn pre_stop(&self) -> Option<&::models::V1Handler> {
    self.pre_stop.as_ref()
  }

  pub fn reset_pre_stop(&mut self) {
    self.pre_stop = None;
  }

}



