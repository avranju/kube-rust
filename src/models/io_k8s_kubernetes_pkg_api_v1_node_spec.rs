/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1NodeSpec : NodeSpec describes the attributes that a node is created with.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1NodeSpec {
  /// External ID of the node assigned by some machine database (e.g. a cloud provider). Deprecated.
  #[serde(rename = "externalID")]
  external_id: Option<String>,
  /// PodCIDR represents the pod IP range assigned to the node.
  #[serde(rename = "podCIDR")]
  pod_cidr: Option<String>,
  /// ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID>
  #[serde(rename = "providerID")]
  provider_id: Option<String>,
  /// If specified, the node's taints.
  #[serde(rename = "taints")]
  taints: Option<Vec<::models::V1Taint>>,
  /// Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration
  #[serde(rename = "unschedulable")]
  unschedulable: Option<bool>
}

impl V1NodeSpec {
  /// NodeSpec describes the attributes that a node is created with.
  pub fn new() -> V1NodeSpec {
    V1NodeSpec {
      external_id: None,
      pod_cidr: None,
      provider_id: None,
      taints: None,
      unschedulable: None
    }
  }

  pub fn set_external_id(&mut self, external_id: String) {
    self.external_id = Some(external_id);
  }

  pub fn with_external_id(mut self, external_id: String) -> V1NodeSpec {
    self.external_id = Some(external_id);
    self
  }

  pub fn external_id(&self) -> Option<&String> {
    self.external_id.as_ref()
  }

  pub fn reset_external_id(&mut self) {
    self.external_id = None;
  }

  pub fn set_pod_cidr(&mut self, pod_cidr: String) {
    self.pod_cidr = Some(pod_cidr);
  }

  pub fn with_pod_cidr(mut self, pod_cidr: String) -> V1NodeSpec {
    self.pod_cidr = Some(pod_cidr);
    self
  }

  pub fn pod_cidr(&self) -> Option<&String> {
    self.pod_cidr.as_ref()
  }

  pub fn reset_pod_cidr(&mut self) {
    self.pod_cidr = None;
  }

  pub fn set_provider_id(&mut self, provider_id: String) {
    self.provider_id = Some(provider_id);
  }

  pub fn with_provider_id(mut self, provider_id: String) -> V1NodeSpec {
    self.provider_id = Some(provider_id);
    self
  }

  pub fn provider_id(&self) -> Option<&String> {
    self.provider_id.as_ref()
  }

  pub fn reset_provider_id(&mut self) {
    self.provider_id = None;
  }

  pub fn set_taints(&mut self, taints: Vec<::models::V1Taint>) {
    self.taints = Some(taints);
  }

  pub fn with_taints(mut self, taints: Vec<::models::V1Taint>) -> V1NodeSpec {
    self.taints = Some(taints);
    self
  }

  pub fn taints(&self) -> Option<&Vec<::models::V1Taint>> {
    self.taints.as_ref()
  }

  pub fn reset_taints(&mut self) {
    self.taints = None;
  }

  pub fn set_unschedulable(&mut self, unschedulable: bool) {
    self.unschedulable = Some(unschedulable);
  }

  pub fn with_unschedulable(mut self, unschedulable: bool) -> V1NodeSpec {
    self.unschedulable = Some(unschedulable);
    self
  }

  pub fn unschedulable(&self) -> Option<&bool> {
    self.unschedulable.as_ref()
  }

  pub fn reset_unschedulable(&mut self) {
    self.unschedulable = None;
  }

}



