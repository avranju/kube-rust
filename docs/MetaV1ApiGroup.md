# MetaV1ApiGroup

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] [default to null]
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] [default to null]
**name** | **String** | name is the name of the group. | [default to null]
**preferred_version** | [***::models::MetaV1GroupVersionForDiscovery**](io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery.md) | preferredVersion is the version preferred by the API server, which probably is the storage version. | [optional] [default to null]
**server_address_by_client_cid_rs** | [**Vec<::models::MetaV1ServerAddressByClientCidr>**](io.k8s.apimachinery.pkg.apis.meta.v1.ServerAddressByClientCIDR.md) | a map of client CIDR to server address that is serving this group. This is to help clients reach servers in the most network-efficient way possible. Clients can use the appropriate server address as per the CIDR that they match. In case of multiple matches, clients should use the longest matching CIDR. The server returns only those CIDRs that it thinks that the client can match. For example: the master will return an internal IP CIDR only, if the client reaches the server using an internal IP. Server looks at X-Forwarded-For header or X-Real-Ip header or request.RemoteAddr (in that order) to get the client IP. | [default to null]
**versions** | [**Vec<::models::MetaV1GroupVersionForDiscovery>**](io.k8s.apimachinery.pkg.apis.meta.v1.GroupVersionForDiscovery.md) | versions are the versions supported in this group. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


