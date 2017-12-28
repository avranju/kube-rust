/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// AutoscalingV1HorizontalPodAutoscalerSpec : specification of a horizontal pod autoscaler.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AutoscalingV1HorizontalPodAutoscalerSpec {
  /// upper limit for the number of pods that can be set by the autoscaler; cannot be smaller than MinReplicas.
  #[serde(rename = "maxReplicas")]
  max_replicas: i32,
  /// lower limit for the number of pods that can be set by the autoscaler, default 1.
  #[serde(rename = "minReplicas")]
  min_replicas: Option<i32>,
  /// reference to scaled resource; horizontal pod autoscaler will learn the current resource consumption and will set the desired number of pods by using its Scale subresource.
  #[serde(rename = "scaleTargetRef")]
  scale_target_ref: ::models::AutoscalingV1CrossVersionObjectReference,
  /// target average CPU utilization (represented as a percentage of requested CPU) over all the pods; if not specified the default autoscaling policy will be used.
  #[serde(rename = "targetCPUUtilizationPercentage")]
  target_cpu_utilization_percentage: Option<i32>
}

impl AutoscalingV1HorizontalPodAutoscalerSpec {
  /// specification of a horizontal pod autoscaler.
  pub fn new(max_replicas: i32, scale_target_ref: ::models::AutoscalingV1CrossVersionObjectReference) -> AutoscalingV1HorizontalPodAutoscalerSpec {
    AutoscalingV1HorizontalPodAutoscalerSpec {
      max_replicas: max_replicas,
      min_replicas: None,
      scale_target_ref: scale_target_ref,
      target_cpu_utilization_percentage: None
    }
  }

  pub fn set_max_replicas(&mut self, max_replicas: i32) {
    self.max_replicas = max_replicas;
  }

  pub fn with_max_replicas(mut self, max_replicas: i32) -> AutoscalingV1HorizontalPodAutoscalerSpec {
    self.max_replicas = max_replicas;
    self
  }

  pub fn max_replicas(&self) -> &i32 {
    &self.max_replicas
  }


  pub fn set_min_replicas(&mut self, min_replicas: i32) {
    self.min_replicas = Some(min_replicas);
  }

  pub fn with_min_replicas(mut self, min_replicas: i32) -> AutoscalingV1HorizontalPodAutoscalerSpec {
    self.min_replicas = Some(min_replicas);
    self
  }

  pub fn min_replicas(&self) -> Option<&i32> {
    self.min_replicas.as_ref()
  }

  pub fn reset_min_replicas(&mut self) {
    self.min_replicas = None;
  }

  pub fn set_scale_target_ref(&mut self, scale_target_ref: ::models::AutoscalingV1CrossVersionObjectReference) {
    self.scale_target_ref = scale_target_ref;
  }

  pub fn with_scale_target_ref(mut self, scale_target_ref: ::models::AutoscalingV1CrossVersionObjectReference) -> AutoscalingV1HorizontalPodAutoscalerSpec {
    self.scale_target_ref = scale_target_ref;
    self
  }

  pub fn scale_target_ref(&self) -> &::models::AutoscalingV1CrossVersionObjectReference {
    &self.scale_target_ref
  }


  pub fn set_target_cpu_utilization_percentage(&mut self, target_cpu_utilization_percentage: i32) {
    self.target_cpu_utilization_percentage = Some(target_cpu_utilization_percentage);
  }

  pub fn with_target_cpu_utilization_percentage(mut self, target_cpu_utilization_percentage: i32) -> AutoscalingV1HorizontalPodAutoscalerSpec {
    self.target_cpu_utilization_percentage = Some(target_cpu_utilization_percentage);
    self
  }

  pub fn target_cpu_utilization_percentage(&self) -> Option<&i32> {
    self.target_cpu_utilization_percentage.as_ref()
  }

  pub fn reset_target_cpu_utilization_percentage(&mut self) {
    self.target_cpu_utilization_percentage = None;
  }

}



