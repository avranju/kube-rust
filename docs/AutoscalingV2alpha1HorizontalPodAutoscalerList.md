# AutoscalingV2alpha1HorizontalPodAutoscalerList

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**items** | [**Vec<::models::AutoscalingV2alpha1HorizontalPodAutoscaler>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.HorizontalPodAutoscaler.md) | items is the list of horizontal pod autoscaler objects. | [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**metadata** | [***::models::MetaV1ListMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta.md) | metadata is the standard list metadata. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


