/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1ThirdPartyResource : A ThirdPartyResource is a generic representation of a resource, it is used by add-ons and plugins to add new resource types to the API.  It consists of one or more Versions of the api.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsV1beta1ThirdPartyResource {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Description is the description of this object.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object metadata
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// Versions are versions for this third party object
  #[serde(rename = "versions")]
  versions: Option<Vec<::models::ExtensionsV1beta1ApiVersion>>
}

impl ExtensionsV1beta1ThirdPartyResource {
  /// A ThirdPartyResource is a generic representation of a resource, it is used by add-ons and plugins to add new resource types to the API.  It consists of one or more Versions of the api.
  pub fn new() -> ExtensionsV1beta1ThirdPartyResource {
    ExtensionsV1beta1ThirdPartyResource {
      api_version: None,
      description: None,
      kind: None,
      metadata: None,
      versions: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> ExtensionsV1beta1ThirdPartyResource {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> ExtensionsV1beta1ThirdPartyResource {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> ExtensionsV1beta1ThirdPartyResource {
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

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> ExtensionsV1beta1ThirdPartyResource {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_versions(&mut self, versions: Vec<::models::ExtensionsV1beta1ApiVersion>) {
    self.versions = Some(versions);
  }

  pub fn with_versions(mut self, versions: Vec<::models::ExtensionsV1beta1ApiVersion>) -> ExtensionsV1beta1ThirdPartyResource {
    self.versions = Some(versions);
    self
  }

  pub fn versions(&self) -> Option<&Vec<::models::ExtensionsV1beta1ApiVersion>> {
    self.versions.as_ref()
  }

  pub fn reset_versions(&mut self) {
    self.versions = None;
  }

}



