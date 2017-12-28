# AuthorizationV1beta1ResourceAttributes

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group** | **String** | Group is the API Group of the Resource.  \&quot;*\&quot; means all. | [optional] [default to null]
**name** | **String** | Name is the name of the resource being requested for a \&quot;get\&quot; or deleted for a \&quot;delete\&quot;. \&quot;\&quot; (empty) means all. | [optional] [default to null]
**namespace** | **String** | Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces \&quot;\&quot; (empty) is defaulted for LocalSubjectAccessReviews \&quot;\&quot; (empty) is empty for cluster-scoped resources \&quot;\&quot; (empty) means \&quot;all\&quot; for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview | [optional] [default to null]
**resource** | **String** | Resource is one of the existing resource types.  \&quot;*\&quot; means all. | [optional] [default to null]
**subresource** | **String** | Subresource is one of the existing resource types.  \&quot;\&quot; means none. | [optional] [default to null]
**verb** | **String** | Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  \&quot;*\&quot; means all. | [optional] [default to null]
**version** | **String** | Version is the API Version of the Resource.  \&quot;*\&quot; means all. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


