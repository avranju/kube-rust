# V1PersistentVolumeClaimStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | **Vec<String>** | AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1 | [optional] [default to null]
**capacity** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Represents the actual resources of the underlying volume. | [optional] [default to null]
**phase** | **String** | Phase represents the current phase of PersistentVolumeClaim. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


