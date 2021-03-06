/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// NetworkingV1NetworkPolicyPort : NetworkPolicyPort describes a port to allow traffic on

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkingV1NetworkPolicyPort {
  /// The port on the given protocol. This can either be a numerical or named port on a pod. If this field is not provided, this matches all port names and numbers.
  #[serde(rename = "port")]
  port: Option<::models::UtilIntstrIntOrString>,
  /// The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP.
  #[serde(rename = "protocol")]
  protocol: Option<String>
}

impl NetworkingV1NetworkPolicyPort {
  /// NetworkPolicyPort describes a port to allow traffic on
  pub fn new() -> NetworkingV1NetworkPolicyPort {
    NetworkingV1NetworkPolicyPort {
      port: None,
      protocol: None
    }
  }

  pub fn set_port(&mut self, port: ::models::UtilIntstrIntOrString) {
    self.port = Some(port);
  }

  pub fn with_port(mut self, port: ::models::UtilIntstrIntOrString) -> NetworkingV1NetworkPolicyPort {
    self.port = Some(port);
    self
  }

  pub fn port(&self) -> Option<&::models::UtilIntstrIntOrString> {
    self.port.as_ref()
  }

  pub fn reset_port(&mut self) {
    self.port = None;
  }

  pub fn set_protocol(&mut self, protocol: String) {
    self.protocol = Some(protocol);
  }

  pub fn with_protocol(mut self, protocol: String) -> NetworkingV1NetworkPolicyPort {
    self.protocol = Some(protocol);
    self
  }

  pub fn protocol(&self) -> Option<&String> {
    self.protocol.as_ref()
  }

  pub fn reset_protocol(&mut self) {
    self.protocol = None;
  }

}



