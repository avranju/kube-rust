/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdmissionregistrationV1alpha1InitializerConfiguration : InitializerConfiguration describes the configuration of initializers.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdmissionregistrationV1alpha1InitializerConfiguration {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Initializers is a list of resources and their default initializers Order-sensitive. When merging multiple InitializerConfigurations, we sort the initializers from different InitializerConfigurations by the name of the InitializerConfigurations; the order of the initializers from the same InitializerConfiguration is preserved.
  #[serde(rename = "initializers")]
  initializers: Option<Vec<::models::AdmissionregistrationV1alpha1Initializer>>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>
}

impl AdmissionregistrationV1alpha1InitializerConfiguration {
  /// InitializerConfiguration describes the configuration of initializers.
  pub fn new() -> AdmissionregistrationV1alpha1InitializerConfiguration {
    AdmissionregistrationV1alpha1InitializerConfiguration {
      api_version: None,
      initializers: None,
      kind: None,
      metadata: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> AdmissionregistrationV1alpha1InitializerConfiguration {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_initializers(&mut self, initializers: Vec<::models::AdmissionregistrationV1alpha1Initializer>) {
    self.initializers = Some(initializers);
  }

  pub fn with_initializers(mut self, initializers: Vec<::models::AdmissionregistrationV1alpha1Initializer>) -> AdmissionregistrationV1alpha1InitializerConfiguration {
    self.initializers = Some(initializers);
    self
  }

  pub fn initializers(&self) -> Option<&Vec<::models::AdmissionregistrationV1alpha1Initializer>> {
    self.initializers.as_ref()
  }

  pub fn reset_initializers(&mut self) {
    self.initializers = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> AdmissionregistrationV1alpha1InitializerConfiguration {
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

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> AdmissionregistrationV1alpha1InitializerConfiguration {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

}



