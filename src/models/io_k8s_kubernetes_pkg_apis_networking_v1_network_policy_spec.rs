/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkingV1NetworkPolicySpec : NetworkPolicySpec provides the specification of a NetworkPolicy

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkingV1NetworkPolicySpec {
  /// List of ingress rules to be applied to the selected pods. Traffic is allowed to a pod if there are no NetworkPolicies selecting the pod (and cluster policy otherwise allows the traffic), OR if the traffic source is the pod's local node, OR if the traffic matches at least one ingress rule across all of the NetworkPolicy objects whose podSelector matches the pod. If this field is empty then this NetworkPolicy does not allow any traffic (and serves solely to ensure that the pods it selects are isolated by default)
  #[serde(rename = "ingress")]
  ingress: Option<Vec<::models::NetworkingV1NetworkPolicyIngressRule>>,
  /// Selects the pods to which this NetworkPolicy object applies. The array of ingress rules is applied to any pods selected by this field. Multiple network policies can select the same set of pods. In this case, the ingress rules for each are combined additively. This field is NOT optional and follows standard label selector semantics. An empty podSelector matches all pods in this namespace.
  #[serde(rename = "podSelector")]
  pod_selector: ::models::MetaV1LabelSelector
}

impl NetworkingV1NetworkPolicySpec {
  /// NetworkPolicySpec provides the specification of a NetworkPolicy
  pub fn new(pod_selector: ::models::MetaV1LabelSelector) -> NetworkingV1NetworkPolicySpec {
    NetworkingV1NetworkPolicySpec {
      ingress: None,
      pod_selector: pod_selector
    }
  }

  pub fn set_ingress(&mut self, ingress: Vec<::models::NetworkingV1NetworkPolicyIngressRule>) {
    self.ingress = Some(ingress);
  }

  pub fn with_ingress(mut self, ingress: Vec<::models::NetworkingV1NetworkPolicyIngressRule>) -> NetworkingV1NetworkPolicySpec {
    self.ingress = Some(ingress);
    self
  }

  pub fn ingress(&self) -> Option<&Vec<::models::NetworkingV1NetworkPolicyIngressRule>> {
    self.ingress.as_ref()
  }

  pub fn reset_ingress(&mut self) {
    self.ingress = None;
  }

  pub fn set_pod_selector(&mut self, pod_selector: ::models::MetaV1LabelSelector) {
    self.pod_selector = pod_selector;
  }

  pub fn with_pod_selector(mut self, pod_selector: ::models::MetaV1LabelSelector) -> NetworkingV1NetworkPolicySpec {
    self.pod_selector = pod_selector;
    self
  }

  pub fn pod_selector(&self) -> &::models::MetaV1LabelSelector {
    &self.pod_selector
  }


}


