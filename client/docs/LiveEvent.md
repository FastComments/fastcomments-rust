# LiveEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::LiveEventType**](LiveEventType.md) |  | 
**timestamp** | Option<**i64**> |  | [optional]
**ts** | Option<**i64**> |  | [optional]
**broadcast_id** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**badges** | Option<[**Vec<models::CommentUserBadgeInfo>**](CommentUserBadgeInfo.md)> |  | [optional]
**notification** | Option<[**models::UserNotification**](UserNotification.md)> |  | [optional]
**vote** | Option<[**models::PubSubVote**](PubSubVote.md)> |  | [optional]
**comment** | Option<[**models::PubSubComment**](PubSubComment.md)> |  | [optional]
**extra_info** | Option<[**models::LiveEventExtraInfo**](LiveEvent_extraInfo.md)> |  | [optional]
**config** | Option<[**serde_json::Value**](.md)> |  | [optional]
**is_closed** | Option<**bool**> |  | [optional]
**uj** | Option<**Vec<String>**> |  | [optional]
**ul** | Option<**Vec<String>**> |  | [optional]
**changes** | Option<**std::collections::HashMap<String, f64>**> | Construct a type with a set of properties K of type T | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


