# GetCommentsPublic200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status_code** | Option<**i32**> |  | [optional]
**status** | [**models::ImportedApiStatusPeriodFailed**](ImportedAPIStatus.FAILED.md) |  | 
**code** | **String** |  | 
**reason** | **String** |  | 
**translated_warning** | Option<**String**> |  | [optional]
**comments** | [**Vec<models::PublicComment>**](PublicComment.md) |  | 
**user** | Option<[**models::UserSessionInfo**](UserSessionInfo.md)> |  | 
**url_id_clean** | Option<**String**> |  | [optional]
**last_gen_date** | Option<**i64**> |  | [optional]
**includes_past_pages** | Option<**bool**> |  | [optional]
**is_demo** | Option<**bool**> |  | [optional]
**comment_count** | Option<**i32**> |  | [optional]
**is_site_admin** | Option<**bool**> |  | [optional]
**has_billing_issue** | Option<**bool**> |  | [optional]
**module_data** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Construct a type with a set of properties K of type T | [optional]
**page_number** | **i32** |  | 
**is_white_labeled** | Option<**bool**> |  | [optional]
**is_prod** | Option<**bool**> |  | [optional]
**is_crawler** | Option<**bool**> |  | [optional]
**notification_count** | Option<**i32**> |  | [optional]
**has_more** | Option<**bool**> |  | [optional]
**is_closed** | Option<**bool**> |  | [optional]
**presence_poll_state** | Option<**i32**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]
**url_id_ws** | Option<**String**> |  | [optional]
**user_id_ws** | Option<**String**> |  | [optional]
**tenant_id_ws** | Option<**String**> |  | [optional]
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


