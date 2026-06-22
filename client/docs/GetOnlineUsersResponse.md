# GetOnlineUsersResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**next_after_user_id** | Option<**String**> |  | [optional]
**next_after_name** | Option<**String**> |  | [optional]
**total_count** | Option<**f64**> |  | [optional]
**anon_count** | Option<**f64**> |  | [optional]
**users** | Option<[**Vec<models::PageUserEntry>**](PageUserEntry.md)> |  | [optional]
**status** | [**models::ApiStatus**](APIStatus.md) |  | 
**reason** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


