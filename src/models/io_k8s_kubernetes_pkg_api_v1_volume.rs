/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Volume : Volume represents a named volume in a pod that may be accessed by any container in the pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1Volume {
  /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
  #[serde(rename = "awsElasticBlockStore")]
  aws_elastic_block_store: Option<::models::V1AwsElasticBlockStoreVolumeSource>,
  /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
  #[serde(rename = "azureDisk")]
  azure_disk: Option<::models::V1AzureDiskVolumeSource>,
  /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
  #[serde(rename = "azureFile")]
  azure_file: Option<::models::V1AzureFileVolumeSource>,
  /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
  #[serde(rename = "cephfs")]
  cephfs: Option<::models::V1CephFsVolumeSource>,
  /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
  #[serde(rename = "cinder")]
  cinder: Option<::models::V1CinderVolumeSource>,
  /// ConfigMap represents a configMap that should populate this volume
  #[serde(rename = "configMap")]
  config_map: Option<::models::V1ConfigMapVolumeSource>,
  /// DownwardAPI represents downward API about the pod that should populate this volume
  #[serde(rename = "downwardAPI")]
  downward_api: Option<::models::V1DownwardApiVolumeSource>,
  /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
  #[serde(rename = "emptyDir")]
  empty_dir: Option<::models::V1EmptyDirVolumeSource>,
  /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
  #[serde(rename = "fc")]
  fc: Option<::models::V1FcVolumeSource>,
  /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. This is an alpha feature and may change in future.
  #[serde(rename = "flexVolume")]
  flex_volume: Option<::models::V1FlexVolumeSource>,
  /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
  #[serde(rename = "flocker")]
  flocker: Option<::models::V1FlockerVolumeSource>,
  /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "gcePersistentDisk")]
  gce_persistent_disk: Option<::models::V1GcePersistentDiskVolumeSource>,
  /// GitRepo represents a git repository at a particular revision.
  #[serde(rename = "gitRepo")]
  git_repo: Option<::models::V1GitRepoVolumeSource>,
  /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
  #[serde(rename = "glusterfs")]
  glusterfs: Option<::models::V1GlusterfsVolumeSource>,
  /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
  #[serde(rename = "hostPath")]
  host_path: Option<::models::V1HostPathVolumeSource>,
  /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://releases.k8s.io/HEAD/examples/volumes/iscsi/README.md
  #[serde(rename = "iscsi")]
  iscsi: Option<::models::V1IscsiVolumeSource>,
  /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
  #[serde(rename = "name")]
  name: String,
  /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
  #[serde(rename = "nfs")]
  nfs: Option<::models::V1NfsVolumeSource>,
  /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
  #[serde(rename = "persistentVolumeClaim")]
  persistent_volume_claim: Option<::models::V1PersistentVolumeClaimVolumeSource>,
  /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
  #[serde(rename = "photonPersistentDisk")]
  photon_persistent_disk: Option<::models::V1PhotonPersistentDiskVolumeSource>,
  /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
  #[serde(rename = "portworxVolume")]
  portworx_volume: Option<::models::V1PortworxVolumeSource>,
  /// Items for all in one resources secrets, configmaps, and downward API
  #[serde(rename = "projected")]
  projected: Option<::models::V1ProjectedVolumeSource>,
  /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
  #[serde(rename = "quobyte")]
  quobyte: Option<::models::V1QuobyteVolumeSource>,
  /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
  #[serde(rename = "rbd")]
  rbd: Option<::models::V1RbdVolumeSource>,
  /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
  #[serde(rename = "scaleIO")]
  scale_io: Option<::models::V1ScaleIoVolumeSource>,
  /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
  #[serde(rename = "secret")]
  secret: Option<::models::V1SecretVolumeSource>,
  /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
  #[serde(rename = "storageos")]
  storageos: Option<::models::V1StorageOsVolumeSource>,
  /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
  #[serde(rename = "vsphereVolume")]
  vsphere_volume: Option<::models::V1VsphereVirtualDiskVolumeSource>
}

impl V1Volume {
  /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
  pub fn new(name: String) -> V1Volume {
    V1Volume {
      aws_elastic_block_store: None,
      azure_disk: None,
      azure_file: None,
      cephfs: None,
      cinder: None,
      config_map: None,
      downward_api: None,
      empty_dir: None,
      fc: None,
      flex_volume: None,
      flocker: None,
      gce_persistent_disk: None,
      git_repo: None,
      glusterfs: None,
      host_path: None,
      iscsi: None,
      name: name,
      nfs: None,
      persistent_volume_claim: None,
      photon_persistent_disk: None,
      portworx_volume: None,
      projected: None,
      quobyte: None,
      rbd: None,
      scale_io: None,
      secret: None,
      storageos: None,
      vsphere_volume: None
    }
  }

  pub fn set_aws_elastic_block_store(&mut self, aws_elastic_block_store: ::models::V1AwsElasticBlockStoreVolumeSource) {
    self.aws_elastic_block_store = Some(aws_elastic_block_store);
  }

  pub fn with_aws_elastic_block_store(mut self, aws_elastic_block_store: ::models::V1AwsElasticBlockStoreVolumeSource) -> V1Volume {
    self.aws_elastic_block_store = Some(aws_elastic_block_store);
    self
  }

  pub fn aws_elastic_block_store(&self) -> Option<&::models::V1AwsElasticBlockStoreVolumeSource> {
    self.aws_elastic_block_store.as_ref()
  }

  pub fn reset_aws_elastic_block_store(&mut self) {
    self.aws_elastic_block_store = None;
  }

  pub fn set_azure_disk(&mut self, azure_disk: ::models::V1AzureDiskVolumeSource) {
    self.azure_disk = Some(azure_disk);
  }

  pub fn with_azure_disk(mut self, azure_disk: ::models::V1AzureDiskVolumeSource) -> V1Volume {
    self.azure_disk = Some(azure_disk);
    self
  }

  pub fn azure_disk(&self) -> Option<&::models::V1AzureDiskVolumeSource> {
    self.azure_disk.as_ref()
  }

  pub fn reset_azure_disk(&mut self) {
    self.azure_disk = None;
  }

  pub fn set_azure_file(&mut self, azure_file: ::models::V1AzureFileVolumeSource) {
    self.azure_file = Some(azure_file);
  }

  pub fn with_azure_file(mut self, azure_file: ::models::V1AzureFileVolumeSource) -> V1Volume {
    self.azure_file = Some(azure_file);
    self
  }

  pub fn azure_file(&self) -> Option<&::models::V1AzureFileVolumeSource> {
    self.azure_file.as_ref()
  }

  pub fn reset_azure_file(&mut self) {
    self.azure_file = None;
  }

  pub fn set_cephfs(&mut self, cephfs: ::models::V1CephFsVolumeSource) {
    self.cephfs = Some(cephfs);
  }

  pub fn with_cephfs(mut self, cephfs: ::models::V1CephFsVolumeSource) -> V1Volume {
    self.cephfs = Some(cephfs);
    self
  }

  pub fn cephfs(&self) -> Option<&::models::V1CephFsVolumeSource> {
    self.cephfs.as_ref()
  }

  pub fn reset_cephfs(&mut self) {
    self.cephfs = None;
  }

  pub fn set_cinder(&mut self, cinder: ::models::V1CinderVolumeSource) {
    self.cinder = Some(cinder);
  }

  pub fn with_cinder(mut self, cinder: ::models::V1CinderVolumeSource) -> V1Volume {
    self.cinder = Some(cinder);
    self
  }

  pub fn cinder(&self) -> Option<&::models::V1CinderVolumeSource> {
    self.cinder.as_ref()
  }

  pub fn reset_cinder(&mut self) {
    self.cinder = None;
  }

  pub fn set_config_map(&mut self, config_map: ::models::V1ConfigMapVolumeSource) {
    self.config_map = Some(config_map);
  }

  pub fn with_config_map(mut self, config_map: ::models::V1ConfigMapVolumeSource) -> V1Volume {
    self.config_map = Some(config_map);
    self
  }

  pub fn config_map(&self) -> Option<&::models::V1ConfigMapVolumeSource> {
    self.config_map.as_ref()
  }

  pub fn reset_config_map(&mut self) {
    self.config_map = None;
  }

  pub fn set_downward_api(&mut self, downward_api: ::models::V1DownwardApiVolumeSource) {
    self.downward_api = Some(downward_api);
  }

  pub fn with_downward_api(mut self, downward_api: ::models::V1DownwardApiVolumeSource) -> V1Volume {
    self.downward_api = Some(downward_api);
    self
  }

  pub fn downward_api(&self) -> Option<&::models::V1DownwardApiVolumeSource> {
    self.downward_api.as_ref()
  }

  pub fn reset_downward_api(&mut self) {
    self.downward_api = None;
  }

  pub fn set_empty_dir(&mut self, empty_dir: ::models::V1EmptyDirVolumeSource) {
    self.empty_dir = Some(empty_dir);
  }

  pub fn with_empty_dir(mut self, empty_dir: ::models::V1EmptyDirVolumeSource) -> V1Volume {
    self.empty_dir = Some(empty_dir);
    self
  }

  pub fn empty_dir(&self) -> Option<&::models::V1EmptyDirVolumeSource> {
    self.empty_dir.as_ref()
  }

  pub fn reset_empty_dir(&mut self) {
    self.empty_dir = None;
  }

  pub fn set_fc(&mut self, fc: ::models::V1FcVolumeSource) {
    self.fc = Some(fc);
  }

  pub fn with_fc(mut self, fc: ::models::V1FcVolumeSource) -> V1Volume {
    self.fc = Some(fc);
    self
  }

  pub fn fc(&self) -> Option<&::models::V1FcVolumeSource> {
    self.fc.as_ref()
  }

  pub fn reset_fc(&mut self) {
    self.fc = None;
  }

  pub fn set_flex_volume(&mut self, flex_volume: ::models::V1FlexVolumeSource) {
    self.flex_volume = Some(flex_volume);
  }

  pub fn with_flex_volume(mut self, flex_volume: ::models::V1FlexVolumeSource) -> V1Volume {
    self.flex_volume = Some(flex_volume);
    self
  }

  pub fn flex_volume(&self) -> Option<&::models::V1FlexVolumeSource> {
    self.flex_volume.as_ref()
  }

  pub fn reset_flex_volume(&mut self) {
    self.flex_volume = None;
  }

  pub fn set_flocker(&mut self, flocker: ::models::V1FlockerVolumeSource) {
    self.flocker = Some(flocker);
  }

  pub fn with_flocker(mut self, flocker: ::models::V1FlockerVolumeSource) -> V1Volume {
    self.flocker = Some(flocker);
    self
  }

  pub fn flocker(&self) -> Option<&::models::V1FlockerVolumeSource> {
    self.flocker.as_ref()
  }

  pub fn reset_flocker(&mut self) {
    self.flocker = None;
  }

  pub fn set_gce_persistent_disk(&mut self, gce_persistent_disk: ::models::V1GcePersistentDiskVolumeSource) {
    self.gce_persistent_disk = Some(gce_persistent_disk);
  }

  pub fn with_gce_persistent_disk(mut self, gce_persistent_disk: ::models::V1GcePersistentDiskVolumeSource) -> V1Volume {
    self.gce_persistent_disk = Some(gce_persistent_disk);
    self
  }

  pub fn gce_persistent_disk(&self) -> Option<&::models::V1GcePersistentDiskVolumeSource> {
    self.gce_persistent_disk.as_ref()
  }

  pub fn reset_gce_persistent_disk(&mut self) {
    self.gce_persistent_disk = None;
  }

  pub fn set_git_repo(&mut self, git_repo: ::models::V1GitRepoVolumeSource) {
    self.git_repo = Some(git_repo);
  }

  pub fn with_git_repo(mut self, git_repo: ::models::V1GitRepoVolumeSource) -> V1Volume {
    self.git_repo = Some(git_repo);
    self
  }

  pub fn git_repo(&self) -> Option<&::models::V1GitRepoVolumeSource> {
    self.git_repo.as_ref()
  }

  pub fn reset_git_repo(&mut self) {
    self.git_repo = None;
  }

  pub fn set_glusterfs(&mut self, glusterfs: ::models::V1GlusterfsVolumeSource) {
    self.glusterfs = Some(glusterfs);
  }

  pub fn with_glusterfs(mut self, glusterfs: ::models::V1GlusterfsVolumeSource) -> V1Volume {
    self.glusterfs = Some(glusterfs);
    self
  }

  pub fn glusterfs(&self) -> Option<&::models::V1GlusterfsVolumeSource> {
    self.glusterfs.as_ref()
  }

  pub fn reset_glusterfs(&mut self) {
    self.glusterfs = None;
  }

  pub fn set_host_path(&mut self, host_path: ::models::V1HostPathVolumeSource) {
    self.host_path = Some(host_path);
  }

  pub fn with_host_path(mut self, host_path: ::models::V1HostPathVolumeSource) -> V1Volume {
    self.host_path = Some(host_path);
    self
  }

  pub fn host_path(&self) -> Option<&::models::V1HostPathVolumeSource> {
    self.host_path.as_ref()
  }

  pub fn reset_host_path(&mut self) {
    self.host_path = None;
  }

  pub fn set_iscsi(&mut self, iscsi: ::models::V1IscsiVolumeSource) {
    self.iscsi = Some(iscsi);
  }

  pub fn with_iscsi(mut self, iscsi: ::models::V1IscsiVolumeSource) -> V1Volume {
    self.iscsi = Some(iscsi);
    self
  }

  pub fn iscsi(&self) -> Option<&::models::V1IscsiVolumeSource> {
    self.iscsi.as_ref()
  }

  pub fn reset_iscsi(&mut self) {
    self.iscsi = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1Volume {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_nfs(&mut self, nfs: ::models::V1NfsVolumeSource) {
    self.nfs = Some(nfs);
  }

  pub fn with_nfs(mut self, nfs: ::models::V1NfsVolumeSource) -> V1Volume {
    self.nfs = Some(nfs);
    self
  }

  pub fn nfs(&self) -> Option<&::models::V1NfsVolumeSource> {
    self.nfs.as_ref()
  }

  pub fn reset_nfs(&mut self) {
    self.nfs = None;
  }

  pub fn set_persistent_volume_claim(&mut self, persistent_volume_claim: ::models::V1PersistentVolumeClaimVolumeSource) {
    self.persistent_volume_claim = Some(persistent_volume_claim);
  }

  pub fn with_persistent_volume_claim(mut self, persistent_volume_claim: ::models::V1PersistentVolumeClaimVolumeSource) -> V1Volume {
    self.persistent_volume_claim = Some(persistent_volume_claim);
    self
  }

  pub fn persistent_volume_claim(&self) -> Option<&::models::V1PersistentVolumeClaimVolumeSource> {
    self.persistent_volume_claim.as_ref()
  }

  pub fn reset_persistent_volume_claim(&mut self) {
    self.persistent_volume_claim = None;
  }

  pub fn set_photon_persistent_disk(&mut self, photon_persistent_disk: ::models::V1PhotonPersistentDiskVolumeSource) {
    self.photon_persistent_disk = Some(photon_persistent_disk);
  }

  pub fn with_photon_persistent_disk(mut self, photon_persistent_disk: ::models::V1PhotonPersistentDiskVolumeSource) -> V1Volume {
    self.photon_persistent_disk = Some(photon_persistent_disk);
    self
  }

  pub fn photon_persistent_disk(&self) -> Option<&::models::V1PhotonPersistentDiskVolumeSource> {
    self.photon_persistent_disk.as_ref()
  }

  pub fn reset_photon_persistent_disk(&mut self) {
    self.photon_persistent_disk = None;
  }

  pub fn set_portworx_volume(&mut self, portworx_volume: ::models::V1PortworxVolumeSource) {
    self.portworx_volume = Some(portworx_volume);
  }

  pub fn with_portworx_volume(mut self, portworx_volume: ::models::V1PortworxVolumeSource) -> V1Volume {
    self.portworx_volume = Some(portworx_volume);
    self
  }

  pub fn portworx_volume(&self) -> Option<&::models::V1PortworxVolumeSource> {
    self.portworx_volume.as_ref()
  }

  pub fn reset_portworx_volume(&mut self) {
    self.portworx_volume = None;
  }

  pub fn set_projected(&mut self, projected: ::models::V1ProjectedVolumeSource) {
    self.projected = Some(projected);
  }

  pub fn with_projected(mut self, projected: ::models::V1ProjectedVolumeSource) -> V1Volume {
    self.projected = Some(projected);
    self
  }

  pub fn projected(&self) -> Option<&::models::V1ProjectedVolumeSource> {
    self.projected.as_ref()
  }

  pub fn reset_projected(&mut self) {
    self.projected = None;
  }

  pub fn set_quobyte(&mut self, quobyte: ::models::V1QuobyteVolumeSource) {
    self.quobyte = Some(quobyte);
  }

  pub fn with_quobyte(mut self, quobyte: ::models::V1QuobyteVolumeSource) -> V1Volume {
    self.quobyte = Some(quobyte);
    self
  }

  pub fn quobyte(&self) -> Option<&::models::V1QuobyteVolumeSource> {
    self.quobyte.as_ref()
  }

  pub fn reset_quobyte(&mut self) {
    self.quobyte = None;
  }

  pub fn set_rbd(&mut self, rbd: ::models::V1RbdVolumeSource) {
    self.rbd = Some(rbd);
  }

  pub fn with_rbd(mut self, rbd: ::models::V1RbdVolumeSource) -> V1Volume {
    self.rbd = Some(rbd);
    self
  }

  pub fn rbd(&self) -> Option<&::models::V1RbdVolumeSource> {
    self.rbd.as_ref()
  }

  pub fn reset_rbd(&mut self) {
    self.rbd = None;
  }

  pub fn set_scale_io(&mut self, scale_io: ::models::V1ScaleIoVolumeSource) {
    self.scale_io = Some(scale_io);
  }

  pub fn with_scale_io(mut self, scale_io: ::models::V1ScaleIoVolumeSource) -> V1Volume {
    self.scale_io = Some(scale_io);
    self
  }

  pub fn scale_io(&self) -> Option<&::models::V1ScaleIoVolumeSource> {
    self.scale_io.as_ref()
  }

  pub fn reset_scale_io(&mut self) {
    self.scale_io = None;
  }

  pub fn set_secret(&mut self, secret: ::models::V1SecretVolumeSource) {
    self.secret = Some(secret);
  }

  pub fn with_secret(mut self, secret: ::models::V1SecretVolumeSource) -> V1Volume {
    self.secret = Some(secret);
    self
  }

  pub fn secret(&self) -> Option<&::models::V1SecretVolumeSource> {
    self.secret.as_ref()
  }

  pub fn reset_secret(&mut self) {
    self.secret = None;
  }

  pub fn set_storageos(&mut self, storageos: ::models::V1StorageOsVolumeSource) {
    self.storageos = Some(storageos);
  }

  pub fn with_storageos(mut self, storageos: ::models::V1StorageOsVolumeSource) -> V1Volume {
    self.storageos = Some(storageos);
    self
  }

  pub fn storageos(&self) -> Option<&::models::V1StorageOsVolumeSource> {
    self.storageos.as_ref()
  }

  pub fn reset_storageos(&mut self) {
    self.storageos = None;
  }

  pub fn set_vsphere_volume(&mut self, vsphere_volume: ::models::V1VsphereVirtualDiskVolumeSource) {
    self.vsphere_volume = Some(vsphere_volume);
  }

  pub fn with_vsphere_volume(mut self, vsphere_volume: ::models::V1VsphereVirtualDiskVolumeSource) -> V1Volume {
    self.vsphere_volume = Some(vsphere_volume);
    self
  }

  pub fn vsphere_volume(&self) -> Option<&::models::V1VsphereVirtualDiskVolumeSource> {
    self.vsphere_volume.as_ref()
  }

  pub fn reset_vsphere_volume(&mut self) {
    self.vsphere_volume = None;
  }

}



