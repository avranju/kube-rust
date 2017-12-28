# ExtensionsV1beta1SupplementalGroupsStrategyOptions

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ranges** | [**Vec<::models::ExtensionsV1beta1IdRange>**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.IDRange.md) | Ranges are the allowed ranges of supplemental groups.  If you would like to force a single supplemental group then supply a single range with the same start and end. | [optional] [default to null]
**rule** | **String** | Rule is the strategy that will dictate what supplemental groups is used in the SecurityContext. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


