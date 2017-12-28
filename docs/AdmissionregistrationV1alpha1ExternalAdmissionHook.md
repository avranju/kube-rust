# AdmissionregistrationV1alpha1ExternalAdmissionHook

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_config** | [***::models::AdmissionregistrationV1alpha1AdmissionHookClientConfig**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.AdmissionHookClientConfig.md) | ClientConfig defines how to communicate with the hook. Required | [default to null]
**failure_policy** | **String** | FailurePolicy defines how unrecognized errors from the admission endpoint are handled - allowed values are Ignore or Fail. Defaults to Ignore. | [optional] [default to null]
**name** | **String** | The name of the external admission webhook. Name should be fully qualified, e.g., imagepolicy.kubernetes.io, where \&quot;imagepolicy\&quot; is the name of the webhook, and kubernetes.io is the name of the organization. Required. | [default to null]
**rules** | [**Vec<::models::AdmissionregistrationV1alpha1RuleWithOperations>**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.RuleWithOperations.md) | Rules describes what operations on what resources/subresources the webhook cares about. The webhook cares about an operation if it matches _any_ Rule. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


