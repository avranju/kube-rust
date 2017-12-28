/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PersistentVolumeSpec : PersistentVolumeSpec is the specification of a persistent volume.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1PersistentVolumeSpec {
  /// AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes
  #[serde(rename = "accessModes")]
  access_modes: Option<Vec<String>>,
  /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
  #[serde(rename = "awsElasticBlockStore")]
  aws_elastic_block_store: Option<::models::V1AwsElasticBlockStoreVolumeSource>,
  /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
  #[serde(rename = "azureDisk")]
  azure_disk: Option<::models::V1AzureDiskVolumeSource>,
  /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
  #[serde(rename = "azureFile")]
  azure_file: Option<::models::V1AzureFileVolumeSource>,
  /// A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity
  #[serde(rename = "capacity")]
  capacity: Option<::std::collections::HashMap<String, ::models::ResourceQuantity>>,
  /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
  #[serde(rename = "cephfs")]
  cephfs: Option<::models::V1CephFsVolumeSource>,
  /// Cinder represents a cinder volume attached and mounted on kubelets host machine More info: https://releases.k8s.io/HEAD/examples/mysql-cinder-pd/README.md
  #[serde(rename = "cinder")]
  cinder: Option<::models::V1CinderVolumeSource>,
  /// ClaimRef is part of a bi-directional binding between PersistentVolume and PersistentVolumeClaim. Expected to be non-nil when bound. claim.VolumeName is the authoritative bind between PV and PVC. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#binding
  #[serde(rename = "claimRef")]
  claim_ref: Option<::models::V1ObjectReference>,
  /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
  #[serde(rename = "fc")]
  fc: Option<::models::V1FcVolumeSource>,
  /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin. This is an alpha feature and may change in future.
  #[serde(rename = "flexVolume")]
  flex_volume: Option<::models::V1FlexVolumeSource>,
  /// Flocker represents a Flocker volume attached to a kubelet's host machine and exposed to the pod for its usage. This depends on the Flocker control service being running
  #[serde(rename = "flocker")]
  flocker: Option<::models::V1FlockerVolumeSource>,
  /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
  #[serde(rename = "gcePersistentDisk")]
  gce_persistent_disk: Option<::models::V1GcePersistentDiskVolumeSource>,
  /// Glusterfs represents a Glusterfs volume that is attached to a host and exposed to the pod. Provisioned by an admin. More info: https://releases.k8s.io/HEAD/examples/volumes/glusterfs/README.md
  #[serde(rename = "glusterfs")]
  glusterfs: Option<::models::V1GlusterfsVolumeSource>,
  /// HostPath represents a directory on the host. Provisioned by a developer or tester. This is useful for single-node development and testing only! On-host storage is not supported in any way and WILL NOT WORK in a multi-node cluster. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
  #[serde(rename = "hostPath")]
  host_path: Option<::models::V1HostPathVolumeSource>,
  /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. Provisioned by an admin.
  #[serde(rename = "iscsi")]
  iscsi: Option<::models::V1IscsiVolumeSource>,
  /// Local represents directly-attached storage with node affinity
  #[serde(rename = "local")]
  local: Option<::models::V1LocalVolumeSource>,
  /// NFS represents an NFS mount on the host. Provisioned by an admin. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
  #[serde(rename = "nfs")]
  nfs: Option<::models::V1NfsVolumeSource>,
  /// What happens to a persistent volume when released from its claim. Valid options are Retain (default) and Recycle. Recycling must be supported by the volume plugin underlying this persistent volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming
  #[serde(rename = "persistentVolumeReclaimPolicy")]
  persistent_volume_reclaim_policy: Option<String>,
  /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
  #[serde(rename = "photonPersistentDisk")]
  photon_persistent_disk: Option<::models::V1PhotonPersistentDiskVolumeSource>,
  /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
  #[serde(rename = "portworxVolume")]
  portworx_volume: Option<::models::V1PortworxVolumeSource>,
  /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
  #[serde(rename = "quobyte")]
  quobyte: Option<::models::V1QuobyteVolumeSource>,
  /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://releases.k8s.io/HEAD/examples/volumes/rbd/README.md
  #[serde(rename = "rbd")]
  rbd: Option<::models::V1RbdVolumeSource>,
  /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
  #[serde(rename = "scaleIO")]
  scale_io: Option<::models::V1ScaleIoVolumeSource>,
  /// Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass.
  #[serde(rename = "storageClassName")]
  storage_class_name: Option<String>,
  /// StorageOS represents a StorageOS volume that is attached to the kubelet's host machine and mounted into the pod More info: https://releases.k8s.io/HEAD/examples/volumes/storageos/README.md
  #[serde(rename = "storageos")]
  storageos: Option<::models::V1StorageOsPersistentVolumeSource>,
  /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
  #[serde(rename = "vsphereVolume")]
  vsphere_volume: Option<::models::V1VsphereVirtualDiskVolumeSource>
}

impl V1PersistentVolumeSpec {
  /// PersistentVolumeSpec is the specification of a persistent volume.
  pub fn new() -> V1PersistentVolumeSpec {
    V1PersistentVolumeSpec {
      access_modes: None,
      aws_elastic_block_store: None,
      azure_disk: None,
      azure_file: None,
      capacity: None,
      cephfs: None,
      cinder: None,
      claim_ref: None,
      fc: None,
      flex_volume: None,
      flocker: None,
      gce_persistent_disk: None,
      glusterfs: None,
      host_path: None,
      iscsi: None,
      local: None,
      nfs: None,
      persistent_volume_reclaim_policy: None,
      photon_persistent_disk: None,
      portworx_volume: None,
      quobyte: None,
      rbd: None,
      scale_io: None,
      storage_class_name: None,
      storageos: None,
      vsphere_volume: None
    }
  }

  pub fn set_access_modes(&mut self, access_modes: Vec<String>) {
    self.access_modes = Some(access_modes);
  }

  pub fn with_access_modes(mut self, access_modes: Vec<String>) -> V1PersistentVolumeSpec {
    self.access_modes = Some(access_modes);
    self
  }

  pub fn access_modes(&self) -> Option<&Vec<String>> {
    self.access_modes.as_ref()
  }

  pub fn reset_access_modes(&mut self) {
    self.access_modes = None;
  }

  pub fn set_aws_elastic_block_store(&mut self, aws_elastic_block_store: ::models::V1AwsElasticBlockStoreVolumeSource) {
    self.aws_elastic_block_store = Some(aws_elastic_block_store);
  }

  pub fn with_aws_elastic_block_store(mut self, aws_elastic_block_store: ::models::V1AwsElasticBlockStoreVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_azure_disk(mut self, azure_disk: ::models::V1AzureDiskVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_azure_file(mut self, azure_file: ::models::V1AzureFileVolumeSource) -> V1PersistentVolumeSpec {
    self.azure_file = Some(azure_file);
    self
  }

  pub fn azure_file(&self) -> Option<&::models::V1AzureFileVolumeSource> {
    self.azure_file.as_ref()
  }

  pub fn reset_azure_file(&mut self) {
    self.azure_file = None;
  }

  pub fn set_capacity(&mut self, capacity: ::std::collections::HashMap<String, ::models::ResourceQuantity>) {
    self.capacity = Some(capacity);
  }

  pub fn with_capacity(mut self, capacity: ::std::collections::HashMap<String, ::models::ResourceQuantity>) -> V1PersistentVolumeSpec {
    self.capacity = Some(capacity);
    self
  }

  pub fn capacity(&self) -> Option<&::std::collections::HashMap<String, ::models::ResourceQuantity>> {
    self.capacity.as_ref()
  }

  pub fn reset_capacity(&mut self) {
    self.capacity = None;
  }

  pub fn set_cephfs(&mut self, cephfs: ::models::V1CephFsVolumeSource) {
    self.cephfs = Some(cephfs);
  }

  pub fn with_cephfs(mut self, cephfs: ::models::V1CephFsVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_cinder(mut self, cinder: ::models::V1CinderVolumeSource) -> V1PersistentVolumeSpec {
    self.cinder = Some(cinder);
    self
  }

  pub fn cinder(&self) -> Option<&::models::V1CinderVolumeSource> {
    self.cinder.as_ref()
  }

  pub fn reset_cinder(&mut self) {
    self.cinder = None;
  }

  pub fn set_claim_ref(&mut self, claim_ref: ::models::V1ObjectReference) {
    self.claim_ref = Some(claim_ref);
  }

  pub fn with_claim_ref(mut self, claim_ref: ::models::V1ObjectReference) -> V1PersistentVolumeSpec {
    self.claim_ref = Some(claim_ref);
    self
  }

  pub fn claim_ref(&self) -> Option<&::models::V1ObjectReference> {
    self.claim_ref.as_ref()
  }

  pub fn reset_claim_ref(&mut self) {
    self.claim_ref = None;
  }

  pub fn set_fc(&mut self, fc: ::models::V1FcVolumeSource) {
    self.fc = Some(fc);
  }

  pub fn with_fc(mut self, fc: ::models::V1FcVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_flex_volume(mut self, flex_volume: ::models::V1FlexVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_flocker(mut self, flocker: ::models::V1FlockerVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_gce_persistent_disk(mut self, gce_persistent_disk: ::models::V1GcePersistentDiskVolumeSource) -> V1PersistentVolumeSpec {
    self.gce_persistent_disk = Some(gce_persistent_disk);
    self
  }

  pub fn gce_persistent_disk(&self) -> Option<&::models::V1GcePersistentDiskVolumeSource> {
    self.gce_persistent_disk.as_ref()
  }

  pub fn reset_gce_persistent_disk(&mut self) {
    self.gce_persistent_disk = None;
  }

  pub fn set_glusterfs(&mut self, glusterfs: ::models::V1GlusterfsVolumeSource) {
    self.glusterfs = Some(glusterfs);
  }

  pub fn with_glusterfs(mut self, glusterfs: ::models::V1GlusterfsVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_host_path(mut self, host_path: ::models::V1HostPathVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_iscsi(mut self, iscsi: ::models::V1IscsiVolumeSource) -> V1PersistentVolumeSpec {
    self.iscsi = Some(iscsi);
    self
  }

  pub fn iscsi(&self) -> Option<&::models::V1IscsiVolumeSource> {
    self.iscsi.as_ref()
  }

  pub fn reset_iscsi(&mut self) {
    self.iscsi = None;
  }

  pub fn set_local(&mut self, local: ::models::V1LocalVolumeSource) {
    self.local = Some(local);
  }

  pub fn with_local(mut self, local: ::models::V1LocalVolumeSource) -> V1PersistentVolumeSpec {
    self.local = Some(local);
    self
  }

  pub fn local(&self) -> Option<&::models::V1LocalVolumeSource> {
    self.local.as_ref()
  }

  pub fn reset_local(&mut self) {
    self.local = None;
  }

  pub fn set_nfs(&mut self, nfs: ::models::V1NfsVolumeSource) {
    self.nfs = Some(nfs);
  }

  pub fn with_nfs(mut self, nfs: ::models::V1NfsVolumeSource) -> V1PersistentVolumeSpec {
    self.nfs = Some(nfs);
    self
  }

  pub fn nfs(&self) -> Option<&::models::V1NfsVolumeSource> {
    self.nfs.as_ref()
  }

  pub fn reset_nfs(&mut self) {
    self.nfs = None;
  }

  pub fn set_persistent_volume_reclaim_policy(&mut self, persistent_volume_reclaim_policy: String) {
    self.persistent_volume_reclaim_policy = Some(persistent_volume_reclaim_policy);
  }

  pub fn with_persistent_volume_reclaim_policy(mut self, persistent_volume_reclaim_policy: String) -> V1PersistentVolumeSpec {
    self.persistent_volume_reclaim_policy = Some(persistent_volume_reclaim_policy);
    self
  }

  pub fn persistent_volume_reclaim_policy(&self) -> Option<&String> {
    self.persistent_volume_reclaim_policy.as_ref()
  }

  pub fn reset_persistent_volume_reclaim_policy(&mut self) {
    self.persistent_volume_reclaim_policy = None;
  }

  pub fn set_photon_persistent_disk(&mut self, photon_persistent_disk: ::models::V1PhotonPersistentDiskVolumeSource) {
    self.photon_persistent_disk = Some(photon_persistent_disk);
  }

  pub fn with_photon_persistent_disk(mut self, photon_persistent_disk: ::models::V1PhotonPersistentDiskVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_portworx_volume(mut self, portworx_volume: ::models::V1PortworxVolumeSource) -> V1PersistentVolumeSpec {
    self.portworx_volume = Some(portworx_volume);
    self
  }

  pub fn portworx_volume(&self) -> Option<&::models::V1PortworxVolumeSource> {
    self.portworx_volume.as_ref()
  }

  pub fn reset_portworx_volume(&mut self) {
    self.portworx_volume = None;
  }

  pub fn set_quobyte(&mut self, quobyte: ::models::V1QuobyteVolumeSource) {
    self.quobyte = Some(quobyte);
  }

  pub fn with_quobyte(mut self, quobyte: ::models::V1QuobyteVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_rbd(mut self, rbd: ::models::V1RbdVolumeSource) -> V1PersistentVolumeSpec {
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

  pub fn with_scale_io(mut self, scale_io: ::models::V1ScaleIoVolumeSource) -> V1PersistentVolumeSpec {
    self.scale_io = Some(scale_io);
    self
  }

  pub fn scale_io(&self) -> Option<&::models::V1ScaleIoVolumeSource> {
    self.scale_io.as_ref()
  }

  pub fn reset_scale_io(&mut self) {
    self.scale_io = None;
  }

  pub fn set_storage_class_name(&mut self, storage_class_name: String) {
    self.storage_class_name = Some(storage_class_name);
  }

  pub fn with_storage_class_name(mut self, storage_class_name: String) -> V1PersistentVolumeSpec {
    self.storage_class_name = Some(storage_class_name);
    self
  }

  pub fn storage_class_name(&self) -> Option<&String> {
    self.storage_class_name.as_ref()
  }

  pub fn reset_storage_class_name(&mut self) {
    self.storage_class_name = None;
  }

  pub fn set_storageos(&mut self, storageos: ::models::V1StorageOsPersistentVolumeSource) {
    self.storageos = Some(storageos);
  }

  pub fn with_storageos(mut self, storageos: ::models::V1StorageOsPersistentVolumeSource) -> V1PersistentVolumeSpec {
    self.storageos = Some(storageos);
    self
  }

  pub fn storageos(&self) -> Option<&::models::V1StorageOsPersistentVolumeSource> {
    self.storageos.as_ref()
  }

  pub fn reset_storageos(&mut self) {
    self.storageos = None;
  }

  pub fn set_vsphere_volume(&mut self, vsphere_volume: ::models::V1VsphereVirtualDiskVolumeSource) {
    self.vsphere_volume = Some(vsphere_volume);
  }

  pub fn with_vsphere_volume(mut self, vsphere_volume: ::models::V1VsphereVirtualDiskVolumeSource) -> V1PersistentVolumeSpec {
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



