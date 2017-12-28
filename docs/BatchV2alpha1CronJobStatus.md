# BatchV2alpha1CronJobStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | [**Vec<::models::V1ObjectReference>**](io.k8s.kubernetes.pkg.api.v1.ObjectReference.md) | A list of pointers to currently running jobs. | [optional] [default to null]
**last_schedule_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Information when was the last time the job was successfully scheduled. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


