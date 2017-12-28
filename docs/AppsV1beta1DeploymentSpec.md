# AppsV1beta1DeploymentSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) | [optional] [default to null]
**paused** | **bool** | Indicates that the deployment is paused. | [optional] [default to null]
**progress_deadline_seconds** | **i32** | The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Once autoRollback is implemented, the deployment controller will automatically rollback failed deployments. Note that progress will not be estimated during the time a deployment is paused. Defaults to 600s. | [optional] [default to null]
**replicas** | **i32** | Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1. | [optional] [default to null]
**revision_history_limit** | **i32** | The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 2. | [optional] [default to null]
**rollback_to** | [***::models::AppsV1beta1RollbackConfig**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.RollbackConfig.md) | The config this deployment is rolling back to. Will be cleared after rollback is done. | [optional] [default to null]
**selector** | [***::models::MetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) | Label selector for pods. Existing ReplicaSets whose pods are selected by this will be the ones affected by this deployment. | [optional] [default to null]
**strategy** | [***::models::AppsV1beta1DeploymentStrategy**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.DeploymentStrategy.md) | The deployment strategy to use to replace existing pods with new ones. | [optional] [default to null]
**template** | [***::models::V1PodTemplateSpec**](io.k8s.kubernetes.pkg.api.v1.PodTemplateSpec.md) | Template describes the pods that will be created. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


