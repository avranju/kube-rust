# V1VolumeMount

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mount_path** | **String** | Path within the container at which the volume should be mounted.  Must not contain &#39;:&#39;. | [default to null]
**name** | **String** | This must match the Name of a Volume. | [default to null]
**read_only** | **bool** | Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false. | [optional] [default to null]
**sub_path** | **String** | Path within the volume from which the container&#39;s volume should be mounted. Defaults to \&quot;\&quot; (volume&#39;s root). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


