# PickFCommentPeriodApiCommentFieldsKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**url_id** | **String** |  | 
**url** | **String** |  | 
**page_title** | Option<**String**> |  | [optional]
**date** | **String** |  | 
**tenant_id** | **String** |  | 
**url_id_raw** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**anon_user_id** | Option<**String**> |  | [optional]
**commenter_email** | Option<**String**> |  | [optional]
**commenter_name** | **String** |  | 
**commenter_link** | Option<**String**> |  | [optional]
**comment** | **String** |  | 
**comment_html** | **String** |  | 
**parent_id** | Option<**String**> |  | [optional]
**local_date_string** | Option<**String**> |  | [optional]
**local_date_hours** | Option<**i32**> |  | [optional]
**votes** | Option<**i32**> |  | [optional]
**votes_up** | Option<**i32**> |  | [optional]
**votes_down** | Option<**i32**> |  | [optional]
**expire_at** | Option<**String**> |  | [optional]
**verified** | **bool** |  | 
**verified_date** | Option<**String**> |  | [optional]
**notification_sent_for_parent** | Option<**bool**> |  | [optional]
**notification_sent_for_parent_tenant** | Option<**bool**> |  | [optional]
**reviewed** | Option<**bool**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**external_parent_id** | Option<**String**> |  | [optional]
**avatar_src** | Option<**String**> |  | [optional]
**is_spam** | Option<**bool**> |  | [optional]
**ai_determined_spam** | Option<**bool**> |  | [optional]
**has_images** | Option<**bool**> |  | [optional]
**has_links** | Option<**bool**> |  | [optional]
**has_code** | Option<**bool**> |  | [optional]
**approved** | **bool** |  | 
**locale** | **String** |  | 
**is_deleted** | Option<**bool**> |  | [optional]
**is_deleted_user** | Option<**bool**> |  | [optional]
**is_by_admin** | Option<**bool**> |  | [optional]
**is_by_moderator** | Option<**bool**> |  | [optional]
**is_pinned** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**flag_count** | Option<**i32**> |  | [optional]
**rating** | Option<**f64**> |  | [optional]
**display_label** | Option<**String**> |  | [optional]
**from_product_id** | Option<**i32**> |  | [optional]
**meta** | Option<[**models::PickFCommentApiCommentFieldsKeysMeta**](Pick_FComment_APICommentFieldsKeys__meta.md)> |  | [optional]
**mentions** | Option<[**Vec<models::CommentUserMentionInfo>**](CommentUserMentionInfo.md)> |  | [optional]
**hash_tags** | Option<[**Vec<models::CommentUserHashTagInfo>**](CommentUserHashTagInfo.md)> |  | [optional]
**badges** | Option<[**Vec<models::CommentUserBadgeInfo>**](CommentUserBadgeInfo.md)> |  | [optional]
**domain** | Option<**String**> |  | [optional]
**moderation_group_ids** | Option<**Vec<String>**> |  | [optional]
**feedback_ids** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


