# \ModerationApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_moderation_vote**](ModerationApi.md#delete_moderation_vote) | **DELETE** /auth/my-account/moderate-comments/vote/{commentId}/{voteId} | 
[**get_api_comments**](ModerationApi.md#get_api_comments) | **GET** /auth/my-account/moderate-comments/api/comments | 
[**get_api_export_status**](ModerationApi.md#get_api_export_status) | **GET** /auth/my-account/moderate-comments/api/export/status | 
[**get_api_ids**](ModerationApi.md#get_api_ids) | **GET** /auth/my-account/moderate-comments/api/ids | 
[**get_ban_users_from_comment**](ModerationApi.md#get_ban_users_from_comment) | **GET** /auth/my-account/moderate-comments/ban-users/from-comment/{commentId} | 
[**get_comment_ban_status**](ModerationApi.md#get_comment_ban_status) | **GET** /auth/my-account/moderate-comments/get-comment-ban-status/{commentId} | 
[**get_comment_children**](ModerationApi.md#get_comment_children) | **GET** /auth/my-account/moderate-comments/comment-children/{commentId} | 
[**get_count**](ModerationApi.md#get_count) | **GET** /auth/my-account/moderate-comments/count | 
[**get_counts**](ModerationApi.md#get_counts) | **GET** /auth/my-account/moderate-comments/banned-users/counts | 
[**get_logs**](ModerationApi.md#get_logs) | **GET** /auth/my-account/moderate-comments/logs/{commentId} | 
[**get_manual_badges**](ModerationApi.md#get_manual_badges) | **GET** /auth/my-account/moderate-comments/get-manual-badges | 
[**get_manual_badges_for_user**](ModerationApi.md#get_manual_badges_for_user) | **GET** /auth/my-account/moderate-comments/get-manual-badges-for-user | 
[**get_moderation_comment**](ModerationApi.md#get_moderation_comment) | **GET** /auth/my-account/moderate-comments/comment/{commentId} | 
[**get_moderation_comment_text**](ModerationApi.md#get_moderation_comment_text) | **GET** /auth/my-account/moderate-comments/get-comment-text/{commentId} | 
[**get_pre_ban_summary**](ModerationApi.md#get_pre_ban_summary) | **GET** /auth/my-account/moderate-comments/pre-ban-summary/{commentId} | 
[**get_search_comments_summary**](ModerationApi.md#get_search_comments_summary) | **GET** /auth/my-account/moderate-comments/search/comments/summary | 
[**get_search_pages**](ModerationApi.md#get_search_pages) | **GET** /auth/my-account/moderate-comments/search/pages | 
[**get_search_sites**](ModerationApi.md#get_search_sites) | **GET** /auth/my-account/moderate-comments/search/sites | 
[**get_search_suggest**](ModerationApi.md#get_search_suggest) | **GET** /auth/my-account/moderate-comments/search/suggest | 
[**get_search_users**](ModerationApi.md#get_search_users) | **GET** /auth/my-account/moderate-comments/search/users | 
[**get_trust_factor**](ModerationApi.md#get_trust_factor) | **GET** /auth/my-account/moderate-comments/get-trust-factor | 
[**get_user_ban_preference**](ModerationApi.md#get_user_ban_preference) | **GET** /auth/my-account/moderate-comments/user-ban-preference | 
[**get_user_internal_profile**](ModerationApi.md#get_user_internal_profile) | **GET** /auth/my-account/moderate-comments/get-user-internal-profile | 
[**post_adjust_comment_votes**](ModerationApi.md#post_adjust_comment_votes) | **POST** /auth/my-account/moderate-comments/adjust-comment-votes/{commentId} | 
[**post_api_export**](ModerationApi.md#post_api_export) | **POST** /auth/my-account/moderate-comments/api/export | 
[**post_ban_user_from_comment**](ModerationApi.md#post_ban_user_from_comment) | **POST** /auth/my-account/moderate-comments/ban-user/from-comment/{commentId} | 
[**post_ban_user_undo**](ModerationApi.md#post_ban_user_undo) | **POST** /auth/my-account/moderate-comments/ban-user/undo | 
[**post_bulk_pre_ban_summary**](ModerationApi.md#post_bulk_pre_ban_summary) | **POST** /auth/my-account/moderate-comments/bulk-pre-ban-summary | 
[**post_comments_by_ids**](ModerationApi.md#post_comments_by_ids) | **POST** /auth/my-account/moderate-comments/comments-by-ids | 
[**post_flag_comment**](ModerationApi.md#post_flag_comment) | **POST** /auth/my-account/moderate-comments/flag-comment/{commentId} | 
[**post_remove_comment**](ModerationApi.md#post_remove_comment) | **POST** /auth/my-account/moderate-comments/remove-comment/{commentId} | 
[**post_restore_deleted_comment**](ModerationApi.md#post_restore_deleted_comment) | **POST** /auth/my-account/moderate-comments/restore-deleted-comment/{commentId} | 
[**post_set_comment_approval_status**](ModerationApi.md#post_set_comment_approval_status) | **POST** /auth/my-account/moderate-comments/set-comment-approval-status/{commentId} | 
[**post_set_comment_review_status**](ModerationApi.md#post_set_comment_review_status) | **POST** /auth/my-account/moderate-comments/set-comment-review-status/{commentId} | 
[**post_set_comment_spam_status**](ModerationApi.md#post_set_comment_spam_status) | **POST** /auth/my-account/moderate-comments/set-comment-spam-status/{commentId} | 
[**post_set_comment_text**](ModerationApi.md#post_set_comment_text) | **POST** /auth/my-account/moderate-comments/set-comment-text/{commentId} | 
[**post_un_flag_comment**](ModerationApi.md#post_un_flag_comment) | **POST** /auth/my-account/moderate-comments/un-flag-comment/{commentId} | 
[**post_vote**](ModerationApi.md#post_vote) | **POST** /auth/my-account/moderate-comments/vote/{commentId} | 
[**put_award_badge**](ModerationApi.md#put_award_badge) | **PUT** /auth/my-account/moderate-comments/award-badge | 
[**put_close_thread**](ModerationApi.md#put_close_thread) | **PUT** /auth/my-account/moderate-comments/close-thread | 
[**put_remove_badge**](ModerationApi.md#put_remove_badge) | **PUT** /auth/my-account/moderate-comments/remove-badge | 
[**put_reopen_thread**](ModerationApi.md#put_reopen_thread) | **PUT** /auth/my-account/moderate-comments/reopen-thread | 
[**set_trust_factor**](ModerationApi.md#set_trust_factor) | **PUT** /auth/my-account/moderate-comments/set-trust-factor | 



## delete_moderation_vote

> models::DeleteModerationVoteResponse delete_moderation_vote(comment_id, vote_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**vote_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::DeleteModerationVoteResponse**](DeleteModerationVoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_comments

> models::GetApiCommentsResponse get_api_comments(page, count, text_search, by_ip_from_comment, filters, search_filters, sorts, demo, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**f64**> |  |  |
**count** | Option<**f64**> |  |  |
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**sorts** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetApiCommentsResponse**](GetApiCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_export_status

> models::GetApiExportStatusResponse get_api_export_status(batch_job_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_job_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetApiExportStatusResponse**](GetApiExportStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_ids

> models::GetApiIdsResponse get_api_ids(text_search, by_ip_from_comment, filters, search_filters, after_id, demo, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**after_id** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetApiIdsResponse**](GetApiIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ban_users_from_comment

> models::GetBanUsersFromCommentResponse get_ban_users_from_comment(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetBanUsersFromCommentResponse**](GetBanUsersFromCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_ban_status

> models::GetCommentBanStatusResponse1 get_comment_ban_status(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentBanStatusResponse1**](GetCommentBanStatusResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_children

> models::GetCommentChildrenResponse get_comment_children(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentChildrenResponse**](GetCommentChildrenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_count

> models::GetCountResponse get_count(text_search, by_ip_from_comment, filter, search_filters, demo, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filter** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCountResponse**](GetCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_counts

> models::GetCountsResponse get_counts(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCountsResponse**](GetCountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs

> models::GetLogsResponse get_logs(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetLogsResponse**](GetLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_badges

> models::GetManualBadgesResponse get_manual_badges(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetManualBadgesResponse**](GetManualBadgesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_badges_for_user

> models::GetManualBadgesForUserResponse get_manual_badges_for_user(badges_user_id, comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**badges_user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetManualBadgesForUserResponse**](GetManualBadgesForUserResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderation_comment

> models::GetModerationCommentResponse get_moderation_comment(comment_id, include_email, include_ip, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**include_email** | Option<**bool**> |  |  |
**include_ip** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetModerationCommentResponse**](GetModerationCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderation_comment_text

> models::GetModerationCommentTextResponse get_moderation_comment_text(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetModerationCommentTextResponse**](GetModerationCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pre_ban_summary

> models::GetPreBanSummaryResponse get_pre_ban_summary(comment_id, include_by_user_id_and_email, include_by_ip, include_by_email_domain, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**include_by_user_id_and_email** | Option<**bool**> |  |  |
**include_by_ip** | Option<**bool**> |  |  |
**include_by_email_domain** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetPreBanSummaryResponse**](GetPreBanSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_comments_summary

> models::GetSearchCommentsSummaryResponse get_search_comments_summary(value, filters, search_filters, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetSearchCommentsSummaryResponse**](GetSearchCommentsSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_pages

> models::GetSearchPagesResponse get_search_pages(value, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetSearchPagesResponse**](GetSearchPagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_sites

> models::GetSearchSitesResponse get_search_sites(value, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetSearchSitesResponse**](GetSearchSitesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggest

> models::GetSearchSuggestResponse get_search_suggest(text_search, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text_search** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetSearchSuggestResponse**](GetSearchSuggestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_users

> models::GetSearchUsersResponse get_search_users(value, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**value** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetSearchUsersResponse**](GetSearchUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trust_factor

> models::GetTrustFactorResponse get_trust_factor(user_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetTrustFactorResponse**](GetTrustFactorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_ban_preference

> models::GetUserBanPreferenceResponse get_user_ban_preference(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserBanPreferenceResponse**](GetUserBanPreferenceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_internal_profile

> models::GetUserInternalProfileResponse1 get_user_internal_profile(comment_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserInternalProfileResponse1**](GetUserInternalProfileResponse_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_adjust_comment_votes

> models::PostAdjustCommentVotesResponse post_adjust_comment_votes(comment_id, adjust_comment_votes_params, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**adjust_comment_votes_params** | [**AdjustCommentVotesParams**](AdjustCommentVotesParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostAdjustCommentVotesResponse**](PostAdjustCommentVotesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_export

> models::PostApiExportResponse post_api_export(text_search, by_ip_from_comment, filters, search_filters, sorts, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**sorts** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostApiExportResponse**](PostApiExportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ban_user_from_comment

> models::PostBanUserFromCommentResponse post_ban_user_from_comment(comment_id, ban_email, ban_email_domain, ban_ip, delete_all_users_comments, banned_until, is_shadow_ban, update_id, ban_reason, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**ban_email** | Option<**bool**> |  |  |
**ban_email_domain** | Option<**bool**> |  |  |
**ban_ip** | Option<**bool**> |  |  |
**delete_all_users_comments** | Option<**bool**> |  |  |
**banned_until** | Option<**String**> |  |  |
**is_shadow_ban** | Option<**bool**> |  |  |
**update_id** | Option<**String**> |  |  |
**ban_reason** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostBanUserFromCommentResponse**](PostBanUserFromCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ban_user_undo

> models::PostBanUserUndoResponse post_ban_user_undo(ban_user_undo_params, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ban_user_undo_params** | [**BanUserUndoParams**](BanUserUndoParams.md) |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostBanUserUndoResponse**](PostBanUserUndoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_bulk_pre_ban_summary

> models::PostBulkPreBanSummaryResponse post_bulk_pre_ban_summary(bulk_pre_ban_params, include_by_user_id_and_email, include_by_ip, include_by_email_domain, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_pre_ban_params** | [**BulkPreBanParams**](BulkPreBanParams.md) |  | [required] |
**include_by_user_id_and_email** | Option<**bool**> |  |  |
**include_by_ip** | Option<**bool**> |  |  |
**include_by_email_domain** | Option<**bool**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostBulkPreBanSummaryResponse**](PostBulkPreBanSummaryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_comments_by_ids

> models::PostCommentsByIdsResponse post_comments_by_ids(comments_by_ids_params, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comments_by_ids_params** | [**CommentsByIdsParams**](CommentsByIdsParams.md) |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostCommentsByIdsResponse**](PostCommentsByIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flag_comment

> models::PostFlagCommentResponse post_flag_comment(comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostFlagCommentResponse**](PostFlagCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_remove_comment

> models::PostRemoveCommentResponse post_remove_comment(comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostRemoveCommentResponse**](PostRemoveCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_restore_deleted_comment

> models::PostRestoreDeletedCommentResponse post_restore_deleted_comment(comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostRestoreDeletedCommentResponse**](PostRestoreDeletedCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_approval_status

> models::PostSetCommentApprovalStatusResponse post_set_comment_approval_status(comment_id, approved, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**approved** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostSetCommentApprovalStatusResponse**](PostSetCommentApprovalStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_review_status

> models::PostSetCommentReviewStatusResponse post_set_comment_review_status(comment_id, reviewed, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**reviewed** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostSetCommentReviewStatusResponse**](PostSetCommentReviewStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_spam_status

> models::PostSetCommentSpamStatusResponse post_set_comment_spam_status(comment_id, spam, perm_not_spam, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**spam** | Option<**bool**> |  |  |
**perm_not_spam** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostSetCommentSpamStatusResponse**](PostSetCommentSpamStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_text

> models::PostSetCommentTextResponse post_set_comment_text(comment_id, set_comment_text_params, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**set_comment_text_params** | [**SetCommentTextParams**](SetCommentTextParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostSetCommentTextResponse**](PostSetCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_un_flag_comment

> models::PostUnFlagCommentResponse post_un_flag_comment(comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostUnFlagCommentResponse**](PostUnFlagCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_vote

> models::PostVoteResponse post_vote(comment_id, direction, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**comment_id** | **String** |  | [required] |
**direction** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostVoteResponse**](PostVoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_award_badge

> models::PutAwardBadgeResponse put_award_badge(badge_id, user_id, comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**badge_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PutAwardBadgeResponse**](PutAwardBadgeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_close_thread

> models::PutCloseThreadResponse put_close_thread(url_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PutCloseThreadResponse**](PutCloseThreadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_remove_badge

> models::PutRemoveBadgeResponse put_remove_badge(badge_id, user_id, comment_id, broadcast_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**badge_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PutRemoveBadgeResponse**](PutRemoveBadgeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_reopen_thread

> models::PutReopenThreadResponse put_reopen_thread(url_id, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url_id** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PutReopenThreadResponse**](PutReopenThreadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trust_factor

> models::SetTrustFactorResponse set_trust_factor(user_id, trust_factor, tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**String**> |  |  |
**trust_factor** | Option<**String**> |  |  |
**tenant_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::SetTrustFactorResponse**](SetTrustFactorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

