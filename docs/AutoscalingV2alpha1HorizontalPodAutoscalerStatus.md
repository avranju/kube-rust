# AutoscalingV2alpha1HorizontalPodAutoscalerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<::models::AutoscalingV2alpha1HorizontalPodAutoscalerCondition>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.HorizontalPodAutoscalerCondition.md) | conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met. | [default to null]
**current_metrics** | [**Vec<::models::AutoscalingV2alpha1MetricStatus>**](io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.MetricStatus.md) | currentMetrics is the last read state of the metrics used by this autoscaler. | [default to null]
**current_replicas** | **i32** | currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler. | [default to null]
**desired_replicas** | **i32** | desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. | [default to null]
**last_scale_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | lastScaleTime is the last time the HorizontalPodAutoscaler scaled the number of pods, used by the autoscaler to control how often the number of pods is changed. | [optional] [default to null]
**observed_generation** | **i64** | observedGeneration is the most recent generation observed by this autoscaler. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


