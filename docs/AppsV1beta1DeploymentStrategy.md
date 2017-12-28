# AppsV1beta1DeploymentStrategy

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rolling_update** | [***::models::AppsV1beta1RollingUpdateDeployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.RollingUpdateDeployment.md) | Rolling update config params. Present only if DeploymentStrategyType &#x3D; RollingUpdate. | [optional] [default to null]
**_type** | **String** | Type of deployment. Can be \&quot;Recreate\&quot; or \&quot;RollingUpdate\&quot;. Default is RollingUpdate. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


