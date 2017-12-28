# CertificatesV1beta1CertificateSigningRequestStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**certificate** | **String** | If request was approved, the controller will place the issued certificate here. | [optional] [default to null]
**conditions** | [**Vec<::models::CertificatesV1beta1CertificateSigningRequestCondition>**](io.k8s.kubernetes.pkg.apis.certificates.v1beta1.CertificateSigningRequestCondition.md) | Conditions applied to the request, such as approval or denial. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


