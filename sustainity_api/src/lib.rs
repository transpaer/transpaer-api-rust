#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.3.0";

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

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Health check
    async fn check_health(
        &self,
        context: &C) -> Result<CheckHealthResponse, ApiError>;

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetAlternativesResponse, ApiError>;

    /// Get library contents.
    async fn get_library(
        &self,
        context: &C) -> Result<GetLibraryResponse, ApiError>;

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: models::LibraryTopic,
        context: &C) -> Result<GetLibraryItemResponse, ApiError>;

    /// Get organisation.
    async fn get_organisation(
        &self,
        id: String,
        context: &C) -> Result<GetOrganisationResponse, ApiError>;

    /// Get product.
    async fn get_product(
        &self,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetProductResponse, ApiError>;

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        context: &C) -> Result<SearchByTextResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Health check
    async fn check_health(
        &self,
        ) -> Result<CheckHealthResponse, ApiError>;

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        id: String,
        region: Option<String>,
        ) -> Result<GetAlternativesResponse, ApiError>;

    /// Get library contents.
    async fn get_library(
        &self,
        ) -> Result<GetLibraryResponse, ApiError>;

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: models::LibraryTopic,
        ) -> Result<GetLibraryItemResponse, ApiError>;

    /// Get organisation.
    async fn get_organisation(
        &self,
        id: String,
        ) -> Result<GetOrganisationResponse, ApiError>;

    /// Get product.
    async fn get_product(
        &self,
        id: String,
        region: Option<String>,
        ) -> Result<GetProductResponse, ApiError>;

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        ) -> Result<SearchByTextResponse, ApiError>;

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
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

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

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        id: String,
        region: Option<String>,
        ) -> Result<GetAlternativesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_alternatives(id, region, &context).await
    }

    /// Get library contents.
    async fn get_library(
        &self,
        ) -> Result<GetLibraryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_library(&context).await
    }

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: models::LibraryTopic,
        ) -> Result<GetLibraryItemResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_library_item(topic, &context).await
    }

    /// Get organisation.
    async fn get_organisation(
        &self,
        id: String,
        ) -> Result<GetOrganisationResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_organisation(id, &context).await
    }

    /// Get product.
    async fn get_product(
        &self,
        id: String,
        region: Option<String>,
        ) -> Result<GetProductResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_product(id, region, &context).await
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
