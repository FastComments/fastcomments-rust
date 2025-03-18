# CommentData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date** | **f64** |  | 
**local_date_string** | Option<**String**> |  | [optional]
**local_date_hours** | Option<**f64**> |  | [optional]
**commenter_name** | **String** |  | 
**commenter_email** | Option<**String**> |  | [optional]
**commenter_link** | Option<**String**> |  | [optional]
**comment** | **String** |  | 
**product_id** | Option<**f64**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**avatar_src** | Option<**String**> |  | [optional]
**parent_id** | Option<**String**> |  | [optional]
**mentions** | Option<[**Vec<models::CommentUserMentionInfo>**](CommentUserMentionInfo.md)> |  | [optional]
**hash_tags** | Option<[**Vec<models::CommentUserHashTagInfo>**](CommentUserHashTagInfo.md)> |  | [optional]
**page_title** | Option<**String**> |  | [optional]
**is_from_my_account_page** | Option<**bool**> |  | [optional]
**url** | **String** |  | 
**url_id** | **String** |  | 
**meta** | Option<[**serde_json::Value**](.md)> |  | [optional]
**moderation_group_ids** | Option<**Vec<String>**> |  | [optional]
**rating** | Option<**f64**> |  | [optional]
**from_offline_restore** | Option<**bool**> |  | [optional]
**autoplay_delay_ms** | Option<**f64**> |  | [optional]
**feedback_ids** | Option<**Vec<String>**> |  | [optional]
**question_values** | Option<[**std::collections::HashMap<String, models::RecordStringStringOrNumberValue>**](Record_string_string_or_number__value.md)> | Construct a type with a set of properties K of type T | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


