# ModerationFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reviewed** | Option<**bool**> |  | [optional]
**approved** | Option<**bool**> |  | [optional]
**is_spam** | Option<**bool**> |  | [optional]
**is_banned_user** | Option<**bool**> |  | [optional]
**is_locked** | Option<**bool**> |  | [optional]
**flag_count_gt** | Option<**f64**> |  | [optional]
**user_id** | Option<**String**> |  | [optional]
**url_id** | Option<**String**> |  | [optional]
**domain** | Option<**String**> |  | [optional]
**moderation_group_ids** | Option<**Vec<String>**> |  | [optional]
**comment_text_search** | Option<**Vec<String>**> | Text search terms. Each term is matched case-insensitively against the comment text. A term wrapped in quotes means exact phrase match. | [optional]
**exact_comment_text** | Option<**String**> | Set by the `exact=\"...\"` search syntax. The comment text must equal this value exactly (case-sensitive, full-string), as opposed to the substring matching of commentTextSearch. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


