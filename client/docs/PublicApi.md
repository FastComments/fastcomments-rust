# \PublicApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_from_comment_public**](PublicApi.md#block_from_comment_public) | **POST** /block-from-comment/{commentId} | 
[**checked_comments_for_blocked**](PublicApi.md#checked_comments_for_blocked) | **GET** /check-blocked-comments | 
[**create_comment_public**](PublicApi.md#create_comment_public) | **POST** /comments/{tenantId} | 
[**delete_comment_public**](PublicApi.md#delete_comment_public) | **DELETE** /comments/{tenantId}/{commentId} | 
[**delete_comment_vote**](PublicApi.md#delete_comment_vote) | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} | 
[**flag_comment_public**](PublicApi.md#flag_comment_public) | **POST** /flag-comment/{commentId} | 
[**get_comment_text**](PublicApi.md#get_comment_text) | **GET** /comments/{tenantId}/{commentId}/text | 
[**get_comment_vote_user_names**](PublicApi.md#get_comment_vote_user_names) | **GET** /comments/{tenantId}/{commentId}/votes | 
[**get_comments_public**](PublicApi.md#get_comments_public) | **GET** /comments/{tenantId} | 
[**get_event_log**](PublicApi.md#get_event_log) | **GET** /event-log/{tenantId} | 
[**get_global_event_log**](PublicApi.md#get_global_event_log) | **GET** /event-log/global/{tenantId} | 
[**get_user_notification_count**](PublicApi.md#get_user_notification_count) | **GET** /user-notifications/get-count | 
[**get_user_notifications**](PublicApi.md#get_user_notifications) | **GET** /user-notifications | 
[**get_user_presence_statuses**](PublicApi.md#get_user_presence_statuses) | **GET** /user-presence-status | 
[**lock_comment**](PublicApi.md#lock_comment) | **POST** /comments/{tenantId}/{commentId}/lock | 
[**pin_comment**](PublicApi.md#pin_comment) | **POST** /comments/{tenantId}/{commentId}/pin | 
[**reset_user_notification_count**](PublicApi.md#reset_user_notification_count) | **POST** /user-notifications/reset-count | 
[**reset_user_notifications**](PublicApi.md#reset_user_notifications) | **POST** /user-notifications/reset | 
[**set_comment_text**](PublicApi.md#set_comment_text) | **POST** /comments/{tenantId}/{commentId}/update-text | 
[**un_block_comment_public**](PublicApi.md#un_block_comment_public) | **DELETE** /block-from-comment/{commentId} | 
[**un_lock_comment**](PublicApi.md#un_lock_comment) | **POST** /comments/{tenantId}/{commentId}/unlock | 
[**un_pin_comment**](PublicApi.md#un_pin_comment) | **POST** /comments/{tenantId}/{commentId}/unpin | 
[**update_user_notification_comment_subscription_status**](PublicApi.md#update_user_notification_comment_subscription_status) | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | 
[**update_user_notification_page_subscription_status**](PublicApi.md#update_user_notification_page_subscription_status) | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | 
[**update_user_notification_status**](PublicApi.md#update_user_notification_status) | **POST** /user-notifications/{notificationId}/mark/{newStatus} | 
[**vote_comment**](PublicApi.md#vote_comment) | **POST** /comments/{tenantId}/{commentId}/vote | 



## block_from_comment_public

> models::BlockFromCommentPublic200Response block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::BlockFromCommentPublic200Response**](BlockFromCommentPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checked_comments_for_blocked

> models::CheckedCommentsForBlocked200Response checked_comments_for_blocked(tenant_id, comment_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_ids** | **String** | A comma separated list of comment ids. | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CheckedCommentsForBlocked200Response**](CheckedCommentsForBlocked_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_comment_public

> models::CreateCommentPublic200Response create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**comment_data** | [**CommentData**](CommentData.md) |  | [required] |
**session_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CreateCommentPublic200Response**](CreateCommentPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_public

> models::DeleteCommentPublic200Response delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |

### Return type

[**models::DeleteCommentPublic200Response**](DeleteCommentPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_vote

> models::DeleteCommentVote200Response delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**vote_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::DeleteCommentVote200Response**](DeleteCommentVote_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_comment_public

> models::FlagCommentPublic200Response flag_comment_public(tenant_id, comment_id, is_flagged, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**is_flagged** | **bool** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::FlagCommentPublic200Response**](FlagCommentPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_text

> models::GetCommentText200Response get_comment_text(tenant_id, comment_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentText200Response**](GetCommentText_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_vote_user_names

> models::GetCommentVoteUserNames200Response get_comment_vote_user_names(tenant_id, comment_id, direction, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**direction** | **f64** | Pass 1 for getting the names of users that up voted, and -1 for the usernames for users that down voted. | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentVoteUserNames200Response**](GetCommentVoteUserNames_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_public

> models::GetCommentsPublic200Response get_comments_public(tenant_id, url_id, page, direction, sso, skip, skip_children, limit, limit_children, count_children, last_gen_date, fetch_page_for_comment_id, include_config, count_all, includei10n, locale, modules, is_crawler, include_notification_count, as_tree, max_tree_depth, use_full_translation_ids, parent_id, search_text, hash_tags, user_id, custom_config_str)


 req tenantId urlId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**direction** | Option<[**SortDirections**](.md)> |  |  |
**sso** | Option<**String**> |  |  |
**skip** | Option<**i32**> |  |  |
**skip_children** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**limit_children** | Option<**i32**> |  |  |
**count_children** | Option<**bool**> |  |  |
**last_gen_date** | Option<**i64**> |  |  |
**fetch_page_for_comment_id** | Option<**String**> |  |  |
**include_config** | Option<**bool**> |  |  |
**count_all** | Option<**bool**> |  |  |
**includei10n** | Option<**bool**> |  |  |
**locale** | Option<**String**> |  |  |
**modules** | Option<**String**> |  |  |
**is_crawler** | Option<**bool**> |  |  |
**include_notification_count** | Option<**bool**> |  |  |
**as_tree** | Option<**bool**> |  |  |
**max_tree_depth** | Option<**i32**> |  |  |
**use_full_translation_ids** | Option<**bool**> |  |  |
**parent_id** | Option<**String**> |  |  |
**search_text** | Option<**String**> |  |  |
**hash_tags** | Option<[**Vec<String>**](String.md)> |  |  |
**user_id** | Option<**String**> |  |  |
**custom_config_str** | Option<**String**> |  |  |

### Return type

[**models::GetCommentsPublic200Response**](GetCommentsPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_log

> models::GetEventLog200Response get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


 req tenantId urlId userIdWS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**user_id_ws** | **String** |  | [required] |
**start_time** | **i64** |  | [required] |
**end_time** | **i64** |  | [required] |

### Return type

[**models::GetEventLog200Response**](GetEventLog_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_event_log

> models::GetEventLog200Response get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


 req tenantId urlId userIdWS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**user_id_ws** | **String** |  | [required] |
**start_time** | **i64** |  | [required] |
**end_time** | **i64** |  | [required] |

### Return type

[**models::GetEventLog200Response**](GetEventLog_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notification_count

> models::GetUserNotificationCount200Response get_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserNotificationCount200Response**](GetUserNotificationCount_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notifications

> models::GetUserNotifications200Response get_user_notifications(tenant_id, page_size, after_id, include_context, after_created_at, unread_only, dm_only, no_dm, include_translations, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**page_size** | Option<**f64**> | Defaults to 20. |  |
**after_id** | Option<**String**> |  |  |
**include_context** | Option<**bool**> |  |  |
**after_created_at** | Option<**f64**> |  |  |
**unread_only** | Option<**bool**> |  |  |
**dm_only** | Option<**bool**> |  |  |
**no_dm** | Option<**bool**> |  |  |
**include_translations** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserNotifications200Response**](GetUserNotifications_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presence_statuses

> models::GetUserPresenceStatuses200Response get_user_presence_statuses(tenant_id, url_id_ws, user_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id_ws** | **String** |  | [required] |
**user_ids** | **String** |  | [required] |

### Return type

[**models::GetUserPresenceStatuses200Response**](GetUserPresenceStatuses_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_comment

> models::LockComment200Response lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::LockComment200Response**](LockComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_comment

> models::PinComment200Response pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PinComment200Response**](PinComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notification_count

> models::ResetUserNotifications200Response reset_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ResetUserNotifications200Response**](ResetUserNotifications_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notifications

> models::ResetUserNotifications200Response reset_user_notifications(tenant_id, after_id, after_created_at, unread_only, dm_only, no_dm, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**after_id** | Option<**String**> |  |  |
**after_created_at** | Option<**f64**> |  |  |
**unread_only** | Option<**bool**> |  |  |
**dm_only** | Option<**bool**> |  |  |
**no_dm** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ResetUserNotifications200Response**](ResetUserNotifications_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_comment_text

> models::SetCommentText200Response set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**comment_text_update_request** | [**CommentTextUpdateRequest**](CommentTextUpdateRequest.md) |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::SetCommentText200Response**](SetCommentText_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_block_comment_public

> models::UnBlockCommentPublic200Response un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UnBlockCommentPublic200Response**](UnBlockCommentPublic_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_lock_comment

> models::LockComment200Response un_lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::LockComment200Response**](LockComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_pin_comment

> models::PinComment200Response un_pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PinComment200Response**](PinComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_comment_subscription_status

> models::UpdateUserNotificationStatus200Response update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso)


Enable or disable notifications for a specific comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |
**opted_in_or_out** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UpdateUserNotificationStatus200Response**](UpdateUserNotificationStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_page_subscription_status

> models::UpdateUserNotificationStatus200Response update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso)


Enable or disable notifications for a page. When users are subscribed to a page, notifications are created for new root comments, and also

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**url** | **String** |  | [required] |
**page_title** | **String** |  | [required] |
**subscribed_or_unsubscribed** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UpdateUserNotificationStatus200Response**](UpdateUserNotificationStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_status

> models::UpdateUserNotificationStatus200Response update_user_notification_status(tenant_id, notification_id, new_status, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |
**new_status** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UpdateUserNotificationStatus200Response**](UpdateUserNotificationStatus_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_comment

> models::VoteComment200Response vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, session_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**vote_body_params** | [**VoteBodyParams**](VoteBodyParams.md) |  | [required] |
**session_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::VoteComment200Response**](VoteComment_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

