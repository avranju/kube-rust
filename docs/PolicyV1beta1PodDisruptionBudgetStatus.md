# PolicyV1beta1PodDisruptionBudgetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_healthy** | **i32** | current number of healthy pods | [default to null]
**desired_healthy** | **i32** | minimum desired number of healthy pods | [default to null]
**disrupted_pods** | [**::std::collections::HashMap<String, ::models::MetaV1Time>**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | DisruptedPods contains information about pods whose eviction was processed by the API server eviction subresource handler but has not yet been observed by the PodDisruptionBudget controller. A pod will be in this map from the time when the API server processed the eviction request to the time when the pod is seen by PDB controller as having been marked for deletion (or after a timeout). The key in the map is the name of the pod and the value is the time when the API server processed the eviction request. If the deletion didn&#39;t occur and a pod is still there it will be removed from the list automatically by PodDisruptionBudget controller after some time. If everything goes smooth this map should be empty for the most of the time. Large number of entries in the map may indicate problems with pod deletions. | [default to null]
**disruptions_allowed** | **i32** | Number of pod disruptions that are currently allowed. | [default to null]
**expected_pods** | **i32** | total number of pods counted by this disruption budget | [default to null]
**observed_generation** | **i64** | Most recent generation observed when updating this PDB status. PodDisruptionsAllowed and other status informatio is valid only if observedGeneration equals to PDB&#39;s object generation. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


