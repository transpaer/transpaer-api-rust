#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, unused_attributes, non_camel_case_types)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::collections::BTreeSet;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};
use crate::server::Authorization;


type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.4.0";

mod auth;
pub use auth::{AuthenticationApi, Claims};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CheckHealthResponse {
    /// Ok
    Ok
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GetLibraryResponse {
    /// Ok
    Ok
    {
        body: models::LibraryContents,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SearchByTextResponse {
    /// Ok
    Ok
    {
        body: models::TextSearchResults,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetCategoryResponse {
    /// Ok
    Ok
    {
        body: models::CategoryFull,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
    ,
    /// Not found
    NotFound
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetLibraryItemResponse {
    /// Ok
    Ok
    {
        body: models::LibraryItemFull,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
    ,
    /// Not found
    NotFound
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetAlternativesResponse {
    /// Ok
    Ok
    {
        body: Vec<models::CategoryAlternatives>,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
    ,
    /// Not found
    NotFound
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetOrganisationResponse {
    /// Ok
    Ok
    {
        body: models::OrganisationFull,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
    ,
    /// Not found
    NotFound
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetProductResponse {
    /// Ok
    Ok
    {
        body: models::ProductFull,
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
    ,
    /// Not found
    NotFound
    {
        access_control_allow_origin:
        String
        ,
        access_control_allow_methods:
        String
        ,
        access_control_allow_headers:
        String
    }
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    /// Health check
    async fn check_health(
        &self,
        context: &C) -> Result<CheckHealthResponse, ApiError>;

    /// Get library contents.
    async fn get_library(
        &self,
        context: &C) -> Result<GetLibraryResponse, ApiError>;

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        context: &C) -> Result<SearchByTextResponse, ApiError>;

    /// Get category.
    async fn get_category(
        &self,
        category: String,
        context: &C) -> Result<GetCategoryResponse, ApiError>;

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: String,
        context: &C) -> Result<GetLibraryItemResponse, ApiError>;

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetAlternativesResponse, ApiError>;

    /// Get organisation.
    async fn get_organisation(
        &self,
        organisation_id_variant: models::OrganisationIdVariant,
        id: String,
        context: &C) -> Result<GetOrganisationResponse, ApiError>;

    /// Get product.
    async fn get_product(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetProductResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn context(&self) -> &C;

    /// Health check
    async fn check_health(
        &self,
        ) -> Result<CheckHealthResponse, ApiError>;

    /// Get library contents.
    async fn get_library(
        &self,
        ) -> Result<GetLibraryResponse, ApiError>;

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        ) -> Result<SearchByTextResponse, ApiError>;

    /// Get category.
    async fn get_category(
        &self,
        category: String,
        ) -> Result<GetCategoryResponse, ApiError>;

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: String,
        ) -> Result<GetLibraryItemResponse, ApiError>;

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        ) -> Result<GetAlternativesResponse, ApiError>;

    /// Get organisation.
    async fn get_organisation(
        &self,
        organisation_id_variant: models::OrganisationIdVariant,
        id: String,
        ) -> Result<GetOrganisationResponse, ApiError>;

    /// Get product.
    async fn get_product(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        ) -> Result<GetProductResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Health check
    async fn check_health(
        &self,
        ) -> Result<CheckHealthResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().check_health(&context).await
    }

    /// Get library contents.
    async fn get_library(
        &self,
        ) -> Result<GetLibraryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_library(&context).await
    }

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        ) -> Result<SearchByTextResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().search_by_text(query, &context).await
    }

    /// Get category.
    async fn get_category(
        &self,
        category: String,
        ) -> Result<GetCategoryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_category(category, &context).await
    }

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: String,
        ) -> Result<GetLibraryItemResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_library_item(topic, &context).await
    }

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        ) -> Result<GetAlternativesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_alternatives(product_id_variant, id, region, &context).await
    }

    /// Get organisation.
    async fn get_organisation(
        &self,
        organisation_id_variant: models::OrganisationIdVariant,
        id: String,
        ) -> Result<GetOrganisationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_organisation(organisation_id_variant, id, &context).await
    }

    /// Get product.
    async fn get_product(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        ) -> Result<GetProductResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_product(product_id_variant, id, region, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
