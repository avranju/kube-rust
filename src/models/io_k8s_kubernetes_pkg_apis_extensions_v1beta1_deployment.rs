/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1Deployment : Deployment enables declarative updates for Pods and ReplicaSets.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtensionsV1beta1Deployment {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object metadata.
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// Specification of the desired behavior of the Deployment.
  #[serde(rename = "spec")]
  spec: Option<::models::ExtensionsV1beta1DeploymentSpec>,
  /// Most recently observed status of the Deployment.
  #[serde(rename = "status")]
  status: Option<::models::ExtensionsV1beta1DeploymentStatus>
}

impl ExtensionsV1beta1Deployment {
  /// Deployment enables declarative updates for Pods and ReplicaSets.
  pub fn new() -> ExtensionsV1beta1Deployment {
    ExtensionsV1beta1Deployment {
      api_version: None,
      kind: None,
      metadata: None,
      spec: None,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> ExtensionsV1beta1Deployment {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> ExtensionsV1beta1Deployment {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_metadata(&mut self, metadata: ::models::MetaV1ObjectMeta) {
    self.metadata = Some(metadata);
  }

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> ExtensionsV1beta1Deployment {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::ExtensionsV1beta1DeploymentSpec) {
    self.spec = Some(spec);
  }

  pub fn with_spec(mut self, spec: ::models::ExtensionsV1beta1DeploymentSpec) -> ExtensionsV1beta1Deployment {
    self.spec = Some(spec);
    self
  }

  pub fn spec(&self) -> Option<&::models::ExtensionsV1beta1DeploymentSpec> {
    self.spec.as_ref()
  }

  pub fn reset_spec(&mut self) {
    self.spec = None;
  }

  pub fn set_status(&mut self, status: ::models::ExtensionsV1beta1DeploymentStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::ExtensionsV1beta1DeploymentStatus) -> ExtensionsV1beta1Deployment {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::ExtensionsV1beta1DeploymentStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



