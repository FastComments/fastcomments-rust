# \ModerationApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_moderation_vote**](ModerationApi.md#delete_moderation_vote) | **DELETE** /auth/my-account/moderate-comments/mod_api/vote/{commentId}/{voteId} | 
[**get_api_comments**](ModerationApi.md#get_api_comments) | **GET** /auth/my-account/moderate-comments/mod_api/api/comments | 
[**get_api_export_status**](ModerationApi.md#get_api_export_status) | **GET** /auth/my-account/moderate-comments/mod_api/api/export/status | 
[**get_api_ids**](ModerationApi.md#get_api_ids) | **GET** /auth/my-account/moderate-comments/mod_api/api/ids | 
[**get_ban_users_from_comment**](ModerationApi.md#get_ban_users_from_comment) | **GET** /auth/my-account/moderate-comments/mod_api/ban-users/from-comment/{commentId} | 
[**get_comment_ban_status**](ModerationApi.md#get_comment_ban_status) | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-ban-status/{commentId} | 
[**get_comment_children**](ModerationApi.md#get_comment_children) | **GET** /auth/my-account/moderate-comments/mod_api/comment-children/{commentId} | 
[**get_count**](ModerationApi.md#get_count) | **GET** /auth/my-account/moderate-comments/mod_api/count | 
[**get_counts**](ModerationApi.md#get_counts) | **GET** /auth/my-account/moderate-comments/banned-users/mod_api/counts | 
[**get_logs**](ModerationApi.md#get_logs) | **GET** /auth/my-account/moderate-comments/mod_api/logs/{commentId} | 
[**get_manual_badges**](ModerationApi.md#get_manual_badges) | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges | 
[**get_manual_badges_for_user**](ModerationApi.md#get_manual_badges_for_user) | **GET** /auth/my-account/moderate-comments/mod_api/get-manual-badges-for-user | 
[**get_moderation_comment**](ModerationApi.md#get_moderation_comment) | **GET** /auth/my-account/moderate-comments/mod_api/comment/{commentId} | 
[**get_moderation_comment_text**](ModerationApi.md#get_moderation_comment_text) | **GET** /auth/my-account/moderate-comments/mod_api/get-comment-text/{commentId} | 
[**get_pre_ban_summary**](ModerationApi.md#get_pre_ban_summary) | **GET** /auth/my-account/moderate-comments/mod_api/pre-ban-summary/{commentId} | 
[**get_search_comments_summary**](ModerationApi.md#get_search_comments_summary) | **GET** /auth/my-account/moderate-comments/mod_api/search/comments/summary | 
[**get_search_pages**](ModerationApi.md#get_search_pages) | **GET** /auth/my-account/moderate-comments/mod_api/search/pages | 
[**get_search_sites**](ModerationApi.md#get_search_sites) | **GET** /auth/my-account/moderate-comments/mod_api/search/sites | 
[**get_search_suggest**](ModerationApi.md#get_search_suggest) | **GET** /auth/my-account/moderate-comments/mod_api/search/suggest | 
[**get_search_users**](ModerationApi.md#get_search_users) | **GET** /auth/my-account/moderate-comments/mod_api/search/users | 
[**get_trust_factor**](ModerationApi.md#get_trust_factor) | **GET** /auth/my-account/moderate-comments/mod_api/get-trust-factor | 
[**get_user_ban_preference**](ModerationApi.md#get_user_ban_preference) | **GET** /auth/my-account/moderate-comments/mod_api/user-ban-preference | 
[**get_user_internal_profile**](ModerationApi.md#get_user_internal_profile) | **GET** /auth/my-account/moderate-comments/mod_api/get-user-internal-profile | 
[**post_adjust_comment_votes**](ModerationApi.md#post_adjust_comment_votes) | **POST** /auth/my-account/moderate-comments/mod_api/adjust-comment-votes/{commentId} | 
[**post_api_export**](ModerationApi.md#post_api_export) | **POST** /auth/my-account/moderate-comments/mod_api/api/export | 
[**post_ban_user_from_comment**](ModerationApi.md#post_ban_user_from_comment) | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/from-comment/{commentId} | 
[**post_ban_user_undo**](ModerationApi.md#post_ban_user_undo) | **POST** /auth/my-account/moderate-comments/mod_api/ban-user/undo | 
[**post_bulk_pre_ban_summary**](ModerationApi.md#post_bulk_pre_ban_summary) | **POST** /auth/my-account/moderate-comments/mod_api/bulk-pre-ban-summary | 
[**post_comments_by_ids**](ModerationApi.md#post_comments_by_ids) | **POST** /auth/my-account/moderate-comments/mod_api/comments-by-ids | 
[**post_flag_comment**](ModerationApi.md#post_flag_comment) | **POST** /auth/my-account/moderate-comments/mod_api/flag-comment/{commentId} | 
[**post_remove_comment**](ModerationApi.md#post_remove_comment) | **POST** /auth/my-account/moderate-comments/mod_api/remove-comment/{commentId} | 
[**post_restore_deleted_comment**](ModerationApi.md#post_restore_deleted_comment) | **POST** /auth/my-account/moderate-comments/mod_api/restore-deleted-comment/{commentId} | 
[**post_set_comment_approval_status**](ModerationApi.md#post_set_comment_approval_status) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-approval-status/{commentId} | 
[**post_set_comment_review_status**](ModerationApi.md#post_set_comment_review_status) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-review-status/{commentId} | 
[**post_set_comment_spam_status**](ModerationApi.md#post_set_comment_spam_status) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-spam-status/{commentId} | 
[**post_set_comment_text**](ModerationApi.md#post_set_comment_text) | **POST** /auth/my-account/moderate-comments/mod_api/set-comment-text/{commentId} | 
[**post_un_flag_comment**](ModerationApi.md#post_un_flag_comment) | **POST** /auth/my-account/moderate-comments/mod_api/un-flag-comment/{commentId} | 
[**post_vote**](ModerationApi.md#post_vote) | **POST** /auth/my-account/moderate-comments/mod_api/vote/{commentId} | 
[**put_award_badge**](ModerationApi.md#put_award_badge) | **PUT** /auth/my-account/moderate-comments/mod_api/award-badge | 
[**put_close_thread**](ModerationApi.md#put_close_thread) | **PUT** /auth/my-account/moderate-comments/mod_api/close-thread | 
[**put_remove_badge**](ModerationApi.md#put_remove_badge) | **PUT** /auth/my-account/moderate-comments/mod_api/remove-badge | 
[**put_reopen_thread**](ModerationApi.md#put_reopen_thread) | **PUT** /auth/my-account/moderate-comments/mod_api/reopen-thread | 
[**set_trust_factor**](ModerationApi.md#set_trust_factor) | **PUT** /auth/my-account/moderate-comments/mod_api/set-trust-factor | 



## delete_moderation_vote

> models::VoteDeleteResponse delete_moderation_vote(tenant_id, comment_id, vote_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**vote_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::VoteDeleteResponse**](VoteDeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_comments

> models::ModerationApiGetCommentsResponse get_api_comments(tenant_id, page, count, text_search, by_ip_from_comment, filters, search_filters, sorts, demo, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**count** | Option<**f64**> |  |  |
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**sorts** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiGetCommentsResponse**](ModerationAPIGetCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_export_status

> models::ModerationExportStatusResponse get_api_export_status(tenant_id, batch_job_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**batch_job_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationExportStatusResponse**](ModerationExportStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_ids

> models::ModerationApiGetCommentIdsResponse get_api_ids(tenant_id, text_search, by_ip_from_comment, filters, search_filters, after_id, demo, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**after_id** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiGetCommentIdsResponse**](ModerationAPIGetCommentIdsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ban_users_from_comment

> models::GetBannedUsersFromCommentResponse get_ban_users_from_comment(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetBannedUsersFromCommentResponse**](GetBannedUsersFromCommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_ban_status

> models::GetCommentBanStatusResponse get_comment_ban_status(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentBanStatusResponse**](GetCommentBanStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment_children

> models::ModerationApiChildCommentsResponse get_comment_children(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiChildCommentsResponse**](ModerationAPIChildCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_count

> models::ModerationApiCountCommentsResponse get_count(tenant_id, text_search, by_ip_from_comment, filter, search_filters, demo, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filter** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**demo** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiCountCommentsResponse**](ModerationAPICountCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_counts

> models::GetBannedUsersCountResponse get_counts(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetBannedUsersCountResponse**](GetBannedUsersCountResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_logs

> models::ModerationApiGetLogsResponse get_logs(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiGetLogsResponse**](ModerationAPIGetLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_badges

> models::GetTenantManualBadgesResponse get_manual_badges(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetTenantManualBadgesResponse**](GetTenantManualBadgesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manual_badges_for_user

> models::GetUserManualBadgesResponse get_manual_badges_for_user(tenant_id, badges_user_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**badges_user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserManualBadgesResponse**](GetUserManualBadgesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderation_comment

> models::ModerationApiCommentResponse get_moderation_comment(tenant_id, comment_id, include_email, include_ip, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**include_email** | Option<**bool**> |  |  |
**include_ip** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiCommentResponse**](ModerationAPICommentResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderation_comment_text

> models::GetCommentTextResponse get_moderation_comment_text(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetCommentTextResponse**](GetCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pre_ban_summary

> models::PreBanSummary get_pre_ban_summary(tenant_id, comment_id, include_by_user_id_and_email, include_by_ip, include_by_email_domain, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**include_by_user_id_and_email** | Option<**bool**> |  |  |
**include_by_ip** | Option<**bool**> |  |  |
**include_by_email_domain** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PreBanSummary**](PreBanSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_comments_summary

> models::ModerationCommentSearchResponse get_search_comments_summary(tenant_id, value, filters, search_filters, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**value** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationCommentSearchResponse**](ModerationCommentSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_pages

> models::ModerationPageSearchResponse get_search_pages(tenant_id, value, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**value** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationPageSearchResponse**](ModerationPageSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_sites

> models::ModerationSiteSearchResponse get_search_sites(tenant_id, value, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**value** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationSiteSearchResponse**](ModerationSiteSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_suggest

> models::ModerationSuggestResponse get_search_suggest(tenant_id, text_search, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**text_search** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationSuggestResponse**](ModerationSuggestResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_users

> models::ModerationUserSearchResponse get_search_users(tenant_id, value, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**value** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationUserSearchResponse**](ModerationUserSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trust_factor

> models::GetUserTrustFactorResponse get_trust_factor(tenant_id, user_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserTrustFactorResponse**](GetUserTrustFactorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_ban_preference

> models::ApiModerateGetUserBanPreferencesResponse get_user_ban_preference(tenant_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiModerateGetUserBanPreferencesResponse**](APIModerateGetUserBanPreferencesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_internal_profile

> models::GetUserInternalProfileResponse get_user_internal_profile(tenant_id, comment_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::GetUserInternalProfileResponse**](GetUserInternalProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_adjust_comment_votes

> models::AdjustVotesResponse post_adjust_comment_votes(tenant_id, comment_id, adjust_comment_votes_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**adjust_comment_votes_params** | [**AdjustCommentVotesParams**](AdjustCommentVotesParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::AdjustVotesResponse**](AdjustVotesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_api_export

> models::ModerationExportResponse post_api_export(tenant_id, text_search, by_ip_from_comment, filters, search_filters, sorts, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**text_search** | Option<**String**> |  |  |
**by_ip_from_comment** | Option<**String**> |  |  |
**filters** | Option<**String**> |  |  |
**search_filters** | Option<**String**> |  |  |
**sorts** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationExportResponse**](ModerationExportResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ban_user_from_comment

> models::BanUserFromCommentResult post_ban_user_from_comment(tenant_id, comment_id, ban_email, ban_email_domain, ban_ip, delete_all_users_comments, banned_until, is_shadow_ban, update_id, ban_reason, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**ban_email** | Option<**bool**> |  |  |
**ban_email_domain** | Option<**bool**> |  |  |
**ban_ip** | Option<**bool**> |  |  |
**delete_all_users_comments** | Option<**bool**> |  |  |
**banned_until** | Option<**String**> |  |  |
**is_shadow_ban** | Option<**bool**> |  |  |
**update_id** | Option<**String**> |  |  |
**ban_reason** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::BanUserFromCommentResult**](BanUserFromCommentResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_ban_user_undo

> models::ApiEmptyResponse post_ban_user_undo(tenant_id, ban_user_undo_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**ban_user_undo_params** | [**BanUserUndoParams**](BanUserUndoParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_bulk_pre_ban_summary

> models::BulkPreBanSummary post_bulk_pre_ban_summary(tenant_id, bulk_pre_ban_params, include_by_user_id_and_email, include_by_ip, include_by_email_domain, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**bulk_pre_ban_params** | [**BulkPreBanParams**](BulkPreBanParams.md) |  | [required] |
**include_by_user_id_and_email** | Option<**bool**> |  |  |
**include_by_ip** | Option<**bool**> |  |  |
**include_by_email_domain** | Option<**bool**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::BulkPreBanSummary**](BulkPreBanSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_comments_by_ids

> models::ModerationApiChildCommentsResponse post_comments_by_ids(tenant_id, comments_by_ids_params, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comments_by_ids_params** | [**CommentsByIdsParams**](CommentsByIdsParams.md) |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ModerationApiChildCommentsResponse**](ModerationAPIChildCommentsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_flag_comment

> models::ApiEmptyResponse post_flag_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_remove_comment

> models::PostRemoveCommentApiResponse post_remove_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::PostRemoveCommentApiResponse**](PostRemoveCommentApiResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_restore_deleted_comment

> models::ApiEmptyResponse post_restore_deleted_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_approval_status

> models::SetCommentApprovedResponse post_set_comment_approval_status(tenant_id, comment_id, approved, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**approved** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::SetCommentApprovedResponse**](SetCommentApprovedResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_review_status

> models::ApiEmptyResponse post_set_comment_review_status(tenant_id, comment_id, reviewed, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**reviewed** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_spam_status

> models::ApiEmptyResponse post_set_comment_spam_status(tenant_id, comment_id, spam, perm_not_spam, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**spam** | Option<**bool**> |  |  |
**perm_not_spam** | Option<**bool**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_set_comment_text

> models::SetCommentTextResponse post_set_comment_text(tenant_id, comment_id, set_comment_text_params, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**set_comment_text_params** | [**SetCommentTextParams**](SetCommentTextParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::SetCommentTextResponse**](SetCommentTextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_un_flag_comment

> models::ApiEmptyResponse post_un_flag_comment(tenant_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_vote

> models::VoteResponse post_vote(tenant_id, comment_id, direction, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**direction** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::VoteResponse**](VoteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_award_badge

> models::AwardUserBadgeResponse put_award_badge(tenant_id, badge_id, user_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**badge_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::AwardUserBadgeResponse**](AwardUserBadgeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_close_thread

> models::ApiEmptyResponse put_close_thread(tenant_id, url_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_remove_badge

> models::RemoveUserBadgeResponse put_remove_badge(tenant_id, badge_id, user_id, comment_id, broadcast_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**badge_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**comment_id** | Option<**String**> |  |  |
**broadcast_id** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::RemoveUserBadgeResponse**](RemoveUserBadgeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_reopen_thread

> models::ApiEmptyResponse put_reopen_thread(tenant_id, url_id, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**sso** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trust_factor

> models::SetUserTrustFactorResponse set_trust_factor(tenant_id, user_id, trust_factor, sso)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**trust_factor** | Option<**String**> |  |  |
**sso** | Option<**String**> |  |  |

### Return type

[**models::SetUserTrustFactorResponse**](SetUserTrustFactorResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

