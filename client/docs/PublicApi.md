# \PublicApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_from_comment_public**](PublicApi.md#block_from_comment_public) | **POST** /block-from-comment/{commentId} | 
[**checked_comments_for_blocked**](PublicApi.md#checked_comments_for_blocked) | **GET** /check-blocked-comments | 
[**create_comment_public**](PublicApi.md#create_comment_public) | **POST** /comments/{tenantId} | 
[**create_feed_post_public**](PublicApi.md#create_feed_post_public) | **POST** /feed-posts/{tenantId} | 
[**create_v1_page_react**](PublicApi.md#create_v1_page_react) | **POST** /page-reacts/v1/likes/{tenantId} | 
[**create_v2_page_react**](PublicApi.md#create_v2_page_react) | **POST** /page-reacts/v2/{tenantId} | 
[**delete_comment_public**](PublicApi.md#delete_comment_public) | **DELETE** /comments/{tenantId}/{commentId} | 
[**delete_comment_vote**](PublicApi.md#delete_comment_vote) | **DELETE** /comments/{tenantId}/{commentId}/vote/{voteId} | 
[**delete_feed_post_public**](PublicApi.md#delete_feed_post_public) | **DELETE** /feed-posts/{tenantId}/{postId} | 
[**delete_v1_page_react**](PublicApi.md#delete_v1_page_react) | **DELETE** /page-reacts/v1/likes/{tenantId} | 
[**delete_v2_page_react**](PublicApi.md#delete_v2_page_react) | **DELETE** /page-reacts/v2/{tenantId} | 
[**flag_comment_public**](PublicApi.md#flag_comment_public) | **POST** /flag-comment/{commentId} | 
[**get_comment_text**](PublicApi.md#get_comment_text) | **GET** /comments/{tenantId}/{commentId}/text | 
[**get_comment_vote_user_names**](PublicApi.md#get_comment_vote_user_names) | **GET** /comments/{tenantId}/{commentId}/votes | 
[**get_comments_for_user**](PublicApi.md#get_comments_for_user) | **GET** /comments-for-user | 
[**get_comments_public**](PublicApi.md#get_comments_public) | **GET** /comments/{tenantId} | 
[**get_event_log**](PublicApi.md#get_event_log) | **GET** /event-log/{tenantId} | 
[**get_feed_posts_public**](PublicApi.md#get_feed_posts_public) | **GET** /feed-posts/{tenantId} | 
[**get_feed_posts_stats**](PublicApi.md#get_feed_posts_stats) | **GET** /feed-posts/{tenantId}/stats | 
[**get_gif_large**](PublicApi.md#get_gif_large) | **GET** /gifs/get-large/{tenantId} | 
[**get_gifs_search**](PublicApi.md#get_gifs_search) | **GET** /gifs/search/{tenantId} | 
[**get_gifs_trending**](PublicApi.md#get_gifs_trending) | **GET** /gifs/trending/{tenantId} | 
[**get_global_event_log**](PublicApi.md#get_global_event_log) | **GET** /event-log/global/{tenantId} | 
[**get_offline_users**](PublicApi.md#get_offline_users) | **GET** /pages/{tenantId}/users/offline | 
[**get_online_users**](PublicApi.md#get_online_users) | **GET** /pages/{tenantId}/users/online | 
[**get_pages_public**](PublicApi.md#get_pages_public) | **GET** /pages/{tenantId} | 
[**get_translations**](PublicApi.md#get_translations) | **GET** /translations/{namespace}/{component} | 
[**get_user_notification_count**](PublicApi.md#get_user_notification_count) | **GET** /user-notifications/get-count | 
[**get_user_notifications**](PublicApi.md#get_user_notifications) | **GET** /user-notifications | 
[**get_user_presence_statuses**](PublicApi.md#get_user_presence_statuses) | **GET** /user-presence-status | 
[**get_user_reacts_public**](PublicApi.md#get_user_reacts_public) | **GET** /feed-posts/{tenantId}/user-reacts | 
[**get_users_info**](PublicApi.md#get_users_info) | **GET** /pages/{tenantId}/users/info | 
[**get_v1_page_likes**](PublicApi.md#get_v1_page_likes) | **GET** /page-reacts/v1/likes/{tenantId} | 
[**get_v2_page_react_users**](PublicApi.md#get_v2_page_react_users) | **GET** /page-reacts/v2/{tenantId}/list | 
[**get_v2_page_reacts**](PublicApi.md#get_v2_page_reacts) | **GET** /page-reacts/v2/{tenantId} | 
[**lock_comment**](PublicApi.md#lock_comment) | **POST** /comments/{tenantId}/{commentId}/lock | 
[**logout_public**](PublicApi.md#logout_public) | **PUT** /auth/logout | 
[**pin_comment**](PublicApi.md#pin_comment) | **POST** /comments/{tenantId}/{commentId}/pin | 
[**react_feed_post_public**](PublicApi.md#react_feed_post_public) | **POST** /feed-posts/{tenantId}/react/{postId} | 
[**reset_user_notification_count**](PublicApi.md#reset_user_notification_count) | **POST** /user-notifications/reset-count | 
[**reset_user_notifications**](PublicApi.md#reset_user_notifications) | **POST** /user-notifications/reset | 
[**search_users**](PublicApi.md#search_users) | **GET** /user-search/{tenantId} | 
[**set_comment_text**](PublicApi.md#set_comment_text) | **POST** /comments/{tenantId}/{commentId}/update-text | 
[**un_block_comment_public**](PublicApi.md#un_block_comment_public) | **DELETE** /block-from-comment/{commentId} | 
[**un_lock_comment**](PublicApi.md#un_lock_comment) | **POST** /comments/{tenantId}/{commentId}/unlock | 
[**un_pin_comment**](PublicApi.md#un_pin_comment) | **POST** /comments/{tenantId}/{commentId}/unpin | 
[**update_feed_post_public**](PublicApi.md#update_feed_post_public) | **PUT** /feed-posts/{tenantId}/{postId} | 
[**update_user_notification_comment_subscription_status**](PublicApi.md#update_user_notification_comment_subscription_status) | **POST** /user-notifications/{notificationId}/mark-opted/{optedInOrOut} | 
[**update_user_notification_page_subscription_status**](PublicApi.md#update_user_notification_page_subscription_status) | **POST** /user-notifications/set-subscription-state/{subscribedOrUnsubscribed} | 
[**update_user_notification_status**](PublicApi.md#update_user_notification_status) | **POST** /user-notifications/{notificationId}/mark/{newStatus} | 
[**upload_image**](PublicApi.md#upload_image) | **POST** /upload-image/{tenantId} | 
[**vote_comment**](PublicApi.md#vote_comment) | **POST** /comments/{tenantId}/{commentId}/vote | 



## block_from_comment_public

> models::BlockFromCommentPublicResponse block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::BlockFromCommentPublicResponse**](BlockFromCommentPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checked_comments_for_blocked

> models::CheckedCommentsForBlockedResponse checked_comments_for_blocked(tenant_id, comment_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_ids** | **String** | A comma separated list of comment ids. | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CheckedCommentsForBlockedResponse**](CheckedCommentsForBlockedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_comment_public

> models::CreateCommentPublicResponse create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id, sso)


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

[**models::CreateCommentPublicResponse**](CreateCommentPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed_post_public

> models::CreateFeedPostPublicResponse create_feed_post_public(tenant_id, create_feed_post_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_feed_post_params** | [**CreateFeedPostParams**](CreateFeedPostParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CreateFeedPostPublicResponse**](CreateFeedPostPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_v1_page_react

> models::CreateV1PageReactResponse create_v1_page_react(tenant_id, url_id, title)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**title** | Option<**String**> |  |  |

### Return type

[**models::CreateV1PageReactResponse**](CreateV1PageReactResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_v2_page_react

> models::CreateV2PageReactResponse create_v2_page_react(tenant_id, url_id, id, title)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**title** | Option<**String**> |  |  |

### Return type

[**models::CreateV2PageReactResponse**](CreateV2PageReactResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_public

> models::DeleteCommentPublicResponse delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::DeleteCommentPublicResponse**](DeleteCommentPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_vote

> models::DeleteCommentVoteResponse delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key, sso)


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

[**models::DeleteCommentVoteResponse**](DeleteCommentVoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_feed_post_public

> models::DeleteFeedPostPublicResponse delete_feed_post_public(tenant_id, post_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::DeleteFeedPostPublicResponse**](DeleteFeedPostPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_v1_page_react

> models::DeleteV1PageReactResponse delete_v1_page_react(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::DeleteV1PageReactResponse**](DeleteV1PageReactResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_v2_page_react

> models::DeleteV2PageReactResponse delete_v2_page_react(tenant_id, url_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::DeleteV2PageReactResponse**](DeleteV2PageReactResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_comment_public

> models::FlagCommentPublicResponse flag_comment_public(tenant_id, comment_id, is_flagged, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**is_flagged** | **bool** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::FlagCommentPublicResponse**](FlagCommentPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_text

> models::GetCommentTextResponse1 get_comment_text(tenant_id, comment_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentTextResponse1**](GetCommentTextResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_vote_user_names

> models::GetCommentVoteUserNamesResponse get_comment_vote_user_names(tenant_id, comment_id, dir, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**dir** | **i32** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentVoteUserNamesResponse**](GetCommentVoteUserNamesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_for_user

> models::GetCommentsForUserResponse1 get_comments_for_user(user_id, direction, replies_to_user_id, page, includei10n, locale, is_crawler)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |
**direction** | Option<[**SortDirections**](SortDirections.md)> |  |  |
**replies_to_user_id** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**includei10n** | Option<**bool**> |  |  |
**locale** | Option<**String**> |  |  |
**is_crawler** | Option<**bool**> |  |  |

### Return type

[**models::GetCommentsForUserResponse1**](GetCommentsForUserResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_public

> models::GetCommentsPublicResponse get_comments_public(tenant_id, url_id, page, direction, sso, skip, skip_children, limit, limit_children, count_children, fetch_page_for_comment_id, include_config, count_all, includei10n, locale, modules, is_crawler, include_notification_count, as_tree, max_tree_depth, use_full_translation_ids, parent_id, search_text, hash_tags, user_id, custom_config_str, after_comment_id, before_comment_id)


 req tenantId urlId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**direction** | Option<[**SortDirections**](SortDirections.md)> |  |  |
**sso** | Option<**String**> |  |  |
**skip** | Option<**i32**> |  |  |
**skip_children** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**limit_children** | Option<**i32**> |  |  |
**count_children** | Option<**bool**> |  |  |
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
**after_comment_id** | Option<**String**> |  |  |
**before_comment_id** | Option<**String**> |  |  |

### Return type

[**models::GetCommentsPublicResponse**](GetCommentsPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_log

> models::GetEventLogResponse1 get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


 req tenantId urlId userIdWS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**user_id_ws** | **String** |  | [required] |
**start_time** | **i64** |  | [required] |
**end_time** | Option<**i64**> |  |  |

### Return type

[**models::GetEventLogResponse1**](GetEventLogResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts_public

> models::GetFeedPostsPublicResponse get_feed_posts_public(tenant_id, after_id, limit, tags, sso, is_crawler, include_user_info)


 req tenantId afterId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**after_id** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |
**sso** | Option<**String**> |  |  |
**is_crawler** | Option<**bool**> |  |  |
**include_user_info** | Option<**bool**> |  |  |

### Return type

[**models::GetFeedPostsPublicResponse**](GetFeedPostsPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts_stats

> models::GetFeedPostsStatsResponse get_feed_posts_stats(tenant_id, post_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_ids** | [**Vec<String>**](String.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetFeedPostsStatsResponse**](GetFeedPostsStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gif_large

> models::GetGifLargeResponse get_gif_large(tenant_id, large_internal_url_sanitized)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**large_internal_url_sanitized** | **String** |  | [required] |

### Return type

[**models::GetGifLargeResponse**](GetGifLargeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gifs_search

> models::GetGifsSearchResponse get_gifs_search(tenant_id, search, locale, rating, page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**search** | **String** |  | [required] |
**locale** | Option<**String**> |  |  |
**rating** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |

### Return type

[**models::GetGifsSearchResponse**](GetGifsSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gifs_trending

> models::GetGifsTrendingResponse get_gifs_trending(tenant_id, locale, rating, page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**locale** | Option<**String**> |  |  |
**rating** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |

### Return type

[**models::GetGifsTrendingResponse**](GetGifsTrendingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_global_event_log

> models::GetGlobalEventLogResponse get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


 req tenantId urlId userIdWS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**user_id_ws** | **String** |  | [required] |
**start_time** | **i64** |  | [required] |
**end_time** | Option<**i64**> |  |  |

### Return type

[**models::GetGlobalEventLogResponse**](GetGlobalEventLogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_offline_users

> models::GetOfflineUsersResponse get_offline_users(tenant_id, url_id, after_name, after_user_id)


Past commenters on the page who are NOT currently online. Sorted by displayName. Use this after exhausting /users/online to render a \"Members\" section. Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** | Page URL identifier (cleaned server-side). | [required] |
**after_name** | Option<**String**> | Cursor: pass nextAfterName from the previous response. |  |
**after_user_id** | Option<**String**> | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |  |

### Return type

[**models::GetOfflineUsersResponse**](GetOfflineUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_online_users

> models::GetOnlineUsersResponse get_online_users(tenant_id, url_id, after_name, after_user_id)


Currently-online viewers of a page: people whose websocket session is subscribed to the page right now. Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** | Page URL identifier (cleaned server-side). | [required] |
**after_name** | Option<**String**> | Cursor: pass nextAfterName from the previous response. |  |
**after_user_id** | Option<**String**> | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |  |

### Return type

[**models::GetOnlineUsersResponse**](GetOnlineUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pages_public

> models::GetPagesPublicResponse get_pages_public(tenant_id, cursor, limit, q, sort_by, has_comments)


List pages for a tenant. Used by the FChat desktop client to populate its room list. Requires `enableFChat` to be true on the resolved custom config for each page. Pages that require SSO are filtered against the requesting user's group access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**cursor** | Option<**String**> | Opaque pagination cursor returned as `nextCursor` from a prior request. Tied to the same `sortBy`. |  |
**limit** | Option<**i32**> | 1..200, default 50 |  |
**q** | Option<**String**> | Optional case-insensitive title prefix filter. |  |
**sort_by** | Option<[**PagesSortBy**](PagesSortBy.md)> | Sort order. `updatedAt` (default, newest first), `commentCount` (most comments first), or `title` (alphabetical). |  |
**has_comments** | Option<**bool**> | If true, only return pages with at least one comment. |  |

### Return type

[**models::GetPagesPublicResponse**](GetPagesPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translations

> models::GetTranslationsResponse1 get_translations(namespace, component, locale, use_full_translation_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**component** | **String** |  | [required] |
**locale** | Option<**String**> |  |  |
**use_full_translation_ids** | Option<**bool**> |  |  |

### Return type

[**models::GetTranslationsResponse1**](GetTranslationsResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notification_count

> models::GetUserNotificationCountResponse1 get_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserNotificationCountResponse1**](GetUserNotificationCountResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notifications

> models::GetUserNotificationsResponse get_user_notifications(tenant_id, url_id, page_size, after_id, include_context, after_created_at, unread_only, dm_only, no_dm, include_translations, include_tenant_notifications, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | Option<**String**> | Used to determine whether the current page is subscribed. |  |
**page_size** | Option<**i32**> |  |  |
**after_id** | Option<**String**> |  |  |
**include_context** | Option<**bool**> |  |  |
**after_created_at** | Option<**i64**> |  |  |
**unread_only** | Option<**bool**> |  |  |
**dm_only** | Option<**bool**> |  |  |
**no_dm** | Option<**bool**> |  |  |
**include_translations** | Option<**bool**> |  |  |
**include_tenant_notifications** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserNotificationsResponse**](GetUserNotificationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presence_statuses

> models::GetUserPresenceStatusesResponse1 get_user_presence_statuses(tenant_id, url_id_ws, user_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id_ws** | **String** |  | [required] |
**user_ids** | **String** |  | [required] |

### Return type

[**models::GetUserPresenceStatusesResponse1**](GetUserPresenceStatusesResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_reacts_public

> models::GetUserReactsPublicResponse get_user_reacts_public(tenant_id, post_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserReactsPublicResponse**](GetUserReactsPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_info

> models::GetUsersInfoResponse get_users_info(tenant_id, ids)


Bulk user info for a tenant. Given userIds, return display info from User / SSOUser. Used by the comment widget to enrich users that just appeared via a presence event. No page context: privacy is enforced uniformly (private profiles are masked).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**ids** | **String** | Comma-delimited userIds. | [required] |

### Return type

[**models::GetUsersInfoResponse**](GetUsersInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_page_likes

> models::GetV1PageLikesResponse get_v1_page_likes(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetV1PageLikesResponse**](GetV1PageLikesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_page_react_users

> models::GetV2PageReactUsersResponse1 get_v2_page_react_users(tenant_id, url_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetV2PageReactUsersResponse1**](GetV2PageReactUsersResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_page_reacts

> models::GetV2PageReactsResponse get_v2_page_reacts(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetV2PageReactsResponse**](GetV2PageReactsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_comment

> models::LockCommentResponse lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::LockCommentResponse**](LockCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_public

> models::ApiEmptyResponse logout_public()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_comment

> models::PinCommentResponse pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PinCommentResponse**](PinCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## react_feed_post_public

> models::ReactFeedPostPublicResponse react_feed_post_public(tenant_id, post_id, react_body_params, is_undo, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_id** | **String** |  | [required] |
**react_body_params** | [**ReactBodyParams**](ReactBodyParams.md) |  | [required] |
**is_undo** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ReactFeedPostPublicResponse**](ReactFeedPostPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notification_count

> models::ResetUserNotificationCountResponse reset_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ResetUserNotificationCountResponse**](ResetUserNotificationCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notifications

> models::ResetUserNotificationsResponse1 reset_user_notifications(tenant_id, after_id, after_created_at, unread_only, dm_only, no_dm, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**after_id** | Option<**String**> |  |  |
**after_created_at** | Option<**i64**> |  |  |
**unread_only** | Option<**bool**> |  |  |
**dm_only** | Option<**bool**> |  |  |
**no_dm** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ResetUserNotificationsResponse1**](ResetUserNotificationsResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> models::SearchUsersResponse1 search_users(tenant_id, url_id, username_starts_with, mention_group_ids, sso, search_section)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**username_starts_with** | Option<**String**> |  |  |
**mention_group_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**sso** | Option<**String**> |  |  |
**search_section** | Option<**String**> |  |  |

### Return type

[**models::SearchUsersResponse1**](SearchUsersResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_comment_text

> models::SetCommentTextResponse1 set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key, sso)


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

[**models::SetCommentTextResponse1**](SetCommentTextResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_block_comment_public

> models::UnBlockCommentPublicResponse un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UnBlockCommentPublicResponse**](UnBlockCommentPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_lock_comment

> models::UnLockCommentResponse un_lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UnLockCommentResponse**](UnLockCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_pin_comment

> models::UnPinCommentResponse un_pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UnPinCommentResponse**](UnPinCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feed_post_public

> models::UpdateFeedPostPublicResponse update_feed_post_public(tenant_id, post_id, update_feed_post_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_id** | **String** |  | [required] |
**update_feed_post_params** | [**UpdateFeedPostParams**](UpdateFeedPostParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UpdateFeedPostPublicResponse**](UpdateFeedPostPublicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_comment_subscription_status

> models::UpdateUserNotificationCommentSubscriptionStatusResponse update_user_notification_comment_subscription_status(tenant_id, notification_id, opted_in_or_out, comment_id, sso)


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

[**models::UpdateUserNotificationCommentSubscriptionStatusResponse**](UpdateUserNotificationCommentSubscriptionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_page_subscription_status

> models::UpdateUserNotificationPageSubscriptionStatusResponse update_user_notification_page_subscription_status(tenant_id, url_id, url, page_title, subscribed_or_unsubscribed, sso)


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

[**models::UpdateUserNotificationPageSubscriptionStatusResponse**](UpdateUserNotificationPageSubscriptionStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_notification_status

> models::UpdateUserNotificationStatusResponse update_user_notification_status(tenant_id, notification_id, new_status, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**notification_id** | **String** |  | [required] |
**new_status** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UpdateUserNotificationStatusResponse**](UpdateUserNotificationStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_image

> models::UploadImageResponse upload_image(tenant_id, file, size_preset, url_id)


Upload and resize an image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**size_preset** | Option<[**SizePreset**](SizePreset.md)> | Size preset: \"Default\" (1000x1000px) or \"CrossPlatform\" (creates sizes for popular devices) |  |
**url_id** | Option<**String**> | Page id that upload is happening from, to configure |  |

### Return type

[**models::UploadImageResponse**](UploadImageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vote_comment

> models::VoteCommentResponse vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, session_id, sso)


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

[**models::VoteCommentResponse**](VoteCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

