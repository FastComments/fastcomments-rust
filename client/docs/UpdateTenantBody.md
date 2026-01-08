# UpdateTenantBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**sign_up_date** | Option<**f64**> |  | [optional]
**package_id** | Option<**String**> |  | [optional]
**payment_frequency** | Option<**f64**> |  | [optional]
**billing_info_valid** | Option<**bool**> |  | [optional]
**billing_handled_externally** | Option<**bool**> |  | [optional]
**created_by** | Option<**String**> |  | [optional]
**is_setup** | Option<**bool**> |  | [optional]
**domain_configuration** | Option<[**Vec<models::ApiDomainConfiguration>**](APIDomainConfiguration.md)> |  | [optional]
**billing_info** | Option<[**models::BillingInfo**](BillingInfo.md)> |  | [optional]
**stripe_customer_id** | Option<**String**> |  | [optional]
**stripe_subscription_id** | Option<**String**> |  | [optional]
**stripe_plan_id** | Option<**String**> |  | [optional]
**enable_profanity_filter** | Option<**bool**> |  | [optional]
**enable_spam_filter** | Option<**bool**> |  | [optional]
**remove_unverified_comments** | Option<**bool**> |  | [optional]
**unverified_comments_tt_lms** | Option<**f64**> |  | [optional]
**comments_require_approval** | Option<**bool**> |  | [optional]
**auto_approve_comment_on_verification** | Option<**bool**> |  | [optional]
**send_profane_to_spam** | Option<**bool**> |  | [optional]
**de_anon_ip_addr** | Option<**f64**> |  | [optional]
**meta** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | [optional]
**managed_by_tenant_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


