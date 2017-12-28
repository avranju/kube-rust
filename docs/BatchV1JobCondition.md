# BatchV1JobCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Last time the condition was checked. | [optional] [default to null]
**last_transition_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Last time the condition transit from one status to another. | [optional] [default to null]
**message** | **String** | Human readable message indicating details about last transition. | [optional] [default to null]
**reason** | **String** | (brief) reason for the condition&#39;s last transition. | [optional] [default to null]
**status** | **String** | Status of the condition, one of True, False, Unknown. | [default to null]
**_type** | **String** | Type of job condition, Complete or Failed. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


