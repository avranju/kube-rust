# \AppsV1beta1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#create_apps_v1beta1_namespaced_controller_revision) | **Post** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions | 
[**create_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#create_apps_v1beta1_namespaced_deployment) | **Post** /apis/apps/v1beta1/namespaces/{namespace}/deployments | 
[**create_apps_v1beta1_namespaced_deployment_rollback**](AppsV1beta1Api.md#create_apps_v1beta1_namespaced_deployment_rollback) | **Post** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/rollback | 
[**create_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#create_apps_v1beta1_namespaced_stateful_set) | **Post** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets | 
[**delete_apps_v1beta1_collection_namespaced_controller_revision**](AppsV1beta1Api.md#delete_apps_v1beta1_collection_namespaced_controller_revision) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions | 
[**delete_apps_v1beta1_collection_namespaced_deployment**](AppsV1beta1Api.md#delete_apps_v1beta1_collection_namespaced_deployment) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/deployments | 
[**delete_apps_v1beta1_collection_namespaced_stateful_set**](AppsV1beta1Api.md#delete_apps_v1beta1_collection_namespaced_stateful_set) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets | 
[**delete_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#delete_apps_v1beta1_namespaced_controller_revision) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions/{name} | 
[**delete_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#delete_apps_v1beta1_namespaced_deployment) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name} | 
[**delete_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#delete_apps_v1beta1_namespaced_stateful_set) | **Delete** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name} | 
[**get_apps_v1beta1_api_resources**](AppsV1beta1Api.md#get_apps_v1beta1_api_resources) | **Get** /apis/apps/v1beta1/ | 
[**list_apps_v1beta1_controller_revision_for_all_namespaces**](AppsV1beta1Api.md#list_apps_v1beta1_controller_revision_for_all_namespaces) | **Get** /apis/apps/v1beta1/controllerrevisions | 
[**list_apps_v1beta1_deployment_for_all_namespaces**](AppsV1beta1Api.md#list_apps_v1beta1_deployment_for_all_namespaces) | **Get** /apis/apps/v1beta1/deployments | 
[**list_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#list_apps_v1beta1_namespaced_controller_revision) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions | 
[**list_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#list_apps_v1beta1_namespaced_deployment) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/deployments | 
[**list_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#list_apps_v1beta1_namespaced_stateful_set) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets | 
[**list_apps_v1beta1_stateful_set_for_all_namespaces**](AppsV1beta1Api.md#list_apps_v1beta1_stateful_set_for_all_namespaces) | **Get** /apis/apps/v1beta1/statefulsets | 
[**patch_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_controller_revision) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions/{name} | 
[**patch_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_deployment) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name} | 
[**patch_apps_v1beta1_namespaced_deployment_scale**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_deployment_scale) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale | 
[**patch_apps_v1beta1_namespaced_deployment_status**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_deployment_status) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/status | 
[**patch_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_stateful_set) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name} | 
[**patch_apps_v1beta1_namespaced_stateful_set_status**](AppsV1beta1Api.md#patch_apps_v1beta1_namespaced_stateful_set_status) | **Patch** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status | 
[**read_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_controller_revision) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions/{name} | 
[**read_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_deployment) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name} | 
[**read_apps_v1beta1_namespaced_deployment_scale**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_deployment_scale) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale | 
[**read_apps_v1beta1_namespaced_deployment_status**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_deployment_status) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/status | 
[**read_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_stateful_set) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name} | 
[**read_apps_v1beta1_namespaced_stateful_set_status**](AppsV1beta1Api.md#read_apps_v1beta1_namespaced_stateful_set_status) | **Get** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status | 
[**replace_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_controller_revision) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/controllerrevisions/{name} | 
[**replace_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_deployment) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name} | 
[**replace_apps_v1beta1_namespaced_deployment_scale**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_deployment_scale) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/scale | 
[**replace_apps_v1beta1_namespaced_deployment_status**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_deployment_status) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/deployments/{name}/status | 
[**replace_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_stateful_set) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name} | 
[**replace_apps_v1beta1_namespaced_stateful_set_status**](AppsV1beta1Api.md#replace_apps_v1beta1_namespaced_stateful_set_status) | **Put** /apis/apps/v1beta1/namespaces/{namespace}/statefulsets/{name}/status | 
[**watch_apps_v1beta1_controller_revision_list_for_all_namespaces**](AppsV1beta1Api.md#watch_apps_v1beta1_controller_revision_list_for_all_namespaces) | **Get** /apis/apps/v1beta1/watch/controllerrevisions | 
[**watch_apps_v1beta1_deployment_list_for_all_namespaces**](AppsV1beta1Api.md#watch_apps_v1beta1_deployment_list_for_all_namespaces) | **Get** /apis/apps/v1beta1/watch/deployments | 
[**watch_apps_v1beta1_namespaced_controller_revision**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_controller_revision) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/controllerrevisions/{name} | 
[**watch_apps_v1beta1_namespaced_controller_revision_list**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_controller_revision_list) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/controllerrevisions | 
[**watch_apps_v1beta1_namespaced_deployment**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_deployment) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/deployments/{name} | 
[**watch_apps_v1beta1_namespaced_deployment_list**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_deployment_list) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/deployments | 
[**watch_apps_v1beta1_namespaced_stateful_set**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_stateful_set) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets/{name} | 
[**watch_apps_v1beta1_namespaced_stateful_set_list**](AppsV1beta1Api.md#watch_apps_v1beta1_namespaced_stateful_set_list) | **Get** /apis/apps/v1beta1/watch/namespaces/{namespace}/statefulsets | 
[**watch_apps_v1beta1_stateful_set_list_for_all_namespaces**](AppsV1beta1Api.md#watch_apps_v1beta1_stateful_set_list_for_all_namespaces) | **Get** /apis/apps/v1beta1/watch/statefulsets | 


# **create_apps_v1beta1_namespaced_controller_revision**
> ::models::AppsV1beta1ControllerRevision create_apps_v1beta1_namespaced_controller_revision(ctx, namespace, body, optional)


create a ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1ControllerRevision**](AppsV1beta1ControllerRevision.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1ControllerRevision**](AppsV1beta1ControllerRevision.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1ControllerRevision**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_apps_v1beta1_namespaced_deployment**
> ::models::AppsV1beta1Deployment create_apps_v1beta1_namespaced_deployment(ctx, namespace, body, optional)


create a Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_apps_v1beta1_namespaced_deployment_rollback**
> ::models::AppsV1beta1DeploymentRollback create_apps_v1beta1_namespaced_deployment_rollback(ctx, name, namespace, body, optional)


create rollback of a Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the DeploymentRollback | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1DeploymentRollback**](AppsV1beta1DeploymentRollback.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the DeploymentRollback | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1DeploymentRollback**](AppsV1beta1DeploymentRollback.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1DeploymentRollback**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.DeploymentRollback.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_apps_v1beta1_namespaced_stateful_set**
> ::models::AppsV1beta1StatefulSet create_apps_v1beta1_namespaced_stateful_set(ctx, namespace, body, optional)


create a StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_apps_v1beta1_collection_namespaced_controller_revision**
> ::models::MetaV1Status delete_apps_v1beta1_collection_namespaced_controller_revision(ctx, namespace, optional)


delete collection of ControllerRevision

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

# **delete_apps_v1beta1_collection_namespaced_deployment**
> ::models::MetaV1Status delete_apps_v1beta1_collection_namespaced_deployment(ctx, namespace, optional)


delete collection of Deployment

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

# **delete_apps_v1beta1_collection_namespaced_stateful_set**
> ::models::MetaV1Status delete_apps_v1beta1_collection_namespaced_stateful_set(ctx, namespace, optional)


delete collection of StatefulSet

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

# **delete_apps_v1beta1_namespaced_controller_revision**
> ::models::MetaV1Status delete_apps_v1beta1_namespaced_controller_revision(ctx, name, namespace, body, optional)


delete a ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ControllerRevision | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ControllerRevision | 
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

# **delete_apps_v1beta1_namespaced_deployment**
> ::models::MetaV1Status delete_apps_v1beta1_namespaced_deployment(ctx, name, namespace, body, optional)


delete a Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
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

# **delete_apps_v1beta1_namespaced_stateful_set**
> ::models::MetaV1Status delete_apps_v1beta1_namespaced_stateful_set(ctx, name, namespace, body, optional)


delete a StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
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

# **get_apps_v1beta1_api_resources**
> ::models::MetaV1ApiResourceList get_apps_v1beta1_api_resources(ctx, )


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

# **list_apps_v1beta1_controller_revision_for_all_namespaces**
> ::models::AppsV1beta1ControllerRevisionList list_apps_v1beta1_controller_revision_for_all_namespaces(ctx, optional)


list or watch objects of kind ControllerRevision

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

[**::models::AppsV1beta1ControllerRevisionList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevisionList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_v1beta1_deployment_for_all_namespaces**
> ::models::AppsV1beta1DeploymentList list_apps_v1beta1_deployment_for_all_namespaces(ctx, optional)


list or watch objects of kind Deployment

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

[**::models::AppsV1beta1DeploymentList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.DeploymentList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_v1beta1_namespaced_controller_revision**
> ::models::AppsV1beta1ControllerRevisionList list_apps_v1beta1_namespaced_controller_revision(ctx, namespace, optional)


list or watch objects of kind ControllerRevision

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

[**::models::AppsV1beta1ControllerRevisionList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevisionList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_v1beta1_namespaced_deployment**
> ::models::AppsV1beta1DeploymentList list_apps_v1beta1_namespaced_deployment(ctx, namespace, optional)


list or watch objects of kind Deployment

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

[**::models::AppsV1beta1DeploymentList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.DeploymentList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_v1beta1_namespaced_stateful_set**
> ::models::AppsV1beta1StatefulSetList list_apps_v1beta1_namespaced_stateful_set(ctx, namespace, optional)


list or watch objects of kind StatefulSet

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

[**::models::AppsV1beta1StatefulSetList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_apps_v1beta1_stateful_set_for_all_namespaces**
> ::models::AppsV1beta1StatefulSetList list_apps_v1beta1_stateful_set_for_all_namespaces(ctx, optional)


list or watch objects of kind StatefulSet

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

[**::models::AppsV1beta1StatefulSetList**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSetList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_controller_revision**
> ::models::AppsV1beta1ControllerRevision patch_apps_v1beta1_namespaced_controller_revision(ctx, name, namespace, body, optional)


partially update the specified ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ControllerRevision | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ControllerRevision | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1ControllerRevision**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_deployment**
> ::models::AppsV1beta1Deployment patch_apps_v1beta1_namespaced_deployment(ctx, name, namespace, body, optional)


partially update the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_deployment_scale**
> ::models::AppsV1beta1Scale patch_apps_v1beta1_namespaced_deployment_scale(ctx, name, namespace, body, optional)


partially update scale of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Scale | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Scale | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Scale**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_deployment_status**
> ::models::AppsV1beta1Deployment patch_apps_v1beta1_namespaced_deployment_status(ctx, name, namespace, body, optional)


partially update status of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_stateful_set**
> ::models::AppsV1beta1StatefulSet patch_apps_v1beta1_namespaced_stateful_set(ctx, name, namespace, body, optional)


partially update the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_apps_v1beta1_namespaced_stateful_set_status**
> ::models::AppsV1beta1StatefulSet patch_apps_v1beta1_namespaced_stateful_set_status(ctx, name, namespace, body, optional)


partially update status of the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_controller_revision**
> ::models::AppsV1beta1ControllerRevision read_apps_v1beta1_namespaced_controller_revision(ctx, name, namespace, optional)


read the specified ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ControllerRevision | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ControllerRevision | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AppsV1beta1ControllerRevision**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_deployment**
> ::models::AppsV1beta1Deployment read_apps_v1beta1_namespaced_deployment(ctx, name, namespace, optional)


read the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_deployment_scale**
> ::models::AppsV1beta1Scale read_apps_v1beta1_namespaced_deployment_scale(ctx, name, namespace, optional)


read scale of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Scale | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Scale | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Scale**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_deployment_status**
> ::models::AppsV1beta1Deployment read_apps_v1beta1_namespaced_deployment_status(ctx, name, namespace, optional)


read status of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_stateful_set**
> ::models::AppsV1beta1StatefulSet read_apps_v1beta1_namespaced_stateful_set(ctx, name, namespace, optional)


read the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 
 **exact** | **bool**| Should the export be exact.  Exact export maintains cluster-specific fields like &#39;Namespace&#39;. | 
 **export** | **bool**| Should this value be exported.  Export strips fields that a user can not specify. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_apps_v1beta1_namespaced_stateful_set_status**
> ::models::AppsV1beta1StatefulSet read_apps_v1beta1_namespaced_stateful_set_status(ctx, name, namespace, optional)


read status of the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_controller_revision**
> ::models::AppsV1beta1ControllerRevision replace_apps_v1beta1_namespaced_controller_revision(ctx, name, namespace, body, optional)


replace the specified ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ControllerRevision | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1ControllerRevision**](AppsV1beta1ControllerRevision.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ControllerRevision | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1ControllerRevision**](AppsV1beta1ControllerRevision.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1ControllerRevision**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.ControllerRevision.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_deployment**
> ::models::AppsV1beta1Deployment replace_apps_v1beta1_namespaced_deployment(ctx, name, namespace, body, optional)


replace the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_deployment_scale**
> ::models::AppsV1beta1Scale replace_apps_v1beta1_namespaced_deployment_scale(ctx, name, namespace, body, optional)


replace scale of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Scale | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1Scale**](AppsV1beta1Scale.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Scale | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1Scale**](AppsV1beta1Scale.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Scale**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Scale.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_deployment_status**
> ::models::AppsV1beta1Deployment replace_apps_v1beta1_namespaced_deployment_status(ctx, name, namespace, body, optional)


replace status of the specified Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1Deployment**](AppsV1beta1Deployment.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1Deployment**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.Deployment.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_stateful_set**
> ::models::AppsV1beta1StatefulSet replace_apps_v1beta1_namespaced_stateful_set(ctx, name, namespace, body, optional)


replace the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_apps_v1beta1_namespaced_stateful_set_status**
> ::models::AppsV1beta1StatefulSet replace_apps_v1beta1_namespaced_stateful_set_status(ctx, name, namespace, body, optional)


replace status of the specified StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AppsV1beta1StatefulSet**](AppsV1beta1StatefulSet.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AppsV1beta1StatefulSet**](io.k8s.kubernetes.pkg.apis.apps.v1beta1.StatefulSet.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_apps_v1beta1_controller_revision_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_controller_revision_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of ControllerRevision

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

# **watch_apps_v1beta1_deployment_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_deployment_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of Deployment

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

# **watch_apps_v1beta1_namespaced_controller_revision**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_controller_revision(ctx, name, namespace, optional)


watch changes to an object of kind ControllerRevision

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ControllerRevision | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ControllerRevision | 
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

# **watch_apps_v1beta1_namespaced_controller_revision_list**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_controller_revision_list(ctx, namespace, optional)


watch individual changes to a list of ControllerRevision

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

# **watch_apps_v1beta1_namespaced_deployment**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_deployment(ctx, name, namespace, optional)


watch changes to an object of kind Deployment

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Deployment | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Deployment | 
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

# **watch_apps_v1beta1_namespaced_deployment_list**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_deployment_list(ctx, namespace, optional)


watch individual changes to a list of Deployment

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

# **watch_apps_v1beta1_namespaced_stateful_set**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_stateful_set(ctx, name, namespace, optional)


watch changes to an object of kind StatefulSet

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the StatefulSet | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the StatefulSet | 
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

# **watch_apps_v1beta1_namespaced_stateful_set_list**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_namespaced_stateful_set_list(ctx, namespace, optional)


watch individual changes to a list of StatefulSet

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

# **watch_apps_v1beta1_stateful_set_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_apps_v1beta1_stateful_set_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of StatefulSet

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

