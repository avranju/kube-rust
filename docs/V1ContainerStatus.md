# V1ContainerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | **String** | Container&#39;s ID in the format &#39;docker://&lt;container_id&gt;&#39;. | [optional] [default to null]
**image** | **String** | The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images | [default to null]
**image_id** | **String** | ImageID of the container&#39;s image. | [default to null]
**last_state** | [***::models::V1ContainerState**](io.k8s.kubernetes.pkg.api.v1.ContainerState.md) | Details about the container&#39;s last termination condition. | [optional] [default to null]
**name** | **String** | This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated. | [default to null]
**ready** | **bool** | Specifies whether the container has passed its readiness probe. | [default to null]
**restart_count** | **i32** | The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC. | [default to null]
**state** | [***::models::V1ContainerState**](io.k8s.kubernetes.pkg.api.v1.ContainerState.md) | Details about the container&#39;s current condition. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


