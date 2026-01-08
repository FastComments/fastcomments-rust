# GetVotes200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::ApiStatus**](APIStatus.md) |  | 
**applied_authorized_votes** | [**Vec<models::PublicVote>**](PublicVote.md) |  | 
**applied_anonymous_votes** | [**Vec<models::PublicVote>**](PublicVote.md) |  | 
**pending_votes** | [**Vec<models::PublicVote>**](PublicVote.md) |  | 
**reason** | **String** |  | 
**code** | **String** |  | 
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**i64**> |  | [optional]
**max_character_length** | Option<**i32**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


