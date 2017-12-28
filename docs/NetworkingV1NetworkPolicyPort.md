# NetworkingV1NetworkPolicyPort

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**port** | [***::models::UtilIntstrIntOrString**](io.k8s.apimachinery.pkg.util.intstr.IntOrString.md) | The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers. | [optional] [default to null]
**protocol** | **String** | The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


