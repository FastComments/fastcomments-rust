# GetFeedPostsPublic200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**my_reacts** | Option<[**std::collections::HashMap<String, std::collections::HashMap<String, bool>>**](std::collections::HashMap.md)> |  | [optional]
**status** | [**models::ImportedApiStatusPeriodFailed**](ImportedAPIStatus.FAILED.md) |  | 
**feed_posts** | [**Vec<models::FeedPost>**](FeedPost.md) |  | 
**user** | Option<[**models::UserSessionInfo**](UserSessionInfo.md)> |  | [optional]
**url_id_ws** | Option<**String**> |  | [optional]
**user_id_ws** | Option<**String**> |  | [optional]
**tenant_id_ws** | Option<**String**> |  | [optional]
**reason** | **String** |  | 
**code** | **String** |  | 
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


