# AutoscalingV2alpha1ObjectMetricStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_value** | [***::models::ResourceQuantity**](io.k8s.apimachinery.pkg.api.resource.Quantity.md) | currentValue is the current value of the metric (as a quantity). | [default to null]
**metric_name** | **String** | metricName is the name of the metric in question. | [default to null]
**target** | [***::models::AutoscalingV2alpha1CrossVersionObjectReference**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.CrossVersionObjectReference.md) | target is the described Kubernetes object. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


