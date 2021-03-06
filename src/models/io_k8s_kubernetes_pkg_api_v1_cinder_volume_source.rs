/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1CinderVolumeSource : Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1CinderVolumeSource {
  /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
  #[serde(rename = "fsType")]
  fs_type: Option<String>,
  /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
  #[serde(rename = "readOnly")]
  read_only: Option<bool>,
  /// volume id used to identify the volume in cinder More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
  #[serde(rename = "volumeID")]
  volume_id: String
}

impl V1CinderVolumeSource {
  /// Represents a cinder volume resource in Openstack. A Cinder volume must exist before mounting to a container. The volume must also be in the same region as the kubelet. Cinder volumes support ownership management and SELinux relabeling.
  pub fn new(volume_id: String) -> V1CinderVolumeSource {
    V1CinderVolumeSource {
      fs_type: None,
      read_only: None,
      volume_id: volume_id
    }
  }

  pub fn set_fs_type(&mut self, fs_type: String) {
    self.fs_type = Some(fs_type);
  }

  pub fn with_fs_type(mut self, fs_type: String) -> V1CinderVolumeSource {
    self.fs_type = Some(fs_type);
    self
  }

  pub fn fs_type(&self) -> Option<&String> {
    self.fs_type.as_ref()
  }

  pub fn reset_fs_type(&mut self) {
    self.fs_type = None;
  }

  pub fn set_read_only(&mut self, read_only: bool) {
    self.read_only = Some(read_only);
  }

  pub fn with_read_only(mut self, read_only: bool) -> V1CinderVolumeSource {
    self.read_only = Some(read_only);
    self
  }

  pub fn read_only(&self) -> Option<&bool> {
    self.read_only.as_ref()
  }

  pub fn reset_read_only(&mut self) {
    self.read_only = None;
  }

  pub fn set_volume_id(&mut self, volume_id: String) {
    self.volume_id = volume_id;
  }

  pub fn with_volume_id(mut self, volume_id: String) -> V1CinderVolumeSource {
    self.volume_id = volume_id;
    self
  }

  pub fn volume_id(&self) -> &String {
    &self.volume_id
  }


}



