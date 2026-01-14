# ApiAuditLog

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**user_id** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**resource_name** | **String** |  | 
**crud_type** | **CrudType** |  (enum: c, r, u, d, login) | 
**from** | Option<**From**> |  (enum: ui, api, cron) | [optional]
**url** | Option<**String**> |  | [optional]
**ip** | Option<**String**> |  | [optional]
**when** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**server_start_date** | Option<**String**> |  | [optional]
**object_details** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Construct a type with a set of properties K of type T | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


