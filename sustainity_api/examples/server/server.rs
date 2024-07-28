//! Main library entry point for sustainity_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use sustainity_api::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        sustainity_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use sustainity_api::{
    Api,
    CheckHealthResponse,
    GetAlternativesResponse,
    GetLibraryResponse,
    GetLibraryItemResponse,
    GetOrganisationResponse,
    GetProductResponse,
    SearchByTextResponse,
};
use sustainity_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Health check
    async fn check_health(
        &self,
        context: &C) -> Result<CheckHealthResponse, ApiError>
    {
        info!("check_health() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get product alternatives.
    async fn get_alternatives(
        &self,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetAlternativesResponse, ApiError>
    {
        info!("get_alternatives(\"{}\", {:?}) - X-Span-ID: {:?}", id, region, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get library contents.
    async fn get_library(
        &self,
        context: &C) -> Result<GetLibraryResponse, ApiError>
    {
        info!("get_library() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get library item.
    async fn get_library_item(
        &self,
        topic: models::LibraryTopic,
        context: &C) -> Result<GetLibraryItemResponse, ApiError>
    {
        info!("get_library_item({:?}) - X-Span-ID: {:?}", topic, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get organisation.
    async fn get_organisation(
        &self,
        organisation_id_variant: models::OrganisationIdVariant,
        id: String,
        context: &C) -> Result<GetOrganisationResponse, ApiError>
    {
        info!("get_organisation({:?}, \"{}\") - X-Span-ID: {:?}", organisation_id_variant, id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get product.
    async fn get_product(
        &self,
        product_id_variant: models::ProductIdVariant,
        id: String,
        region: Option<String>,
        context: &C) -> Result<GetProductResponse, ApiError>
    {
        info!("get_product({:?}, \"{}\", {:?}) - X-Span-ID: {:?}", product_id_variant, id, region, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Text search.
    async fn search_by_text(
        &self,
        query: String,
        context: &C) -> Result<SearchByTextResponse, ApiError>
    {
        info!("search_by_text(\"{}\") - X-Span-ID: {:?}", query, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
