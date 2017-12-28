/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AuthenticationV1TokenReview : TokenReview attempts to authenticate a token to a known user. Note: TokenReview requests may be cached by the webhook token authenticator plugin in the kube-apiserver.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationV1TokenReview {
  /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
  #[serde(rename = "apiVersion")]
  api_version: Option<String>,
  /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
  #[serde(rename = "kind")]
  kind: Option<String>,
  #[serde(rename = "metadata")]
  metadata: Option<::models::MetaV1ObjectMeta>,
  /// Spec holds information about the request being evaluated
  #[serde(rename = "spec")]
  spec: ::models::AuthenticationV1TokenReviewSpec,
  /// Status is filled in by the server and indicates whether the request can be authenticated.
  #[serde(rename = "status")]
  status: Option<::models::AuthenticationV1TokenReviewStatus>
}

impl AuthenticationV1TokenReview {
  /// TokenReview attempts to authenticate a token to a known user. Note: TokenReview requests may be cached by the webhook token authenticator plugin in the kube-apiserver.
  pub fn new(spec: ::models::AuthenticationV1TokenReviewSpec) -> AuthenticationV1TokenReview {
    AuthenticationV1TokenReview {
      api_version: None,
      kind: None,
      metadata: None,
      spec: spec,
      status: None
    }
  }

  pub fn set_api_version(&mut self, api_version: String) {
    self.api_version = Some(api_version);
  }

  pub fn with_api_version(mut self, api_version: String) -> AuthenticationV1TokenReview {
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

  pub fn with_kind(mut self, kind: String) -> AuthenticationV1TokenReview {
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

  pub fn with_metadata(mut self, metadata: ::models::MetaV1ObjectMeta) -> AuthenticationV1TokenReview {
    self.metadata = Some(metadata);
    self
  }

  pub fn metadata(&self) -> Option<&::models::MetaV1ObjectMeta> {
    self.metadata.as_ref()
  }

  pub fn reset_metadata(&mut self) {
    self.metadata = None;
  }

  pub fn set_spec(&mut self, spec: ::models::AuthenticationV1TokenReviewSpec) {
    self.spec = spec;
  }

  pub fn with_spec(mut self, spec: ::models::AuthenticationV1TokenReviewSpec) -> AuthenticationV1TokenReview {
    self.spec = spec;
    self
  }

  pub fn spec(&self) -> &::models::AuthenticationV1TokenReviewSpec {
    &self.spec
  }


  pub fn set_status(&mut self, status: ::models::AuthenticationV1TokenReviewStatus) {
    self.status = Some(status);
  }

  pub fn with_status(mut self, status: ::models::AuthenticationV1TokenReviewStatus) -> AuthenticationV1TokenReview {
    self.status = Some(status);
    self
  }

  pub fn status(&self) -> Option<&::models::AuthenticationV1TokenReviewStatus> {
    self.status.as_ref()
  }

  pub fn reset_status(&mut self) {
    self.status = None;
  }

}



