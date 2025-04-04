# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**checkHealth**](default_api.md#checkHealth) | **GET** / | Health check
**getLibrary**](default_api.md#getLibrary) | **GET** /library | Get library contents.
**searchByText**](default_api.md#searchByText) | **GET** /search/text | Text search.
**getLibraryItem**](default_api.md#getLibraryItem) | **GET** /library/{topic} | Get library item.
**getAlternatives**](default_api.md#getAlternatives) | **GET** /product/{productIdVariant}:{id}/alternatives | Get product alternatives.
**getOrganisation**](default_api.md#getOrganisation) | **GET** /organisation/{organisationIdVariant}:{id} | Get organisation.
**getProduct**](default_api.md#getProduct) | **GET** /product/{productIdVariant}:{id} | Get product.


# **checkHealth**
> checkHealth()
Health check

Service health check.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getLibrary**
> models::LibraryContents getLibrary()
Get library contents.

Returns a list of summaries of all library items.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::LibraryContents**](libraryContents.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchByText**
> models::TextSearchResults searchByText(query)
Text search.

Returns results of a search using a text query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **query** | **String**| Text query for search. | 

### Return type

[**models::TextSearchResults**](textSearchResults.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getLibraryItem**
> models::LibraryItemFull getLibraryItem(topic)
Get library item.

Returns a full library item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **topic** | [****](.md)| Library topic. | 

### Return type

[**models::LibraryItemFull**](libraryItemFull.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAlternatives**
> Vec<models::CategoryAlternatives> getAlternatives(product_id_variant, id, optional)
Get product alternatives.

Returns a list of alternative products for each of products category.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **product_id_variant** | [****](.md)| Variant of a product ID. | 
  **id** | **String**| ID of a resource. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **product_id_variant** | [****](.md)| Variant of a product ID. | 
 **id** | **String**| ID of a resource. | 
 **region** | **String**| Region code. | 

### Return type

[**Vec<models::CategoryAlternatives>**](categoryAlternatives.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getOrganisation**
> models::OrganisationFull getOrganisation(organisation_id_variant, id)
Get organisation.

Returns full info about a specified organisation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **organisation_id_variant** | [****](.md)| Variant of an organisation ID. | 
  **id** | **String**| ID of a resource. | 

### Return type

[**models::OrganisationFull**](organisationFull.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getProduct**
> models::ProductFull getProduct(product_id_variant, id, optional)
Get product.

Returns full info about a specified product.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **product_id_variant** | [****](.md)| Variant of a product ID. | 
  **id** | **String**| ID of a resource. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **product_id_variant** | [****](.md)| Variant of a product ID. | 
 **id** | **String**| ID of a resource. | 
 **region** | **String**| Region code. | 

### Return type

[**models::ProductFull**](productFull.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

