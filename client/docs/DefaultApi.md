# \DefaultApi

All URIs are relative to *https://fastcomments.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain_config**](DefaultApi.md#add_domain_config) | **POST** /api/v1/domain-configs | 
[**add_hash_tag**](DefaultApi.md#add_hash_tag) | **POST** /api/v1/hash-tags | 
[**add_hash_tags_bulk**](DefaultApi.md#add_hash_tags_bulk) | **POST** /api/v1/hash-tags/bulk | 
[**add_page**](DefaultApi.md#add_page) | **POST** /api/v1/pages | 
[**add_sso_user**](DefaultApi.md#add_sso_user) | **POST** /api/v1/sso-users | 
[**aggregate**](DefaultApi.md#aggregate) | **POST** /api/v1/aggregate | 
[**aggregate_question_results**](DefaultApi.md#aggregate_question_results) | **GET** /api/v1/question-results-aggregation | 
[**block_user_from_comment**](DefaultApi.md#block_user_from_comment) | **POST** /api/v1/comments/{id}/block | 
[**bulk_aggregate_question_results**](DefaultApi.md#bulk_aggregate_question_results) | **POST** /api/v1/question-results-aggregation/bulk | 
[**change_ticket_state**](DefaultApi.md#change_ticket_state) | **PATCH** /api/v1/tickets/{id}/state | 
[**combine_comments_with_question_results**](DefaultApi.md#combine_comments_with_question_results) | **GET** /api/v1/question-results-aggregation/combine/comments | 
[**create_email_template**](DefaultApi.md#create_email_template) | **POST** /api/v1/email-templates | 
[**create_feed_post**](DefaultApi.md#create_feed_post) | **POST** /api/v1/feed-posts | 
[**create_moderator**](DefaultApi.md#create_moderator) | **POST** /api/v1/moderators | 
[**create_question_config**](DefaultApi.md#create_question_config) | **POST** /api/v1/question-configs | 
[**create_question_result**](DefaultApi.md#create_question_result) | **POST** /api/v1/question-results | 
[**create_subscription**](DefaultApi.md#create_subscription) | **POST** /api/v1/subscriptions | 
[**create_tenant**](DefaultApi.md#create_tenant) | **POST** /api/v1/tenants | 
[**create_tenant_package**](DefaultApi.md#create_tenant_package) | **POST** /api/v1/tenant-packages | 
[**create_tenant_user**](DefaultApi.md#create_tenant_user) | **POST** /api/v1/tenant-users | 
[**create_ticket**](DefaultApi.md#create_ticket) | **POST** /api/v1/tickets | 
[**create_user_badge**](DefaultApi.md#create_user_badge) | **POST** /api/v1/user-badges | 
[**create_vote**](DefaultApi.md#create_vote) | **POST** /api/v1/votes | 
[**delete_comment**](DefaultApi.md#delete_comment) | **DELETE** /api/v1/comments/{id} | 
[**delete_domain_config**](DefaultApi.md#delete_domain_config) | **DELETE** /api/v1/domain-configs/{domain} | 
[**delete_email_template**](DefaultApi.md#delete_email_template) | **DELETE** /api/v1/email-templates/{id} | 
[**delete_email_template_render_error**](DefaultApi.md#delete_email_template_render_error) | **DELETE** /api/v1/email-templates/{id}/render-errors/{errorId} | 
[**delete_hash_tag**](DefaultApi.md#delete_hash_tag) | **DELETE** /api/v1/hash-tags/{tag} | 
[**delete_moderator**](DefaultApi.md#delete_moderator) | **DELETE** /api/v1/moderators/{id} | 
[**delete_notification_count**](DefaultApi.md#delete_notification_count) | **DELETE** /api/v1/notification-count/{id} | 
[**delete_page**](DefaultApi.md#delete_page) | **DELETE** /api/v1/pages/{id} | 
[**delete_pending_webhook_event**](DefaultApi.md#delete_pending_webhook_event) | **DELETE** /api/v1/pending-webhook-events/{id} | 
[**delete_question_config**](DefaultApi.md#delete_question_config) | **DELETE** /api/v1/question-configs/{id} | 
[**delete_question_result**](DefaultApi.md#delete_question_result) | **DELETE** /api/v1/question-results/{id} | 
[**delete_sso_user**](DefaultApi.md#delete_sso_user) | **DELETE** /api/v1/sso-users/{id} | 
[**delete_subscription**](DefaultApi.md#delete_subscription) | **DELETE** /api/v1/subscriptions/{id} | 
[**delete_tenant**](DefaultApi.md#delete_tenant) | **DELETE** /api/v1/tenants/{id} | 
[**delete_tenant_package**](DefaultApi.md#delete_tenant_package) | **DELETE** /api/v1/tenant-packages/{id} | 
[**delete_tenant_user**](DefaultApi.md#delete_tenant_user) | **DELETE** /api/v1/tenant-users/{id} | 
[**delete_user_badge**](DefaultApi.md#delete_user_badge) | **DELETE** /api/v1/user-badges/{id} | 
[**delete_vote**](DefaultApi.md#delete_vote) | **DELETE** /api/v1/votes/{id} | 
[**flag_comment**](DefaultApi.md#flag_comment) | **POST** /api/v1/comments/{id}/flag | 
[**get_audit_logs**](DefaultApi.md#get_audit_logs) | **GET** /api/v1/audit-logs | 
[**get_cached_notification_count**](DefaultApi.md#get_cached_notification_count) | **GET** /api/v1/notification-count/{id} | 
[**get_comment**](DefaultApi.md#get_comment) | **GET** /api/v1/comments/{id} | 
[**get_comments**](DefaultApi.md#get_comments) | **GET** /api/v1/comments | 
[**get_domain_config**](DefaultApi.md#get_domain_config) | **GET** /api/v1/domain-configs/{domain} | 
[**get_domain_configs**](DefaultApi.md#get_domain_configs) | **GET** /api/v1/domain-configs | 
[**get_email_template**](DefaultApi.md#get_email_template) | **GET** /api/v1/email-templates/{id} | 
[**get_email_template_definitions**](DefaultApi.md#get_email_template_definitions) | **GET** /api/v1/email-templates/definitions | 
[**get_email_template_render_errors**](DefaultApi.md#get_email_template_render_errors) | **GET** /api/v1/email-templates/{id}/render-errors | 
[**get_email_templates**](DefaultApi.md#get_email_templates) | **GET** /api/v1/email-templates | 
[**get_feed_posts**](DefaultApi.md#get_feed_posts) | **GET** /api/v1/feed-posts | 
[**get_hash_tags**](DefaultApi.md#get_hash_tags) | **GET** /api/v1/hash-tags | 
[**get_moderator**](DefaultApi.md#get_moderator) | **GET** /api/v1/moderators/{id} | 
[**get_moderators**](DefaultApi.md#get_moderators) | **GET** /api/v1/moderators | 
[**get_notification_count**](DefaultApi.md#get_notification_count) | **GET** /api/v1/notifications/count | 
[**get_notifications**](DefaultApi.md#get_notifications) | **GET** /api/v1/notifications | 
[**get_page_by_urlid**](DefaultApi.md#get_page_by_urlid) | **GET** /api/v1/pages/by-url-id | 
[**get_pages**](DefaultApi.md#get_pages) | **GET** /api/v1/pages | 
[**get_pending_webhook_event_count**](DefaultApi.md#get_pending_webhook_event_count) | **GET** /api/v1/pending-webhook-events/count | 
[**get_pending_webhook_events**](DefaultApi.md#get_pending_webhook_events) | **GET** /api/v1/pending-webhook-events | 
[**get_question_config**](DefaultApi.md#get_question_config) | **GET** /api/v1/question-configs/{id} | 
[**get_question_configs**](DefaultApi.md#get_question_configs) | **GET** /api/v1/question-configs | 
[**get_question_result**](DefaultApi.md#get_question_result) | **GET** /api/v1/question-results/{id} | 
[**get_question_results**](DefaultApi.md#get_question_results) | **GET** /api/v1/question-results | 
[**get_sso_user_by_email**](DefaultApi.md#get_sso_user_by_email) | **GET** /api/v1/sso-users/by-email/{email} | 
[**get_sso_user_by_id**](DefaultApi.md#get_sso_user_by_id) | **GET** /api/v1/sso-users/by-id/{id} | 
[**get_sso_users**](DefaultApi.md#get_sso_users) | **GET** /api/v1/sso-users | 
[**get_subscriptions**](DefaultApi.md#get_subscriptions) | **GET** /api/v1/subscriptions | 
[**get_tenant**](DefaultApi.md#get_tenant) | **GET** /api/v1/tenants/{id} | 
[**get_tenant_daily_usages**](DefaultApi.md#get_tenant_daily_usages) | **GET** /api/v1/tenant-daily-usage | 
[**get_tenant_package**](DefaultApi.md#get_tenant_package) | **GET** /api/v1/tenant-packages/{id} | 
[**get_tenant_packages**](DefaultApi.md#get_tenant_packages) | **GET** /api/v1/tenant-packages | 
[**get_tenant_user**](DefaultApi.md#get_tenant_user) | **GET** /api/v1/tenant-users/{id} | 
[**get_tenant_users**](DefaultApi.md#get_tenant_users) | **GET** /api/v1/tenant-users | 
[**get_tenants**](DefaultApi.md#get_tenants) | **GET** /api/v1/tenants | 
[**get_ticket**](DefaultApi.md#get_ticket) | **GET** /api/v1/tickets/{id} | 
[**get_tickets**](DefaultApi.md#get_tickets) | **GET** /api/v1/tickets | 
[**get_user**](DefaultApi.md#get_user) | **GET** /api/v1/users/{id} | 
[**get_user_badge**](DefaultApi.md#get_user_badge) | **GET** /api/v1/user-badges/{id} | 
[**get_user_badge_progress_by_id**](DefaultApi.md#get_user_badge_progress_by_id) | **GET** /api/v1/user-badge-progress/{id} | 
[**get_user_badge_progress_by_user_id**](DefaultApi.md#get_user_badge_progress_by_user_id) | **GET** /api/v1/user-badge-progress/user/{userId} | 
[**get_user_badge_progress_list**](DefaultApi.md#get_user_badge_progress_list) | **GET** /api/v1/user-badge-progress | 
[**get_user_badges**](DefaultApi.md#get_user_badges) | **GET** /api/v1/user-badges | 
[**get_votes**](DefaultApi.md#get_votes) | **GET** /api/v1/votes | 
[**get_votes_for_user**](DefaultApi.md#get_votes_for_user) | **GET** /api/v1/votes/for-user | 
[**patch_domain_config**](DefaultApi.md#patch_domain_config) | **PATCH** /api/v1/domain-configs/{domainToUpdate} | 
[**patch_hash_tag**](DefaultApi.md#patch_hash_tag) | **PATCH** /api/v1/hash-tags/{tag} | 
[**patch_page**](DefaultApi.md#patch_page) | **PATCH** /api/v1/pages/{id} | 
[**patch_sso_user**](DefaultApi.md#patch_sso_user) | **PATCH** /api/v1/sso-users/{id} | 
[**put_domain_config**](DefaultApi.md#put_domain_config) | **PUT** /api/v1/domain-configs/{domainToUpdate} | 
[**put_sso_user**](DefaultApi.md#put_sso_user) | **PUT** /api/v1/sso-users/{id} | 
[**render_email_template**](DefaultApi.md#render_email_template) | **POST** /api/v1/email-templates/render | 
[**replace_tenant_package**](DefaultApi.md#replace_tenant_package) | **PUT** /api/v1/tenant-packages/{id} | 
[**replace_tenant_user**](DefaultApi.md#replace_tenant_user) | **PUT** /api/v1/tenant-users/{id} | 
[**save_comment**](DefaultApi.md#save_comment) | **POST** /api/v1/comments | 
[**save_comments_bulk**](DefaultApi.md#save_comments_bulk) | **POST** /api/v1/comments/bulk | 
[**send_invite**](DefaultApi.md#send_invite) | **POST** /api/v1/moderators/{id}/send-invite | 
[**send_login_link**](DefaultApi.md#send_login_link) | **POST** /api/v1/tenant-users/{id}/send-login-link | 
[**un_block_user_from_comment**](DefaultApi.md#un_block_user_from_comment) | **POST** /api/v1/comments/{id}/un-block | 
[**un_flag_comment**](DefaultApi.md#un_flag_comment) | **POST** /api/v1/comments/{id}/un-flag | 
[**update_comment**](DefaultApi.md#update_comment) | **PATCH** /api/v1/comments/{id} | 
[**update_email_template**](DefaultApi.md#update_email_template) | **PATCH** /api/v1/email-templates/{id} | 
[**update_feed_post**](DefaultApi.md#update_feed_post) | **PATCH** /api/v1/feed-posts/{id} | 
[**update_moderator**](DefaultApi.md#update_moderator) | **PATCH** /api/v1/moderators/{id} | 
[**update_notification**](DefaultApi.md#update_notification) | **PATCH** /api/v1/notifications/{id} | 
[**update_question_config**](DefaultApi.md#update_question_config) | **PATCH** /api/v1/question-configs/{id} | 
[**update_question_result**](DefaultApi.md#update_question_result) | **PATCH** /api/v1/question-results/{id} | 
[**update_subscription**](DefaultApi.md#update_subscription) | **PATCH** /api/v1/subscriptions/{id} | 
[**update_tenant**](DefaultApi.md#update_tenant) | **PATCH** /api/v1/tenants/{id} | 
[**update_tenant_package**](DefaultApi.md#update_tenant_package) | **PATCH** /api/v1/tenant-packages/{id} | 
[**update_tenant_user**](DefaultApi.md#update_tenant_user) | **PATCH** /api/v1/tenant-users/{id} | 
[**update_user_badge**](DefaultApi.md#update_user_badge) | **PUT** /api/v1/user-badges/{id} | 



## add_domain_config

> models::AddDomainConfigResponse add_domain_config(tenant_id, add_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**add_domain_config_params** | [**AddDomainConfigParams**](AddDomainConfigParams.md) |  | [required] |

### Return type

[**models::AddDomainConfigResponse**](AddDomainConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_hash_tag

> models::CreateHashTagResponse add_hash_tag(tenant_id, create_hash_tag_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**create_hash_tag_body** | Option<[**CreateHashTagBody**](CreateHashTagBody.md)> |  |  |

### Return type

[**models::CreateHashTagResponse**](CreateHashTagResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_hash_tags_bulk

> models::BulkCreateHashTagsResponse add_hash_tags_bulk(tenant_id, bulk_create_hash_tags_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  |  |
**bulk_create_hash_tags_body** | Option<[**BulkCreateHashTagsBody**](BulkCreateHashTagsBody.md)> |  |  |

### Return type

[**models::BulkCreateHashTagsResponse**](BulkCreateHashTagsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_page

> models::AddPageApiResponse add_page(tenant_id, create_api_page_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_api_page_data** | [**CreateApiPageData**](CreateApiPageData.md) |  | [required] |

### Return type

[**models::AddPageApiResponse**](AddPageAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_sso_user

> models::AddSsoUserApiResponse add_sso_user(tenant_id, create_apisso_user_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_apisso_user_data** | [**CreateApissoUserData**](CreateApissoUserData.md) |  | [required] |

### Return type

[**models::AddSsoUserApiResponse**](AddSSOUserAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aggregate

> models::AggregateResponse aggregate(tenant_id, aggregation_request, parent_tenant_id, include_stats)


Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations. Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**aggregation_request** | [**AggregationRequest**](AggregationRequest.md) |  | [required] |
**parent_tenant_id** | Option<**String**> |  |  |
**include_stats** | Option<**bool**> |  |  |

### Return type

[**models::AggregateResponse**](AggregateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## aggregate_question_results

> models::AggregateQuestionResultsResponse aggregate_question_results(tenant_id, question_id, question_ids, url_id, time_bucket, start_date, force_recalculate)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**question_id** | Option<**String**> |  |  |
**question_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**url_id** | Option<**String**> |  |  |
**time_bucket** | Option<[**AggregateTimeBucket**](AggregateTimeBucket.md)> |  |  |
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**force_recalculate** | Option<**bool**> |  |  |

### Return type

[**models::AggregateQuestionResultsResponse**](AggregateQuestionResultsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_user_from_comment

> models::BlockSuccess block_user_from_comment(tenant_id, id, block_from_comment_params, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**block_from_comment_params** | [**BlockFromCommentParams**](BlockFromCommentParams.md) |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::BlockSuccess**](BlockSuccess.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bulk_aggregate_question_results

> models::BulkAggregateQuestionResultsResponse bulk_aggregate_question_results(tenant_id, bulk_aggregate_question_results_request, force_recalculate)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**bulk_aggregate_question_results_request** | [**BulkAggregateQuestionResultsRequest**](BulkAggregateQuestionResultsRequest.md) |  | [required] |
**force_recalculate** | Option<**bool**> |  |  |

### Return type

[**models::BulkAggregateQuestionResultsResponse**](BulkAggregateQuestionResultsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_ticket_state

> models::ChangeTicketStateResponse change_ticket_state(tenant_id, user_id, id, change_ticket_state_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**change_ticket_state_body** | [**ChangeTicketStateBody**](ChangeTicketStateBody.md) |  | [required] |

### Return type

[**models::ChangeTicketStateResponse**](ChangeTicketStateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## combine_comments_with_question_results

> models::CombineQuestionResultsWithCommentsResponse combine_comments_with_question_results(tenant_id, question_id, question_ids, url_id, start_date, force_recalculate, min_value, max_value, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**question_id** | Option<**String**> |  |  |
**question_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**url_id** | Option<**String**> |  |  |
**start_date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**force_recalculate** | Option<**bool**> |  |  |
**min_value** | Option<**f64**> |  |  |
**max_value** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**models::CombineQuestionResultsWithCommentsResponse**](CombineQuestionResultsWithCommentsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_email_template

> models::CreateEmailTemplateResponse create_email_template(tenant_id, create_email_template_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_email_template_body** | [**CreateEmailTemplateBody**](CreateEmailTemplateBody.md) |  | [required] |

### Return type

[**models::CreateEmailTemplateResponse**](CreateEmailTemplateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_feed_post

> models::CreateFeedPostsResponse create_feed_post(tenant_id, create_feed_post_params, broadcast_id, is_live, do_spam_check, skip_dup_check)


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

[**models::CreateFeedPostsResponse**](CreateFeedPostsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_moderator

> models::CreateModeratorResponse create_moderator(tenant_id, create_moderator_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_moderator_body** | [**CreateModeratorBody**](CreateModeratorBody.md) |  | [required] |

### Return type

[**models::CreateModeratorResponse**](CreateModeratorResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_question_config

> models::CreateQuestionConfigResponse create_question_config(tenant_id, create_question_config_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_question_config_body** | [**CreateQuestionConfigBody**](CreateQuestionConfigBody.md) |  | [required] |

### Return type

[**models::CreateQuestionConfigResponse**](CreateQuestionConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_question_result

> models::CreateQuestionResultResponse create_question_result(tenant_id, create_question_result_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_question_result_body** | [**CreateQuestionResultBody**](CreateQuestionResultBody.md) |  | [required] |

### Return type

[**models::CreateQuestionResultResponse**](CreateQuestionResultResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subscription

> models::CreateSubscriptionApiResponse create_subscription(tenant_id, create_api_user_subscription_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_api_user_subscription_data** | [**CreateApiUserSubscriptionData**](CreateApiUserSubscriptionData.md) |  | [required] |

### Return type

[**models::CreateSubscriptionApiResponse**](CreateSubscriptionAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant

> models::CreateTenantResponse create_tenant(tenant_id, create_tenant_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_tenant_body** | [**CreateTenantBody**](CreateTenantBody.md) |  | [required] |

### Return type

[**models::CreateTenantResponse**](CreateTenantResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant_package

> models::CreateTenantPackageResponse create_tenant_package(tenant_id, create_tenant_package_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_tenant_package_body** | [**CreateTenantPackageBody**](CreateTenantPackageBody.md) |  | [required] |

### Return type

[**models::CreateTenantPackageResponse**](CreateTenantPackageResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_tenant_user

> models::CreateTenantUserResponse create_tenant_user(tenant_id, create_tenant_user_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_tenant_user_body** | [**CreateTenantUserBody**](CreateTenantUserBody.md) |  | [required] |

### Return type

[**models::CreateTenantUserResponse**](CreateTenantUserResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_ticket

> models::CreateTicketResponse create_ticket(tenant_id, user_id, create_ticket_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |
**create_ticket_body** | [**CreateTicketBody**](CreateTicketBody.md) |  | [required] |

### Return type

[**models::CreateTicketResponse**](CreateTicketResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_badge

> models::ApiCreateUserBadgeResponse create_user_badge(tenant_id, create_user_badge_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_user_badge_params** | [**CreateUserBadgeParams**](CreateUserBadgeParams.md) |  | [required] |

### Return type

[**models::ApiCreateUserBadgeResponse**](APICreateUserBadgeResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_vote

> models::VoteResponse create_vote(tenant_id, comment_id, direction, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | **String** |  | [required] |
**direction** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::VoteResponse**](VoteResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_comment

> models::DeleteCommentResult delete_comment(tenant_id, id, context_user_id, is_live)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**context_user_id** | Option<**String**> |  |  |
**is_live** | Option<**bool**> |  |  |

### Return type

[**models::DeleteCommentResult**](DeleteCommentResult.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain_config

> models::DeleteDomainConfigResponse delete_domain_config(tenant_id, domain)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |

### Return type

[**models::DeleteDomainConfigResponse**](DeleteDomainConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_template

> models::ApiEmptyResponse delete_email_template(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_email_template_render_error

> models::ApiEmptyResponse delete_email_template_render_error(tenant_id, id, error_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**error_id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_hash_tag

> models::ApiEmptyResponse delete_hash_tag(tag, tenant_id, delete_hash_tag_request_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**delete_hash_tag_request_body** | Option<[**DeleteHashTagRequestBody**](DeleteHashTagRequestBody.md)> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_moderator

> models::ApiEmptyResponse delete_moderator(tenant_id, id, send_email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**send_email** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_notification_count

> models::ApiEmptyResponse delete_notification_count(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_page

> models::DeletePageApiResponse delete_page(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::DeletePageApiResponse**](DeletePageAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pending_webhook_event

> models::ApiEmptyResponse delete_pending_webhook_event(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_question_config

> models::ApiEmptyResponse delete_question_config(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_question_result

> models::ApiEmptyResponse delete_question_result(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sso_user

> models::DeleteSsoUserApiResponse delete_sso_user(tenant_id, id, delete_comments, comment_delete_mode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**delete_comments** | Option<**bool**> |  |  |
**comment_delete_mode** | Option<**String**> |  |  |

### Return type

[**models::DeleteSsoUserApiResponse**](DeleteSSOUserAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> models::DeleteSubscriptionApiResponse delete_subscription(tenant_id, id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::DeleteSubscriptionApiResponse**](DeleteSubscriptionAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant

> models::ApiEmptyResponse delete_tenant(tenant_id, id, sure)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**sure** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_package

> models::ApiEmptyResponse delete_tenant_package(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_tenant_user

> models::ApiEmptyResponse delete_tenant_user(tenant_id, id, delete_comments, comment_delete_mode)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**delete_comments** | Option<**String**> |  |  |
**comment_delete_mode** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_badge

> models::ApiEmptySuccessResponse delete_user_badge(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiEmptySuccessResponse**](APIEmptySuccessResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_vote

> models::VoteDeleteResponse delete_vote(tenant_id, id, edit_key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**edit_key** | Option<**String**> |  |  |

### Return type

[**models::VoteDeleteResponse**](VoteDeleteResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## flag_comment

> models::FlagCommentResponse flag_comment(tenant_id, id, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::FlagCommentResponse**](FlagCommentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_audit_logs

> models::GetAuditLogsResponse get_audit_logs(tenant_id, limit, skip, order, after, before)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**order** | Option<[**SortDir**](SortDir.md)> |  |  |
**after** | Option<**f64**> |  |  |
**before** | Option<**f64**> |  |  |

### Return type

[**models::GetAuditLogsResponse**](GetAuditLogsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cached_notification_count

> models::GetCachedNotificationCountResponse get_cached_notification_count(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetCachedNotificationCountResponse**](GetCachedNotificationCountResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comment

> models::ApiGetCommentResponse get_comment(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiGetCommentResponse**](APIGetCommentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_comments

> models::ApiGetCommentsResponse get_comments(tenant_id, page, limit, skip, as_tree, skip_children, limit_children, max_tree_depth, url_id, user_id, anon_user_id, context_user_id, hash_tag, parent_id, direction, from_date, to_date)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**as_tree** | Option<**bool**> |  |  |
**skip_children** | Option<**i32**> |  |  |
**limit_children** | Option<**i32**> |  |  |
**max_tree_depth** | Option<**i32**> |  |  |
**url_id** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |
**context_user_id** | Option<**String**> |  |  |
**hash_tag** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |
**direction** | Option<[**SortDirections**](SortDirections.md)> |  |  |
**from_date** | Option<**i64**> |  |  |
**to_date** | Option<**i64**> |  |  |

### Return type

[**models::ApiGetCommentsResponse**](APIGetCommentsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_config

> models::GetDomainConfigResponse get_domain_config(tenant_id, domain)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain** | **String** |  | [required] |

### Return type

[**models::GetDomainConfigResponse**](GetDomainConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_configs

> models::GetDomainConfigsResponse get_domain_configs(tenant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |

### Return type

[**models::GetDomainConfigsResponse**](GetDomainConfigsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_template

> models::GetEmailTemplateResponse get_email_template(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetEmailTemplateResponse**](GetEmailTemplateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_template_definitions

> models::GetEmailTemplateDefinitionsResponse get_email_template_definitions(tenant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |

### Return type

[**models::GetEmailTemplateDefinitionsResponse**](GetEmailTemplateDefinitionsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_template_render_errors

> models::GetEmailTemplateRenderErrorsResponse get_email_template_render_errors(tenant_id, id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetEmailTemplateRenderErrorsResponse**](GetEmailTemplateRenderErrorsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_email_templates

> models::GetEmailTemplatesResponse get_email_templates(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetEmailTemplatesResponse**](GetEmailTemplatesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feed_posts

> models::GetFeedPostsResponse get_feed_posts(tenant_id, after_id, limit, tags)


 req tenantId afterId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**after_id** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |
**tags** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::GetFeedPostsResponse**](GetFeedPostsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hash_tags

> models::GetHashTagsResponse get_hash_tags(tenant_id, page)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |

### Return type

[**models::GetHashTagsResponse**](GetHashTagsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderator

> models::GetModeratorResponse get_moderator(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetModeratorResponse**](GetModeratorResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_moderators

> models::GetModeratorsResponse get_moderators(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetModeratorsResponse**](GetModeratorsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notification_count

> models::GetNotificationCountResponse get_notification_count(tenant_id, user_id, url_id, from_comment_id, viewed, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**url_id** | Option<**String**> |  |  |
**from_comment_id** | Option<**String**> |  |  |
**viewed** | Option<**bool**> |  |  |
**r#type** | Option<**String**> |  |  |

### Return type

[**models::GetNotificationCountResponse**](GetNotificationCountResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_notifications

> models::GetNotificationsResponse get_notifications(tenant_id, user_id, url_id, from_comment_id, viewed, r#type, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**url_id** | Option<**String**> |  |  |
**from_comment_id** | Option<**String**> |  |  |
**viewed** | Option<**bool**> |  |  |
**r#type** | Option<**String**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetNotificationsResponse**](GetNotificationsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_page_by_urlid

> models::GetPageByUrlidApiResponse get_page_by_urlid(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetPageByUrlidApiResponse**](GetPageByURLIdAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pages

> models::GetPagesApiResponse get_pages(tenant_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |

### Return type

[**models::GetPagesApiResponse**](GetPagesAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_webhook_event_count

> models::GetPendingWebhookEventCountResponse get_pending_webhook_event_count(tenant_id, comment_id, external_id, event_type, r#type, domain, attempt_count_gt)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | Option<**String**> |  |  |
**external_id** | Option<**String**> |  |  |
**event_type** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**attempt_count_gt** | Option<**f64**> |  |  |

### Return type

[**models::GetPendingWebhookEventCountResponse**](GetPendingWebhookEventCountResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_webhook_events

> models::GetPendingWebhookEventsResponse get_pending_webhook_events(tenant_id, comment_id, external_id, event_type, r#type, domain, attempt_count_gt, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**comment_id** | Option<**String**> |  |  |
**external_id** | Option<**String**> |  |  |
**event_type** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**domain** | Option<**String**> |  |  |
**attempt_count_gt** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetPendingWebhookEventsResponse**](GetPendingWebhookEventsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_question_config

> models::GetQuestionConfigResponse get_question_config(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetQuestionConfigResponse**](GetQuestionConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_question_configs

> models::GetQuestionConfigsResponse get_question_configs(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetQuestionConfigsResponse**](GetQuestionConfigsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_question_result

> models::GetQuestionResultResponse get_question_result(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetQuestionResultResponse**](GetQuestionResultResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_question_results

> models::GetQuestionResultsResponse get_question_results(tenant_id, url_id, user_id, start_date, question_id, question_ids, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | Option<**String**> |  |  |
**user_id** | Option<**String**> |  |  |
**start_date** | Option<**String**> |  |  |
**question_id** | Option<**String**> |  |  |
**question_ids** | Option<**String**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetQuestionResultsResponse**](GetQuestionResultsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sso_user_by_email

> models::GetSsoUserByEmailApiResponse get_sso_user_by_email(tenant_id, email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**email** | **String** |  | [required] |

### Return type

[**models::GetSsoUserByEmailApiResponse**](GetSSOUserByEmailAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sso_user_by_id

> models::GetSsoUserByIdApiResponse get_sso_user_by_id(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetSsoUserByIdApiResponse**](GetSSOUserByIdAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sso_users

> models::GetSsoUsersResponse get_sso_users(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**i32**> |  |  |

### Return type

[**models::GetSsoUsersResponse**](GetSSOUsersResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> models::GetSubscriptionsApiResponse get_subscriptions(tenant_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::GetSubscriptionsApiResponse**](GetSubscriptionsAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant

> models::GetTenantResponse get_tenant(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetTenantResponse**](GetTenantResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_daily_usages

> models::GetTenantDailyUsagesResponse get_tenant_daily_usages(tenant_id, year_number, month_number, day_number, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**year_number** | Option<**f64**> |  |  |
**month_number** | Option<**f64**> |  |  |
**day_number** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetTenantDailyUsagesResponse**](GetTenantDailyUsagesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_package

> models::GetTenantPackageResponse get_tenant_package(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetTenantPackageResponse**](GetTenantPackageResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_packages

> models::GetTenantPackagesResponse get_tenant_packages(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetTenantPackagesResponse**](GetTenantPackagesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_user

> models::GetTenantUserResponse get_tenant_user(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetTenantUserResponse**](GetTenantUserResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_users

> models::GetTenantUsersResponse get_tenant_users(tenant_id, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetTenantUsersResponse**](GetTenantUsersResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenants

> models::GetTenantsResponse get_tenants(tenant_id, meta, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**meta** | Option<**String**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::GetTenantsResponse**](GetTenantsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket

> models::GetTicketResponse get_ticket(tenant_id, id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::GetTicketResponse**](GetTicketResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tickets

> models::GetTicketsResponse get_tickets(tenant_id, user_id, state, skip, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**state** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**models::GetTicketsResponse**](GetTicketsResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::GetUserResponse get_user(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::GetUserResponse**](GetUserResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge

> models::ApiGetUserBadgeResponse get_user_badge(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiGetUserBadgeResponse**](APIGetUserBadgeResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_by_id

> models::ApiGetUserBadgeProgressResponse get_user_badge_progress_by_id(tenant_id, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**models::ApiGetUserBadgeProgressResponse**](APIGetUserBadgeProgressResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_by_user_id

> models::ApiGetUserBadgeProgressResponse get_user_badge_progress_by_user_id(tenant_id, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::ApiGetUserBadgeProgressResponse**](APIGetUserBadgeProgressResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badge_progress_list

> models::ApiGetUserBadgeProgressListResponse get_user_badge_progress_list(tenant_id, user_id, limit, skip)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**skip** | Option<**f64**> |  |  |

### Return type

[**models::ApiGetUserBadgeProgressListResponse**](APIGetUserBadgeProgressListResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_badges

> models::ApiGetUserBadgesResponse get_user_badges(tenant_id, user_id, badge_id, r#type, displayed_on_comments, limit, skip)


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

[**models::ApiGetUserBadgesResponse**](APIGetUserBadgesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_votes

> models::GetVotesResponse get_votes(tenant_id, url_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |

### Return type

[**models::GetVotesResponse**](GetVotesResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_votes_for_user

> models::GetVotesForUserResponse get_votes_for_user(tenant_id, url_id, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**url_id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::GetVotesForUserResponse**](GetVotesForUserResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_domain_config

> models::PatchDomainConfigResponse patch_domain_config(tenant_id, domain_to_update, patch_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain_to_update** | **String** |  | [required] |
**patch_domain_config_params** | [**PatchDomainConfigParams**](PatchDomainConfigParams.md) |  | [required] |

### Return type

[**models::PatchDomainConfigResponse**](PatchDomainConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_hash_tag

> models::UpdateHashTagResponse patch_hash_tag(tag, tenant_id, update_hash_tag_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tag** | **String** |  | [required] |
**tenant_id** | Option<**String**> |  |  |
**update_hash_tag_body** | Option<[**UpdateHashTagBody**](UpdateHashTagBody.md)> |  |  |

### Return type

[**models::UpdateHashTagResponse**](UpdateHashTagResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_page

> models::PatchPageApiResponse patch_page(tenant_id, id, update_api_page_data)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_api_page_data** | [**UpdateApiPageData**](UpdateApiPageData.md) |  | [required] |

### Return type

[**models::PatchPageApiResponse**](PatchPageAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_sso_user

> models::PatchSsoUserApiResponse patch_sso_user(tenant_id, id, update_apisso_user_data, update_comments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_apisso_user_data** | [**UpdateApissoUserData**](UpdateApissoUserData.md) |  | [required] |
**update_comments** | Option<**bool**> |  |  |

### Return type

[**models::PatchSsoUserApiResponse**](PatchSSOUserAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_domain_config

> models::PutDomainConfigResponse put_domain_config(tenant_id, domain_to_update, update_domain_config_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**domain_to_update** | **String** |  | [required] |
**update_domain_config_params** | [**UpdateDomainConfigParams**](UpdateDomainConfigParams.md) |  | [required] |

### Return type

[**models::PutDomainConfigResponse**](PutDomainConfigResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_sso_user

> models::PutSsoUserApiResponse put_sso_user(tenant_id, id, update_apisso_user_data, update_comments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_apisso_user_data** | [**UpdateApissoUserData**](UpdateApissoUserData.md) |  | [required] |
**update_comments** | Option<**bool**> |  |  |

### Return type

[**models::PutSsoUserApiResponse**](PutSSOUserAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## render_email_template

> models::RenderEmailTemplateResponse render_email_template(tenant_id, render_email_template_body, locale)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**render_email_template_body** | [**RenderEmailTemplateBody**](RenderEmailTemplateBody.md) |  | [required] |
**locale** | Option<**String**> |  |  |

### Return type

[**models::RenderEmailTemplateResponse**](RenderEmailTemplateResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_tenant_package

> models::ApiEmptyResponse replace_tenant_package(tenant_id, id, replace_tenant_package_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**replace_tenant_package_body** | [**ReplaceTenantPackageBody**](ReplaceTenantPackageBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_tenant_user

> models::ApiEmptyResponse replace_tenant_user(tenant_id, id, replace_tenant_user_body, update_comments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**replace_tenant_user_body** | [**ReplaceTenantUserBody**](ReplaceTenantUserBody.md) |  | [required] |
**update_comments** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_comment

> models::ApiSaveCommentResponse save_comment(tenant_id, create_comment_params, is_live, do_spam_check, send_emails, populate_notifications)


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

[**models::ApiSaveCommentResponse**](APISaveCommentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_comments_bulk

> Vec<models::SaveCommentsBulkResponse> save_comments_bulk(tenant_id, create_comment_params, is_live, do_spam_check, send_emails, populate_notifications)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**create_comment_params** | [**Vec<models::CreateCommentParams>**](CreateCommentParams.md) |  | [required] |
**is_live** | Option<**bool**> |  |  |
**do_spam_check** | Option<**bool**> |  |  |
**send_emails** | Option<**bool**> |  |  |
**populate_notifications** | Option<**bool**> |  |  |

### Return type

[**Vec<models::SaveCommentsBulkResponse>**](SaveCommentsBulkResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_invite

> models::ApiEmptyResponse send_invite(tenant_id, id, from_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**from_name** | **String** |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_login_link

> models::ApiEmptyResponse send_login_link(tenant_id, id, redirect_url)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**redirect_url** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_block_user_from_comment

> models::UnblockSuccess un_block_user_from_comment(tenant_id, id, un_block_from_comment_params, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**un_block_from_comment_params** | [**UnBlockFromCommentParams**](UnBlockFromCommentParams.md) |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::UnblockSuccess**](UnblockSuccess.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_flag_comment

> models::FlagCommentResponse un_flag_comment(tenant_id, id, user_id, anon_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**user_id** | Option<**String**> |  |  |
**anon_user_id** | Option<**String**> |  |  |

### Return type

[**models::FlagCommentResponse**](FlagCommentResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_comment

> models::ApiEmptyResponse update_comment(tenant_id, id, updatable_comment_params, context_user_id, do_spam_check, is_live)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**updatable_comment_params** | [**UpdatableCommentParams**](UpdatableCommentParams.md) |  | [required] |
**context_user_id** | Option<**String**> |  |  |
**do_spam_check** | Option<**bool**> |  |  |
**is_live** | Option<**bool**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email_template

> models::ApiEmptyResponse update_email_template(tenant_id, id, update_email_template_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_email_template_body** | [**UpdateEmailTemplateBody**](UpdateEmailTemplateBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_feed_post

> models::ApiEmptyResponse update_feed_post(tenant_id, id, feed_post)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**feed_post** | [**FeedPost**](FeedPost.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_moderator

> models::ApiEmptyResponse update_moderator(tenant_id, id, update_moderator_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_moderator_body** | [**UpdateModeratorBody**](UpdateModeratorBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_notification

> models::ApiEmptyResponse update_notification(tenant_id, id, update_notification_body, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_notification_body** | [**UpdateNotificationBody**](UpdateNotificationBody.md) |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_question_config

> models::ApiEmptyResponse update_question_config(tenant_id, id, update_question_config_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_question_config_body** | [**UpdateQuestionConfigBody**](UpdateQuestionConfigBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_question_result

> models::ApiEmptyResponse update_question_result(tenant_id, id, update_question_result_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_question_result_body** | [**UpdateQuestionResultBody**](UpdateQuestionResultBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription

> models::UpdateSubscriptionApiResponse update_subscription(tenant_id, id, update_api_user_subscription_data, user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_api_user_subscription_data** | [**UpdateApiUserSubscriptionData**](UpdateApiUserSubscriptionData.md) |  | [required] |
**user_id** | Option<**String**> |  |  |

### Return type

[**models::UpdateSubscriptionApiResponse**](UpdateSubscriptionAPIResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant

> models::ApiEmptyResponse update_tenant(tenant_id, id, update_tenant_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_tenant_body** | [**UpdateTenantBody**](UpdateTenantBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_package

> models::ApiEmptyResponse update_tenant_package(tenant_id, id, update_tenant_package_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_tenant_package_body** | [**UpdateTenantPackageBody**](UpdateTenantPackageBody.md) |  | [required] |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant_user

> models::ApiEmptyResponse update_tenant_user(tenant_id, id, update_tenant_user_body, update_comments)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_tenant_user_body** | [**UpdateTenantUserBody**](UpdateTenantUserBody.md) |  | [required] |
**update_comments** | Option<**String**> |  |  |

### Return type

[**models::ApiEmptyResponse**](APIEmptyResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_badge

> models::ApiEmptySuccessResponse update_user_badge(tenant_id, id, update_user_badge_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** |  | [required] |
**id** | **String** |  | [required] |
**update_user_badge_params** | [**UpdateUserBadgeParams**](UpdateUserBadgeParams.md) |  | [required] |

### Return type

[**models::ApiEmptySuccessResponse**](APIEmptySuccessResponse.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

