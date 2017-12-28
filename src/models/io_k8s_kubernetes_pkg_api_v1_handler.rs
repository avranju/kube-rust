/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// V1Handler : Handler defines a specific action that should be taken

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct V1Handler {
  /// One and only one of the following should be specified. Exec specifies the action to take.
  #[serde(rename = "exec")]
  exec: Option<::models::V1ExecAction>,
  /// HTTPGet specifies the http request to perform.
  #[serde(rename = "httpGet")]
  http_get: Option<::models::V1HttpGetAction>,
  /// TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
  #[serde(rename = "tcpSocket")]
  tcp_socket: Option<::models::V1TcpSocketAction>
}

impl V1Handler {
  /// Handler defines a specific action that should be taken
  pub fn new() -> V1Handler {
    V1Handler {
      exec: None,
      http_get: None,
      tcp_socket: None
    }
  }

  pub fn set_exec(&mut self, exec: ::models::V1ExecAction) {
    self.exec = Some(exec);
  }

  pub fn with_exec(mut self, exec: ::models::V1ExecAction) -> V1Handler {
    self.exec = Some(exec);
    self
  }

  pub fn exec(&self) -> Option<&::models::V1ExecAction> {
    self.exec.as_ref()
  }

  pub fn reset_exec(&mut self) {
    self.exec = None;
  }

  pub fn set_http_get(&mut self, http_get: ::models::V1HttpGetAction) {
    self.http_get = Some(http_get);
  }

  pub fn with_http_get(mut self, http_get: ::models::V1HttpGetAction) -> V1Handler {
    self.http_get = Some(http_get);
    self
  }

  pub fn http_get(&self) -> Option<&::models::V1HttpGetAction> {
    self.http_get.as_ref()
  }

  pub fn reset_http_get(&mut self) {
    self.http_get = None;
  }

  pub fn set_tcp_socket(&mut self, tcp_socket: ::models::V1TcpSocketAction) {
    self.tcp_socket = Some(tcp_socket);
  }

  pub fn with_tcp_socket(mut self, tcp_socket: ::models::V1TcpSocketAction) -> V1Handler {
    self.tcp_socket = Some(tcp_socket);
    self
  }

  pub fn tcp_socket(&self) -> Option<&::models::V1TcpSocketAction> {
    self.tcp_socket.as_ref()
  }

  pub fn reset_tcp_socket(&mut self) {
    self.tcp_socket = None;
  }

}



