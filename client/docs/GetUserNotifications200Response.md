# GetUserNotifications200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**translations** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | [optional]
**is_subscribed** | **bool** |  | 
**has_more** | **bool** |  | 
**notifications** | [**Vec<models::RenderableUserNotification>**](RenderableUserNotification.md) |  | 
**status** | [**models::ImportedApiStatusPeriodFailed**](ImportedAPIStatus.FAILED.md) |  | 
**reason** | **String** |  | 
**code** | **String** |  | 
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


