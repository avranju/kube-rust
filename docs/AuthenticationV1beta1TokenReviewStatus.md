# AuthenticationV1beta1TokenReviewStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authenticated** | **bool** | Authenticated indicates that the token was associated with a known user. | [optional] [default to null]
**error** | **String** | Error indicates that the token couldn&#39;t be checked | [optional] [default to null]
**user** | [***::models::AuthenticationV1beta1UserInfo**](io.k8s.kubernetes.pkg.apis.authentication.v1beta1.UserInfo.md) | User is the UserInfo associated with the provided token. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


