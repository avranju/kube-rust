# AdmissionregistrationV1alpha1Initializer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**failure_policy** | **String** | FailurePolicy defines what happens if the responsible initializer controller fails to takes action. Allowed values are Ignore, or Fail. If \&quot;Ignore\&quot; is set, initializer is removed from the initializers list of an object if the timeout is reached; If \&quot;Fail\&quot; is set, admissionregistration returns timeout error if the timeout is reached. | [optional] [default to null]
**name** | **String** | Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where \&quot;alwayspullimages\&quot; is the name of the webhook, and kubernetes.io is the name of the organization. Required | [default to null]
**rules** | [**Vec<::models::AdmissionregistrationV1alpha1Rule>**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.Rule.md) | Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


