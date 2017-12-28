# \AuthorizationV1beta1Api

All URIs are relative to *https://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_authorization_v1beta1_namespaced_local_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_namespaced_local_subject_access_review) | **Post** /apis/authorization.k8s.io/v1beta1/namespaces/{namespace}/localsubjectaccessreviews | 
[**create_authorization_v1beta1_self_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_self_subject_access_review) | **Post** /apis/authorization.k8s.io/v1beta1/selfsubjectaccessreviews | 
[**create_authorization_v1beta1_subject_access_review**](AuthorizationV1beta1Api.md#create_authorization_v1beta1_subject_access_review) | **Post** /apis/authorization.k8s.io/v1beta1/subjectaccessreviews | 
[**get_authorization_v1beta1_api_resources**](AuthorizationV1beta1Api.md#get_authorization_v1beta1_api_resources) | **Get** /apis/authorization.k8s.io/v1beta1/ | 


# **create_authorization_v1beta1_namespaced_local_subject_access_review**
> ::models::AuthorizationV1beta1LocalSubjectAccessReview create_authorization_v1beta1_namespaced_local_subject_access_review(ctx, namespace, body, optional)


create a LocalSubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **namespace** | **String**| object name and auth scope, such as for teams and projects | 
  **body** | [**AuthorizationV1beta1LocalSubjectAccessReview**](AuthorizationV1beta1LocalSubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **namespace** | **String**| object name and auth scope, such as for teams and projects | 
 **body** | [**AuthorizationV1beta1LocalSubjectAccessReview**](AuthorizationV1beta1LocalSubjectAccessReview.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AuthorizationV1beta1LocalSubjectAccessReview**](io.k8s.kubernetes.pkg.apis.authorization.v1beta1.LocalSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_authorization_v1beta1_self_subject_access_review**
> ::models::AuthorizationV1beta1SelfSubjectAccessReview create_authorization_v1beta1_self_subject_access_review(ctx, body, optional)


create a SelfSubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**AuthorizationV1beta1SelfSubjectAccessReview**](AuthorizationV1beta1SelfSubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AuthorizationV1beta1SelfSubjectAccessReview**](AuthorizationV1beta1SelfSubjectAccessReview.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AuthorizationV1beta1SelfSubjectAccessReview**](io.k8s.kubernetes.pkg.apis.authorization.v1beta1.SelfSubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_authorization_v1beta1_subject_access_review**
> ::models::AuthorizationV1beta1SubjectAccessReview create_authorization_v1beta1_subject_access_review(ctx, body, optional)


create a SubjectAccessReview

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **body** | [**AuthorizationV1beta1SubjectAccessReview**](AuthorizationV1beta1SubjectAccessReview.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **body** | [**AuthorizationV1beta1SubjectAccessReview**](AuthorizationV1beta1SubjectAccessReview.md)|  | 
 **pretty** | **String**| If &#39;true&#39;, then the output is pretty printed. | 

### Return type

[**::models::AuthorizationV1beta1SubjectAccessReview**](io.k8s.kubernetes.pkg.apis.authorization.v1beta1.SubjectAccessReview.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

 - **Content-Type**: */*
 - **Accept**: application/json, application/yaml, application/vnd.kubernetes.protobuf

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_authorization_v1beta1_api_resources**
> ::models::MetaV1ApiResourceList get_authorization_v1beta1_api_resources(ctx, )


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

