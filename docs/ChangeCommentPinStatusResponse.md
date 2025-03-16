# ChangeCommentPinStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment_positions** | [**std::collections::HashMap<String, models::RecordStringBeforeStringOrNullAfterStringOrNullValue>**](Record_string__before_string_or_null__after_string_or_null___value.md) | Construct a type with a set of properties K of type T | 
**status** | [**models::ImportedApiStatusPeriodFailed**](ImportedAPIStatus.FAILED.md) |  | 
**status_code** | Option<**f64**> |  | [optional]
**reason** | **String** |  | 
**code** | **String** |  | 
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**f64**> |  | [optional]
**max_character_length** | Option<**f64**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


