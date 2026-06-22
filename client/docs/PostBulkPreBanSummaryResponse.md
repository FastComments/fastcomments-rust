# PostBulkPreBanSummaryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::ApiStatus**](APIStatus.md) |  | 
**total_related_comment_count** | Option<**i32**> |  | [optional]
**email_domains** | Option<**Vec<String>**> |  | [optional]
**emails** | Option<**Vec<String>**> |  | [optional]
**user_ids** | Option<**Vec<String>**> |  | [optional]
**ip_hashes** | Option<**Vec<String>**> |  | [optional]
**reason** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


