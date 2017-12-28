# ExtensionsV1beta1Scale

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**metadata** | [***::models::MetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) | Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata. | [optional] [default to null]
**spec** | [***::models::ExtensionsV1beta1ScaleSpec**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.ScaleSpec.md) | defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. | [optional] [default to null]
**status** | [***::models::ExtensionsV1beta1ScaleStatus**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.ScaleStatus.md) | current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


