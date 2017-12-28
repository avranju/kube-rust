# ExtensionsV1beta1DeploymentStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_replicas** | **i32** | Total number of available pods (ready for at least minReadySeconds) targeted by this deployment. | [optional] [default to null]
**collision_count** | **i64** | Count of hash collisions for the Deployment. The Deployment controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ReplicaSet. | [optional] [default to null]
**conditions** | [**Vec<::models::ExtensionsV1beta1DeploymentCondition>**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DeploymentCondition.md) | Represents the latest available observations of a deployment&#39;s current state. | [optional] [default to null]
**observed_generation** | **i64** | The generation observed by the deployment controller. | [optional] [default to null]
**ready_replicas** | **i32** | Total number of ready pods targeted by this deployment. | [optional] [default to null]
**replicas** | **i32** | Total number of non-terminated pods targeted by this deployment (their labels match the selector). | [optional] [default to null]
**unavailable_replicas** | **i32** | Total number of unavailable pods targeted by this deployment. | [optional] [default to null]
**updated_replicas** | **i32** | Total number of non-terminated pods targeted by this deployment that have the desired template spec. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


