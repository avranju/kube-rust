# V1PodStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<::models::V1PodCondition>**](io.k8s.kubernetes.pkg.api.v1.PodCondition.md) | Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions | [optional] [default to null]
**container_statuses** | [**Vec<::models::V1ContainerStatus>**](io.k8s.kubernetes.pkg.api.v1.ContainerStatus.md) | The list has one entry per container in the manifest. Each entry is currently the output of &#x60;docker inspect&#x60;. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status | [optional] [default to null]
**host_ip** | **String** | IP address of the host to which the pod is assigned. Empty if not yet scheduled. | [optional] [default to null]
**init_container_statuses** | [**Vec<::models::V1ContainerStatus>**](io.k8s.kubernetes.pkg.api.v1.ContainerStatus.md) | The list has one entry per init container in the manifest. The most recent successful init container will have ready &#x3D; true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status | [optional] [default to null]
**message** | **String** | A human readable message indicating details about why the pod is in this condition. | [optional] [default to null]
**phase** | **String** | Current condition of the pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase | [optional] [default to null]
**pod_ip** | **String** | IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated. | [optional] [default to null]
**qos_class** | **String** | The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://github.com/kubernetes/kubernetes/blob/master/docs/design/resource-qos.md | [optional] [default to null]
**reason** | **String** | A brief CamelCase message indicating details about why the pod is in this state. e.g. &#39;OutOfDisk&#39; | [optional] [default to null]
**start_time** | [***::models::MetaV1Time**](io.k8s.apimachinery.pkg.apis.meta.v1.Time.md) | RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


