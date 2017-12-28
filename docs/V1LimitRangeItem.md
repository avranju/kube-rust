# V1LimitRangeItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Default resource requirement limit value by resource name if resource limit is omitted. | [optional] [default to null]
**default_request** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | DefaultRequest is the default resource requirement request value by resource name if resource request is omitted. | [optional] [default to null]
**max** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Max usage constraints on this kind by resource name. | [optional] [default to null]
**max_limit_request_ratio** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | MaxLimitRequestRatio if specified, the named resource must have a request and limit that are both non-zero where limit divided by request is less than or equal to the enumerated value; this represents the max burst for the named resource. | [optional] [default to null]
**min** | [**::std::collections::HashMap<String, ::models::ResourceQuantity>**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | Min usage constraints on this kind by resource name. | [optional] [default to null]
**_type** | **String** | Type of resource that this limit applies to. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


