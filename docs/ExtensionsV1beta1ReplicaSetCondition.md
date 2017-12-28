# ExtensionsV1beta1ReplicaSetCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_transition_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | The last time the condition transitioned from one status to another. | [optional] [default to null]
**message** | **String** | A human readable message indicating details about the transition. | [optional] [default to null]
**reason** | **String** | The reason for the condition&#39;s last transition. | [optional] [default to null]
**status** | **String** | Status of the condition, one of True, False, Unknown. | [default to null]
**_type** | **String** | Type of replica set condition. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


