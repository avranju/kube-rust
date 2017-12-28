# V1VsphereVirtualDiskVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fs_type** | **String** | Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \&quot;ext4\&quot;, \&quot;xfs\&quot;, \&quot;ntfs\&quot;. Implicitly inferred to be \&quot;ext4\&quot; if unspecified. | [optional] [default to null]
**storage_policy_id** | **String** | Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName. | [optional] [default to null]
**storage_policy_name** | **String** | Storage Policy Based Management (SPBM) profile name. | [optional] [default to null]
**volume_path** | **String** | Path that identifies vSphere volume vmdk | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


