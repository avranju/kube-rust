# V1HttpGetAction

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** | Host name to connect to, defaults to the pod IP. You probably want to set \&quot;Host\&quot; in httpHeaders instead. | [optional] [default to null]
**http_headers** | [**Vec<::models::V1HttpHeader>**](io.k8s.kubernetes.pkg.api.v1.HTTPHeader.md) | Custom headers to set in the request. HTTP allows repeated headers. | [optional] [default to null]
**path** | **String** | Path to access on the HTTP server. | [optional] [default to null]
**port** | [***::models::UtilIntstrIntOrString**](io.k8s.apimachinery.pkg.util.intstr.IntOrString.md) | Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. | [default to null]
**scheme** | **String** | Scheme to use for connecting to the host. Defaults to HTTP. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


