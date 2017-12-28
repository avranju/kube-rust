/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// MetaV1ApiResourceList : APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaV1ApiResourceList {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// groupVersion is the group and version this APIResourceList is for.
  #[serde(rename = "groupVersion")]
  group_version: String,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// resources contains the name of the resources and if they are namespaced.
  #[serde(rename = "resources")]
  resources: Vec<::models::MetaV1ApiResource>
}

impl MetaV1ApiResourceList {
  /// APIResourceList is a list of APIResource, it is used to expose the name of the resources supported in a specific group and version, and if the resource is namespaced.
  pub fn new(group_version: String, resources: Vec<::models::MetaV1ApiResource>) -> MetaV1ApiResourceList {
    MetaV1ApiResourceList {
      api_version: None,
      group_version: group_version,
      kind: None,
      resources: resources
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> MetaV1ApiResourceList {
    self.api_version = Some(api_version);
    self
  }

  pub fn api_version(&self) -> Option<&String> {
    self.api_version.as_ref()
  }

  pub fn reset_api_version(&mut self) {
    self.api_version = None;
  }

  pub fn set_group_version(&mut self, group_version: String) {
    self.group_version = group_version;
  }

  pub fn with_group_version(mut self, group_version: String) -> MetaV1ApiResourceList {
    self.group_version = group_version;
    self
  }

  pub fn group_version(&self) -> &String {
    &self.group_version
  }


  pub fn set_kind(&mut self, kind: String) {
    self.kind = Some(kind);
  }

  pub fn with_kind(mut self, kind: String) -> MetaV1ApiResourceList {
    self.kind = Some(kind);
    self
  }

  pub fn kind(&self) -> Option<&String> {
    self.kind.as_ref()
  }

  pub fn reset_kind(&mut self) {
    self.kind = None;
  }

  pub fn set_resources(&mut self, resources: Vec<::models::MetaV1ApiResource>) {
    self.resources = resources;
  }

  pub fn with_resources(mut self, resources: Vec<::models::MetaV1ApiResource>) -> MetaV1ApiResourceList {
    self.resources = resources;
    self
  }

  pub fn resources(&self) -> &Vec<::models::MetaV1ApiResource> {
    &self.resources
  }


}



