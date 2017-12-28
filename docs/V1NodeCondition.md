# V1NodeCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_heartbeat_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Last time we got an update on a given condition. | [optional] [default to null]
**last_transition_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Last time the condition transit from one status to another. | [optional] [default to null]
**message** | **String** | Human readable message indicating details about last transition. | [optional] [default to null]
**reason** | **String** | (brief) reason for the condition&#39;s last transition. | [optional] [default to null]
**status** | **String** | Status of the condition, one of True, False, Unknown. | [default to null]
**_type** | **String** | Type of node condition. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


