# AuthorizationV1SubjectAccessReviewSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**extra** | [**::std::collections::HashMap<String, Vec<String>>**](array.md) | Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer it needs a reflection here. | [optional] [default to null]
**groups** | **Vec<String>** | Groups is the groups you&#39;re testing for. | [optional] [default to null]
**non_resource_attributes** | [***::models::AuthorizationV1NonResourceAttributes**](io.k8s.kubernetes.pkg.apis.authorization.v1.NonResourceAttributes.md) | NonResourceAttributes describes information for a non-resource access request | [optional] [default to null]
**resource_attributes** | [***::models::AuthorizationV1ResourceAttributes**](io.k8s.kubernetes.pkg.apis.authorization.v1.ResourceAttributes.md) | ResourceAuthorizationAttributes describes information for a resource access request | [optional] [default to null]
**user** | **String** | User is the user you&#39;re testing for. If you specify \&quot;User\&quot; but not \&quot;Groups\&quot;, then is it interpreted as \&quot;What if User were not a member of any groups | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


