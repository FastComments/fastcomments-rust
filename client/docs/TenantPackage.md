# TenantPackage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**name** | **String** |  | 
**tenant_id** | **String** |  | 
**created_at** | **String** |  | 
**monthly_cost_usd** | Option<**f64**> |  | 
**yearly_cost_usd** | Option<**f64**> |  | 
**monthly_stripe_plan_id** | Option<**String**> |  | 
**yearly_stripe_plan_id** | Option<**String**> |  | 
**max_monthly_page_loads** | **f64** |  | 
**max_monthly_api_credits** | **f64** |  | 
**max_monthly_small_widgets_credits** | **f64** |  | 
**max_monthly_comments** | **f64** |  | 
**max_concurrent_users** | **f64** |  | 
**max_tenant_users** | **f64** |  | 
**max_sso_users** | **f64** |  | 
**max_moderators** | **f64** |  | 
**max_domains** | **f64** |  | 
**max_white_labeled_tenants** | **f64** |  | 
**max_monthly_event_log_requests** | **f64** |  | 
**has_white_labeling** | **bool** |  | 
**has_debranding** | **bool** |  | 
**has_llm_spam_detection** | **bool** |  | 
**for_who_text** | **String** |  | 
**feature_taglines** | **Vec<String>** |  | 
**has_auditing** | **bool** |  | 
**has_flex_pricing** | **bool** |  | 
**enable_saml** | Option<**bool**> |  | [optional]
**flex_page_load_cost_cents** | Option<**f64**> |  | [optional]
**flex_page_load_unit** | Option<**f64**> |  | [optional]
**flex_comment_cost_cents** | Option<**f64**> |  | [optional]
**flex_comment_unit** | Option<**f64**> |  | [optional]
**flex_sso_user_cost_cents** | Option<**f64**> |  | [optional]
**flex_sso_user_unit** | Option<**f64**> |  | [optional]
**flex_api_credit_cost_cents** | Option<**f64**> |  | [optional]
**flex_api_credit_unit** | Option<**f64**> |  | [optional]
**flex_small_widgets_credit_cost_cents** | Option<**f64**> |  | [optional]
**flex_small_widgets_credit_unit** | Option<**f64**> |  | [optional]
**flex_moderator_cost_cents** | Option<**f64**> |  | [optional]
**flex_moderator_unit** | Option<**f64**> |  | [optional]
**flex_admin_cost_cents** | Option<**f64**> |  | [optional]
**flex_admin_unit** | Option<**f64**> |  | [optional]
**flex_domain_cost_cents** | Option<**f64**> |  | [optional]
**flex_domain_unit** | Option<**f64**> |  | [optional]
**flex_chat_gpt_cost_cents** | Option<**f64**> |  | [optional]
**flex_chat_gpt_unit** | Option<**f64**> |  | [optional]
**flex_minimum_cost_cents** | Option<**f64**> |  | [optional]
**flex_managed_tenant_cost_cents** | Option<**f64**> |  | [optional]
**flex_sso_admin_cost_cents** | Option<**f64**> |  | [optional]
**flex_sso_admin_unit** | Option<**f64**> |  | [optional]
**flex_sso_moderator_cost_cents** | Option<**f64**> |  | [optional]
**flex_sso_moderator_unit** | Option<**f64**> |  | [optional]
**is_sso_billing_monthly_active_users** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


