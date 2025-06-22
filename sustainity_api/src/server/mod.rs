use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::{models, header, AuthenticationApi};

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     CheckHealthResponse,
     GetLibraryResponse,
     SearchByTextResponse,
     GetLibraryItemResponse,
     GetAlternativesResponse,
     GetOrganisationResponse,
     GetProductResponse
};

mod server_auth;

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/$",
            r"^/library$",
            r"^/library/(?P<topic>[^/?#]*)$",
            r"^/organisation/(?P<organisationIdVariant>[^/?#]*):(?P<id>[^/?#]*)$",
            r"^/product/(?P<productIdVariant>[^/?#]*):(?P<id>[^/?#]*)$",
            r"^/product/(?P<productIdVariant>[^/?#]*):(?P<id>[^/?#]*)/alternatives$",
            r"^/search/text$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_: usize = 0;
    pub(crate) static ID_LIBRARY: usize = 1;
    pub(crate) static ID_LIBRARY_TOPIC: usize = 2;
    lazy_static! {
        pub static ref REGEX_LIBRARY_TOPIC: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/library/(?P<topic>[^/?#]*)$")
                .expect("Unable to create regex for LIBRARY_TOPIC");
    }
    pub(crate) static ID_ORGANISATION_ORGANISATIONIDVARIANT_ID: usize = 3;
    lazy_static! {
        pub static ref REGEX_ORGANISATION_ORGANISATIONIDVARIANT_ID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/organisation/(?P<organisationIdVariant>[^/?#]*):(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for ORGANISATION_ORGANISATIONIDVARIANT_ID");
    }
    pub(crate) static ID_PRODUCT_PRODUCTIDVARIANT_ID: usize = 4;
    lazy_static! {
        pub static ref REGEX_PRODUCT_PRODUCTIDVARIANT_ID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/product/(?P<productIdVariant>[^/?#]*):(?P<id>[^/?#]*)$")
                .expect("Unable to create regex for PRODUCT_PRODUCTIDVARIANT_ID");
    }
    pub(crate) static ID_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES: usize = 5;
    lazy_static! {
        pub static ref REGEX_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/product/(?P<productIdVariant>[^/?#]*):(?P<id>[^/?#]*)/alternatives$")
                .expect("Unable to create regex for PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES");
    }
    pub(crate) static ID_SEARCH_TEXT: usize = 6;
}


pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        let service = Service::new(self.api_impl.clone());

        future::ok(service)
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future {
        async fn run<T, C>(
            mut api_impl: T,
            req: (Request<Body>, C),
        ) -> Result<Response<Body>, crate::ServiceError> where
            T: Api<C> + Clone + Send + 'static,
            C: Has<XSpanIdString>  + Send + Sync + 'static
        {
            let (request, context) = req;
            let (parts, body) = request.into_parts();
            let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
            let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

            match method {

            // CheckHealth - GET /
            hyper::Method::GET if path.matched(paths::ID_) => {
                                let result = api_impl.check_health(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CheckHealthResponse::Ok
                                                    {
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetLibrary - GET /library
            hyper::Method::GET if path.matched(paths::ID_LIBRARY) => {
                                let result = api_impl.get_library(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLibraryResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // SearchByText - GET /search/text
            hyper::Method::GET if path.matched(paths::ID_SEARCH_TEXT) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_query = query_params.iter().filter(|e| e.0 == "query").map(|e| e.1.clone())
                    .next();
                let param_query = match param_query {
                    Some(param_query) => {
                        let param_query =
                            <String as std::str::FromStr>::from_str
                                (&param_query);
                        match param_query {
                            Ok(param_query) => Some(param_query),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter query - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter query")),
                        }
                    },
                    None => None,
                };
                let param_query = match param_query {
                    Some(param_query) => param_query,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter query"))
                        .expect("Unable to create Bad Request response for missing query parameter query")),
                };

                                let result = api_impl.search_by_text(
                                            param_query,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                SearchByTextResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetLibraryItem - GET /library/{topic}
            hyper::Method::GET if path.matched(paths::ID_LIBRARY_TOPIC) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_LIBRARY_TOPIC
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE LIBRARY_TOPIC in set but failed match against \"{}\"", path, paths::REGEX_LIBRARY_TOPIC.as_str())
                    );

                let param_topic = match percent_encoding::percent_decode(path_params["topic"].as_bytes()).decode_utf8() {
                    Ok(param_topic) => match param_topic.parse::<models::LibraryTopic>() {
                        Ok(param_topic) => param_topic,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter topic: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["topic"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_library_item(
                                            param_topic,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetLibraryItemResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                                GetLibraryItemResponse::NotFound
                                                    {
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetAlternatives - GET /product/{productIdVariant}:{id}/alternatives
            hyper::Method::GET if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES in set but failed match against \"{}\"", path, paths::REGEX_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES.as_str())
                    );

                let param_product_id_variant = match percent_encoding::percent_decode(path_params["productIdVariant"].as_bytes()).decode_utf8() {
                    Ok(param_product_id_variant) => match param_product_id_variant.parse::<models::ProductIdVariant>() {
                        Ok(param_product_id_variant) => param_product_id_variant,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter productIdVariant: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["productIdVariant"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<String>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_region = query_params.iter().filter(|e| e.0 == "region").map(|e| e.1.clone())
                    .next();
                let param_region = match param_region {
                    Some(param_region) => {
                        let param_region =
                            <String as std::str::FromStr>::from_str
                                (&param_region);
                        match param_region {
                            Ok(param_region) => Some(param_region),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter region - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter region")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_alternatives(
                                            param_product_id_variant,
                                            param_id,
                                            param_region,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetAlternativesResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                                GetAlternativesResponse::NotFound
                                                    {
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetOrganisation - GET /organisation/{organisationIdVariant}:{id}
            hyper::Method::GET if path.matched(paths::ID_ORGANISATION_ORGANISATIONIDVARIANT_ID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ORGANISATION_ORGANISATIONIDVARIANT_ID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ORGANISATION_ORGANISATIONIDVARIANT_ID in set but failed match against \"{}\"", path, paths::REGEX_ORGANISATION_ORGANISATIONIDVARIANT_ID.as_str())
                    );

                let param_organisation_id_variant = match percent_encoding::percent_decode(path_params["organisationIdVariant"].as_bytes()).decode_utf8() {
                    Ok(param_organisation_id_variant) => match param_organisation_id_variant.parse::<models::OrganisationIdVariant>() {
                        Ok(param_organisation_id_variant) => param_organisation_id_variant,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter organisationIdVariant: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["organisationIdVariant"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<String>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_organisation(
                                            param_organisation_id_variant,
                                            param_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetOrganisationResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                                GetOrganisationResponse::NotFound
                                                    {
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetProduct - GET /product/{productIdVariant}:{id}
            hyper::Method::GET if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_PRODUCT_PRODUCTIDVARIANT_ID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE PRODUCT_PRODUCTIDVARIANT_ID in set but failed match against \"{}\"", path, paths::REGEX_PRODUCT_PRODUCTIDVARIANT_ID.as_str())
                    );

                let param_product_id_variant = match percent_encoding::percent_decode(path_params["productIdVariant"].as_bytes()).decode_utf8() {
                    Ok(param_product_id_variant) => match param_product_id_variant.parse::<models::ProductIdVariant>() {
                        Ok(param_product_id_variant) => param_product_id_variant,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter productIdVariant: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["productIdVariant"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_id = match percent_encoding::percent_decode(path_params["id"].as_bytes()).decode_utf8() {
                    Ok(param_id) => match param_id.parse::<String>() {
                        Ok(param_id) => param_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter id: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["id"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_region = query_params.iter().filter(|e| e.0 == "region").map(|e| e.1.clone())
                    .next();
                let param_region = match param_region {
                    Some(param_region) => {
                        let param_region =
                            <String as std::str::FromStr>::from_str
                                (&param_region);
                        match param_region {
                            Ok(param_region) => Some(param_region),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter region - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter region")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_product(
                                            param_product_id_variant,
                                            param_id,
                                            param_region,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetProductResponse::Ok
                                                    {
                                                        body,
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for application/json"));
                                                    // JSON Body
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);

                                                },
                                                GetProductResponse::NotFound
                                                    {
                                                        access_control_allow_origin,
                                                        access_control_allow_methods,
                                                        access_control_allow_headers
                                                    }
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");

                                                    let access_control_allow_origin = match header::IntoHeaderValue(access_control_allow_origin).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_origin header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-origin"),
                                                        access_control_allow_origin
                                                    );

                                                    let access_control_allow_methods = match header::IntoHeaderValue(access_control_allow_methods).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_methods header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-methods"),
                                                        access_control_allow_methods
                                                    );

                                                    let access_control_allow_headers = match header::IntoHeaderValue(access_control_allow_headers).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling access_control_allow_headers header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("access-control-allow-headers"),
                                                        access_control_allow_headers
                                                    );

                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_) => method_not_allowed(),
            _ if path.matched(paths::ID_LIBRARY) => method_not_allowed(),
            _ if path.matched(paths::ID_LIBRARY_TOPIC) => method_not_allowed(),
            _ if path.matched(paths::ID_ORGANISATION_ORGANISATIONIDVARIANT_ID) => method_not_allowed(),
            _ if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID) => method_not_allowed(),
            _ if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES) => method_not_allowed(),
            _ if path.matched(paths::ID_SEARCH_TEXT) => method_not_allowed(),
                _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .expect("Unable to create Not Found response"))
            }
        }
        Box::pin(run(
            self.api_impl.clone(),
            req,
        ))
    }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // CheckHealth - GET /
            hyper::Method::GET if path.matched(paths::ID_) => Some("CheckHealth"),
            // GetLibrary - GET /library
            hyper::Method::GET if path.matched(paths::ID_LIBRARY) => Some("GetLibrary"),
            // SearchByText - GET /search/text
            hyper::Method::GET if path.matched(paths::ID_SEARCH_TEXT) => Some("SearchByText"),
            // GetLibraryItem - GET /library/{topic}
            hyper::Method::GET if path.matched(paths::ID_LIBRARY_TOPIC) => Some("GetLibraryItem"),
            // GetAlternatives - GET /product/{productIdVariant}:{id}/alternatives
            hyper::Method::GET if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID_ALTERNATIVES) => Some("GetAlternatives"),
            // GetOrganisation - GET /organisation/{organisationIdVariant}:{id}
            hyper::Method::GET if path.matched(paths::ID_ORGANISATION_ORGANISATIONIDVARIANT_ID) => Some("GetOrganisation"),
            // GetProduct - GET /product/{productIdVariant}:{id}
            hyper::Method::GET if path.matched(paths::ID_PRODUCT_PRODUCTIDVARIANT_ID) => Some("GetProduct"),
            _ => None,
        }
    }
}
