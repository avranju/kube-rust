# ExtensionsV1beta1DaemonSetUpdateStrategy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rolling_update** | [***::models::ExtensionsV1beta1RollingUpdateDaemonSet**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.RollingUpdateDaemonSet.md) | Rolling update config params. Present only if type &#x3D; \&quot;RollingUpdate\&quot;. | [optional] [default to null]
**_type** | **String** | Type of daemon set update. Can be \&quot;RollingUpdate\&quot; or \&quot;OnDelete\&quot;. Default is OnDelete. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


