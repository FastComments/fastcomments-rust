# GetUserNotificationsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**translations** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | [optional]
**is_subscribed** | Option<**bool**> |  | [optional]
**has_more** | Option<**bool**> |  | [optional]
**notifications** | Option<[**Vec<models::RenderableUserNotification>**](RenderableUserNotification.md)> |  | [optional]
**status** | [**models::ApiStatus**](APIStatus.md) |  | 
**reason** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


