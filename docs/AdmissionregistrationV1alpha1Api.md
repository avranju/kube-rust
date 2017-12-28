# \AdmissionregistrationV1alpha1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#create_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Post** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations | 
[**create_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#create_admissionregistration_v1alpha1_initializer_configuration) | **Post** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations | 
[**delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration) | **Delete** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations | 
[**delete_admissionregistration_v1alpha1_collection_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#delete_admissionregistration_v1alpha1_collection_initializer_configuration) | **Delete** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations | 
[**delete_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#delete_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Delete** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name} | 
[**delete_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#delete_admissionregistration_v1alpha1_initializer_configuration) | **Delete** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name} | 
[**get_admissionregistration_v1alpha1_api_resources**](AdmissionregistrationV1alpha1Api.md#get_admissionregistration_v1alpha1_api_resources) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/ | 
[**list_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#list_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations | 
[**list_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#list_admissionregistration_v1alpha1_initializer_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations | 
[**patch_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#patch_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Patch** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name} | 
[**patch_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#patch_admissionregistration_v1alpha1_initializer_configuration) | **Patch** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name} | 
[**read_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#read_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name} | 
[**read_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#read_admissionregistration_v1alpha1_initializer_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name} | 
[**replace_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#replace_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Put** /apis/admissionregistration.k8s.io/v1alpha1/externaladmissionhookconfigurations/{name} | 
[**replace_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#replace_admissionregistration_v1alpha1_initializer_configuration) | **Put** /apis/admissionregistration.k8s.io/v1alpha1/initializerconfigurations/{name} | 
[**watch_admissionregistration_v1alpha1_external_admission_hook_configuration**](AdmissionregistrationV1alpha1Api.md#watch_admissionregistration_v1alpha1_external_admission_hook_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/watch/externaladmissionhookconfigurations/{name} | 
[**watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list**](AdmissionregistrationV1alpha1Api.md#watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/watch/externaladmissionhookconfigurations | 
[**watch_admissionregistration_v1alpha1_initializer_configuration**](AdmissionregistrationV1alpha1Api.md#watch_admissionregistration_v1alpha1_initializer_configuration) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations/{name} | 
[**watch_admissionregistration_v1alpha1_initializer_configuration_list**](AdmissionregistrationV1alpha1Api.md#watch_admissionregistration_v1alpha1_initializer_configuration_list) | **Get** /apis/admissionregistration.k8s.io/v1alpha1/watch/initializerconfigurations | 


# **create_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration create_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, body, optional)


create an ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_admissionregistration_v1alpha1_initializer_configuration**
> ::models::AdmissionregistrationV1alpha1InitializerConfiguration create_admissionregistration_v1alpha1_initializer_configuration(ctx, body, optional)


create an InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**AdmissionregistrationV1alpha1InitializerConfiguration**](AdmissionregistrationV1alpha1InitializerConfiguration.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AdmissionregistrationV1alpha1InitializerConfiguration**](AdmissionregistrationV1alpha1InitializerConfiguration.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1InitializerConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration**
> ::models::MetaV1Status delete_admissionregistration_v1alpha1_collection_external_admission_hook_configuration(ctx, optional)


delete collection of ExternalAdmissionHookConfiguration

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

# **delete_admissionregistration_v1alpha1_collection_initializer_configuration**
> ::models::MetaV1Status delete_admissionregistration_v1alpha1_collection_initializer_configuration(ctx, optional)


delete collection of InitializerConfiguration

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

# **delete_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::MetaV1Status delete_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, name, body, optional)


delete an ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
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

# **delete_admissionregistration_v1alpha1_initializer_configuration**
> ::models::MetaV1Status delete_admissionregistration_v1alpha1_initializer_configuration(ctx, name, body, optional)


delete an InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the InitializerConfiguration | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the InitializerConfiguration | 
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

# **get_admissionregistration_v1alpha1_api_resources**
> ::models::MetaV1ApiResourceList get_admissionregistration_v1alpha1_api_resources(ctx, )


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

# **list_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationList list_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, optional)


list or watch objects of kind ExternalAdmissionHookConfiguration

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

[**::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfigurationList**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHookConfigurationList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_admissionregistration_v1alpha1_initializer_configuration**
> ::models::AdmissionregistrationV1alpha1InitializerConfigurationList list_admissionregistration_v1alpha1_initializer_configuration(ctx, optional)


list or watch objects of kind InitializerConfiguration

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

[**::models::AdmissionregistrationV1alpha1InitializerConfigurationList**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfigurationList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration patch_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, name, body, optional)


partially update the specified ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_admissionregistration_v1alpha1_initializer_configuration**
> ::models::AdmissionregistrationV1alpha1InitializerConfiguration patch_admissionregistration_v1alpha1_initializer_configuration(ctx, name, body, optional)


partially update the specified InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the InitializerConfiguration | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the InitializerConfiguration | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1InitializerConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration read_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, name, optional)


read the specified ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_admissionregistration_v1alpha1_initializer_configuration**
> ::models::AdmissionregistrationV1alpha1InitializerConfiguration read_admissionregistration_v1alpha1_initializer_configuration(ctx, name, optional)


read the specified InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the InitializerConfiguration | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the InitializerConfiguration | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AdmissionregistrationV1alpha1InitializerConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration replace_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, name, body, optional)


replace the specified ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
  **body** | [**AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
 **body** | [**AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1ExternalAdmissionHookConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_admissionregistration_v1alpha1_initializer_configuration**
> ::models::AdmissionregistrationV1alpha1InitializerConfiguration replace_admissionregistration_v1alpha1_initializer_configuration(ctx, name, body, optional)


replace the specified InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the InitializerConfiguration | 
  **body** | [**AdmissionregistrationV1alpha1InitializerConfiguration**](AdmissionregistrationV1alpha1InitializerConfiguration.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the InitializerConfiguration | 
 **body** | [**AdmissionregistrationV1alpha1InitializerConfiguration**](AdmissionregistrationV1alpha1InitializerConfiguration.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AdmissionregistrationV1alpha1InitializerConfiguration**](io.k8s.kubernetes.pkg.apis.admissionregistration.v1alpha1.InitializerConfiguration.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_admissionregistration_v1alpha1_external_admission_hook_configuration**
> ::models::MetaV1WatchEvent watch_admissionregistration_v1alpha1_external_admission_hook_configuration(ctx, name, optional)


watch changes to an object of kind ExternalAdmissionHookConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ExternalAdmissionHookConfiguration | 
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

# **watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list**
> ::models::MetaV1WatchEvent watch_admissionregistration_v1alpha1_external_admission_hook_configuration_list(ctx, optional)


watch individual changes to a list of ExternalAdmissionHookConfiguration

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

# **watch_admissionregistration_v1alpha1_initializer_configuration**
> ::models::MetaV1WatchEvent watch_admissionregistration_v1alpha1_initializer_configuration(ctx, name, optional)


watch changes to an object of kind InitializerConfiguration

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the InitializerConfiguration | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the InitializerConfiguration | 
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

# **watch_admissionregistration_v1alpha1_initializer_configuration_list**
> ::models::MetaV1WatchEvent watch_admissionregistration_v1alpha1_initializer_configuration_list(ctx, optional)


watch individual changes to a list of InitializerConfiguration

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

