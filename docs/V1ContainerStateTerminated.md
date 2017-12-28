# V1ContainerStateTerminated

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | **String** | Container&#39;s ID in the format &#39;docker://&lt;container_id&gt;&#39; | [optional] [default to null]
**exit_code** | **i32** | Exit status from the last termination of the container | [default to null]
**finished_at** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Time at which the container last terminated | [optional] [default to null]
**message** | **String** | Message regarding the last termination of the container | [optional] [default to null]
**reason** | **String** | (brief) reason from the last termination of the container | [optional] [default to null]
**signal** | **i32** | Signal from the last termination of the container | [optional] [default to null]
**started_at** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | Time at which previous execution of the container started | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


