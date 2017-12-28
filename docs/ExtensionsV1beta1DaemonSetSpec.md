# ExtensionsV1beta1DaemonSetSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready). | [optional] [default to null]
**revision_history_limit** | **i32** | The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10. | [optional] [default to null]
**selector** | [***::models::MetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) | A label query over pods that are managed by the daemon set. Must match in order to be controlled. If empty, defaulted to labels on Pod template. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors | [optional] [default to null]
**template** | [***::models::V1PodTemplateSpec**](io.k8s.kubernetes.pkg.api.v1.PodTemplateSpec.md) | An object that describes the pod that will be created. The DaemonSet will create exactly one copy of this pod on every node that matches the template&#39;s node selector (or on every node if no node selector is specified). More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#pod-template | [default to null]
**template_generation** | **i64** | DEPRECATED. A sequence number representing a specific generation of the template. Populated by the system. It can be set only during the creation. | [optional] [default to null]
**update_strategy** | [***::models::ExtensionsV1beta1DaemonSetUpdateStrategy**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DaemonSetUpdateStrategy.md) | An update strategy to replace existing DaemonSet pods with new pods. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


