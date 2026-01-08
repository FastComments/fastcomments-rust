# ApiTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**email** | Option<**String**> |  | [optional]
**sign_up_date** | **f64** |  | 
**package_id** | **String** |  | 
**payment_frequency** | **f64** |  | 
**billing_info_valid** | **bool** |  | 
**billing_handled_externally** | Option<**bool**> |  | [optional]
**created_by** | **String** |  | 
**is_setup** | **bool** |  | 
**domain_configuration** | [**Vec<models::ApiDomainConfiguration>**](APIDomainConfiguration.md) |  | 
**billing_info** | Option<[**models::BillingInfo**](BillingInfo.md)> |  | [optional]
**stripe_customer_id** | Option<**String**> |  | [optional]
**stripe_subscription_id** | Option<**String**> |  | [optional]
**stripe_plan_id** | Option<**String**> |  | [optional]
**enable_profanity_filter** | **bool** |  | 
**enable_spam_filter** | **bool** |  | 
**last_billing_issue_reminder_date** | Option<**String**> |  | [optional]
**remove_unverified_comments** | Option<**bool**> |  | [optional]
**unverified_comments_tt_lms** | Option<**f64**> |  | [optional]
**comments_require_approval** | Option<**bool**> |  | [optional]
**auto_approve_comment_on_verification** | Option<**bool**> |  | [optional]
**send_profane_to_spam** | Option<**bool**> |  | [optional]
**has_flex_pricing** | Option<**bool**> |  | [optional]
**has_auditing** | Option<**bool**> |  | [optional]
**flex_last_billed_amount** | Option<**f64**> |  | [optional]
**de_anon_ip_addr** | Option<**f64**> |  | [optional]
**meta** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


