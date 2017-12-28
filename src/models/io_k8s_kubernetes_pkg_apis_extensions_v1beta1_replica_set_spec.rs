/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1ReplicaSetSpec : ReplicaSetSpec is the specification of a ReplicaSet.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsV1beta1ReplicaSetSpec {
  /// Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready)
  #[serde(rename = "minReadySeconds")]
  min_ready_seconds: Option<i32>,
  /// Replicas is the number of desired replicas. This is a pointer to distinguish between explicit zero and unspecified. Defaults to 1. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller/#what-is-a-replicationcontroller
  #[serde(rename = "replicas")]
  replicas: Option<i32>,
  /// Selector is a label query over pods that should match the replica count. If the selector is empty, it is defaulted to the labels present on the pod template. Label keys and values that must match in order to be controlled by this replica set. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
  #[serde(rename = "selector")]
  selector: Option<::models::MetaV1LabelSelector>,
  /// Template is the object that describes the pod that will be created if insufficient replicas are detected. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template
  #[serde(rename = "template")]
  template: Option<::models::V1PodTemplateSpec>
}

impl ExtensionsV1beta1ReplicaSetSpec {
  /// ReplicaSetSpec is the specification of a ReplicaSet.
  pub fn new() -> ExtensionsV1beta1ReplicaSetSpec {
    ExtensionsV1beta1ReplicaSetSpec {
      min_ready_seconds: None,
      replicas: None,
      selector: None,
      template: None
    }
  }

  pub fn set_min_ready_seconds(&mut self, min_ready_seconds: i32) {
    self.min_ready_seconds = Some(min_ready_seconds);
  }

  pub fn with_min_ready_seconds(mut self, min_ready_seconds: i32) -> ExtensionsV1beta1ReplicaSetSpec {
    self.min_ready_seconds = Some(min_ready_seconds);
    self
  }

  pub fn min_ready_seconds(&self) -> Option<&i32> {
    self.min_ready_seconds.as_ref()
  }

  pub fn reset_min_ready_seconds(&mut self) {
    self.min_ready_seconds = None;
  }

  pub fn set_replicas(&mut self, replicas: i32) {
    self.replicas = Some(replicas);
  }

  pub fn with_replicas(mut self, replicas: i32) -> ExtensionsV1beta1ReplicaSetSpec {
    self.replicas = Some(replicas);
    self
  }

  pub fn replicas(&self) -> Option<&i32> {
    self.replicas.as_ref()
  }

  pub fn reset_replicas(&mut self) {
    self.replicas = None;
  }

  pub fn set_selector(&mut self, selector: ::models::MetaV1LabelSelector) {
    self.selector = Some(selector);
  }

  pub fn with_selector(mut self, selector: ::models::MetaV1LabelSelector) -> ExtensionsV1beta1ReplicaSetSpec {
    self.selector = Some(selector);
    self
  }

  pub fn selector(&self) -> Option<&::models::MetaV1LabelSelector> {
    self.selector.as_ref()
  }

  pub fn reset_selector(&mut self) {
    self.selector = None;
  }

  pub fn set_template(&mut self, template: ::models::V1PodTemplateSpec) {
    self.template = Some(template);
  }

  pub fn with_template(mut self, template: ::models::V1PodTemplateSpec) -> ExtensionsV1beta1ReplicaSetSpec {
    self.template = Some(template);
    self
  }

  pub fn template(&self) -> Option<&::models::V1PodTemplateSpec> {
    self.template.as_ref()
  }

  pub fn reset_template(&mut self) {
    self.template = None;
  }

}



