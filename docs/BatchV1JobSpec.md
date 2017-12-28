# BatchV1JobSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_deadline_seconds** | **i64** | Optional duration in seconds relative to the startTime that the job may be active before the system tries to terminate it; value must be positive integer | [optional] [default to null]
**completions** | **i32** | Specifies the desired number of successfully finished pods the job should be run with.  Setting to nil means that the success of any pod signals the success of all pods, and allows parallelism to have any positive value.  Setting to 1 means that parallelism is limited to 1 and the success of that pod signals the success of the job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [optional] [default to null]
**manual_selector** | **bool** | manualSelector controls generation of pod labels and pod selectors. Leave &#x60;manualSelector&#x60; unset unless you are certain what you are doing. When false or unset, the system pick labels unique to this job and appends those labels to the pod template.  When true, the user is responsible for picking unique labels and specifying the selector.  Failure to pick a unique label may cause this and other jobs to not function correctly.  However, You may see &#x60;manualSelector&#x3D;true&#x60; in jobs that were created with the old &#x60;extensions/v1beta1&#x60; API. More info: https://git.k8s.io/community/contributors/design-proposals/selector-generation.md | [optional] [default to null]
**parallelism** | **i32** | Specifies the maximum desired number of pods the job should run at any given time. The actual number of pods running in steady state will be less than this number when ((.spec.completions - .status.successful) &lt; .spec.parallelism), i.e. when the work left to do is less than max parallelism. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [optional] [default to null]
**selector** | [***::models::MetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) | A label query over pods that should match the pod count. Normally, the system sets this field for you. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors | [optional] [default to null]
**template** | [***::models::V1PodTemplateSpec**](io.k8s.kubernetes.pkg.api.v1.PodTemplateSpec.md) | Describes the pod that will be created when executing a job. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


