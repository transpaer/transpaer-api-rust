//! CLI tool driving the API client
use anyhow::{anyhow, Context, Result};
use log::{debug, info};
// models may be unused if all inputs are primitive types
#[allow(unused_imports)]
use sustainity_api::{
    models, ApiNoContext, Client, ContextWrapperExt,
    CheckHealthResponse,
    GetLibraryResponse,
    SearchByTextResponse,
    GetLibraryItemResponse,
    GetAlternativesResponse,
    GetOrganisationResponse,
    GetProductResponse,
};
use simple_logger::SimpleLogger;
use structopt::StructOpt;
use swagger::{AuthData, ContextBuilder, EmptyContext, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(
    ContextBuilder,
    EmptyContext,
    Option<AuthData>,
    XSpanIdString
);

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Sustainity",
    version = "0.4.0",
    about = "CLI access to Sustainity"
)]
struct Cli {
    #[structopt(subcommand)]
    operation: Operation,

    /// Address or hostname of the server hosting this API, including optional port
    #[structopt(short = "a", long, default_value = "http://localhost")]
    server_address: String,

    /// Path to the client private key if using client-side TLS authentication
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long, requires_all(&["client-certificate", "server-certificate"]))]
    client_key: Option<String>,

    /// Path to the client's public certificate associated with the private key
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long, requires_all(&["client-key", "server-certificate"]))]
    client_certificate: Option<String>,

    /// Path to CA certificate used to authenticate the server
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
    #[structopt(long)]
    server_certificate: Option<String>,

    /// If set, write output to file instead of stdout
    #[structopt(short, long)]
    output_file: Option<String>,

    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[derive(StructOpt, Debug)]
enum Operation {
    /// Health check
    CheckHealth {
    },
    /// Get library contents.
    GetLibrary {
    },
    /// Text search.
    SearchByText {
        /// Text query for search.
        query: String,
    },
    /// Get library item.
    GetLibraryItem {
        /// Library topic.
        #[structopt(parse(try_from_str = parse_json))]
        topic: models::LibraryTopic,
    },
    /// Get product alternatives.
    GetAlternatives {
        /// Variant of a product ID.
        #[structopt(parse(try_from_str = parse_json))]
        product_id_variant: models::ProductIdVariant,
        /// ID of a resource.
        id: String,
        /// Region code.
        region: Option<String>,
    },
    /// Get organisation.
    GetOrganisation {
        /// Variant of an organisation ID.
        #[structopt(parse(try_from_str = parse_json))]
        organisation_id_variant: models::OrganisationIdVariant,
        /// ID of a resource.
        id: String,
    },
    /// Get product.
    GetProduct {
        /// Variant of a product ID.
        #[structopt(parse(try_from_str = parse_json))]
        product_id_variant: models::ProductIdVariant,
        /// ID of a resource.
        id: String,
        /// Region code.
        region: Option<String>,
    },
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    if args.client_certificate.is_some() {
        debug!("Using mutual TLS");
        let client = Client::try_new_https_mutual(
            &args.server_address,
            args.server_certificate.clone().unwrap(),
            args.client_key.clone().unwrap(),
            args.client_certificate.clone().unwrap(),
        )
        .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else if args.server_certificate.is_some() {
        debug!("Using TLS with pinned server certificate");
        let client =
            Client::try_new_https_pinned(&args.server_address, args.server_certificate.clone().unwrap())
                .context("Failed to create HTTPS client")?;
        Ok(Box::new(client.with_context(context)))
    } else {
        debug!("Using client without certificates");
        let client =
            Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
        Ok(Box::new(client.with_context(context)))
    }
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
fn create_client(args: &Cli, context: ClientContext) -> Result<Box<dyn ApiNoContext<ClientContext>>> {
    let client =
        Client::try_new(&args.server_address).context("Failed to create HTTP(S) client")?;
    Ok(Box::new(client.with_context(context)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    if let Some(log_level) = args.verbosity.log_level() {
        SimpleLogger::new().with_level(log_level.to_level_filter()).init()?;
    }

    debug!("Arguments: {:?}", &args);

    let auth_data: Option<AuthData> = None;

    #[allow(trivial_casts)]
    let context = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        auth_data,
        XSpanIdString::default()
    );

    let client = create_client(&args, context)?;

    let result = match args.operation {
        Operation::CheckHealth {
        } => {
            info!("Performing a CheckHealth request");

            let result = client.check_health(
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                CheckHealthResponse::Ok
                {
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                    +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::GetLibrary {
        } => {
            info!("Performing a GetLibrary request");

            let result = client.get_library(
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetLibraryResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::SearchByText {
            query,
        } => {
            info!("Performing a SearchByText request");

            let result = client.search_by_text(
                query,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                SearchByTextResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::GetLibraryItem {
            topic,
        } => {
            info!("Performing a GetLibraryItem request on {:?}", (
                &topic
            ));

            let result = client.get_library_item(
                topic,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetLibraryItemResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
                GetLibraryItemResponse::NotFound
                {
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "NotFound\n".to_string()
                    +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::GetAlternatives {
            product_id_variant,
            id,
            region,
        } => {
            info!("Performing a GetAlternatives request on {:?}", (
                &product_id_variant,
                &id
            ));

            let result = client.get_alternatives(
                product_id_variant,
                id,
                region,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetAlternativesResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
                GetAlternativesResponse::NotFound
                {
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "NotFound\n".to_string()
                    +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::GetOrganisation {
            organisation_id_variant,
            id,
        } => {
            info!("Performing a GetOrganisation request on {:?}", (
                &organisation_id_variant,
                &id
            ));

            let result = client.get_organisation(
                organisation_id_variant,
                id,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetOrganisationResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
                GetOrganisationResponse::NotFound
                {
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "NotFound\n".to_string()
                    +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
        Operation::GetProduct {
            product_id_variant,
            id,
            region,
        } => {
            info!("Performing a GetProduct request on {:?}", (
                &product_id_variant,
                &id
            ));

            let result = client.get_product(
                product_id_variant,
                id,
                region,
            ).await?;
            debug!("Result: {:?}", result);

            match result {
                GetProductResponse::Ok
                {
                    body,
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "Ok\n".to_string()
                   +
                    &format!("body: {}\n", serde_json::to_string_pretty(&body)?) +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
                GetProductResponse::NotFound
                {
                    access_control_allow_origin,
                    access_control_allow_methods,
                    access_control_allow_headers,
                }
                => "NotFound\n".to_string()
                    +
                    &format!(
                        "access_control_allow_origin: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_origin)?
                    ) +
                    &format!(
                        "access_control_allow_methods: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_methods)?
                    ) +
                    &format!(
                        "access_control_allow_headers: {}\n",
                        serde_json::to_string_pretty(&access_control_allow_headers)?
                    ),
            }
        }
    };

    if let Some(output_file) = args.output_file {
        std::fs::write(output_file, result)?
    } else {
        println!("{}", result);
    }
    Ok(())
}

// May be unused if all inputs are primitive types
#[allow(dead_code)]
fn parse_json<'a, T: serde::de::Deserialize<'a>>(json_string: &'a str) -> Result<T> {
    serde_json::from_str(json_string).map_err(|err| anyhow!("Error parsing input: {}", err))
}
