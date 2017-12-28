# V1Handler

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exec** | [***::models::V1ExecAction**](io.k8s.kubernetes.pkg.api.v1.ExecAction.md) | One and only one of the following should be specified. Exec specifies the action to take. | [optional] [default to null]
**http_get** | [***::models::V1HttpGetAction**](io.k8s.kubernetes.pkg.api.v1.HTTPGetAction.md) | HTTPGet specifies the http request to perform. | [optional] [default to null]
**tcp_socket** | [***::models::V1TcpSocketAction**](io.k8s.kubernetes.pkg.api.v1.TCPSocketAction.md) | TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


