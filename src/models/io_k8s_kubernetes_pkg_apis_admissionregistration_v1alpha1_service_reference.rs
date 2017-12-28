/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AdmissionregistrationV1alpha1ServiceReference : ServiceReference holds a reference to Service.legacy.k8s.io

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdmissionregistrationV1alpha1ServiceReference {
  /// Name is the name of the service Required
  #[serde(rename = "name")]
  name: String,
  /// Namespace is the namespace of the service Required
  #[serde(rename = "namespace")]
  namespace: String
}

impl AdmissionregistrationV1alpha1ServiceReference {
  /// ServiceReference holds a reference to Service.legacy.k8s.io
  pub fn new(name: String, namespace: String) -> AdmissionregistrationV1alpha1ServiceReference {
    AdmissionregistrationV1alpha1ServiceReference {
      name: name,
      namespace: namespace
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> AdmissionregistrationV1alpha1ServiceReference {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


  pub fn set_namespace(&mut self, namespace: String) {
    self.namespace = namespace;
  }

  pub fn with_namespace(mut self, namespace: String) -> AdmissionregistrationV1alpha1ServiceReference {
    self.namespace = namespace;
    self
  }

  pub fn namespace(&self) -> &String {
    &self.namespace
  }


}



