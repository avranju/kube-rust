# \RbacAuthorizationV1alpha1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_cluster_role) | **Post** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**create_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_cluster_role_binding) | **Post** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**create_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_namespaced_role) | **Post** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**create_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#create_rbac_authorization_v1alpha1_namespaced_role_binding) | **Post** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_cluster_role) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**delete_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_cluster_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**delete_rbac_authorization_v1alpha1_collection_cluster_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_cluster_role) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**delete_rbac_authorization_v1alpha1_collection_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_cluster_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**delete_rbac_authorization_v1alpha1_collection_namespaced_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_namespaced_role) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**delete_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_namespaced_role) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**delete_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#delete_rbac_authorization_v1alpha1_namespaced_role_binding) | **Delete** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**get_rbac_authorization_v1alpha1_api_resources**](RbacAuthorizationV1alpha1Api.md#get_rbac_authorization_v1alpha1_api_resources) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/ | 
[**list_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles | 
[**list_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings | 
[**list_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles | 
[**list_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings | 
[**list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/rolebindings | 
[**list_rbac_authorization_v1alpha1_role_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#list_rbac_authorization_v1alpha1_role_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/roles | 
[**patch_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_cluster_role) | **Patch** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**patch_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_cluster_role_binding) | **Patch** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**patch_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_namespaced_role) | **Patch** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**patch_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#patch_rbac_authorization_v1alpha1_namespaced_role_binding) | **Patch** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**read_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**read_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**read_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**read_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#read_rbac_authorization_v1alpha1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**replace_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_cluster_role) | **Put** /apis/rbac.authorization.k8s.io/v1alpha1/clusterroles/{name} | 
[**replace_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_cluster_role_binding) | **Put** /apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name} | 
[**replace_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_namespaced_role) | **Put** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name} | 
[**replace_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#replace_rbac_authorization_v1alpha1_namespaced_role_binding) | **Put** /apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role_binding**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_cluster_role_binding_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_binding_list) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings | 
[**watch_rbac_authorization_v1alpha1_cluster_role_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_cluster_role_list) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterroles | 
[**watch_rbac_authorization_v1alpha1_namespaced_role**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles/{name} | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_binding**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_binding) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/rolebindings/{name} | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_binding_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_binding_list) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/rolebindings | 
[**watch_rbac_authorization_v1alpha1_namespaced_role_list**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_namespaced_role_list) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles | 
[**watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/rolebindings | 
[**watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces**](RbacAuthorizationV1alpha1Api.md#watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces) | **Get** /apis/rbac.authorization.k8s.io/v1alpha1/watch/roles | 


# **create_rbac_authorization_v1alpha1_cluster_role**
> ::models::RbacV1alpha1ClusterRole create_rbac_authorization_v1alpha1_cluster_role(ctx, body, optional)


create a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**RbacV1alpha1ClusterRole**](RbacV1alpha1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**RbacV1alpha1ClusterRole**](RbacV1alpha1ClusterRole.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::RbacV1alpha1ClusterRoleBinding create_rbac_authorization_v1alpha1_cluster_role_binding(ctx, body, optional)


create a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**RbacV1alpha1ClusterRoleBinding**](RbacV1alpha1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**RbacV1alpha1ClusterRoleBinding**](RbacV1alpha1ClusterRoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_namespaced_role**
> ::models::RbacV1alpha1Role create_rbac_authorization_v1alpha1_namespaced_role(ctx, namespace, body, optional)


create a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**RbacV1alpha1Role**](RbacV1alpha1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**RbacV1alpha1Role**](RbacV1alpha1Role.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::RbacV1alpha1RoleBinding create_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, namespace, body, optional)


create a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**RbacV1alpha1RoleBinding**](RbacV1alpha1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**RbacV1alpha1RoleBinding**](RbacV1alpha1RoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_rbac_authorization_v1alpha1_cluster_role**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_cluster_role(ctx, name, body, optional)


delete a ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
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

# **delete_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, body, optional)


delete a ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
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

# **delete_rbac_authorization_v1alpha1_collection_cluster_role**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_collection_cluster_role(ctx, optional)


delete collection of ClusterRole

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

# **delete_rbac_authorization_v1alpha1_collection_cluster_role_binding**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_collection_cluster_role_binding(ctx, optional)


delete collection of ClusterRoleBinding

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

# **delete_rbac_authorization_v1alpha1_collection_namespaced_role**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_collection_namespaced_role(ctx, namespace, optional)


delete collection of Role

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

# **delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_collection_namespaced_role_binding(ctx, namespace, optional)


delete collection of RoleBinding

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

# **delete_rbac_authorization_v1alpha1_namespaced_role**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, body, optional)


delete a Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
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

# **delete_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::MetaV1Status delete_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, body, optional)


delete a RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1DeleteOptions**](MetaV1DeleteOptions.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
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

# **get_rbac_authorization_v1alpha1_api_resources**
> ::models::MetaV1ApiResourceList get_rbac_authorization_v1alpha1_api_resources(ctx, )


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

# **list_rbac_authorization_v1alpha1_cluster_role**
> ::models::RbacV1alpha1ClusterRoleList list_rbac_authorization_v1alpha1_cluster_role(ctx, optional)


list or watch objects of kind ClusterRole

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

[**::models::RbacV1alpha1ClusterRoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::RbacV1alpha1ClusterRoleBindingList list_rbac_authorization_v1alpha1_cluster_role_binding(ctx, optional)


list or watch objects of kind ClusterRoleBinding

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

[**::models::RbacV1alpha1ClusterRoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_namespaced_role**
> ::models::RbacV1alpha1RoleList list_rbac_authorization_v1alpha1_namespaced_role(ctx, namespace, optional)


list or watch objects of kind Role

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

[**::models::RbacV1alpha1RoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::RbacV1alpha1RoleBindingList list_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, namespace, optional)


list or watch objects of kind RoleBinding

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

[**::models::RbacV1alpha1RoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces**
> ::models::RbacV1alpha1RoleBindingList list_rbac_authorization_v1alpha1_role_binding_for_all_namespaces(ctx, optional)


list or watch objects of kind RoleBinding

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

[**::models::RbacV1alpha1RoleBindingList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBindingList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_rbac_authorization_v1alpha1_role_for_all_namespaces**
> ::models::RbacV1alpha1RoleList list_rbac_authorization_v1alpha1_role_for_all_namespaces(ctx, optional)


list or watch objects of kind Role

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

[**::models::RbacV1alpha1RoleList**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleList.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf, application/json;stream=watch, application/vnd.kubernetes.protobuf;stream=watch

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_cluster_role**
> ::models::RbacV1alpha1ClusterRole patch_rbac_authorization_v1alpha1_cluster_role(ctx, name, body, optional)


partially update the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::RbacV1alpha1ClusterRoleBinding patch_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, body, optional)


partially update the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_namespaced_role**
> ::models::RbacV1alpha1Role patch_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, body, optional)


partially update the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **patch_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::RbacV1alpha1RoleBinding patch_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, body, optional)


partially update the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**MetaV1Patch**](MetaV1Patch.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: application/json-patch+json, application/merge-patch+json, application/strategic-merge-patch+json
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_cluster_role**
> ::models::RbacV1alpha1ClusterRole read_rbac_authorization_v1alpha1_cluster_role(ctx, name, optional)


read the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::RbacV1alpha1ClusterRoleBinding read_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, optional)


read the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_namespaced_role**
> ::models::RbacV1alpha1Role read_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, optional)


read the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **read_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::RbacV1alpha1RoleBinding read_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, optional)


read the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_cluster_role**
> ::models::RbacV1alpha1ClusterRole replace_rbac_authorization_v1alpha1_cluster_role(ctx, name, body, optional)


replace the specified ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
  **body** | [**RbacV1alpha1ClusterRole**](RbacV1alpha1ClusterRole.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
 **body** | [**RbacV1alpha1ClusterRole**](RbacV1alpha1ClusterRole.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRole**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRole.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::RbacV1alpha1ClusterRoleBinding replace_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, body, optional)


replace the specified ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
  **body** | [**RbacV1alpha1ClusterRoleBinding**](RbacV1alpha1ClusterRoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
 **body** | [**RbacV1alpha1ClusterRoleBinding**](RbacV1alpha1ClusterRoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1ClusterRoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_namespaced_role**
> ::models::RbacV1alpha1Role replace_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, body, optional)


replace the specified Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**RbacV1alpha1Role**](RbacV1alpha1Role.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**RbacV1alpha1Role**](RbacV1alpha1Role.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1Role**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Role.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **replace_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::RbacV1alpha1RoleBinding replace_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, body, optional)


replace the specified RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**RbacV1alpha1RoleBinding**](RbacV1alpha1RoleBinding.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**RbacV1alpha1RoleBinding**](RbacV1alpha1RoleBinding.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::RbacV1alpha1RoleBinding**](io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.RoleBinding.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **watch_rbac_authorization_v1alpha1_cluster_role**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role(ctx, name, optional)


watch changes to an object of kind ClusterRole

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRole | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRole | 
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

# **watch_rbac_authorization_v1alpha1_cluster_role_binding**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_binding(ctx, name, optional)


watch changes to an object of kind ClusterRoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the ClusterRoleBinding | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the ClusterRoleBinding | 
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

# **watch_rbac_authorization_v1alpha1_cluster_role_binding_list**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_binding_list(ctx, optional)


watch individual changes to a list of ClusterRoleBinding

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

# **watch_rbac_authorization_v1alpha1_cluster_role_list**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_cluster_role_list(ctx, optional)


watch individual changes to a list of ClusterRole

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

# **watch_rbac_authorization_v1alpha1_namespaced_role**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role(ctx, name, namespace, optional)


watch changes to an object of kind Role

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the Role | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the Role | 
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

# **watch_rbac_authorization_v1alpha1_namespaced_role_binding**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_binding(ctx, name, namespace, optional)


watch changes to an object of kind RoleBinding

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| name of the RoleBinding | 
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| name of the RoleBinding | 
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

# **watch_rbac_authorization_v1alpha1_namespaced_role_binding_list**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_binding_list(ctx, namespace, optional)


watch individual changes to a list of RoleBinding

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

# **watch_rbac_authorization_v1alpha1_namespaced_role_list**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_namespaced_role_list(ctx, namespace, optional)


watch individual changes to a list of Role

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

# **watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_role_binding_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of RoleBinding

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

# **watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces**
> ::models::MetaV1WatchEvent watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces(ctx, optional)


watch individual changes to a list of Role

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

