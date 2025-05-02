# \DefaultApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain_config**](DefaultApi.md#add_domain_config) | **POST** /api/v1/domain-configs | 
[**aggregate**](DefaultApi.md#aggregate) | **POST** /api/v1/aggregate | 
[**aggregate_question_results**](DefaultApi.md#aggregate_question_results) | **GET** /api/v1/question-results-aggregation | 
[**block_user_from_comment**](DefaultApi.md#block_user_from_comment) | **POST** /api/v1/comments/{id}/block | 
[**bulk_aggregate_question_results**](DefaultApi.md#bulk_aggregate_question_results) | **POST** /api/v1/question-results-aggregation/bulk | 
[**combine_comments_with_question_results**](DefaultApi.md#combine_comments_with_question_results) | **GET** /api/v1/question-results-aggregation/combine/comments | 
[**create_feed_post**](DefaultApi.md#create_feed_post) | **POST** /api/v1/feed-posts | 
[**create_user_badge**](DefaultApi.md#create_user_badge) | **POST** /api/v1/user-badges | 
[**delete_comment**](DefaultApi.md#delete_comment) | **DELETE** /api/v1/comments/{id} | 
[**delete_domain_config**](DefaultApi.md#delete_domain_config) | **DELETE** /api/v1/domain-configs/{domain} | 
[**delete_user_badge**](DefaultApi.md#delete_user_badge) | **DELETE** /api/v1/user-badges/{id} | 
[**flag_comment**](DefaultApi.md#flag_comment) | **POST** /api/v1/comments/{id}/flag | 
[**get_audit_logs**](DefaultApi.md#get_audit_logs) | **GET** /api/v1/audit-logs | 
[**get_comment**](DefaultApi.md#get_comment) | **GET** /api/v1/comments/{id} | 
[**get_comments**](DefaultApi.md#get_comments) | **GET** /api/v1/comments | 
[**get_domain_config**](DefaultApi.md#get_domain_config) | **GET** /api/v1/domain-configs/{domain} | 
[**get_domain_configs**](DefaultApi.md#get_domain_configs) | **GET** /api/v1/domain-configs | 
[**get_feed_posts**](DefaultApi.md#get_feed_posts) | **GET** /api/v1/feed-posts | 
[**get_user_badge**](DefaultApi.md#get_user_badge) | **GET** /api/v1/user-badges/{id} | 
[**get_user_badge_progress_by_id**](DefaultApi.md#get_user_badge_progress_by_id) | **GET** /api/v1/user-badge-progress/{id} | 
[**get_user_badge_progress_by_user_id**](DefaultApi.md#get_user_badge_progress_by_user_id) | **GET** /api/v1/user-badge-progress/user/{userId} | 
[**get_user_badge_progress_list**](DefaultApi.md#get_user_badge_progress_list) | **GET** /api/v1/user-badge-progress | 
[**get_user_badges**](DefaultApi.md#get_user_badges) | **GET** /api/v1/user-badges | 
[**patch_domain_config**](DefaultApi.md#patch_domain_config) | **PATCH** /api/v1/domain-configs/{domainToUpdate} | 
[**put_domain_config**](DefaultApi.md#put_domain_config) | **PUT** /api/v1/domain-configs/{domainToUpdate} | 
[**save_comment**](DefaultApi.md#save_comment) | **POST** /api/v1/comments | 
[**un_block_user_from_comment**](DefaultApi.md#un_block_user_from_comment) | **POST** /api/v1/comments/{id}/un-block | 
[**un_flag_comment**](DefaultApi.md#un_flag_comment) | **POST** /api/v1/comments/{id}/un-flag | 
[**update_comment**](DefaultApi.md#update_comment) | **PATCH** /api/v1/comments/{id} | 
[**update_feed_post**](DefaultApi.md#update_feed_post) | **PATCH** /api/v1/feed-posts/{id} | 
[**update_user_badge**](DefaultApi.md#update_user_badge) | **PUT** /api/v1/user-badges/{id} | 



## add_domain_config

> models::AddDomainConfig200Response add_domain_config(tenant_id, add_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**add_domain_config_params** | [**AddDomainConfigParams**](AddDomainConfigParams.md) |  | [required] |

### Return type

[**models::AddDomainConfig200Response**](AddDomainConfig_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aggregate

> models::AggregationResponse aggregate(tenant_id, aggregation_request, parent_tenant_id, include_stats)


Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**aggregation_request** | [**AggregationRequest**](AggregationRequest.md) |  | [required] |
**parent_tenant_id** | Option<**String**> |  |  |
**include_stats** | Option<**bool**> |  |  |

### Return type

[**models::AggregationResponse**](AggregationResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aggregate_question_results

> models::AggregateQuestionResults200Response aggregate_question_results(tenant_id, question_id, question_ids, url_id, time_bucket, start_date, force_recalculate)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**question_id** | Option<**String**> |  |  |
**question_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**url_id** | Option<**String**> |  |  |
**time_bucket** | Option<[**AggregateTimeBucket**](.md)> |  |  |
**start_date** | Option<**String**> |  |  |
**force_recalculate** | Option<**bool**> |  |  |

### Return type

[**models::AggregateQuestionResults200Response**](AggregateQuestionResults_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_user_from_comment

> models::BlockFromCommentPublic200Response block_user_from_comment(tenant_id, id, block_from_comment_params, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**block_from_comment_params** | [**BlockFromCommentParams**](BlockFromCommentParams.md) |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::BlockFromCommentPublic200Response**](BlockFromCommentPublic_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_aggregate_question_results

> models::BulkAggregateQuestionResults200Response bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**bulk_aggregate_question_results_request** | [**BulkAggregateQuestionResultsRequest**](BulkAggregateQuestionResultsRequest.md) |  | [required] |
**force_recalculate** | Option<**bool**> |  |  |

### Return type

[**models::BulkAggregateQuestionResults200Response**](BulkAggregateQuestionResults_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## combine_comments_with_question_results

> models::CombineCommentsWithQuestionResults200Response combine_comments_with_question_results(tenant_id, question_id, question_ids, url_id, start_date, force_recalculate, min_value, max_value, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**question_id** | Option<**String**> |  |  |
**question_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**url_id** | Option<**String**> |  |  |
**start_date** | Option<**String**> |  |  |
**force_recalculate** | Option<**bool**> |  |  |
**min_value** | Option<**f64**> |  |  |
**max_value** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**models::CombineCommentsWithQuestionResults200Response**](CombineCommentsWithQuestionResults_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed_post

> models::CreateFeedPost200Response create_feed_post(tenant_id, create_feed_post_params, broadcast_id, is_live, do_spam_check, skip_dup_check)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_feed_post_params** | [**CreateFeedPostParams**](CreateFeedPostParams.md) |  | [required] |
**broadcast_id** | Option<**String**> |  |  |
**is_live** | Option<**bool**> |  |  |
**do_spam_check** | Option<**bool**> |  |  |
**skip_dup_check** | Option<**bool**> |  |  |

### Return type

[**models::CreateFeedPost200Response**](CreateFeedPost_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_badge

> models::CreateUserBadge200Response create_user_badge(tenant_id, create_user_badge_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_user_badge_params** | [**CreateUserBadgeParams**](CreateUserBadgeParams.md) |  | [required] |

### Return type

[**models::CreateUserBadge200Response**](CreateUserBadge_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> models::DeleteComment200Response delete_comment(tenant_id, id, context_user_id, is_live)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**context_user_id** | Option<**String**> |  |  |
**is_live** | Option<**bool**> |  |  |

### Return type

[**models::DeleteComment200Response**](DeleteComment_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain_config

> models::DeleteDomainConfig200Response delete_domain_config(tenant_id, domain)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |

### Return type

[**models::DeleteDomainConfig200Response**](DeleteDomainConfig_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_badge

> models::UpdateUserBadge200Response delete_user_badge(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::UpdateUserBadge200Response**](UpdateUserBadge_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_comment

> models::FlagComment200Response flag_comment(tenant_id, id, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::FlagComment200Response**](FlagComment_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_logs

> models::GetAuditLogs200Response get_audit_logs(tenant_id, limit, skip, order, after, before)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**order** | Option<[**SortDir**](.md)> |  |  |
**after** | Option<**f64**> |  |  |
**before** | Option<**f64**> |  |  |

### Return type

[**models::GetAuditLogs200Response**](GetAuditLogs_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment

> models::GetComment200Response get_comment(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetComment200Response**](GetComment_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments

> models::GetComments200Response get_comments(tenant_id, page, limit, skip, as_tree, skip_children, limit_children, max_tree_depth, url_id, user_id, anon_user_id, context_user_id, hash_tag, parent_id, direction)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**as_tree** | Option<**bool**> |  |  |
**skip_children** | Option<**f64**> |  |  |
**limit_children** | Option<**f64**> |  |  |
**max_tree_depth** | Option<**f64**> |  |  |
**url_id** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |
**context_user_id** | Option<**String**> |  |  |
**hash_tag** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |
**direction** | Option<[**SortDirections**](.md)> |  |  |

### Return type

[**models::GetComments200Response**](GetComments_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_config

> models::GetDomainConfig200Response get_domain_config(tenant_id, domain)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |

### Return type

[**models::GetDomainConfig200Response**](GetDomainConfig_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_configs

> models::GetDomainConfigs200Response get_domain_configs(tenant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |

### Return type

[**models::GetDomainConfigs200Response**](GetDomainConfigs_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts

> models::GetFeedPosts200Response get_feed_posts(tenant_id, after_id, limit, tags)


 req tenantId afterId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**after_id** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetFeedPosts200Response**](GetFeedPosts_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge

> models::GetUserBadge200Response get_user_badge(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetUserBadge200Response**](GetUserBadge_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_by_id

> models::GetUserBadgeProgressById200Response get_user_badge_progress_by_id(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetUserBadgeProgressById200Response**](GetUserBadgeProgressById_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_by_user_id

> models::GetUserBadgeProgressById200Response get_user_badge_progress_by_user_id(tenant_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::GetUserBadgeProgressById200Response**](GetUserBadgeProgressById_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_list

> models::GetUserBadgeProgressList200Response get_user_badge_progress_list(tenant_id, user_id, limit, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetUserBadgeProgressList200Response**](GetUserBadgeProgressList_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badges

> models::GetUserBadges200Response get_user_badges(tenant_id, user_id, badge_id, r#type, displayed_on_comments, limit, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**badge_id** | Option<**String**> |  |  |
**r#type** | Option<**f64**> |  |  |
**displayed_on_comments** | Option<**bool**> |  |  |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetUserBadges200Response**](GetUserBadges_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_domain_config

> models::GetDomainConfig200Response patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain_to_update** | **String** |  | [required] |
**patch_domain_config_params** | [**PatchDomainConfigParams**](PatchDomainConfigParams.md) |  | [required] |

### Return type

[**models::GetDomainConfig200Response**](GetDomainConfig_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_domain_config

> models::GetDomainConfig200Response put_domain_config(tenant_id, domain_to_update, update_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain_to_update** | **String** |  | [required] |
**update_domain_config_params** | [**UpdateDomainConfigParams**](UpdateDomainConfigParams.md) |  | [required] |

### Return type

[**models::GetDomainConfig200Response**](GetDomainConfig_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_comment

> models::SaveComment200Response save_comment(tenant_id, create_comment_params, is_live, do_spam_check, send_emails, populate_notifications)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_comment_params** | [**CreateCommentParams**](CreateCommentParams.md) |  | [required] |
**is_live** | Option<**bool**> |  |  |
**do_spam_check** | Option<**bool**> |  |  |
**send_emails** | Option<**bool**> |  |  |
**populate_notifications** | Option<**bool**> |  |  |

### Return type

[**models::SaveComment200Response**](SaveComment_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_block_user_from_comment

> models::UnBlockCommentPublic200Response un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**un_block_from_comment_params** | [**UnBlockFromCommentParams**](UnBlockFromCommentParams.md) |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::UnBlockCommentPublic200Response**](UnBlockCommentPublic_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_flag_comment

> models::FlagComment200Response un_flag_comment(tenant_id, id, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::FlagComment200Response**](FlagComment_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment

> models::FlagCommentPublic200Response update_comment(tenant_id, id, body, context_user_id, do_spam_check, is_live)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**body** | **models::PickApiCommentPeriodUpdatableCommentFields** |  | [required] |
**context_user_id** | Option<**String**> |  |  |
**do_spam_check** | Option<**bool**> |  |  |
**is_live** | Option<**bool**> |  |  |

### Return type

[**models::FlagCommentPublic200Response**](FlagCommentPublic_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feed_post

> models::FlagCommentPublic200Response update_feed_post(tenant_id, id, feed_post)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**feed_post** | [**FeedPost**](FeedPost.md) |  | [required] |

### Return type

[**models::FlagCommentPublic200Response**](FlagCommentPublic_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_badge

> models::UpdateUserBadge200Response update_user_badge(tenant_id, id, update_user_badge_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_user_badge_params** | [**UpdateUserBadgeParams**](UpdateUserBadgeParams.md) |  | [required] |

### Return type

[**models::UpdateUserBadge200Response**](UpdateUserBadge_200_response.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

