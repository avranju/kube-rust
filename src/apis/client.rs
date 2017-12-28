use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  admissionregistration_api: Box<::apis::AdmissionregistrationApi>,
  admissionregistration_v1alpha1_api: Box<::apis::AdmissionregistrationV1alpha1Api>,
  apiregistration_api: Box<::apis::ApiregistrationApi>,
  apiregistration_v1beta1_api: Box<::apis::ApiregistrationV1beta1Api>,
  apis_api: Box<::apis::ApisApi>,
  apps_api: Box<::apis::AppsApi>,
  apps_v1beta1_api: Box<::apis::AppsV1beta1Api>,
  authentication_api: Box<::apis::AuthenticationApi>,
  authentication_v1_api: Box<::apis::AuthenticationV1Api>,
  authentication_v1beta1_api: Box<::apis::AuthenticationV1beta1Api>,
  authorization_api: Box<::apis::AuthorizationApi>,
  authorization_v1_api: Box<::apis::AuthorizationV1Api>,
  authorization_v1beta1_api: Box<::apis::AuthorizationV1beta1Api>,
  autoscaling_api: Box<::apis::AutoscalingApi>,
  autoscaling_v1_api: Box<::apis::AutoscalingV1Api>,
  autoscaling_v2alpha1_api: Box<::apis::AutoscalingV2alpha1Api>,
  batch_api: Box<::apis::BatchApi>,
  batch_v1_api: Box<::apis::BatchV1Api>,
  batch_v2alpha1_api: Box<::apis::BatchV2alpha1Api>,
  certificates_api: Box<::apis::CertificatesApi>,
  certificates_v1beta1_api: Box<::apis::CertificatesV1beta1Api>,
  core_api: Box<::apis::CoreApi>,
  core_v1_api: Box<::apis::CoreV1Api>,
  extensions_api: Box<::apis::ExtensionsApi>,
  extensions_v1beta1_api: Box<::apis::ExtensionsV1beta1Api>,
  logs_api: Box<::apis::LogsApi>,
  networking_api: Box<::apis::NetworkingApi>,
  networking_v1_api: Box<::apis::NetworkingV1Api>,
  policy_api: Box<::apis::PolicyApi>,
  policy_v1beta1_api: Box<::apis::PolicyV1beta1Api>,
  rbac_authorization_api: Box<::apis::RbacAuthorizationApi>,
  rbac_authorization_v1alpha1_api: Box<::apis::RbacAuthorizationV1alpha1Api>,
  rbac_authorization_v1beta1_api: Box<::apis::RbacAuthorizationV1beta1Api>,
  settings_api: Box<::apis::SettingsApi>,
  settings_v1alpha1_api: Box<::apis::SettingsV1alpha1Api>,
  storage_api: Box<::apis::StorageApi>,
  storage_v1_api: Box<::apis::StorageV1Api>,
  storage_v1beta1_api: Box<::apis::StorageV1beta1Api>,
  version_api: Box<::apis::VersionApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      admissionregistration_api: Box::new(::apis::AdmissionregistrationApiClient::new(rc.clone())),
      admissionregistration_v1alpha1_api: Box::new(::apis::AdmissionregistrationV1alpha1ApiClient::new(rc.clone())),
      apiregistration_api: Box::new(::apis::ApiregistrationApiClient::new(rc.clone())),
      apiregistration_v1beta1_api: Box::new(::apis::ApiregistrationV1beta1ApiClient::new(rc.clone())),
      apis_api: Box::new(::apis::ApisApiClient::new(rc.clone())),
      apps_api: Box::new(::apis::AppsApiClient::new(rc.clone())),
      apps_v1beta1_api: Box::new(::apis::AppsV1beta1ApiClient::new(rc.clone())),
      authentication_api: Box::new(::apis::AuthenticationApiClient::new(rc.clone())),
      authentication_v1_api: Box::new(::apis::AuthenticationV1ApiClient::new(rc.clone())),
      authentication_v1beta1_api: Box::new(::apis::AuthenticationV1beta1ApiClient::new(rc.clone())),
      authorization_api: Box::new(::apis::AuthorizationApiClient::new(rc.clone())),
      authorization_v1_api: Box::new(::apis::AuthorizationV1ApiClient::new(rc.clone())),
      authorization_v1beta1_api: Box::new(::apis::AuthorizationV1beta1ApiClient::new(rc.clone())),
      autoscaling_api: Box::new(::apis::AutoscalingApiClient::new(rc.clone())),
      autoscaling_v1_api: Box::new(::apis::AutoscalingV1ApiClient::new(rc.clone())),
      autoscaling_v2alpha1_api: Box::new(::apis::AutoscalingV2alpha1ApiClient::new(rc.clone())),
      batch_api: Box::new(::apis::BatchApiClient::new(rc.clone())),
      batch_v1_api: Box::new(::apis::BatchV1ApiClient::new(rc.clone())),
      batch_v2alpha1_api: Box::new(::apis::BatchV2alpha1ApiClient::new(rc.clone())),
      certificates_api: Box::new(::apis::CertificatesApiClient::new(rc.clone())),
      certificates_v1beta1_api: Box::new(::apis::CertificatesV1beta1ApiClient::new(rc.clone())),
      core_api: Box::new(::apis::CoreApiClient::new(rc.clone())),
      core_v1_api: Box::new(::apis::CoreV1ApiClient::new(rc.clone())),
      extensions_api: Box::new(::apis::ExtensionsApiClient::new(rc.clone())),
      extensions_v1beta1_api: Box::new(::apis::ExtensionsV1beta1ApiClient::new(rc.clone())),
      logs_api: Box::new(::apis::LogsApiClient::new(rc.clone())),
      networking_api: Box::new(::apis::NetworkingApiClient::new(rc.clone())),
      networking_v1_api: Box::new(::apis::NetworkingV1ApiClient::new(rc.clone())),
      policy_api: Box::new(::apis::PolicyApiClient::new(rc.clone())),
      policy_v1beta1_api: Box::new(::apis::PolicyV1beta1ApiClient::new(rc.clone())),
      rbac_authorization_api: Box::new(::apis::RbacAuthorizationApiClient::new(rc.clone())),
      rbac_authorization_v1alpha1_api: Box::new(::apis::RbacAuthorizationV1alpha1ApiClient::new(rc.clone())),
      rbac_authorization_v1beta1_api: Box::new(::apis::RbacAuthorizationV1beta1ApiClient::new(rc.clone())),
      settings_api: Box::new(::apis::SettingsApiClient::new(rc.clone())),
      settings_v1alpha1_api: Box::new(::apis::SettingsV1alpha1ApiClient::new(rc.clone())),
      storage_api: Box::new(::apis::StorageApiClient::new(rc.clone())),
      storage_v1_api: Box::new(::apis::StorageV1ApiClient::new(rc.clone())),
      storage_v1beta1_api: Box::new(::apis::StorageV1beta1ApiClient::new(rc.clone())),
      version_api: Box::new(::apis::VersionApiClient::new(rc.clone())),
    }
  }

  pub fn admissionregistration_api(&self) -> &::apis::AdmissionregistrationApi{
    self.admissionregistration_api.as_ref()
  }

  pub fn admissionregistration_v1alpha1_api(&self) -> &::apis::AdmissionregistrationV1alpha1Api{
    self.admissionregistration_v1alpha1_api.as_ref()
  }

  pub fn apiregistration_api(&self) -> &::apis::ApiregistrationApi{
    self.apiregistration_api.as_ref()
  }

  pub fn apiregistration_v1beta1_api(&self) -> &::apis::ApiregistrationV1beta1Api{
    self.apiregistration_v1beta1_api.as_ref()
  }

  pub fn apis_api(&self) -> &::apis::ApisApi{
    self.apis_api.as_ref()
  }

  pub fn apps_api(&self) -> &::apis::AppsApi{
    self.apps_api.as_ref()
  }

  pub fn apps_v1beta1_api(&self) -> &::apis::AppsV1beta1Api{
    self.apps_v1beta1_api.as_ref()
  }

  pub fn authentication_api(&self) -> &::apis::AuthenticationApi{
    self.authentication_api.as_ref()
  }

  pub fn authentication_v1_api(&self) -> &::apis::AuthenticationV1Api{
    self.authentication_v1_api.as_ref()
  }

  pub fn authentication_v1beta1_api(&self) -> &::apis::AuthenticationV1beta1Api{
    self.authentication_v1beta1_api.as_ref()
  }

  pub fn authorization_api(&self) -> &::apis::AuthorizationApi{
    self.authorization_api.as_ref()
  }

  pub fn authorization_v1_api(&self) -> &::apis::AuthorizationV1Api{
    self.authorization_v1_api.as_ref()
  }

  pub fn authorization_v1beta1_api(&self) -> &::apis::AuthorizationV1beta1Api{
    self.authorization_v1beta1_api.as_ref()
  }

  pub fn autoscaling_api(&self) -> &::apis::AutoscalingApi{
    self.autoscaling_api.as_ref()
  }

  pub fn autoscaling_v1_api(&self) -> &::apis::AutoscalingV1Api{
    self.autoscaling_v1_api.as_ref()
  }

  pub fn autoscaling_v2alpha1_api(&self) -> &::apis::AutoscalingV2alpha1Api{
    self.autoscaling_v2alpha1_api.as_ref()
  }

  pub fn batch_api(&self) -> &::apis::BatchApi{
    self.batch_api.as_ref()
  }

  pub fn batch_v1_api(&self) -> &::apis::BatchV1Api{
    self.batch_v1_api.as_ref()
  }

  pub fn batch_v2alpha1_api(&self) -> &::apis::BatchV2alpha1Api{
    self.batch_v2alpha1_api.as_ref()
  }

  pub fn certificates_api(&self) -> &::apis::CertificatesApi{
    self.certificates_api.as_ref()
  }

  pub fn certificates_v1beta1_api(&self) -> &::apis::CertificatesV1beta1Api{
    self.certificates_v1beta1_api.as_ref()
  }

  pub fn core_api(&self) -> &::apis::CoreApi{
    self.core_api.as_ref()
  }

  pub fn core_v1_api(&self) -> &::apis::CoreV1Api{
    self.core_v1_api.as_ref()
  }

  pub fn extensions_api(&self) -> &::apis::ExtensionsApi{
    self.extensions_api.as_ref()
  }

  pub fn extensions_v1beta1_api(&self) -> &::apis::ExtensionsV1beta1Api{
    self.extensions_v1beta1_api.as_ref()
  }

  pub fn logs_api(&self) -> &::apis::LogsApi{
    self.logs_api.as_ref()
  }

  pub fn networking_api(&self) -> &::apis::NetworkingApi{
    self.networking_api.as_ref()
  }

  pub fn networking_v1_api(&self) -> &::apis::NetworkingV1Api{
    self.networking_v1_api.as_ref()
  }

  pub fn policy_api(&self) -> &::apis::PolicyApi{
    self.policy_api.as_ref()
  }

  pub fn policy_v1beta1_api(&self) -> &::apis::PolicyV1beta1Api{
    self.policy_v1beta1_api.as_ref()
  }

  pub fn rbac_authorization_api(&self) -> &::apis::RbacAuthorizationApi{
    self.rbac_authorization_api.as_ref()
  }

  pub fn rbac_authorization_v1alpha1_api(&self) -> &::apis::RbacAuthorizationV1alpha1Api{
    self.rbac_authorization_v1alpha1_api.as_ref()
  }

  pub fn rbac_authorization_v1beta1_api(&self) -> &::apis::RbacAuthorizationV1beta1Api{
    self.rbac_authorization_v1beta1_api.as_ref()
  }

  pub fn settings_api(&self) -> &::apis::SettingsApi{
    self.settings_api.as_ref()
  }

  pub fn settings_v1alpha1_api(&self) -> &::apis::SettingsV1alpha1Api{
    self.settings_v1alpha1_api.as_ref()
  }

  pub fn storage_api(&self) -> &::apis::StorageApi{
    self.storage_api.as_ref()
  }

  pub fn storage_v1_api(&self) -> &::apis::StorageV1Api{
    self.storage_v1_api.as_ref()
  }

  pub fn storage_v1beta1_api(&self) -> &::apis::StorageV1beta1Api{
    self.storage_v1beta1_api.as_ref()
  }

  pub fn version_api(&self) -> &::apis::VersionApi{
    self.version_api.as_ref()
  }


}
