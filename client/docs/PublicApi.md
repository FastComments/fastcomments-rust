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

> models::BlockSuccess block_from_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::BlockSuccess**](BlockSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checked_comments_for_blocked

> models::CheckBlockedCommentsResponse checked_comments_for_blocked(tenant_id, comment_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_ids** | **String** | A comma separated list of comment ids. | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CheckBlockedCommentsResponse**](CheckBlockedCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_comment_public

> models::SaveCommentsResponseWithPresence create_comment_public(tenant_id, url_id, broadcast_id, comment_data, session_id, sso)


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

[**models::SaveCommentsResponseWithPresence**](SaveCommentsResponseWithPresence.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed_post_public

> models::CreateFeedPostResponse create_feed_post_public(tenant_id, create_feed_post_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_feed_post_params** | [**CreateFeedPostParams**](CreateFeedPostParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CreateFeedPostResponse**](CreateFeedPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_v1_page_react

> models::CreateV1PageReact create_v1_page_react(tenant_id, url_id, title)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**title** | Option<**String**> |  |  |

### Return type

[**models::CreateV1PageReact**](CreateV1PageReact.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_v2_page_react

> models::CreateV1PageReact create_v2_page_react(tenant_id, url_id, id, title)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**title** | Option<**String**> |  |  |

### Return type

[**models::CreateV1PageReact**](CreateV1PageReact.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_public

> models::PublicApiDeleteCommentResponse delete_comment_public(tenant_id, comment_id, broadcast_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PublicApiDeleteCommentResponse**](PublicAPIDeleteCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment_vote

> models::VoteDeleteResponse delete_comment_vote(tenant_id, comment_id, vote_id, url_id, broadcast_id, edit_key, sso)


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

[**models::VoteDeleteResponse**](VoteDeleteResponse.md)

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

> models::CreateV1PageReact delete_v1_page_react(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::CreateV1PageReact**](CreateV1PageReact.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_v2_page_react

> models::CreateV1PageReact delete_v2_page_react(tenant_id, url_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::CreateV1PageReact**](CreateV1PageReact.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_comment_public

> models::ApiEmptyResponse flag_comment_public(tenant_id, comment_id, is_flagged, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**is_flagged** | **bool** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_text

> models::PublicApiGetCommentTextResponse get_comment_text(tenant_id, comment_id, edit_key, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PublicApiGetCommentTextResponse**](PublicAPIGetCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_vote_user_names

> models::GetCommentVoteUserNamesSuccessResponse get_comment_vote_user_names(tenant_id, comment_id, dir, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**dir** | **i32** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentVoteUserNamesSuccessResponse**](GetCommentVoteUserNamesSuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_for_user

> models::GetCommentsForUserResponse get_comments_for_user(user_id, direction, replies_to_user_id, page, includei10n, locale, is_crawler)


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

[**models::GetCommentsForUserResponse**](GetCommentsForUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments_public

> models::GetCommentsResponseWithPresencePublicComment get_comments_public(tenant_id, url_id, page, direction, sso, skip, skip_children, limit, limit_children, count_children, fetch_page_for_comment_id, include_config, count_all, includei10n, locale, modules, is_crawler, include_notification_count, as_tree, max_tree_depth, use_full_translation_ids, parent_id, search_text, hash_tags, user_id, custom_config_str, after_comment_id, before_comment_id)


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

[**models::GetCommentsResponseWithPresencePublicComment**](GetCommentsResponseWithPresence_PublicComment_.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_log

> models::GetEventLogResponse get_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


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

[**models::GetEventLogResponse**](GetEventLogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts_public

> models::PublicFeedPostsResponse get_feed_posts_public(tenant_id, after_id, limit, tags, sso, is_crawler, include_user_info)


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

[**models::PublicFeedPostsResponse**](PublicFeedPostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts_stats

> models::FeedPostsStatsResponse get_feed_posts_stats(tenant_id, post_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_ids** | [**Vec<String>**](String.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::FeedPostsStatsResponse**](FeedPostsStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_gif_large

> models::GifGetLargeResponse get_gif_large(tenant_id, large_internal_url_sanitized)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**large_internal_url_sanitized** | **String** |  | [required] |

### Return type

[**models::GifGetLargeResponse**](GifGetLargeResponse.md)

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

> models::GetEventLogResponse get_global_event_log(tenant_id, url_id, user_id_ws, start_time, end_time)


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

[**models::GetEventLogResponse**](GetEventLogResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_offline_users

> models::PageUsersOfflineResponse get_offline_users(tenant_id, url_id, after_name, after_user_id)


Past commenters on the page who are NOT currently online. Sorted by displayName. Use this after exhausting /users/online to render a \"Members\" section. Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** | Page URL identifier (cleaned server-side). | [required] |
**after_name** | Option<**String**> | Cursor: pass nextAfterName from the previous response. |  |
**after_user_id** | Option<**String**> | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |  |

### Return type

[**models::PageUsersOfflineResponse**](PageUsersOfflineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_online_users

> models::PageUsersOnlineResponse get_online_users(tenant_id, url_id, after_name, after_user_id)


Currently-online viewers of a page: people whose websocket session is subscribed to the page right now. Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** | Page URL identifier (cleaned server-side). | [required] |
**after_name** | Option<**String**> | Cursor: pass nextAfterName from the previous response. |  |
**after_user_id** | Option<**String**> | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |  |

### Return type

[**models::PageUsersOnlineResponse**](PageUsersOnlineResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pages_public

> models::GetPublicPagesResponse get_pages_public(tenant_id, cursor, limit, q, sort_by, has_comments)


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

[**models::GetPublicPagesResponse**](GetPublicPagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translations

> models::GetTranslationsResponse get_translations(namespace, component, locale, use_full_translation_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**namespace** | **String** |  | [required] |
**component** | **String** |  | [required] |
**locale** | Option<**String**> |  |  |
**use_full_translation_ids** | Option<**bool**> |  |  |

### Return type

[**models::GetTranslationsResponse**](GetTranslationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notification_count

> models::GetUserNotificationCountResponse get_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserNotificationCountResponse**](GetUserNotificationCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_notifications

> models::GetMyNotificationsResponse get_user_notifications(tenant_id, url_id, page_size, after_id, include_context, after_created_at, unread_only, dm_only, no_dm, include_translations, include_tenant_notifications, sso)


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

[**models::GetMyNotificationsResponse**](GetMyNotificationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_presence_statuses

> models::GetUserPresenceStatusesResponse get_user_presence_statuses(tenant_id, url_id_ws, user_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id_ws** | **String** |  | [required] |
**user_ids** | **String** |  | [required] |

### Return type

[**models::GetUserPresenceStatusesResponse**](GetUserPresenceStatusesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_reacts_public

> models::UserReactsResponse get_user_reacts_public(tenant_id, post_ids, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UserReactsResponse**](UserReactsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_info

> models::PageUsersInfoResponse get_users_info(tenant_id, ids)


Bulk user info for a tenant. Given userIds, return display info from User / SSOUser. Used by the comment widget to enrich users that just appeared via a presence event. No page context: privacy is enforced uniformly (private profiles are masked).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**ids** | **String** | Comma-delimited userIds. | [required] |

### Return type

[**models::PageUsersInfoResponse**](PageUsersInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v1_page_likes

> models::GetV1PageLikes get_v1_page_likes(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetV1PageLikes**](GetV1PageLikes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_page_react_users

> models::GetV2PageReactUsersResponse get_v2_page_react_users(tenant_id, url_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetV2PageReactUsersResponse**](GetV2PageReactUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_v2_page_reacts

> models::GetV2PageReacts get_v2_page_reacts(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetV2PageReacts**](GetV2PageReacts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_comment

> models::ApiEmptyResponse lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

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

> models::ChangeCommentPinStatusResponse pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ChangeCommentPinStatusResponse**](ChangeCommentPinStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## react_feed_post_public

> models::ReactFeedPostResponse react_feed_post_public(tenant_id, post_id, react_body_params, is_undo, broadcast_id, sso)


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

[**models::ReactFeedPostResponse**](ReactFeedPostResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notification_count

> models::ResetUserNotificationsResponse reset_user_notification_count(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ResetUserNotificationsResponse**](ResetUserNotificationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_user_notifications

> models::ResetUserNotificationsResponse reset_user_notifications(tenant_id, after_id, after_created_at, unread_only, dm_only, no_dm, sso)


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

[**models::ResetUserNotificationsResponse**](ResetUserNotificationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> models::SearchUsersResult search_users(tenant_id, url_id, username_starts_with, mention_group_ids, sso, search_section)


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

[**models::SearchUsersResult**](SearchUsersResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_comment_text

> models::PublicApiSetCommentTextResponse set_comment_text(tenant_id, comment_id, broadcast_id, comment_text_update_request, edit_key, sso)


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

[**models::PublicApiSetCommentTextResponse**](PublicAPISetCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_block_comment_public

> models::UnblockSuccess un_block_comment_public(tenant_id, comment_id, public_block_from_comment_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**public_block_from_comment_params** | [**PublicBlockFromCommentParams**](PublicBlockFromCommentParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::UnblockSuccess**](UnblockSuccess.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_lock_comment

> models::ApiEmptyResponse un_lock_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_pin_comment

> models::ChangeCommentPinStatusResponse un_pin_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ChangeCommentPinStatusResponse**](ChangeCommentPinStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feed_post_public

> models::CreateFeedPostResponse update_feed_post_public(tenant_id, post_id, update_feed_post_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**post_id** | **String** |  | [required] |
**update_feed_post_params** | [**UpdateFeedPostParams**](UpdateFeedPostParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::CreateFeedPostResponse**](CreateFeedPostResponse.md)

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

> models::VoteResponse vote_comment(tenant_id, comment_id, url_id, broadcast_id, vote_body_params, session_id, sso)


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

[**models::VoteResponse**](VoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

