# V1Taint

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**effect** | **String** | Required. The effect of the taint on pods that do not tolerate the taint. Valid effects are NoSchedule, PreferNoSchedule and NoExecute. | [default to null]
**key** | **String** | Required. The taint key to be applied to a node. | [default to null]
**time_added** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | TimeAdded represents the time at which the taint was added. It is only written for NoExecute taints. | [optional] [default to null]
**value** | **String** | Required. The taint value corresponding to the taint key. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


