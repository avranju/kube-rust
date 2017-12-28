# \AutoscalingV1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#create_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Post** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers | 
[**delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler) | **Delete** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers | 
[**delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Delete** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name} | 
[**get_autoscaling_v1_api_resources**](AutoscalingV1Api.md#get_autoscaling_v1_api_resources) | **Get** /apis/autoscaling/v1/ | 
[**list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces**](AutoscalingV1Api.md#list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces) | **Get** /apis/autoscaling/v1/horizontalpodautoscalers | 
[**list_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#list_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Get** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers | 
[**patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Patch** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name} | 
[**patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**](AutoscalingV1Api.md#patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status) | **Patch** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status | 
[**read_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#read_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Get** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name} | 
[**read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**](AutoscalingV1Api.md#read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status) | **Get** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status | 
[**replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Put** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name} | 
[**replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**](AutoscalingV1Api.md#replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status) | **Put** /apis/autoscaling/v1/namespaces/{namespace}/horizontalpodautoscalers/{name}/status | 
[**watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces**](AutoscalingV1Api.md#watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces) | **Get** /apis/autoscaling/v1/watch/horizontalpodautoscalers | 
[**watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler**](AutoscalingV1Api.md#watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler) | **Get** /apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers/{name} | 
[**watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list**](AutoscalingV1Api.md#watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list) | **Get** /apis/autoscaling/v1/watch/namespaces/{namespace}/horizontalpodautoscalers | 


# **create_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::AutoscalingV1HorizontalPodAutoscaler create_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, namespace, body, optional)


create a HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler**
> ::models::MetaV1Status delete_autoscaling_v1_collection_namespaced_horizontal_pod_autoscaler(ctx, namespace, optional)


delete collection of HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::MetaV1Status delete_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, name, namespace, body, optional)


delete a HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **get_autoscaling_v1_api_resources**
> ::models::MetaV1ApiResourceList get_autoscaling_v1_api_resources(ctx, )


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

# **list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces**
> ::models::AutoscalingV1HorizontalPodAutoscalerList list_autoscaling_v1_horizontal_pod_autoscaler_for_all_namespaces(ctx, optional)


list or watch objects of kind HorizontalPodAutoscaler

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

[**::models::AutoscalingV1HorizontalPodAutoscalerList**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscalerList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::AutoscalingV1HorizontalPodAutoscalerList list_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, namespace, optional)


list or watch objects of kind HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **field_selector** | **String**| A selector to restrict the list of returned objects by their fields. Defaults to everything. | 
 **include_uninitialized** | **bool**| If true, partially initialized resources are included in the response. | 
 **label_selector** | **String**| A selector to restrict the list of returned objects by their labels. Defaults to everything. | 
 **resource_version** | **String**| When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it&#39;s 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv. | 
 **timeout_seconds** | **i32**| Timeout for the list/watch call. | 
 **watch** | **bool**| Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscalerList**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscalerList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::AutoscalingV1HorizontalPodAutoscaler patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, name, namespace, body, optional)


partially update the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**
> ::models::AutoscalingV1HorizontalPodAutoscaler patch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(ctx, name, namespace, body, optional)


partially update status of the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::AutoscalingV1HorizontalPodAutoscaler read_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, name, namespace, optional)


read the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**
> ::models::AutoscalingV1HorizontalPodAutoscaler read_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(ctx, name, namespace, optional)


read status of the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::AutoscalingV1HorizontalPodAutoscaler replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, name, namespace, body, optional)


replace the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status**
> ::models::AutoscalingV1HorizontalPodAutoscaler replace_autoscaling_v1_namespaced_horizontal_pod_autoscaler_status(ctx, name, namespace, body, optional)


replace status of the specified HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AutoscalingV1HorizontalPodAutoscaler**](AutoscalingV1HorizontalPodAutoscaler.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AutoscalingV1HorizontalPodAutoscaler**](io.k8s.kubernetes.pkg.apis.autoscaling.v1.HorizontalPodAutoscaler.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_autoscaling_v1_horizontal_pod_autoscaler_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of HorizontalPodAutoscaler

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

# **watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler**
> ::models::MetaV1WatchEvent watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler(ctx, name, namespace, optional)


watch changes to an object of kind HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the HorizontalPodAutoscaler | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the HorizontalPodAutoscaler | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

# **watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list**
> ::models::MetaV1WatchEvent watch_autoscaling_v1_namespaced_horizontal_pod_autoscaler_list(ctx, namespace, optional)


watch individual changes to a list of HorizontalPodAutoscaler

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
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

