# V1IscsiVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chap_auth_discovery** | **bool** | whether support iSCSI Discovery CHAP authentication | [optional] [default to null]
**chap_auth_session** | **bool** | whether support iSCSI Session CHAP authentication | [optional] [default to null]
**fs_type** | **String** | Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: \&quot;ext4\&quot;, \&quot;xfs\&quot;, \&quot;ntfs\&quot;. Implicitly inferred to be \&quot;ext4\&quot; if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi | [optional] [default to null]
**iqn** | **String** | Target iSCSI Qualified Name. | [default to null]
**iscsi_interface** | **String** | Optional: Defaults to &#39;default&#39; (tcp). iSCSI interface name that uses an iSCSI transport. | [optional] [default to null]
**lun** | **i32** | iSCSI target lun number. | [default to null]
**portals** | **Vec<String>** | iSCSI target portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260). | [optional] [default to null]
**read_only** | **bool** | ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. | [optional] [default to null]
**secret_ref** | [***::models::V1LocalObjectReference**](io.k8s.kubernetes.pkg.api.v1.LocalObjectReference.md) | CHAP secret for iSCSI target and initiator authentication | [optional] [default to null]
**target_portal** | **String** | iSCSI target portal. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260). | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


