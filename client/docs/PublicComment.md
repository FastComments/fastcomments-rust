# PublicComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**date** | **String** |  | 
**user_id** | Option<**String**> |  | [optional]
**anon_user_id** | Option<**String**> |  | [optional]
**commenter_name** | **String** |  | 
**commenter_link** | Option<**String**> |  | [optional]
**comment_html** | **String** |  | 
**parent_id** | Option<**String**> |  | [optional]
**votes** | Option<**i32**> |  | [optional]
**votes_up** | Option<**i32**> |  | [optional]
**votes_down** | Option<**i32**> |  | [optional]
**verified** | **bool** |  | 
**avatar_src** | Option<**String**> |  | [optional]
**is_spam** | Option<**bool**> |  | [optional]
**has_images** | Option<**bool**> |  | [optional]
**is_deleted** | Option<**bool**> |  | [optional]
**is_deleted_user** | Option<**bool**> |  | [optional]
**is_by_admin** | Option<**bool**> |  | [optional]
**is_by_moderator** | Option<**bool**> |  | [optional]
**is_pinned** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**rating** | Option<**f64**> |  | [optional]
**display_label** | Option<**String**> |  | [optional]
**badges** | Option<[**Vec<models::CommentUserBadgeInfo>**](CommentUserBadgeInfo.md)> |  | [optional]
**feedback_ids** | Option<**Vec<String>**> |  | [optional]
**view_count** | Option<**f64**> |  | [optional]
**is_unread** | Option<**bool**> |  | [optional]
**my_vote_id** | Option<**String**> |  | [optional]
**is_voted_down** | Option<**bool**> |  | [optional]
**is_voted_up** | Option<**bool**> |  | [optional]
**has_children** | Option<**bool**> | This is always set when asTree=true | [optional]
**nested_children_count** | Option<**i32**> | The total nested child count included in this response (may be more available w/ pagination) Only set with asTree=true, otherwise this will be null. | [optional]
**child_count** | Option<**i32**> | You must ask the API to count children (with asTree=true&countChildren=true), otherwise this will be null. This will be the complete direct child count, whereas children may only contain a subset based on pagination. | [optional]
**children** | Option<[**Vec<models::PublicComment>**](PublicComment.md)> |  | [optional]
**is_flagged** | Option<**bool**> |  | [optional]
**is_blocked** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


