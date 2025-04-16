# FComment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**tenant_id** | **String** |  | 
**url_id** | **String** |  | 
**url_id_raw** | Option<**String**> |  | [optional]
**url** | **String** |  | 
**page_title** | Option<**String**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**anon_user_id** | Option<**String**> |  | [optional]
**commenter_email** | Option<**String**> |  | [optional]
**commenter_name** | **String** |  | 
**commenter_link** | Option<**String**> |  | [optional]
**comment** | **String** |  | 
**comment_html** | **String** |  | 
**parent_id** | Option<**String**> |  | [optional]
**date** | Option<**String**> |  | 
**local_date_string** | Option<**String**> |  | [optional]
**local_date_hours** | Option<**i32**> |  | [optional]
**votes** | Option<**i32**> |  | [optional]
**votes_up** | Option<**i32**> |  | [optional]
**votes_down** | Option<**i32**> |  | [optional]
**expire_at** | Option<**String**> |  | [optional]
**verified** | **bool** |  | 
**verified_date** | Option<**String**> |  | [optional]
**verification_id** | Option<**String**> |  | [optional]
**notification_sent_for_parent** | Option<**bool**> |  | [optional]
**notification_sent_for_parent_tenant** | Option<**bool**> |  | [optional]
**reviewed** | Option<**bool**> |  | [optional]
**imported** | Option<**bool**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**external_parent_id** | Option<**String**> |  | [optional]
**avatar_src** | Option<**String**> |  | [optional]
**is_spam** | Option<**bool**> |  | [optional]
**ai_determined_spam** | Option<**bool**> |  | [optional]
**has_images** | Option<**bool**> |  | [optional]
**page_number** | Option<**i32**> |  | [optional]
**page_number_of** | Option<**i32**> |  | [optional]
**page_number_nf** | Option<**i32**> |  | [optional]
**has_links** | Option<**bool**> |  | [optional]
**has_code** | Option<**bool**> |  | [optional]
**approved** | **bool** |  | 
**locale** | Option<**String**> |  | 
**is_deleted** | Option<**bool**> |  | [optional]
**is_deleted_user** | Option<**bool**> |  | [optional]
**is_banned_user** | Option<**bool**> |  | [optional]
**is_by_admin** | Option<**bool**> |  | [optional]
**is_by_moderator** | Option<**bool**> |  | [optional]
**is_pinned** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**flag_count** | Option<**i32**> |  | [optional]
**rating** | Option<**f64**> |  | [optional]
**display_label** | Option<**String**> |  | [optional]
**from_product_id** | Option<**i32**> |  | [optional]
**meta** | Option<[**models::FCommentMeta**](FComment_meta.md)> |  | [optional]
**ip_hash** | Option<**String**> |  | [optional]
**mentions** | Option<[**Vec<models::CommentUserMentionInfo>**](CommentUserMentionInfo.md)> |  | [optional]
**hash_tags** | Option<[**Vec<models::CommentUserHashTagInfo>**](CommentUserHashTagInfo.md)> |  | [optional]
**badges** | Option<[**Vec<models::CommentUserBadgeInfo>**](CommentUserBadgeInfo.md)> |  | [optional]
**domain** | Option<**String**> |  | [optional]
**veteran_badge_processed** | Option<**String**> |  | [optional]
**moderation_group_ids** | Option<**Vec<String>**> |  | [optional]
**did_process_badges** | Option<**bool**> |  | [optional]
**from_offline_restore** | Option<**bool**> |  | [optional]
**autoplay_job_id** | Option<**String**> |  | [optional]
**autoplay_delay_ms** | Option<**i64**> |  | [optional]
**feedback_ids** | Option<**Vec<String>**> |  | [optional]
**logs** | Option<[**Vec<Vec<serde_json::Value>>**](Vec.md)> |  | [optional]
**group_ids** | Option<**Vec<String>**> |  | [optional]
**view_count** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


