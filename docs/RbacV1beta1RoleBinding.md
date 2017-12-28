# RbacV1beta1RoleBinding

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**metadata** | [***::models::MetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) | Standard object&#39;s metadata. | [optional] [default to null]
**role_ref** | [***::models::RbacV1beta1RoleRef**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.RoleRef.md) | RoleRef can reference a Role in the current namespace or a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error. | [default to null]
**subjects** | [**Vec<::models::RbacV1beta1Subject>**](io.k8s.kubernetes.pkg.apis.rbac.v1beta1.Subject.md) | Subjects holds references to the objects the role applies to. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


