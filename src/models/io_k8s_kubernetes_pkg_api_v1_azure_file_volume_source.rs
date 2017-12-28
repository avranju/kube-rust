/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1AzureFileVolumeSource : AzureFile represents an Azure File Service mount on the host and bind mount to the pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1AzureFileVolumeSource {
  /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// the name of secret that contains Azure Storage Account Name and Key
  #[serde(rename = "secretName")]
  secret_name: String,
  /// Share Name
  #[serde(rename = "shareName")]
  share_name: String
}

impl V1AzureFileVolumeSource {
  /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
  pub fn new(secret_name: String, share_name: String) -> V1AzureFileVolumeSource {
    V1AzureFileVolumeSource {
      read_only: None,
      secret_name: secret_name,
      share_name: share_name
    }
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1AzureFileVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_secret_name(&mut self, secret_name: String) {
    self.secret_name = secret_name;
  }

  pub fn with_secret_name(mut self, secret_name: String) -> V1AzureFileVolumeSource {
    self.secret_name = secret_name;
    self
  }

  pub fn secret_name(&self) -> &String {
    &self.secret_name
  }


  pub fn set_share_name(&mut self, share_name: String) {
    self.share_name = share_name;
  }

  pub fn with_share_name(mut self, share_name: String) -> V1AzureFileVolumeSource {
    self.share_name = share_name;
    self
  }

  pub fn share_name(&self) -> &String {
    &self.share_name
  }


}



