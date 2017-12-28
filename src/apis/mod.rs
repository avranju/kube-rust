use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod admissionregistration_api;
pub use self::admissionregistration_api::{ AdmissionregistrationApi, AdmissionregistrationApiClient };
mod admissionregistration_v1alpha1_api;
pub use self::admissionregistration_v1alpha1_api::{ AdmissionregistrationV1alpha1Api, AdmissionregistrationV1alpha1ApiClient };
mod apiregistration_api;
pub use self::apiregistration_api::{ ApiregistrationApi, ApiregistrationApiClient };
mod apiregistration_v1beta1_api;
pub use self::apiregistration_v1beta1_api::{ ApiregistrationV1beta1Api, ApiregistrationV1beta1ApiClient };
mod apis_api;
pub use self::apis_api::{ ApisApi, ApisApiClient };
mod apps_api;
pub use self::apps_api::{ AppsApi, AppsApiClient };
mod apps_v1beta1_api;
pub use self::apps_v1beta1_api::{ AppsV1beta1Api, AppsV1beta1ApiClient };
mod authentication_api;
pub use self::authentication_api::{ AuthenticationApi, AuthenticationApiClient };
mod authentication_v1_api;
pub use self::authentication_v1_api::{ AuthenticationV1Api, AuthenticationV1ApiClient };
mod authentication_v1beta1_api;
pub use self::authentication_v1beta1_api::{ AuthenticationV1beta1Api, AuthenticationV1beta1ApiClient };
mod authorization_api;
pub use self::authorization_api::{ AuthorizationApi, AuthorizationApiClient };
mod authorization_v1_api;
pub use self::authorization_v1_api::{ AuthorizationV1Api, AuthorizationV1ApiClient };
mod authorization_v1beta1_api;
pub use self::authorization_v1beta1_api::{ AuthorizationV1beta1Api, AuthorizationV1beta1ApiClient };
mod autoscaling_api;
pub use self::autoscaling_api::{ AutoscalingApi, AutoscalingApiClient };
mod autoscaling_v1_api;
pub use self::autoscaling_v1_api::{ AutoscalingV1Api, AutoscalingV1ApiClient };
mod autoscaling_v2alpha1_api;
pub use self::autoscaling_v2alpha1_api::{ AutoscalingV2alpha1Api, AutoscalingV2alpha1ApiClient };
mod batch_api;
pub use self::batch_api::{ BatchApi, BatchApiClient };
mod batch_v1_api;
pub use self::batch_v1_api::{ BatchV1Api, BatchV1ApiClient };
mod batch_v2alpha1_api;
pub use self::batch_v2alpha1_api::{ BatchV2alpha1Api, BatchV2alpha1ApiClient };
mod certificates_api;
pub use self::certificates_api::{ CertificatesApi, CertificatesApiClient };
mod certificates_v1beta1_api;
pub use self::certificates_v1beta1_api::{ CertificatesV1beta1Api, CertificatesV1beta1ApiClient };
mod core_api;
pub use self::core_api::{ CoreApi, CoreApiClient };
mod core_v1_api;
pub use self::core_v1_api::{ CoreV1Api, CoreV1ApiClient };
mod extensions_api;
pub use self::extensions_api::{ ExtensionsApi, ExtensionsApiClient };
mod extensions_v1beta1_api;
pub use self::extensions_v1beta1_api::{ ExtensionsV1beta1Api, ExtensionsV1beta1ApiClient };
mod logs_api;
pub use self::logs_api::{ LogsApi, LogsApiClient };
mod networking_api;
pub use self::networking_api::{ NetworkingApi, NetworkingApiClient };
mod networking_v1_api;
pub use self::networking_v1_api::{ NetworkingV1Api, NetworkingV1ApiClient };
mod policy_api;
pub use self::policy_api::{ PolicyApi, PolicyApiClient };
mod policy_v1beta1_api;
pub use self::policy_v1beta1_api::{ PolicyV1beta1Api, PolicyV1beta1ApiClient };
mod rbac_authorization_api;
pub use self::rbac_authorization_api::{ RbacAuthorizationApi, RbacAuthorizationApiClient };
mod rbac_authorization_v1alpha1_api;
pub use self::rbac_authorization_v1alpha1_api::{ RbacAuthorizationV1alpha1Api, RbacAuthorizationV1alpha1ApiClient };
mod rbac_authorization_v1beta1_api;
pub use self::rbac_authorization_v1beta1_api::{ RbacAuthorizationV1beta1Api, RbacAuthorizationV1beta1ApiClient };
mod settings_api;
pub use self::settings_api::{ SettingsApi, SettingsApiClient };
mod settings_v1alpha1_api;
pub use self::settings_v1alpha1_api::{ SettingsV1alpha1Api, SettingsV1alpha1ApiClient };
mod storage_api;
pub use self::storage_api::{ StorageApi, StorageApiClient };
mod storage_v1_api;
pub use self::storage_v1_api::{ StorageV1Api, StorageV1ApiClient };
mod storage_v1beta1_api;
pub use self::storage_v1beta1_api::{ StorageV1beta1Api, StorageV1beta1ApiClient };
mod version_api;
pub use self::version_api::{ VersionApi, VersionApiClient };

pub mod configuration;
pub mod client;
