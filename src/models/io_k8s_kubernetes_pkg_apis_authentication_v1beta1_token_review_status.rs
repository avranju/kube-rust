/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AuthenticationV1beta1TokenReviewStatus : TokenReviewStatus is the result of the token authentication request.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthenticationV1beta1TokenReviewStatus {
  /// Authenticated indicates that the token was associated with a known user.
  #[serde(rename = "authenticated")]
  authenticated: Option<bool>,
  /// Error indicates that the token couldn't be checked
  #[serde(rename = "error")]
  error: Option<String>,
  /// User is the UserInfo associated with the provided token.
  #[serde(rename = "user")]
  user: Option<::models::AuthenticationV1beta1UserInfo>
}

impl AuthenticationV1beta1TokenReviewStatus {
  /// TokenReviewStatus is the result of the token authentication request.
  pub fn new() -> AuthenticationV1beta1TokenReviewStatus {
    AuthenticationV1beta1TokenReviewStatus {
      authenticated: None,
      error: None,
      user: None
    }
  }

  pub fn set_authenticated(&mut self, authenticated: bool) {
    self.authenticated = Some(authenticated);
  }

  pub fn with_authenticated(mut self, authenticated: bool) -> AuthenticationV1beta1TokenReviewStatus {
    self.authenticated = Some(authenticated);
    self
  }

  pub fn authenticated(&self) -> Option<&bool> {
    self.authenticated.as_ref()
  }

  pub fn reset_authenticated(&mut self) {
    self.authenticated = None;
  }

  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> AuthenticationV1beta1TokenReviewStatus {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_user(&mut self, user: ::models::AuthenticationV1beta1UserInfo) {
    self.user = Some(user);
  }

  pub fn with_user(mut self, user: ::models::AuthenticationV1beta1UserInfo) -> AuthenticationV1beta1TokenReviewStatus {
    self.user = Some(user);
    self
  }

  pub fn user(&self) -> Option<&::models::AuthenticationV1beta1UserInfo> {
    self.user.as_ref()
  }

  pub fn reset_user(&mut self) {
    self.user = None;
  }

}



