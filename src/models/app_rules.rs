use serde::{Deserialize, Serialize};

/// An App Rule controls user provisioning and entitlement assignment for an application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    /// Match type: "all" requires ALL conditions to match, "any" requires ANY condition to match
    #[serde(rename = "match")]
    pub match_type: Option<String>,
    /// Execution priority - lower numbers execute first
    pub position: Option<i32>,
    #[serde(default)]
    pub conditions: Vec<AppRuleCondition>,
    #[serde(default)]
    pub actions: Vec<AppRuleAction>,
}

/// A condition that must be evaluated for a rule to apply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRuleCondition {
    /// The source field to evaluate (e.g., "has_role", "member_of", custom attribute name)
    pub source: String,
    /// The comparison operator (e.g., "=", "!=", "contains", "regex")
    pub operator: String,
    /// The value to compare against
    pub value: String,
}

/// An action to perform when the rule's conditions are met
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRuleAction {
    /// The action type (e.g., "set_*", "add_*", "put_*")
    pub action: String,
    /// Values for the action (can be single value or array depending on action type)
    pub value: Vec<String>,
    /// Optional expression for dynamic values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// Optional macro for value transformation
    #[serde(rename = "macro", skip_serializing_if = "Option::is_none")]
    pub macro_value: Option<String>,
    /// Optional scripted macro
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriplet: Option<String>,
}

/// Request to create a new app rule
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRuleRequest {
    /// Name of the rule
    pub name: String,
    /// Whether the rule is active (default: true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Match type: "all" or "any"
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    /// Execution priority (lower numbers execute first)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// Array of conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CreateAppRuleCondition>>,
    /// Array of actions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<CreateAppRuleAction>>,
}

/// Condition for rule creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRuleCondition {
    pub source: String,
    pub operator: String,
    pub value: String,
}

/// Action for rule creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppRuleAction {
    pub action: String,
    pub value: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "macro", skip_serializing_if = "Option::is_none")]
    pub macro_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriplet: Option<String>,
}

/// Request to update an existing app rule
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppRuleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<CreateAppRuleCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<CreateAppRuleAction>>,
}

/// Available condition definition (from /rules/conditions endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionDef {
    /// Unique identifier for this condition type
    pub value: String,
    /// Human-readable name
    pub name: String,
}

/// Available action definition (from /rules/actions endpoint)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleActionDef {
    /// Unique identifier for this action type
    pub value: String,
    /// Human-readable name
    pub name: String,
}

/// Operator for a condition type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionOperator {
    /// Operator value (e.g., "=", "!=", "ri", "nri")
    pub value: String,
    /// Human-readable name (e.g., "equals", "does not equal", "contains", "does not contain")
    pub name: String,
}

/// Available value for a condition type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionValue {
    /// Value identifier
    pub value: String,
    /// Human-readable name
    pub name: String,
}

/// Available value for an action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleActionValue {
    /// Value identifier
    pub value: String,
    /// Human-readable name
    pub name: String,
}

/// Request to sort/reorder rules
#[derive(Debug, Serialize, Deserialize)]
pub struct SortRulesRequest {
    /// Array of rule IDs in desired execution order
    pub rule_ids: Vec<i64>,
}

/// Query parameters for listing rules
#[derive(Debug, Default, Serialize)]
pub struct AppRuleQueryParams {
    /// Filter by enabled status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Filter by condition name and value (supports wildcards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_condition: Option<String>,
    /// Filter by condition type: "builtin", "custom", or "none"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_condition_type: Option<String>,
    /// Filter by action name and value (supports wildcards)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_action: Option<String>,
    /// Filter by action type: "builtin", "custom", or "none"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_action_type: Option<String>,
}
