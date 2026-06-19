# HeaderAccountNotification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | **String** |  | 
**title** | **String** |  | 
**message** | **String** |  | 
**messages_by_locale** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | 
**dates** | Option<**std::collections::HashMap<String, String>**> | Construct a type with a set of properties K of type T | 
**severity** | **String** |  | 
**link_url** | Option<**String**> |  | 
**link_text** | Option<**String**> |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**r#type** | Option<**String**> | Discriminator for notifications with a special layout/click handler (e.g. \"feedback-offer\"). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


