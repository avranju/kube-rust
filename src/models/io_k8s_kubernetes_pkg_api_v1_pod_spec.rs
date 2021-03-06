/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1PodSpec : PodSpec is a description of a pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct V1PodSpec {
  /// Optional duration in seconds the pod may be active on the node relative to StartTime before the system will actively try to mark it failed and kill associated containers. Value must be a positive integer.
  #[serde(rename = "activeDeadlineSeconds")]
  active_deadline_seconds: Option<i64>,
  /// If specified, the pod's scheduling constraints
  #[serde(rename = "affinity")]
  affinity: Option<::models::V1Affinity>,
  /// AutomountServiceAccountToken indicates whether a service account token should be automatically mounted.
  #[serde(rename = "automountServiceAccountToken")]
  automount_service_account_token: Option<bool>,
  /// List of containers belonging to the pod. Containers cannot currently be added or removed. There must be at least one container in a Pod. Cannot be updated.
  #[serde(rename = "containers")]
  containers: Vec<::models::V1Container>,
  /// Set DNS policy for containers within the pod. One of 'ClusterFirstWithHostNet', 'ClusterFirst' or 'Default'. Defaults to \"ClusterFirst\". To have DNS options set along with hostNetwork, you have to specify DNS policy explicitly to 'ClusterFirstWithHostNet'.
  #[serde(rename = "dnsPolicy")]
  dns_policy: Option<String>,
  /// HostAliases is an optional list of hosts and IPs that will be injected into the pod's hosts file if specified. This is only valid for non-hostNetwork pods.
  #[serde(rename = "hostAliases")]
  host_aliases: Option<Vec<::models::V1HostAlias>>,
  /// Use the host's ipc namespace. Optional: Default to false.
  #[serde(rename = "hostIPC")]
  host_ipc: Option<bool>,
  /// Host networking requested for this pod. Use the host's network namespace. If this option is set, the ports that will be used must be specified. Default to false.
  #[serde(rename = "hostNetwork")]
  host_network: Option<bool>,
  /// Use the host's pid namespace. Optional: Default to false.
  #[serde(rename = "hostPID")]
  host_pid: Option<bool>,
  /// Specifies the hostname of the Pod If not specified, the pod's hostname will be set to a system-defined value.
  #[serde(rename = "hostname")]
  hostname: Option<String>,
  /// ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
  #[serde(rename = "imagePullSecrets")]
  image_pull_secrets: Option<Vec<::models::V1LocalObjectReference>>,
  /// List of initialization containers belonging to the pod. Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, or Liveness probes. The resourceRequirements of an init container are taken into account during scheduling by finding the highest request/limit for each resource type, and then using the max of of that value or the sum of the normal containers. Limits are applied to init containers in a similar fashion. Init containers cannot currently be added or removed. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/
  #[serde(rename = "initContainers")]
  init_containers: Option<Vec<::models::V1Container>>,
  /// NodeName is a request to schedule this pod onto a specific node. If it is non-empty, the scheduler simply schedules this pod onto that node, assuming that it fits resource requirements.
  #[serde(rename = "nodeName")]
  node_name: Option<String>,
  /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
  #[serde(rename = "nodeSelector")]
  node_selector: Option<::std::collections::HashMap<String, String>>,
  /// Restart policy for all containers within the pod. One of Always, OnFailure, Never. Default to Always. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#restart-policy
  #[serde(rename = "restartPolicy")]
  restart_policy: Option<String>,
  /// If specified, the pod will be dispatched by specified scheduler. If not specified, the pod will be dispatched by default scheduler.
  #[serde(rename = "schedulerName")]
  scheduler_name: Option<String>,
  /// SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
  #[serde(rename = "securityContext")]
  security_context: Option<::models::V1PodSecurityContext>,
  /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName. Deprecated: Use serviceAccountName instead.
  #[serde(rename = "serviceAccount")]
  service_account: Option<String>,
  /// ServiceAccountName is the name of the ServiceAccount to use to run this pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
  #[serde(rename = "serviceAccountName")]
  service_account_name: Option<String>,
  /// If specified, the fully qualified Pod hostname will be \"<hostname>.<subdomain>.<pod namespace>.svc.<cluster domain>\". If not specified, the pod will not have a domainname at all.
  #[serde(rename = "subdomain")]
  subdomain: Option<String>,
  /// Optional duration in seconds the pod needs to terminate gracefully. May be decreased in delete request. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period will be used instead. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. Defaults to 30 seconds.
  #[serde(rename = "terminationGracePeriodSeconds")]
  termination_grace_period_seconds: Option<i64>,
  /// If specified, the pod's tolerations.
  #[serde(rename = "tolerations")]
  tolerations: Option<Vec<::models::V1Toleration>>,
  /// List of volumes that can be mounted by containers belonging to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes
  #[serde(rename = "volumes")]
  volumes: Option<Vec<::models::V1Volume>>
}

impl V1PodSpec {
  /// PodSpec is a description of a pod.
  pub fn new(containers: Vec<::models::V1Container>) -> V1PodSpec {
    V1PodSpec {
      active_deadline_seconds: None,
      affinity: None,
      automount_service_account_token: None,
      containers: containers,
      dns_policy: None,
      host_aliases: None,
      host_ipc: None,
      host_network: None,
      host_pid: None,
      hostname: None,
      image_pull_secrets: None,
      init_containers: None,
      node_name: None,
      node_selector: None,
      restart_policy: None,
      scheduler_name: None,
      security_context: None,
      service_account: None,
      service_account_name: None,
      subdomain: None,
      termination_grace_period_seconds: None,
      tolerations: None,
      volumes: None
    }
  }

  pub fn set_active_deadline_seconds(&mut self, active_deadline_seconds: i64) {
    self.active_deadline_seconds = Some(active_deadline_seconds);
  }

  pub fn with_active_deadline_seconds(mut self, active_deadline_seconds: i64) -> V1PodSpec {
    self.active_deadline_seconds = Some(active_deadline_seconds);
    self
  }

  pub fn active_deadline_seconds(&self) -> Option<&i64> {
    self.active_deadline_seconds.as_ref()
  }

  pub fn reset_active_deadline_seconds(&mut self) {
    self.active_deadline_seconds = None;
  }

  pub fn set_affinity(&mut self, affinity: ::models::V1Affinity) {
    self.affinity = Some(affinity);
  }

  pub fn with_affinity(mut self, affinity: ::models::V1Affinity) -> V1PodSpec {
    self.affinity = Some(affinity);
    self
  }

  pub fn affinity(&self) -> Option<&::models::V1Affinity> {
    self.affinity.as_ref()
  }

  pub fn reset_affinity(&mut self) {
    self.affinity = None;
  }

  pub fn set_automount_service_account_token(&mut self, automount_service_account_token: bool) {
    self.automount_service_account_token = Some(automount_service_account_token);
  }

  pub fn with_automount_service_account_token(mut self, automount_service_account_token: bool) -> V1PodSpec {
    self.automount_service_account_token = Some(automount_service_account_token);
    self
  }

  pub fn automount_service_account_token(&self) -> Option<&bool> {
    self.automount_service_account_token.as_ref()
  }

  pub fn reset_automount_service_account_token(&mut self) {
    self.automount_service_account_token = None;
  }

  pub fn set_containers(&mut self, containers: Vec<::models::V1Container>) {
    self.containers = containers;
  }

  pub fn with_containers(mut self, containers: Vec<::models::V1Container>) -> V1PodSpec {
    self.containers = containers;
    self
  }

  pub fn containers(&self) -> &Vec<::models::V1Container> {
    &self.containers
  }


  pub fn set_dns_policy(&mut self, dns_policy: String) {
    self.dns_policy = Some(dns_policy);
  }

  pub fn with_dns_policy(mut self, dns_policy: String) -> V1PodSpec {
    self.dns_policy = Some(dns_policy);
    self
  }

  pub fn dns_policy(&self) -> Option<&String> {
    self.dns_policy.as_ref()
  }

  pub fn reset_dns_policy(&mut self) {
    self.dns_policy = None;
  }

  pub fn set_host_aliases(&mut self, host_aliases: Vec<::models::V1HostAlias>) {
    self.host_aliases = Some(host_aliases);
  }

  pub fn with_host_aliases(mut self, host_aliases: Vec<::models::V1HostAlias>) -> V1PodSpec {
    self.host_aliases = Some(host_aliases);
    self
  }

  pub fn host_aliases(&self) -> Option<&Vec<::models::V1HostAlias>> {
    self.host_aliases.as_ref()
  }

  pub fn reset_host_aliases(&mut self) {
    self.host_aliases = None;
  }

  pub fn set_host_ipc(&mut self, host_ipc: bool) {
    self.host_ipc = Some(host_ipc);
  }

  pub fn with_host_ipc(mut self, host_ipc: bool) -> V1PodSpec {
    self.host_ipc = Some(host_ipc);
    self
  }

  pub fn host_ipc(&self) -> Option<&bool> {
    self.host_ipc.as_ref()
  }

  pub fn reset_host_ipc(&mut self) {
    self.host_ipc = None;
  }

  pub fn set_host_network(&mut self, host_network: bool) {
    self.host_network = Some(host_network);
  }

  pub fn with_host_network(mut self, host_network: bool) -> V1PodSpec {
    self.host_network = Some(host_network);
    self
  }

  pub fn host_network(&self) -> Option<&bool> {
    self.host_network.as_ref()
  }

  pub fn reset_host_network(&mut self) {
    self.host_network = None;
  }

  pub fn set_host_pid(&mut self, host_pid: bool) {
    self.host_pid = Some(host_pid);
  }

  pub fn with_host_pid(mut self, host_pid: bool) -> V1PodSpec {
    self.host_pid = Some(host_pid);
    self
  }

  pub fn host_pid(&self) -> Option<&bool> {
    self.host_pid.as_ref()
  }

  pub fn reset_host_pid(&mut self) {
    self.host_pid = None;
  }

  pub fn set_hostname(&mut self, hostname: String) {
    self.hostname = Some(hostname);
  }

  pub fn with_hostname(mut self, hostname: String) -> V1PodSpec {
    self.hostname = Some(hostname);
    self
  }

  pub fn hostname(&self) -> Option<&String> {
    self.hostname.as_ref()
  }

  pub fn reset_hostname(&mut self) {
    self.hostname = None;
  }

  pub fn set_image_pull_secrets(&mut self, image_pull_secrets: Vec<::models::V1LocalObjectReference>) {
    self.image_pull_secrets = Some(image_pull_secrets);
  }

  pub fn with_image_pull_secrets(mut self, image_pull_secrets: Vec<::models::V1LocalObjectReference>) -> V1PodSpec {
    self.image_pull_secrets = Some(image_pull_secrets);
    self
  }

  pub fn image_pull_secrets(&self) -> Option<&Vec<::models::V1LocalObjectReference>> {
    self.image_pull_secrets.as_ref()
  }

  pub fn reset_image_pull_secrets(&mut self) {
    self.image_pull_secrets = None;
  }

  pub fn set_init_containers(&mut self, init_containers: Vec<::models::V1Container>) {
    self.init_containers = Some(init_containers);
  }

  pub fn with_init_containers(mut self, init_containers: Vec<::models::V1Container>) -> V1PodSpec {
    self.init_containers = Some(init_containers);
    self
  }

  pub fn init_containers(&self) -> Option<&Vec<::models::V1Container>> {
    self.init_containers.as_ref()
  }

  pub fn reset_init_containers(&mut self) {
    self.init_containers = None;
  }

  pub fn set_node_name(&mut self, node_name: String) {
    self.node_name = Some(node_name);
  }

  pub fn with_node_name(mut self, node_name: String) -> V1PodSpec {
    self.node_name = Some(node_name);
    self
  }

  pub fn node_name(&self) -> Option<&String> {
    self.node_name.as_ref()
  }

  pub fn reset_node_name(&mut self) {
    self.node_name = None;
  }

  pub fn set_node_selector(&mut self, node_selector: ::std::collections::HashMap<String, String>) {
    self.node_selector = Some(node_selector);
  }

  pub fn with_node_selector(mut self, node_selector: ::std::collections::HashMap<String, String>) -> V1PodSpec {
    self.node_selector = Some(node_selector);
    self
  }

  pub fn node_selector(&self) -> Option<&::std::collections::HashMap<String, String>> {
    self.node_selector.as_ref()
  }

  pub fn reset_node_selector(&mut self) {
    self.node_selector = None;
  }

  pub fn set_restart_policy(&mut self, restart_policy: String) {
    self.restart_policy = Some(restart_policy);
  }

  pub fn with_restart_policy(mut self, restart_policy: String) -> V1PodSpec {
    self.restart_policy = Some(restart_policy);
    self
  }

  pub fn restart_policy(&self) -> Option<&String> {
    self.restart_policy.as_ref()
  }

  pub fn reset_restart_policy(&mut self) {
    self.restart_policy = None;
  }

  pub fn set_scheduler_name(&mut self, scheduler_name: String) {
    self.scheduler_name = Some(scheduler_name);
  }

  pub fn with_scheduler_name(mut self, scheduler_name: String) -> V1PodSpec {
    self.scheduler_name = Some(scheduler_name);
    self
  }

  pub fn scheduler_name(&self) -> Option<&String> {
    self.scheduler_name.as_ref()
  }

  pub fn reset_scheduler_name(&mut self) {
    self.scheduler_name = None;
  }

  pub fn set_security_context(&mut self, security_context: ::models::V1PodSecurityContext) {
    self.security_context = Some(security_context);
  }

  pub fn with_security_context(mut self, security_context: ::models::V1PodSecurityContext) -> V1PodSpec {
    self.security_context = Some(security_context);
    self
  }

  pub fn security_context(&self) -> Option<&::models::V1PodSecurityContext> {
    self.security_context.as_ref()
  }

  pub fn reset_security_context(&mut self) {
    self.security_context = None;
  }

  pub fn set_service_account(&mut self, service_account: String) {
    self.service_account = Some(service_account);
  }

  pub fn with_service_account(mut self, service_account: String) -> V1PodSpec {
    self.service_account = Some(service_account);
    self
  }

  pub fn service_account(&self) -> Option<&String> {
    self.service_account.as_ref()
  }

  pub fn reset_service_account(&mut self) {
    self.service_account = None;
  }

  pub fn set_service_account_name(&mut self, service_account_name: String) {
    self.service_account_name = Some(service_account_name);
  }

  pub fn with_service_account_name(mut self, service_account_name: String) -> V1PodSpec {
    self.service_account_name = Some(service_account_name);
    self
  }

  pub fn service_account_name(&self) -> Option<&String> {
    self.service_account_name.as_ref()
  }

  pub fn reset_service_account_name(&mut self) {
    self.service_account_name = None;
  }

  pub fn set_subdomain(&mut self, subdomain: String) {
    self.subdomain = Some(subdomain);
  }

  pub fn with_subdomain(mut self, subdomain: String) -> V1PodSpec {
    self.subdomain = Some(subdomain);
    self
  }

  pub fn subdomain(&self) -> Option<&String> {
    self.subdomain.as_ref()
  }

  pub fn reset_subdomain(&mut self) {
    self.subdomain = None;
  }

  pub fn set_termination_grace_period_seconds(&mut self, termination_grace_period_seconds: i64) {
    self.termination_grace_period_seconds = Some(termination_grace_period_seconds);
  }

  pub fn with_termination_grace_period_seconds(mut self, termination_grace_period_seconds: i64) -> V1PodSpec {
    self.termination_grace_period_seconds = Some(termination_grace_period_seconds);
    self
  }

  pub fn termination_grace_period_seconds(&self) -> Option<&i64> {
    self.termination_grace_period_seconds.as_ref()
  }

  pub fn reset_termination_grace_period_seconds(&mut self) {
    self.termination_grace_period_seconds = None;
  }

  pub fn set_tolerations(&mut self, tolerations: Vec<::models::V1Toleration>) {
    self.tolerations = Some(tolerations);
  }

  pub fn with_tolerations(mut self, tolerations: Vec<::models::V1Toleration>) -> V1PodSpec {
    self.tolerations = Some(tolerations);
    self
  }

  pub fn tolerations(&self) -> Option<&Vec<::models::V1Toleration>> {
    self.tolerations.as_ref()
  }

  pub fn reset_tolerations(&mut self) {
    self.tolerations = None;
  }

  pub fn set_volumes(&mut self, volumes: Vec<::models::V1Volume>) {
    self.volumes = Some(volumes);
  }

  pub fn with_volumes(mut self, volumes: Vec<::models::V1Volume>) -> V1PodSpec {
    self.volumes = Some(volumes);
    self
  }

  pub fn volumes(&self) -> Option<&Vec<::models::V1Volume>> {
    self.volumes.as_ref()
  }

  pub fn reset_volumes(&mut self) {
    self.volumes = None;
  }

}



