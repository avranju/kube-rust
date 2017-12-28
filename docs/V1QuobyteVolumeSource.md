# V1QuobyteVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | **String** | Group to map volume access to Default is no group | [optional] [default to null]
**read_only** | **bool** | ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false. | [optional] [default to null]
**registry** | **String** | Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes | [default to null]
**user** | **String** | User to map volume access to Defaults to serivceaccount user | [optional] [default to null]
**volume** | **String** | Volume is a string that references an already created Quobyte volume by name. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


