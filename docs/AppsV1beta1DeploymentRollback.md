# AppsV1beta1DeploymentRollback

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**name** | **String** | Required: This must match the Name of a deployment. | [default to null]
**rollback_to** | [***::models::AppsV1beta1RollbackConfig**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.RollbackConfig.md) | The config of this deployment rollback. | [default to null]
**updated_annotations** | **::std::collections::HashMap<String, String>** | The annotations to be updated to a deployment | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


