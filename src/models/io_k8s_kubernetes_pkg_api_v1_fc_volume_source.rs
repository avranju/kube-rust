/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1FcVolumeSource : Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1FcVolumeSource {
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified.
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// Required: FC target lun number
  #[serde(rename = "lun")]
  lun: i32,
  /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// Required: FC target worldwide names (WWNs)
  #[serde(rename = "targetWWNs")]
  target_ww_ns: Vec<String>
}

impl V1FcVolumeSource {
  /// Represents a Fibre Channel volume. Fibre Channel volumes can only be mounted as read/write once. Fibre Channel volumes support ownership management and SELinux relabeling.
  pub fn new(lun: i32, target_ww_ns: Vec<String>) -> V1FcVolumeSource {
    V1FcVolumeSource {
      fs_type: None,
      lun: lun,
      read_only: None,
      target_ww_ns: target_ww_ns
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1FcVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_lun(&mut self, lun: i32) {
    self.lun = lun;
  }

  pub fn with_lun(mut self, lun: i32) -> V1FcVolumeSource {
    self.lun = lun;
    self
  }

  pub fn lun(&self) -> &i32 {
    &self.lun
  }


  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1FcVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_target_ww_ns(&mut self, target_ww_ns: Vec<String>) {
    self.target_ww_ns = target_ww_ns;
  }

  pub fn with_target_ww_ns(mut self, target_ww_ns: Vec<String>) -> V1FcVolumeSource {
    self.target_ww_ns = target_ww_ns;
    self
  }

  pub fn target_ww_ns(&self) -> &Vec<String> {
    &self.target_ww_ns
  }


}



