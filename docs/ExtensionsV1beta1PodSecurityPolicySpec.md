# ExtensionsV1beta1PodSecurityPolicySpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowed_capabilities** | **Vec<String>** | AllowedCapabilities is a list of capabilities that can be requested to add to the container. Capabilities in this field may be added at the pod author&#39;s discretion. You must not list a capability in both AllowedCapabilities and RequiredDropCapabilities. | [optional] [default to null]
**default_add_capabilities** | **Vec<String>** | DefaultAddCapabilities is the default set of capabilities that will be added to the container unless the pod spec specifically drops the capability.  You may not list a capabiility in both DefaultAddCapabilities and RequiredDropCapabilities. | [optional] [default to null]
**fs_group** | [***::models::ExtensionsV1beta1FsGroupStrategyOptions**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.FSGroupStrategyOptions.md) | FSGroup is the strategy that will dictate what fs group is used by the SecurityContext. | [default to null]
**host_ipc** | **bool** | hostIPC determines if the policy allows the use of HostIPC in the pod spec. | [optional] [default to null]
**host_network** | **bool** | hostNetwork determines if the policy allows the use of HostNetwork in the pod spec. | [optional] [default to null]
**host_pid** | **bool** | hostPID determines if the policy allows the use of HostPID in the pod spec. | [optional] [default to null]
**host_ports** | [**Vec<::models::ExtensionsV1beta1HostPortRange>**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.HostPortRange.md) | hostPorts determines which host port ranges are allowed to be exposed. | [optional] [default to null]
**privileged** | **bool** | privileged determines if a pod can request to be run as privileged. | [optional] [default to null]
**read_only_root_filesystem** | **bool** | ReadOnlyRootFilesystem when set to true will force containers to run with a read only root file system.  If the container specifically requests to run with a non-read only root file system the PSP should deny the pod. If set to false the container may run with a read only root file system if it wishes but it will not be forced to. | [optional] [default to null]
**required_drop_capabilities** | **Vec<String>** | RequiredDropCapabilities are the capabilities that will be dropped from the container.  These are required to be dropped and cannot be added. | [optional] [default to null]
**run_as_user** | [***::models::ExtensionsV1beta1RunAsUserStrategyOptions**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.RunAsUserStrategyOptions.md) | runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set. | [default to null]
**se_linux** | [***::models::ExtensionsV1beta1SeLinuxStrategyOptions**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.SELinuxStrategyOptions.md) | seLinux is the strategy that will dictate the allowable labels that may be set. | [default to null]
**supplemental_groups** | [***::models::ExtensionsV1beta1SupplementalGroupsStrategyOptions**](io.k8s.kubernetes.pkg.apis.extensions.v1beta1.SupplementalGroupsStrategyOptions.md) | SupplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext. | [default to null]
**volumes** | **Vec<String>** | volumes is a white list of allowed volume plugins.  Empty indicates that all plugins may be used. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


