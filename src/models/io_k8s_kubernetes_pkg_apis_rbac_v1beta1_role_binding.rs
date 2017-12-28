/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// RbacV1beta1RoleBinding : RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace. It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given namespace only have effect in that namespace.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RbacV1beta1RoleBinding {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  /// Standard object's metadata.
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
  #[serde(rename = "roleRef")]
  role_ref: ::models::RbacV1beta1RoleRef,
  /// Subjects holds references to the objects the role applies to.
  #[serde(rename = "subjects")]
  subjects: Vec<::models::RbacV1beta1Subject>
}

impl RbacV1beta1RoleBinding {
  /// RoleBinding references a role, but does not contain it.  It can reference a Role in the same namespace or a ClusterRole in the global namespace. It adds who information via Subjects and namespace information by which namespace it exists in.  RoleBindings in a given namespace only have effect in that namespace.
  pub fn new(role_ref: ::models::RbacV1beta1RoleRef, subjects: Vec<::models::RbacV1beta1Subject>) -> RbacV1beta1RoleBinding {
    RbacV1beta1RoleBinding {
      api_version: None,
      kind: None,
      metadata: None,
      role_ref: role_ref,
      subjects: subjects
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> RbacV1beta1RoleBinding {
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

  pub fn with_kind(mut self, kind: String) -> RbacV1beta1RoleBinding {
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

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> RbacV1beta1RoleBinding {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_role_ref(&mut self, role_ref: ::models::RbacV1beta1RoleRef) {
    self.role_ref = role_ref;
  }

  pub fn with_role_ref(mut self, role_ref: ::models::RbacV1beta1RoleRef) -> RbacV1beta1RoleBinding {
    self.role_ref = role_ref;
    self
  }

  pub fn role_ref(&self) -> &::models::RbacV1beta1RoleRef {
    &self.role_ref
  }


  pub fn set_subjects(&mut self, subjects: Vec<::models::RbacV1beta1Subject>) {
    self.subjects = subjects;
  }

  pub fn with_subjects(mut self, subjects: Vec<::models::RbacV1beta1Subject>) -> RbacV1beta1RoleBinding {
    self.subjects = subjects;
    self
  }

  pub fn subjects(&self) -> &Vec<::models::RbacV1beta1Subject> {
    &self.subjects
  }


}



