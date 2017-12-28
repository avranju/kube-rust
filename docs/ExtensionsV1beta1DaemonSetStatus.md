# ExtensionsV1beta1DaemonSetStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collision_count** | **i64** | Count of hash collisions for the DaemonSet. The DaemonSet controller uses this field as a collision avoidance mechanism when it needs to create the name for the newest ControllerRevision. | [optional] [default to null]
**current_number_scheduled** | **i32** | The number of nodes that are running at least 1 daemon pod and are supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | [default to null]
**desired_number_scheduled** | **i32** | The total number of nodes that should be running the daemon pod (including nodes correctly running the daemon pod). More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | [default to null]
**number_available** | **i32** | The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and available (ready for at least spec.minReadySeconds) | [optional] [default to null]
**number_misscheduled** | **i32** | The number of nodes that are running the daemon pod, but are not supposed to run the daemon pod. More info: https://kubernetes.io/docs/concepts/workloads/controllers/daemonset/ | [default to null]
**number_ready** | **i32** | The number of nodes that should be running the daemon pod and have one or more of the daemon pod running and ready. | [default to null]
**number_unavailable** | **i32** | The number of nodes that should be running the daemon pod and have none of the daemon pod running and available (ready for at least spec.minReadySeconds) | [optional] [default to null]
**observed_generation** | **i64** | The most recent generation observed by the daemon set controller. | [optional] [default to null]
**updated_number_scheduled** | **i32** | The total number of nodes that are running updated daemon pod | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


