# V1EnvVarSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_map_key_ref** | [***::models::V1ConfigMapKeySelector**](io.k8s.kubernetes.pkg.api.v1.ConfigMapKeySelector.md) | Selects a key of a ConfigMap. | [optional] [default to null]
**field_ref** | [***::models::V1ObjectFieldSelector**](io.k8s.kubernetes.pkg.api.v1.ObjectFieldSelector.md) | Selects a field of the pod: supports metadata.name, metadata.namespace, metadata.labels, metadata.annotations, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP. | [optional] [default to null]
**resource_field_ref** | [***::models::V1ResourceFieldSelector**](io.k8s.kubernetes.pkg.api.v1.ResourceFieldSelector.md) | Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported. | [optional] [default to null]
**secret_key_ref** | [***::models::V1SecretKeySelector**](io.k8s.kubernetes.pkg.api.v1.SecretKeySelector.md) | Selects a key of a secret in the pod&#39;s namespace | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


