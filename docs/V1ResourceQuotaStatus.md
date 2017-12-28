# V1ResourceQuotaStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hard** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Hard is the set of enforced hard limits for each named resource. More info: https://git.k8s.io/community/contributors/design-proposals/admission_control_resource_quota.md | [optional] [default to null]
**used** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Used is the current observed total usage of the resource in the namespace. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


