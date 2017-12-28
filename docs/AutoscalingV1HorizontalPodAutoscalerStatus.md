# AutoscalingV1HorizontalPodAutoscalerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_cpu_utilization_percentage** | **i32** | current average CPU utilization over all pods, represented as a percentage of requested CPU, e.g. 70 means that an average pod is using now 70% of its requested CPU. | [optional] [default to null]
**current_replicas** | **i32** | current number of replicas of pods managed by this autoscaler. | [default to null]
**desired_replicas** | **i32** | desired number of replicas of pods managed by this autoscaler. | [default to null]
**last_scale_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | last time the HorizontalPodAutoscaler scaled the number of pods; used by the autoscaler to control how often the number of pods is changed. | [optional] [default to null]
**observed_generation** | **i64** | most recent generation observed by this autoscaler. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


