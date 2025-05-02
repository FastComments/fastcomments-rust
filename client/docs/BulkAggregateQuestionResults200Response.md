# BulkAggregateQuestionResults200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::ImportedApiStatusPeriodFailed**](ImportedAPIStatus.FAILED.md) |  | 
**data** | [**std::collections::HashMap<String, models::QuestionResultAggregationOverall>**](QuestionResultAggregationOverall.md) | Construct a type with a set of properties K of type T | 
**reason** | **String** |  | 
**code** | **String** |  | 
**secondary_code** | Option<**String**> |  | [optional]
**banned_until** | Option<**f64**> |  | [optional]
**max_character_length** | Option<**f64**> |  | [optional]
**translated_error** | Option<**String**> |  | [optional]
**custom_config** | Option<[**models::CustomConfigParameters**](CustomConfigParameters.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


