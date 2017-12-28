# \CertificatesV1beta1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#create_certificates_v1beta1_certificate_signing_request) | **Post** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests | 
[**delete_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#delete_certificates_v1beta1_certificate_signing_request) | **Delete** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name} | 
[**delete_certificates_v1beta1_collection_certificate_signing_request**](CertificatesV1beta1Api.md#delete_certificates_v1beta1_collection_certificate_signing_request) | **Delete** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests | 
[**get_certificates_v1beta1_api_resources**](CertificatesV1beta1Api.md#get_certificates_v1beta1_api_resources) | **Get** /apis/certificates.k8s.io/v1beta1/ | 
[**list_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#list_certificates_v1beta1_certificate_signing_request) | **Get** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests | 
[**patch_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#patch_certificates_v1beta1_certificate_signing_request) | **Patch** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name} | 
[**read_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#read_certificates_v1beta1_certificate_signing_request) | **Get** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name} | 
[**replace_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#replace_certificates_v1beta1_certificate_signing_request) | **Put** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name} | 
[**replace_certificates_v1beta1_certificate_signing_request_approval**](CertificatesV1beta1Api.md#replace_certificates_v1beta1_certificate_signing_request_approval) | **Put** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/approval | 
[**replace_certificates_v1beta1_certificate_signing_request_status**](CertificatesV1beta1Api.md#replace_certificates_v1beta1_certificate_signing_request_status) | **Put** /apis/certificates.k8s.io/v1beta1/certificatesigningrequests/{name}/status | 
[**watch_certificates_v1beta1_certificate_signing_request**](CertificatesV1beta1Api.md#watch_certificates_v1beta1_certificate_signing_request) | **Get** /apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests/{name} | 
[**watch_certificates_v1beta1_certificate_signing_request_list**](CertificatesV1beta1Api.md#watch_certificates_v1beta1_certificate_signing_request_list) | **Get** /apis/certificates.k8s.io/v1beta1/watch/certificatesigningrequests | 


# **create_certificates_v1beta1_certificate_signing_request**
> ::models::CertificatesV1beta1CertificateSigningRequest create_certificates_v1beta1_certificate_signing_request(ctx, body, optional)


create a CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_certificates_v1beta1_certificate_signing_request**
> ::models::MetaV1Status delete_certificates_v1beta1_certificate_signing_request(ctx, name, body, optional)


delete a CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **grace_period_seconds** | **i32**| The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately. | 
 **orphan_dependents** | **bool**| Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the \&quot;orphan\&quot; finalizer will be added to/removed from the object&#39;s finalizers list. Either this field or PropagationPolicy may be set, but not both. | 
 **propagation_policy** | **String**| Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. | 

### Return type

[**::models::MetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_certificates_v1beta1_collection_certificate_signing_request**
> ::models::MetaV1Status delete_certificates_v1beta1_collection_certificate_signing_request(ctx, optional)


delete collection of CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::MetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_certificates_v1beta1_api_resources**
> ::models::MetaV1ApiResourceList get_certificates_v1beta1_api_resources(ctx, )


get available resources

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::MetaV1ApiResourceList**](io.k8s.apimachinery.pkg.apis.meta.v1.APIResourceList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json, application/yaml, application/vnd.kubernetes.protobuf
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_certificates_v1beta1_certificate_signing_request**
> ::models::CertificatesV1beta1CertificateSigningRequestList list_certificates_v1beta1_certificate_signing_request(ctx, optional)


list or watch objects of kind CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequestList**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequestList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_certificates_v1beta1_certificate_signing_request**
> ::models::CertificatesV1beta1CertificateSigningRequest patch_certificates_v1beta1_certificate_signing_request(ctx, name, body, optional)


partially update the specified CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_certificates_v1beta1_certificate_signing_request**
> ::models::CertificatesV1beta1CertificateSigningRequest read_certificates_v1beta1_certificate_signing_request(ctx, name, optional)


read the specified CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_certificates_v1beta1_certificate_signing_request**
> ::models::CertificatesV1beta1CertificateSigningRequest replace_certificates_v1beta1_certificate_signing_request(ctx, name, body, optional)


replace the specified CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
  **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_certificates_v1beta1_certificate_signing_request_approval**
> ::models::CertificatesV1beta1CertificateSigningRequest replace_certificates_v1beta1_certificate_signing_request_approval(ctx, name, body, optional)


replace approval of the specified CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
  **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_certificates_v1beta1_certificate_signing_request_status**
> ::models::CertificatesV1beta1CertificateSigningRequest replace_certificates_v1beta1_certificate_signing_request_status(ctx, name, body, optional)


replace status of the specified CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
  **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **body** | [**CertificatesV1beta1CertificateSigningRequest**](CertificatesV1beta1CertificateSigningRequest.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::CertificatesV1beta1CertificateSigningRequest**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequest.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_certificates_v1beta1_certificate_signing_request**
> ::models::MetaV1WatchEvent watch_certificates_v1beta1_certificate_signing_request(ctx, name, optional)


watch changes to an object of kind CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the CertificateSigningRequest | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the CertificateSigningRequest | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::MetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_certificates_v1beta1_certificate_signing_request_list**
> ::models::MetaV1WatchEvent watch_certificates_v1beta1_certificate_signing_request_list(ctx, optional)


watch individual changes to a list of CertificateSigningRequest

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::MetaV1WatchEvent**](io.k8s.apimachinery.pkg.apis.meta.v1.WatchEvent.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

