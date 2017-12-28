# V1Affinity

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_affinity** | [***::models::V1NodeAffinity**](io.k8s.kubernetes.pkg.api.v1.NodeAffinity.md) | Describes node affinity scheduling rules for the pod. | [optional] [default to null]
**pod_affinity** | [***::models::V1PodAffinity**](io.k8s.kubernetes.pkg.api.v1.PodAffinity.md) | Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)). | [optional] [default to null]
**pod_anti_affinity** | [***::models::V1PodAntiAffinity**](io.k8s.kubernetes.pkg.api.v1.PodAntiAffinity.md) | Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)). | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


