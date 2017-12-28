/* 
 * Kubernetes
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: v1.7.12
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ExtensionsV1beta1SupplementalGroupsStrategyOptions : SupplementalGroupsStrategyOptions defines the strategy type and options used to create the strategy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionsV1beta1SupplementalGroupsStrategyOptions {
  /// Ranges are the allowed ranges of supplemental groups.  If you would like to force a single supplemental group then supply a single range with the same start and end.
  #[serde(rename = "ranges")]
  ranges: Option<Vec<::models::ExtensionsV1beta1IdRange>>,
  /// Rule is the strategy that will dictate what supplemental groups is used in the SecurityContext.
  #[serde(rename = "rule")]
  rule: Option<String>
}

impl ExtensionsV1beta1SupplementalGroupsStrategyOptions {
  /// SupplementalGroupsStrategyOptions defines the strategy type and options used to create the strategy.
  pub fn new() -> ExtensionsV1beta1SupplementalGroupsStrategyOptions {
    ExtensionsV1beta1SupplementalGroupsStrategyOptions {
      ranges: None,
      rule: None
    }
  }

  pub fn set_ranges(&mut self, ranges: Vec<::models::ExtensionsV1beta1IdRange>) {
    self.ranges = Some(ranges);
  }

  pub fn with_ranges(mut self, ranges: Vec<::models::ExtensionsV1beta1IdRange>) -> ExtensionsV1beta1SupplementalGroupsStrategyOptions {
    self.ranges = Some(ranges);
    self
  }

  pub fn ranges(&self) -> Option<&Vec<::models::ExtensionsV1beta1IdRange>> {
    self.ranges.as_ref()
  }

  pub fn reset_ranges(&mut self) {
    self.ranges = None;
  }

  pub fn set_rule(&mut self, rule: String) {
    self.rule = Some(rule);
  }

  pub fn with_rule(mut self, rule: String) -> ExtensionsV1beta1SupplementalGroupsStrategyOptions {
    self.rule = Some(rule);
    self
  }

  pub fn rule(&self) -> Option<&String> {
    self.rule.as_ref()
  }

  pub fn reset_rule(&mut self) {
    self.rule = None;
  }

}



