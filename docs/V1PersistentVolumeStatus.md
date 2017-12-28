# V1PersistentVolumeStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | A human-readable message indicating details about why the volume is in this state. | [optional] [default to null]
**phase** | **String** | Phase indicates if a volume is available, bound to a claim, or released by a claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#phase | [optional] [default to null]
**reason** | **String** | Reason is a brief CamelCase string that describes any failure and is meant for machine parsing and tidy display in the CLI. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


