# AutoscalingV2alpha1MetricStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**object** | [***::models::AutoscalingV2alpha1ObjectMetricStatus**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.ObjectMetricStatus.md) | object refers to a metric describing a single kubernetes object (for example, hits-per-second on an Ingress object). | [optional] [default to null]
**pods** | [***::models::AutoscalingV2alpha1PodsMetricStatus**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.PodsMetricStatus.md) | pods refers to a metric describing each pod in the current scale target (for example, transactions-processed-per-second).  The values will be averaged together before being compared to the target value. | [optional] [default to null]
**resource** | [***::models::AutoscalingV2alpha1ResourceMetricStatus**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.ResourceMetricStatus.md) | resource refers to a resource metric (such as those specified in requests and limits) known to Kubernetes describing each pod in the current scale target (e.g. CPU or memory). Such metrics are built in to Kubernetes, and have special scaling options on top of those available to normal per-pod metrics using the \&quot;pods\&quot; source. | [optional] [default to null]
**_type** | **String** | type is the type of metric source.  It will match one of the fields below. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


