# MetaV1StatusDetails

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**causes** | [**Vec<::models::MetaV1StatusCause>**](io.k8s.apimachinery.pkg.apis.meta.v1.StatusCause.md) | The Causes array includes more details associated with the StatusReason failure. Not all StatusReasons may provide detailed causes. | [optional] [default to null]
**group** | **String** | The group attribute of the resource associated with the status StatusReason. | [optional] [default to null]
**kind** | **String** | The kind attribute of the resource associated with the status StatusReason. On some operations may differ from the requested resource Kind. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**name** | **String** | The name attribute of the resource associated with the status StatusReason (when there is a single name which can be described). | [optional] [default to null]
**retry_after_seconds** | **i32** | If specified, the time in seconds before the operation should be retried. | [optional] [default to null]
**uid** | **String** | UID of the resource. (when there is a single resource which can be described). More info: http://kubernetes.io/docs/user-guide/identifiers#uids | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


