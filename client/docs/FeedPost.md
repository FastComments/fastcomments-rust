# FeedPost

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**tenant_id** | **String** |  | 
**title** | Option<**String**> |  | [optional]
**from_user_id** | Option<**String**> |  | [optional]
**from_user_display_name** | Option<**String**> |  | [optional]
**from_user_avatar** | Option<**String**> |  | [optional]
**from_ip_hash** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**weight** | Option<**f64**> |  | [optional]
**meta** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | [optional]
**content_html** | Option<**String**> |  | [optional]
**media** | Option<[**Vec<models::FeedPostMediaItem>**](FeedPostMediaItem.md)> |  | [optional]
**links** | Option<[**Vec<models::FeedPostLink>**](FeedPostLink.md)> |  | [optional]
**created_at** | **String** |  | 
**reacts** | Option<**std::collections::HashMap<String, i32>**> |  | [optional]
**comment_count** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


