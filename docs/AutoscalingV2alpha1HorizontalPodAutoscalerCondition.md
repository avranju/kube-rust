# AutoscalingV2alpha1HorizontalPodAutoscalerCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_transition_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | lastTransitionTime is the last time the condition transitioned from one status to another | [optional] [default to null]
**message** | **String** | message is a human-readable explanation containing details about the transition | [optional] [default to null]
**reason** | **String** | reason is the reason for the condition&#39;s last transition. | [optional] [default to null]
**status** | **String** | status is the status of the condition (True, False, Unknown) | [default to null]
**_type** | **String** | type describes the current condition | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


