/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Container : A single application container that you want to run within a pod.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1Container {
  /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
  #[serde(rename = "args")]
  args: Option<Vec<String>>,
  /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
  #[serde(rename = "command")]
  command: Option<Vec<String>>,
  /// List of environment variables to set in the container. Cannot be updated.
  #[serde(rename = "env")]
  env: Option<Vec<::models::V1EnvVar>>,
  /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
  #[serde(rename = "envFrom")]
  env_from: Option<Vec<::models::V1EnvFromSource>>,
  /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images
  #[serde(rename = "image")]
  image: String,
  /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
  #[serde(rename = "imagePullPolicy")]
  image_pull_policy: Option<String>,
  /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
  #[serde(rename = "lifecycle")]
  lifecycle: Option<::models::V1Lifecycle>,
  /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
  #[serde(rename = "livenessProbe")]
  liveness_probe: Option<::models::V1Probe>,
  /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
  #[serde(rename = "name")]
  name: String,
  /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default \"0.0.0.0\" address inside a container will be accessible from the network. Cannot be updated.
  #[serde(rename = "ports")]
  ports: Option<Vec<::models::V1ContainerPort>>,
  /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
  #[serde(rename = "readinessProbe")]
  readiness_probe: Option<::models::V1Probe>,
  /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
  #[serde(rename = "resources")]
  resources: Option<::models::V1ResourceRequirements>,
  /// Security options the pod should run with. More info: https://kubernetes.io/docs/concepts/policy/security-context/ More info: https://git.k8s.io/community/contributors/design-proposals/security_context.md
  #[serde(rename = "securityContext")]
  security_context: Option<::models::V1SecurityContext>,
  /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
  #[serde(rename = "stdin")]
  stdin: Option<bool>,
  /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
  #[serde(rename = "stdinOnce")]
  stdin_once: Option<bool>,
  /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
  #[serde(rename = "terminationMessagePath")]
  termination_message_path: Option<String>,
  /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
  #[serde(rename = "terminationMessagePolicy")]
  termination_message_policy: Option<String>,
  /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
  #[serde(rename = "tty")]
  tty: Option<bool>,
  /// Pod volumes to mount into the container's filesystem. Cannot be updated.
  #[serde(rename = "volumeMounts")]
  volume_mounts: Option<Vec<::models::V1VolumeMount>>,
  /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
  #[serde(rename = "workingDir")]
  working_dir: Option<String>
}

impl V1Container {
  /// A single application container that you want to run within a pod.
  pub fn new(image: String, name: String) -> V1Container {
    V1Container {
      args: None,
      command: None,
      env: None,
      env_from: None,
      image: image,
      image_pull_policy: None,
      lifecycle: None,
      liveness_probe: None,
      name: name,
      ports: None,
      readiness_probe: None,
      resources: None,
      security_context: None,
      stdin: None,
      stdin_once: None,
      termination_message_path: None,
      termination_message_policy: None,
      tty: None,
      volume_mounts: None,
      working_dir: None
    }
  }

  pub fn set_args(&mut self, args: Vec<String>) {
    self.args = Some(args);
  }

  pub fn with_args(mut self, args: Vec<String>) -> V1Container {
    self.args = Some(args);
    self
  }

  pub fn args(&self) -> Option<&Vec<String>> {
    self.args.as_ref()
  }

  pub fn reset_args(&mut self) {
    self.args = None;
  }

  pub fn set_command(&mut self, command: Vec<String>) {
    self.command = Some(command);
  }

  pub fn with_command(mut self, command: Vec<String>) -> V1Container {
    self.command = Some(command);
    self
  }

  pub fn command(&self) -> Option<&Vec<String>> {
    self.command.as_ref()
  }

  pub fn reset_command(&mut self) {
    self.command = None;
  }

  pub fn set_env(&mut self, env: Vec<::models::V1EnvVar>) {
    self.env = Some(env);
  }

  pub fn with_env(mut self, env: Vec<::models::V1EnvVar>) -> V1Container {
    self.env = Some(env);
    self
  }

  pub fn env(&self) -> Option<&Vec<::models::V1EnvVar>> {
    self.env.as_ref()
  }

  pub fn reset_env(&mut self) {
    self.env = None;
  }

  pub fn set_env_from(&mut self, env_from: Vec<::models::V1EnvFromSource>) {
    self.env_from = Some(env_from);
  }

  pub fn with_env_from(mut self, env_from: Vec<::models::V1EnvFromSource>) -> V1Container {
    self.env_from = Some(env_from);
    self
  }

  pub fn env_from(&self) -> Option<&Vec<::models::V1EnvFromSource>> {
    self.env_from.as_ref()
  }

  pub fn reset_env_from(&mut self) {
    self.env_from = None;
  }

  pub fn set_image(&mut self, image: String) {
    self.image = image;
  }

  pub fn with_image(mut self, image: String) -> V1Container {
    self.image = image;
    self
  }

  pub fn image(&self) -> &String {
    &self.image
  }


  pub fn set_image_pull_policy(&mut self, image_pull_policy: String) {
    self.image_pull_policy = Some(image_pull_policy);
  }

  pub fn with_image_pull_policy(mut self, image_pull_policy: String) -> V1Container {
    self.image_pull_policy = Some(image_pull_policy);
    self
  }

  pub fn image_pull_policy(&self) -> Option<&String> {
    self.image_pull_policy.as_ref()
  }

  pub fn reset_image_pull_policy(&mut self) {
    self.image_pull_policy = None;
  }

  pub fn set_lifecycle(&mut self, lifecycle: ::models::V1Lifecycle) {
    self.lifecycle = Some(lifecycle);
  }

  pub fn with_lifecycle(mut self, lifecycle: ::models::V1Lifecycle) -> V1Container {
    self.lifecycle = Some(lifecycle);
    self
  }

  pub fn lifecycle(&self) -> Option<&::models::V1Lifecycle> {
    self.lifecycle.as_ref()
  }

  pub fn reset_lifecycle(&mut self) {
    self.lifecycle = None;
  }

  pub fn set_liveness_probe(&mut self, liveness_probe: ::models::V1Probe) {
    self.liveness_probe = Some(liveness_probe);
  }

  pub fn with_liveness_probe(mut self, liveness_probe: ::models::V1Probe) -> V1Container {
    self.liveness_probe = Some(liveness_probe);
    self
  }

  pub fn liveness_probe(&self) -> Option<&::models::V1Probe> {
    self.liveness_probe.as_ref()
  }

  pub fn reset_liveness_probe(&mut self) {
    self.liveness_probe = None;
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> V1Container {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_ports(&mut self, ports: Vec<::models::V1ContainerPort>) {
    self.ports = Some(ports);
  }

  pub fn with_ports(mut self, ports: Vec<::models::V1ContainerPort>) -> V1Container {
    self.ports = Some(ports);
    self
  }

  pub fn ports(&self) -> Option<&Vec<::models::V1ContainerPort>> {
    self.ports.as_ref()
  }

  pub fn reset_ports(&mut self) {
    self.ports = None;
  }

  pub fn set_readiness_probe(&mut self, readiness_probe: ::models::V1Probe) {
    self.readiness_probe = Some(readiness_probe);
  }

  pub fn with_readiness_probe(mut self, readiness_probe: ::models::V1Probe) -> V1Container {
    self.readiness_probe = Some(readiness_probe);
    self
  }

  pub fn readiness_probe(&self) -> Option<&::models::V1Probe> {
    self.readiness_probe.as_ref()
  }

  pub fn reset_readiness_probe(&mut self) {
    self.readiness_probe = None;
  }

  pub fn set_resources(&mut self, resources: ::models::V1ResourceRequirements) {
    self.resources = Some(resources);
  }

  pub fn with_resources(mut self, resources: ::models::V1ResourceRequirements) -> V1Container {
    self.resources = Some(resources);
    self
  }

  pub fn resources(&self) -> Option<&::models::V1ResourceRequirements> {
    self.resources.as_ref()
  }

  pub fn reset_resources(&mut self) {
    self.resources = None;
  }

  pub fn set_security_context(&mut self, security_context: ::models::V1SecurityContext) {
    self.security_context = Some(security_context);
  }

  pub fn with_security_context(mut self, security_context: ::models::V1SecurityContext) -> V1Container {
    self.security_context = Some(security_context);
    self
  }

  pub fn security_context(&self) -> Option<&::models::V1SecurityContext> {
    self.security_context.as_ref()
  }

  pub fn reset_security_context(&mut self) {
    self.security_context = None;
  }

  pub fn set_stdin(&mut self, stdin: bool) {
    self.stdin = Some(stdin);
  }

  pub fn with_stdin(mut self, stdin: bool) -> V1Container {
    self.stdin = Some(stdin);
    self
  }

  pub fn stdin(&self) -> Option<&bool> {
    self.stdin.as_ref()
  }

  pub fn reset_stdin(&mut self) {
    self.stdin = None;
  }

  pub fn set_stdin_once(&mut self, stdin_once: bool) {
    self.stdin_once = Some(stdin_once);
  }

  pub fn with_stdin_once(mut self, stdin_once: bool) -> V1Container {
    self.stdin_once = Some(stdin_once);
    self
  }

  pub fn stdin_once(&self) -> Option<&bool> {
    self.stdin_once.as_ref()
  }

  pub fn reset_stdin_once(&mut self) {
    self.stdin_once = None;
  }

  pub fn set_termination_message_path(&mut self, termination_message_path: String) {
    self.termination_message_path = Some(termination_message_path);
  }

  pub fn with_termination_message_path(mut self, termination_message_path: String) -> V1Container {
    self.termination_message_path = Some(termination_message_path);
    self
  }

  pub fn termination_message_path(&self) -> Option<&String> {
    self.termination_message_path.as_ref()
  }

  pub fn reset_termination_message_path(&mut self) {
    self.termination_message_path = None;
  }

  pub fn set_termination_message_policy(&mut self, termination_message_policy: String) {
    self.termination_message_policy = Some(termination_message_policy);
  }

  pub fn with_termination_message_policy(mut self, termination_message_policy: String) -> V1Container {
    self.termination_message_policy = Some(termination_message_policy);
    self
  }

  pub fn termination_message_policy(&self) -> Option<&String> {
    self.termination_message_policy.as_ref()
  }

  pub fn reset_termination_message_policy(&mut self) {
    self.termination_message_policy = None;
  }

  pub fn set_tty(&mut self, tty: bool) {
    self.tty = Some(tty);
  }

  pub fn with_tty(mut self, tty: bool) -> V1Container {
    self.tty = Some(tty);
    self
  }

  pub fn tty(&self) -> Option<&bool> {
    self.tty.as_ref()
  }

  pub fn reset_tty(&mut self) {
    self.tty = None;
  }

  pub fn set_volume_mounts(&mut self, volume_mounts: Vec<::models::V1VolumeMount>) {
    self.volume_mounts = Some(volume_mounts);
  }

  pub fn with_volume_mounts(mut self, volume_mounts: Vec<::models::V1VolumeMount>) -> V1Container {
    self.volume_mounts = Some(volume_mounts);
    self
  }

  pub fn volume_mounts(&self) -> Option<&Vec<::models::V1VolumeMount>> {
    self.volume_mounts.as_ref()
  }

  pub fn reset_volume_mounts(&mut self) {
    self.volume_mounts = None;
  }

  pub fn set_working_dir(&mut self, working_dir: String) {
    self.working_dir = Some(working_dir);
  }

  pub fn with_working_dir(mut self, working_dir: String) -> V1Container {
    self.working_dir = Some(working_dir);
    self
  }

  pub fn working_dir(&self) -> Option<&String> {
    self.working_dir.as_ref()
  }

  pub fn reset_working_dir(&mut self) {
    self.working_dir = None;
  }

}



