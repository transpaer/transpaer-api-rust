#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

/// Match accuracy.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Accuracy(f64);

impl std::convert::From<f64> for Accuracy {
    fn from(x: f64) -> Self {
        Accuracy(x)
    }
}

impl std::convert::From<Accuracy> for f64 {
    fn from(x: Accuracy) -> Self {
        x.0
    }
}

impl std::ops::Deref for Accuracy {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl std::ops::DerefMut for Accuracy {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}


/// Details of BCorp evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BCorpMedallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    /// ID of a resource.
    #[serde(rename = "id")]
    #[validate(
            length(max = 32),
        )]
    pub id: String,

}


impl BCorpMedallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, id: String, ) -> BCorpMedallion {
        BCorpMedallion {
            medallion_variant,
            id,
        }
    }
}

/// Converts the BCorpMedallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for BCorpMedallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),


            Some("id".to_string()),
            Some(self.id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BCorpMedallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BCorpMedallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BCorpMedallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BCorpMedallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BCorpMedallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in BCorpMedallion".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in BCorpMedallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BCorpMedallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<BCorpMedallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BCorpMedallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for BCorpMedallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<BCorpMedallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BCorpMedallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into BCorpMedallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Name of a badge.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum BadgeName {
    #[serde(rename = "bcorp")]
    Bcorp,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "tco")]
    Tco,
}

impl std::fmt::Display for BadgeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            BadgeName::Bcorp => write!(f, "bcorp"),
            BadgeName::Eu => write!(f, "eu"),
            BadgeName::Tco => write!(f, "tco"),
        }
    }
}

impl std::str::FromStr for BadgeName {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "bcorp" => std::result::Result::Ok(BadgeName::Bcorp),
            "eu" => std::result::Result::Ok(BadgeName::Eu),
            "tco" => std::result::Result::Ok(BadgeName::Tco),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// List of product alternative in the given category.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CategoryAlternatives {
    /// Short string for labels, titles, summaries...
    #[serde(rename = "category")]
    #[validate(
            length(max = 1024),
        )]
    pub category: String,

    #[serde(rename = "alternatives")]
    pub alternatives: Vec<models::ProductShort>,

}


impl CategoryAlternatives {
    #[allow(clippy::new_without_default)]
    pub fn new(category: String, alternatives: Vec<models::ProductShort>, ) -> CategoryAlternatives {
        CategoryAlternatives {
            category,
            alternatives,
        }
    }
}

/// Converts the CategoryAlternatives value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CategoryAlternatives {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("category".to_string()),
            Some(self.category.to_string()),

            // Skipping alternatives in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CategoryAlternatives value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CategoryAlternatives {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub category: Vec<String>,
            pub alternatives: Vec<Vec<models::ProductShort>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CategoryAlternatives".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "category" => intermediate_rep.category.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "alternatives" => return std::result::Result::Err("Parsing a container in this style is not supported in CategoryAlternatives".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing CategoryAlternatives".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CategoryAlternatives {
            category: intermediate_rep.category.into_iter().next().ok_or_else(|| "category missing in CategoryAlternatives".to_string())?,
            alternatives: intermediate_rep.alternatives.into_iter().next().ok_or_else(|| "alternatives missing in CategoryAlternatives".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CategoryAlternatives> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CategoryAlternatives>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CategoryAlternatives>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CategoryAlternatives - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CategoryAlternatives> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CategoryAlternatives as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CategoryAlternatives - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Describes where the related data was retrieved from.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DataSource {
    #[serde(rename = "wiki")]
    Wiki,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "eu")]
    Eu,
}

impl std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DataSource::Wiki => write!(f, "wiki"),
            DataSource::Off => write!(f, "off"),
            DataSource::Eu => write!(f, "eu"),
        }
    }
}

impl std::str::FromStr for DataSource {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "wiki" => std::result::Result::Ok(DataSource::Wiki),
            "off" => std::result::Result::Ok(DataSource::Off),
            "eu" => std::result::Result::Ok(DataSource::Eu),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Details of EU Ecolabel evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EuEcolabelMedallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    /// Match accuracy.
    #[serde(rename = "matchAccuracy")]
    #[validate(
            range(min = 0.0, max = 1.0),
        )]
    pub match_accuracy: f64,

}


impl EuEcolabelMedallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, match_accuracy: f64, ) -> EuEcolabelMedallion {
        EuEcolabelMedallion {
            medallion_variant,
            match_accuracy,
        }
    }
}

/// Converts the EuEcolabelMedallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EuEcolabelMedallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),


            Some("matchAccuracy".to_string()),
            Some(self.match_accuracy.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EuEcolabelMedallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EuEcolabelMedallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub match_accuracy: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EuEcolabelMedallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "matchAccuracy" => intermediate_rep.match_accuracy.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EuEcolabelMedallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EuEcolabelMedallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in EuEcolabelMedallion".to_string())?,
            match_accuracy: intermediate_rep.match_accuracy.into_iter().next().ok_or_else(|| "matchAccuracy missing in EuEcolabelMedallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EuEcolabelMedallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EuEcolabelMedallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EuEcolabelMedallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EuEcolabelMedallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EuEcolabelMedallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EuEcolabelMedallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EuEcolabelMedallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Details of Fashion Transparency Index evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FtiMedallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    #[serde(rename = "score")]
    pub score: i32,

}


impl FtiMedallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, score: i32, ) -> FtiMedallion {
        FtiMedallion {
            medallion_variant,
            score,
        }
    }
}

/// Converts the FtiMedallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for FtiMedallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),


            Some("score".to_string()),
            Some(self.score.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FtiMedallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FtiMedallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub score: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FtiMedallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FtiMedallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FtiMedallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in FtiMedallion".to_string())?,
            score: intermediate_rep.score.into_iter().next().ok_or_else(|| "score missing in FtiMedallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FtiMedallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<FtiMedallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FtiMedallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FtiMedallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<FtiMedallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FtiMedallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FtiMedallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Global Trade Item Number.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Gtin(String);

impl std::convert::From<String> for Gtin {
    fn from(x: String) -> Self {
        Gtin(x)
    }
}

impl std::string::ToString for Gtin {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Gtin {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Gtin(x.to_string()))
    }
}

impl std::convert::From<Gtin> for String {
    fn from(x: Gtin) -> Self {
        x.0
    }
}

impl std::ops::Deref for Gtin {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Gtin {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// ID of a resource.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Id(String);

impl std::convert::From<String> for Id {
    fn from(x: String) -> Self {
        Id(x)
    }
}

impl std::string::ToString for Id {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Id {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Id(x.to_string()))
    }
}

impl std::convert::From<Id> for String {
    fn from(x: Id) -> Self {
        x.0
    }
}

impl std::ops::Deref for Id {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Id {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Image path/URL with its source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Image {
    /// Short string for labels, titles, summaries...
    #[serde(rename = "image")]
    #[validate(
            length(max = 1024),
        )]
    pub image: String,

    #[serde(rename = "source")]
    pub source: models::DataSource,

}


impl Image {
    #[allow(clippy::new_without_default)]
    pub fn new(image: String, source: models::DataSource, ) -> Image {
        Image {
            image,
            source,
        }
    }
}

/// Converts the Image value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Image {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("image".to_string()),
            Some(self.image.to_string()),

            // Skipping source in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Image value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub image: Vec<String>,
            pub source: Vec<models::DataSource>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Image".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "image" => intermediate_rep.image.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(<models::DataSource as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Image".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Image {
            image: intermediate_rep.image.into_iter().next().ok_or_else(|| "image missing in Image".to_string())?,
            source: intermediate_rep.source.into_iter().next().ok_or_else(|| "source missing in Image".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Image> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Image>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Image>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Image - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Image> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Image as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Image - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// List of all items in the library.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LibraryContents {
    #[serde(rename = "items")]
    pub items: Vec<models::LibraryItemShort>,

}


impl LibraryContents {
    #[allow(clippy::new_without_default)]
    pub fn new(items: Vec<models::LibraryItemShort>, ) -> LibraryContents {
        LibraryContents {
            items,
        }
    }
}

/// Converts the LibraryContents value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LibraryContents {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping items in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LibraryContents value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LibraryContents {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::LibraryItemShort>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LibraryContents".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "items" => return std::result::Result::Err("Parsing a container in this style is not supported in LibraryContents".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing LibraryContents".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LibraryContents {
            items: intermediate_rep.items.into_iter().next().ok_or_else(|| "items missing in LibraryContents".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LibraryContents> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LibraryContents>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LibraryContents>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LibraryContents - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LibraryContents> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LibraryContents as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LibraryContents - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Full contents of a library item.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LibraryItemFull {
    #[serde(rename = "id")]
    pub id: models::LibraryTopic,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "title")]
    #[validate(
            length(max = 1024),
        )]
    pub title: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "summary")]
    #[validate(
            length(max = 1024),
        )]
    pub summary: String,

    /// Long string for descriptions, articles...
    #[serde(rename = "article")]
    #[validate(
            length(max = 1048576),
        )]
    pub article: String,

    #[serde(rename = "presentation")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub presentation: Option<models::Presentation>,

}


impl LibraryItemFull {
    #[allow(clippy::new_without_default)]
    pub fn new(id: models::LibraryTopic, title: String, summary: String, article: String, ) -> LibraryItemFull {
        LibraryItemFull {
            id,
            title,
            summary,
            article,
            presentation: None,
        }
    }
}

/// Converts the LibraryItemFull value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LibraryItemFull {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("summary".to_string()),
            Some(self.summary.to_string()),


            Some("article".to_string()),
            Some(self.article.to_string()),

            // Skipping presentation in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LibraryItemFull value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LibraryItemFull {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<models::LibraryTopic>,
            pub title: Vec<String>,
            pub summary: Vec<String>,
            pub article: Vec<String>,
            pub presentation: Vec<models::Presentation>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LibraryItemFull".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<models::LibraryTopic as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "summary" => intermediate_rep.summary.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "article" => intermediate_rep.article.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "presentation" => intermediate_rep.presentation.push(<models::Presentation as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LibraryItemFull".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LibraryItemFull {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in LibraryItemFull".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in LibraryItemFull".to_string())?,
            summary: intermediate_rep.summary.into_iter().next().ok_or_else(|| "summary missing in LibraryItemFull".to_string())?,
            article: intermediate_rep.article.into_iter().next().ok_or_else(|| "article missing in LibraryItemFull".to_string())?,
            presentation: intermediate_rep.presentation.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LibraryItemFull> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LibraryItemFull>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LibraryItemFull>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LibraryItemFull - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LibraryItemFull> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LibraryItemFull as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LibraryItemFull - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Short summary of a library item.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LibraryItemShort {
    #[serde(rename = "id")]
    pub id: models::LibraryTopic,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "title")]
    #[validate(
            length(max = 1024),
        )]
    pub title: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "summary")]
    #[validate(
            length(max = 1024),
        )]
    pub summary: String,

}


impl LibraryItemShort {
    #[allow(clippy::new_without_default)]
    pub fn new(id: models::LibraryTopic, title: String, summary: String, ) -> LibraryItemShort {
        LibraryItemShort {
            id,
            title,
            summary,
        }
    }
}

/// Converts the LibraryItemShort value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LibraryItemShort {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping id in query parameter serialization


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("summary".to_string()),
            Some(self.summary.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LibraryItemShort value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LibraryItemShort {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<models::LibraryTopic>,
            pub title: Vec<String>,
            pub summary: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LibraryItemShort".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<models::LibraryTopic as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "summary" => intermediate_rep.summary.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LibraryItemShort".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LibraryItemShort {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in LibraryItemShort".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in LibraryItemShort".to_string())?,
            summary: intermediate_rep.summary.into_iter().next().ok_or_else(|| "summary missing in LibraryItemShort".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LibraryItemShort> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LibraryItemShort>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LibraryItemShort>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LibraryItemShort - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LibraryItemShort> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LibraryItemShort as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LibraryItemShort - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Name of a topic in the library.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum LibraryTopic {
    #[serde(rename = "info:main")]
    InfoColonMain,
    #[serde(rename = "info:for_producers")]
    InfoColonForProducers,
    #[serde(rename = "info:faq")]
    InfoColonFaq,
    #[serde(rename = "data:wiki")]
    DataColonWiki,
    #[serde(rename = "data:open_food_facts")]
    DataColonOpenFoodFacts,
    #[serde(rename = "cert:bcorp")]
    CertColonBcorp,
    #[serde(rename = "cert:eu_ecolabel")]
    CertColonEuEcolabel,
    #[serde(rename = "cert:tco")]
    CertColonTco,
    #[serde(rename = "cert:fti")]
    CertColonFti,
    #[serde(rename = "other:not_found")]
    OtherColonNotFound,
}

impl std::fmt::Display for LibraryTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LibraryTopic::InfoColonMain => write!(f, "info:main"),
            LibraryTopic::InfoColonForProducers => write!(f, "info:for_producers"),
            LibraryTopic::InfoColonFaq => write!(f, "info:faq"),
            LibraryTopic::DataColonWiki => write!(f, "data:wiki"),
            LibraryTopic::DataColonOpenFoodFacts => write!(f, "data:open_food_facts"),
            LibraryTopic::CertColonBcorp => write!(f, "cert:bcorp"),
            LibraryTopic::CertColonEuEcolabel => write!(f, "cert:eu_ecolabel"),
            LibraryTopic::CertColonTco => write!(f, "cert:tco"),
            LibraryTopic::CertColonFti => write!(f, "cert:fti"),
            LibraryTopic::OtherColonNotFound => write!(f, "other:not_found"),
        }
    }
}

impl std::str::FromStr for LibraryTopic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "info:main" => std::result::Result::Ok(LibraryTopic::InfoColonMain),
            "info:for_producers" => std::result::Result::Ok(LibraryTopic::InfoColonForProducers),
            "info:faq" => std::result::Result::Ok(LibraryTopic::InfoColonFaq),
            "data:wiki" => std::result::Result::Ok(LibraryTopic::DataColonWiki),
            "data:open_food_facts" => std::result::Result::Ok(LibraryTopic::DataColonOpenFoodFacts),
            "cert:bcorp" => std::result::Result::Ok(LibraryTopic::CertColonBcorp),
            "cert:eu_ecolabel" => std::result::Result::Ok(LibraryTopic::CertColonEuEcolabel),
            "cert:tco" => std::result::Result::Ok(LibraryTopic::CertColonTco),
            "cert:fti" => std::result::Result::Ok(LibraryTopic::CertColonFti),
            "other:not_found" => std::result::Result::Ok(LibraryTopic::OtherColonNotFound),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Long string for descriptions, articles...
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LongString(String);

impl std::convert::From<String> for LongString {
    fn from(x: String) -> Self {
        LongString(x)
    }
}

impl std::string::ToString for LongString {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for LongString {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(LongString(x.to_string()))
    }
}

impl std::convert::From<LongString> for String {
    fn from(x: LongString) -> Self {
        x.0
    }
}

impl std::ops::Deref for LongString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for LongString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Long text with its source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LongText {
    /// Long string for descriptions, articles...
    #[serde(rename = "text")]
    #[validate(
            length(max = 1048576),
        )]
    pub text: String,

    #[serde(rename = "source")]
    pub source: models::DataSource,

}


impl LongText {
    #[allow(clippy::new_without_default)]
    pub fn new(text: String, source: models::DataSource, ) -> LongText {
        LongText {
            text,
            source,
        }
    }
}

/// Converts the LongText value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LongText {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("text".to_string()),
            Some(self.text.to_string()),

            // Skipping source in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LongText value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LongText {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub source: Vec<models::DataSource>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LongText".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(<models::DataSource as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing LongText".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LongText {
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in LongText".to_string())?,
            source: intermediate_rep.source.into_iter().next().ok_or_else(|| "source missing in LongText".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LongText> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LongText>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LongText>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LongText - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LongText> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LongText as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LongText - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Combines data from any medallion.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Medallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    /// ID of a resource.
    #[serde(rename = "id")]
    #[validate(
            length(max = 32),
        )]
    pub id: String,

    /// Match accuracy.
    #[serde(rename = "matchAccuracy")]
    #[validate(
            range(min = 0.0, max = 1.0),
        )]
    pub match_accuracy: f64,

    #[serde(rename = "score")]
    pub score: models::SustainityScore,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "brandName")]
    #[validate(
            length(max = 1024),
        )]
    pub brand_name: String,

}


impl Medallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, id: String, match_accuracy: f64, score: models::SustainityScore, brand_name: String, ) -> Medallion {
        Medallion {
            medallion_variant,
            id,
            match_accuracy,
            score,
            brand_name,
        }
    }
}

/// Converts the Medallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Medallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),


            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("matchAccuracy".to_string()),
            Some(self.match_accuracy.to_string()),

            // Skipping score in query parameter serialization


            Some("brandName".to_string()),
            Some(self.brand_name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Medallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Medallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub id: Vec<String>,
            pub match_accuracy: Vec<f64>,
            pub score: Vec<models::SustainityScore>,
            pub brand_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Medallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "matchAccuracy" => intermediate_rep.match_accuracy.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<models::SustainityScore as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "brandName" => intermediate_rep.brand_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Medallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Medallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in Medallion".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Medallion".to_string())?,
            match_accuracy: intermediate_rep.match_accuracy.into_iter().next().ok_or_else(|| "matchAccuracy missing in Medallion".to_string())?,
            score: intermediate_rep.score.into_iter().next().ok_or_else(|| "score missing in Medallion".to_string())?,
            brand_name: intermediate_rep.brand_name.into_iter().next().ok_or_else(|| "brandName missing in Medallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Medallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Medallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Medallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Medallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Medallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Medallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Medallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Full organisation data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrganisationFull {
    /// ID of a resource.
    #[serde(rename = "organisationId")]
    #[validate(
            length(max = 32),
        )]
    pub organisation_id: String,

    #[serde(rename = "names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<models::ShortText>>,

    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub descriptions: Option<Vec<models::LongText>>,

    #[serde(rename = "images")]
    pub images: Vec<models::Image>,

    #[serde(rename = "websites")]
    pub websites: Vec<models::ShortString>,

    #[serde(rename = "products")]
    pub products: Vec<models::ProductShort>,

    #[serde(rename = "medallions")]
    pub medallions: Vec<models::Medallion>,

}


impl OrganisationFull {
    #[allow(clippy::new_without_default)]
    pub fn new(organisation_id: String, images: Vec<models::Image>, websites: Vec<models::ShortString>, products: Vec<models::ProductShort>, medallions: Vec<models::Medallion>, ) -> OrganisationFull {
        OrganisationFull {
            organisation_id,
            names: None,
            descriptions: None,
            images,
            websites,
            products,
            medallions,
        }
    }
}

/// Converts the OrganisationFull value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OrganisationFull {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("organisationId".to_string()),
            Some(self.organisation_id.to_string()),

            // Skipping names in query parameter serialization

            // Skipping descriptions in query parameter serialization

            // Skipping images in query parameter serialization


            Some("websites".to_string()),
            Some(self.websites.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping products in query parameter serialization

            // Skipping medallions in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrganisationFull value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrganisationFull {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub organisation_id: Vec<String>,
            pub names: Vec<Vec<models::ShortText>>,
            pub descriptions: Vec<Vec<models::LongText>>,
            pub images: Vec<Vec<models::Image>>,
            pub websites: Vec<Vec<models::ShortString>>,
            pub products: Vec<Vec<models::ProductShort>>,
            pub medallions: Vec<Vec<models::Medallion>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OrganisationFull".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "organisationId" => intermediate_rep.organisation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "names" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    "descriptions" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    "images" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    "websites" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    "products" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    "medallions" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationFull".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing OrganisationFull".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrganisationFull {
            organisation_id: intermediate_rep.organisation_id.into_iter().next().ok_or_else(|| "organisationId missing in OrganisationFull".to_string())?,
            names: intermediate_rep.names.into_iter().next(),
            descriptions: intermediate_rep.descriptions.into_iter().next(),
            images: intermediate_rep.images.into_iter().next().ok_or_else(|| "images missing in OrganisationFull".to_string())?,
            websites: intermediate_rep.websites.into_iter().next().ok_or_else(|| "websites missing in OrganisationFull".to_string())?,
            products: intermediate_rep.products.into_iter().next().ok_or_else(|| "products missing in OrganisationFull".to_string())?,
            medallions: intermediate_rep.medallions.into_iter().next().ok_or_else(|| "medallions missing in OrganisationFull".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OrganisationFull> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OrganisationFull>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OrganisationFull>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OrganisationFull - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OrganisationFull> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OrganisationFull as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OrganisationFull - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Extract from organisation data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct OrganisationShort {
    /// ID of a resource.
    #[serde(rename = "organisationId")]
    #[validate(
            length(max = 32),
        )]
    pub organisation_id: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "name")]
    #[validate(
            length(max = 1024),
        )]
    pub name: String,

    /// Long string for descriptions, articles...
    #[serde(rename = "description")]
    #[validate(
            length(max = 1048576),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "badges")]
    pub badges: Vec<models::BadgeName>,

    #[serde(rename = "scores")]
    pub scores: std::collections::HashMap<String, i32>,

}


impl OrganisationShort {
    #[allow(clippy::new_without_default)]
    pub fn new(organisation_id: String, name: String, badges: Vec<models::BadgeName>, scores: std::collections::HashMap<String, i32>, ) -> OrganisationShort {
        OrganisationShort {
            organisation_id,
            name,
            description: None,
            badges,
            scores,
        }
    }
}

/// Converts the OrganisationShort value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for OrganisationShort {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("organisationId".to_string()),
            Some(self.organisation_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

            // Skipping badges in query parameter serialization

            // Skipping scores in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a OrganisationShort value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for OrganisationShort {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub organisation_id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub badges: Vec<Vec<models::BadgeName>>,
            pub scores: Vec<std::collections::HashMap<String, i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing OrganisationShort".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "organisationId" => intermediate_rep.organisation_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "badges" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationShort".to_string()),
                    "scores" => return std::result::Result::Err("Parsing a container in this style is not supported in OrganisationShort".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing OrganisationShort".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(OrganisationShort {
            organisation_id: intermediate_rep.organisation_id.into_iter().next().ok_or_else(|| "organisationId missing in OrganisationShort".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in OrganisationShort".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            badges: intermediate_rep.badges.into_iter().next().ok_or_else(|| "badges missing in OrganisationShort".to_string())?,
            scores: intermediate_rep.scores.into_iter().next().ok_or_else(|| "scores missing in OrganisationShort".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<OrganisationShort> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<OrganisationShort>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<OrganisationShort>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for OrganisationShort - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<OrganisationShort> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <OrganisationShort as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into OrganisationShort - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Additional data to present together with a library item.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Presentation {
    #[serde(rename = "data")]
    pub data: Vec<models::PresentationEntry>,

}


impl Presentation {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<models::PresentationEntry>, ) -> Presentation {
        Presentation {
            data,
        }
    }
}

/// Converts the Presentation value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Presentation {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping data in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Presentation value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Presentation {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub data: Vec<Vec<models::PresentationEntry>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Presentation".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "data" => return std::result::Result::Err("Parsing a container in this style is not supported in Presentation".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Presentation".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Presentation {
            data: intermediate_rep.data.into_iter().next().ok_or_else(|| "data missing in Presentation".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Presentation> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Presentation>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Presentation>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Presentation - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Presentation> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Presentation as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Presentation - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// An entry in a presentation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PresentationEntry {
    /// ID of a resource.
    #[serde(rename = "id")]
    #[validate(
            length(max = 32),
        )]
    pub id: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "name")]
    #[validate(
            length(max = 1024),
        )]
    pub name: String,

    #[serde(rename = "score")]
    pub score: i32,

}


impl PresentationEntry {
    #[allow(clippy::new_without_default)]
    pub fn new(id: String, name: String, score: i32, ) -> PresentationEntry {
        PresentationEntry {
            id,
            name,
            score,
        }
    }
}

/// Converts the PresentationEntry value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PresentationEntry {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("score".to_string()),
            Some(self.score.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PresentationEntry value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PresentationEntry {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub name: Vec<String>,
            pub score: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PresentationEntry".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PresentationEntry".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PresentationEntry {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in PresentationEntry".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in PresentationEntry".to_string())?,
            score: intermediate_rep.score.into_iter().next().ok_or_else(|| "score missing in PresentationEntry".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PresentationEntry> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PresentationEntry>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PresentationEntry>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PresentationEntry - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PresentationEntry> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PresentationEntry as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PresentationEntry - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Full product data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProductFull {
    /// ID of a resource.
    #[serde(rename = "productId")]
    #[validate(
            length(max = 32),
        )]
    pub product_id: String,

    #[serde(rename = "gtins")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub gtins: Option<Vec<models::Gtin>>,

    #[serde(rename = "names")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub names: Option<Vec<models::ShortText>>,

    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub descriptions: Option<Vec<models::LongText>>,

    #[serde(rename = "images")]
    pub images: Vec<models::Image>,

    #[serde(rename = "manufacturers")]
    pub manufacturers: Vec<models::OrganisationShort>,

    #[serde(rename = "alternatives")]
    pub alternatives: Vec<models::CategoryAlternatives>,

    #[serde(rename = "medallions")]
    pub medallions: Vec<models::Medallion>,

}


impl ProductFull {
    #[allow(clippy::new_without_default)]
    pub fn new(product_id: String, images: Vec<models::Image>, manufacturers: Vec<models::OrganisationShort>, alternatives: Vec<models::CategoryAlternatives>, medallions: Vec<models::Medallion>, ) -> ProductFull {
        ProductFull {
            product_id,
            gtins: None,
            names: None,
            descriptions: None,
            images,
            manufacturers,
            alternatives,
            medallions,
        }
    }
}

/// Converts the ProductFull value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ProductFull {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("productId".to_string()),
            Some(self.product_id.to_string()),


            self.gtins.as_ref().map(|gtins| {
                vec![
                    "gtins".to_string(),
                    gtins.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

            // Skipping names in query parameter serialization

            // Skipping descriptions in query parameter serialization

            // Skipping images in query parameter serialization

            // Skipping manufacturers in query parameter serialization

            // Skipping alternatives in query parameter serialization

            // Skipping medallions in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProductFull value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProductFull {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub product_id: Vec<String>,
            pub gtins: Vec<Vec<models::Gtin>>,
            pub names: Vec<Vec<models::ShortText>>,
            pub descriptions: Vec<Vec<models::LongText>>,
            pub images: Vec<Vec<models::Image>>,
            pub manufacturers: Vec<Vec<models::OrganisationShort>>,
            pub alternatives: Vec<Vec<models::CategoryAlternatives>>,
            pub medallions: Vec<Vec<models::Medallion>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProductFull".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "productId" => intermediate_rep.product_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "gtins" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "names" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "descriptions" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "images" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "manufacturers" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "alternatives" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    "medallions" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductFull".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProductFull".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProductFull {
            product_id: intermediate_rep.product_id.into_iter().next().ok_or_else(|| "productId missing in ProductFull".to_string())?,
            gtins: intermediate_rep.gtins.into_iter().next(),
            names: intermediate_rep.names.into_iter().next(),
            descriptions: intermediate_rep.descriptions.into_iter().next(),
            images: intermediate_rep.images.into_iter().next().ok_or_else(|| "images missing in ProductFull".to_string())?,
            manufacturers: intermediate_rep.manufacturers.into_iter().next().ok_or_else(|| "manufacturers missing in ProductFull".to_string())?,
            alternatives: intermediate_rep.alternatives.into_iter().next().ok_or_else(|| "alternatives missing in ProductFull".to_string())?,
            medallions: intermediate_rep.medallions.into_iter().next().ok_or_else(|| "medallions missing in ProductFull".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProductFull> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ProductFull>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProductFull>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProductFull - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ProductFull> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProductFull as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProductFull - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Extract from product data.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProductShort {
    /// ID of a resource.
    #[serde(rename = "productId")]
    #[validate(
            length(max = 32),
        )]
    pub product_id: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "name")]
    #[validate(
            length(max = 1024),
        )]
    pub name: String,

    /// Long string for descriptions, articles...
    #[serde(rename = "description")]
    #[validate(
            length(max = 1048576),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "badges")]
    pub badges: Vec<models::BadgeName>,

    #[serde(rename = "scores")]
    pub scores: std::collections::HashMap<String, i32>,

}


impl ProductShort {
    #[allow(clippy::new_without_default)]
    pub fn new(product_id: String, name: String, badges: Vec<models::BadgeName>, scores: std::collections::HashMap<String, i32>, ) -> ProductShort {
        ProductShort {
            product_id,
            name,
            description: None,
            badges,
            scores,
        }
    }
}

/// Converts the ProductShort value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ProductShort {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("productId".to_string()),
            Some(self.product_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

            // Skipping badges in query parameter serialization

            // Skipping scores in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProductShort value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProductShort {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub product_id: Vec<String>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub badges: Vec<Vec<models::BadgeName>>,
            pub scores: Vec<std::collections::HashMap<String, i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProductShort".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "productId" => intermediate_rep.product_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "badges" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductShort".to_string()),
                    "scores" => return std::result::Result::Err("Parsing a container in this style is not supported in ProductShort".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProductShort".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProductShort {
            product_id: intermediate_rep.product_id.into_iter().next().ok_or_else(|| "productId missing in ProductShort".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ProductShort".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            badges: intermediate_rep.badges.into_iter().next().ok_or_else(|| "badges missing in ProductShort".to_string())?,
            scores: intermediate_rep.scores.into_iter().next().ok_or_else(|| "scores missing in ProductShort".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProductShort> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ProductShort>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProductShort>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProductShort - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ProductShort> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProductShort as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProductShort - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Code of a region.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RegionCode(String);

impl std::convert::From<String> for RegionCode {
    fn from(x: String) -> Self {
        RegionCode(x)
    }
}

impl std::string::ToString for RegionCode {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for RegionCode {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(RegionCode(x.to_string()))
    }
}

impl std::convert::From<RegionCode> for String {
    fn from(x: RegionCode) -> Self {
        x.0
    }
}

impl std::ops::Deref for RegionCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for RegionCode {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Name of a scorer.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum ScorerName {
    #[serde(rename = "fti")]
    Fti,
}

impl std::fmt::Display for ScorerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ScorerName::Fti => write!(f, "fti"),
        }
    }
}

impl std::str::FromStr for ScorerName {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "fti" => std::result::Result::Ok(ScorerName::Fti),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// Short string for labels, titles, summaries...
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShortString(String);

impl std::convert::From<String> for ShortString {
    fn from(x: String) -> Self {
        ShortString(x)
    }
}

impl std::string::ToString for ShortString {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for ShortString {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ShortString(x.to_string()))
    }
}

impl std::convert::From<ShortString> for String {
    fn from(x: ShortString) -> Self {
        x.0
    }
}

impl std::ops::Deref for ShortString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ShortString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Short text with its source.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ShortText {
    /// Short string for labels, titles, summaries...
    #[serde(rename = "text")]
    #[validate(
            length(max = 1024),
        )]
    pub text: String,

    #[serde(rename = "source")]
    pub source: models::DataSource,

}


impl ShortText {
    #[allow(clippy::new_without_default)]
    pub fn new(text: String, source: models::DataSource, ) -> ShortText {
        ShortText {
            text,
            source,
        }
    }
}

/// Converts the ShortText value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ShortText {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("text".to_string()),
            Some(self.text.to_string()),

            // Skipping source in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ShortText value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ShortText {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub source: Vec<models::DataSource>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ShortText".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "source" => intermediate_rep.source.push(<models::DataSource as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ShortText".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ShortText {
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in ShortText".to_string())?,
            source: intermediate_rep.source.into_iter().next().ok_or_else(|| "source missing in ShortText".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ShortText> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ShortText>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ShortText>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ShortText - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ShortText> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ShortText as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ShortText - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Details of Sustainity evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SustainityMedallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    #[serde(rename = "score")]
    pub score: models::SustainityScore,

}


impl SustainityMedallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, score: models::SustainityScore, ) -> SustainityMedallion {
        SustainityMedallion {
            medallion_variant,
            score,
        }
    }
}

/// Converts the SustainityMedallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SustainityMedallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),

            // Skipping score in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SustainityMedallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SustainityMedallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub score: Vec<models::SustainityScore>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SustainityMedallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<models::SustainityScore as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SustainityMedallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SustainityMedallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in SustainityMedallion".to_string())?,
            score: intermediate_rep.score.into_iter().next().ok_or_else(|| "score missing in SustainityMedallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SustainityMedallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SustainityMedallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SustainityMedallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SustainityMedallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SustainityMedallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SustainityMedallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SustainityMedallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Explanation of calculation of the Sustainity score.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SustainityScore {
    #[serde(rename = "total")]
    pub total: f64,

    #[serde(rename = "tree")]
    pub tree: Vec<models::SustainityScoreBranch>,

}


impl SustainityScore {
    #[allow(clippy::new_without_default)]
    pub fn new(total: f64, tree: Vec<models::SustainityScoreBranch>, ) -> SustainityScore {
        SustainityScore {
            total,
            tree,
        }
    }
}

/// Converts the SustainityScore value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SustainityScore {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("total".to_string()),
            Some(self.total.to_string()),

            // Skipping tree in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SustainityScore value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SustainityScore {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub total: Vec<f64>,
            pub tree: Vec<Vec<models::SustainityScoreBranch>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SustainityScore".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "total" => intermediate_rep.total.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "tree" => return std::result::Result::Err("Parsing a container in this style is not supported in SustainityScore".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SustainityScore".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SustainityScore {
            total: intermediate_rep.total.into_iter().next().ok_or_else(|| "total missing in SustainityScore".to_string())?,
            tree: intermediate_rep.tree.into_iter().next().ok_or_else(|| "tree missing in SustainityScore".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SustainityScore> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SustainityScore>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SustainityScore>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SustainityScore - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SustainityScore> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SustainityScore as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SustainityScore - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Part of explanation of calculation of the Sustainity score.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SustainityScoreBranch {
    /// A single letter symbol.
    #[serde(rename = "symbol")]
    #[validate(
            length(min = 1, max = 1),
        )]
    pub symbol: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "description")]
    #[validate(
            length(max = 1024),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "weight")]
    pub weight: i32,

    #[serde(rename = "score")]
    pub score: f64,

    #[serde(rename = "branches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub branches: Option<Vec<models::SustainityScoreBranch>>,

}


impl SustainityScoreBranch {
    #[allow(clippy::new_without_default)]
    pub fn new(symbol: String, weight: i32, score: f64, ) -> SustainityScoreBranch {
        SustainityScoreBranch {
            symbol,
            description: None,
            weight,
            score,
            branches: None,
        }
    }
}

/// Converts the SustainityScoreBranch value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SustainityScoreBranch {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("symbol".to_string()),
            Some(self.symbol.to_string()),


            self.description.as_ref().map(|description| {
                vec![
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),


            Some("weight".to_string()),
            Some(self.weight.to_string()),


            Some("score".to_string()),
            Some(self.score.to_string()),

            // Skipping branches in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SustainityScoreBranch value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SustainityScoreBranch {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub symbol: Vec<String>,
            pub description: Vec<String>,
            pub weight: Vec<i32>,
            pub score: Vec<f64>,
            pub branches: Vec<Vec<models::SustainityScoreBranch>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SustainityScoreBranch".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "symbol" => intermediate_rep.symbol.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "weight" => intermediate_rep.weight.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "score" => intermediate_rep.score.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "branches" => return std::result::Result::Err("Parsing a container in this style is not supported in SustainityScoreBranch".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SustainityScoreBranch".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SustainityScoreBranch {
            symbol: intermediate_rep.symbol.into_iter().next().ok_or_else(|| "symbol missing in SustainityScoreBranch".to_string())?,
            description: intermediate_rep.description.into_iter().next(),
            weight: intermediate_rep.weight.into_iter().next().ok_or_else(|| "weight missing in SustainityScoreBranch".to_string())?,
            score: intermediate_rep.score.into_iter().next().ok_or_else(|| "score missing in SustainityScoreBranch".to_string())?,
            branches: intermediate_rep.branches.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SustainityScoreBranch> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SustainityScoreBranch>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SustainityScoreBranch>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SustainityScoreBranch - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SustainityScoreBranch> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SustainityScoreBranch as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SustainityScoreBranch - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// A single letter symbol.
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Symbol(String);

impl std::convert::From<String> for Symbol {
    fn from(x: String) -> Self {
        Symbol(x)
    }
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
       self.0.to_string()
    }
}

impl std::str::FromStr for Symbol {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Symbol(x.to_string()))
    }
}

impl std::convert::From<Symbol> for String {
    fn from(x: Symbol) -> Self {
        x.0
    }
}

impl std::ops::Deref for Symbol {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for Symbol {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}


/// Details of TCO evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TcoMedallion {
    #[serde(rename = "medallionVariant")]
    pub medallion_variant: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "brandName")]
    #[validate(
            length(max = 1024),
        )]
    pub brand_name: String,

}


impl TcoMedallion {
    #[allow(clippy::new_without_default)]
    pub fn new(medallion_variant: String, brand_name: String, ) -> TcoMedallion {
        TcoMedallion {
            medallion_variant,
            brand_name,
        }
    }
}

/// Converts the TcoMedallion value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TcoMedallion {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("medallionVariant".to_string()),
            Some(self.medallion_variant.to_string()),


            Some("brandName".to_string()),
            Some(self.brand_name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TcoMedallion value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TcoMedallion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub medallion_variant: Vec<String>,
            pub brand_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TcoMedallion".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "medallionVariant" => intermediate_rep.medallion_variant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "brandName" => intermediate_rep.brand_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TcoMedallion".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TcoMedallion {
            medallion_variant: intermediate_rep.medallion_variant.into_iter().next().ok_or_else(|| "medallionVariant missing in TcoMedallion".to_string())?,
            brand_name: intermediate_rep.brand_name.into_iter().next().ok_or_else(|| "brandName missing in TcoMedallion".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TcoMedallion> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TcoMedallion>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TcoMedallion>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TcoMedallion - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TcoMedallion> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TcoMedallion as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TcoMedallion - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// An entry in text search results.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextSearchResult {
    #[serde(rename = "variant")]
    pub variant: models::TextSearchResultVariant,

    /// ID of a resource.
    #[serde(rename = "id")]
    #[validate(
            length(max = 32),
        )]
    pub id: String,

    /// Short string for labels, titles, summaries...
    #[serde(rename = "label")]
    #[validate(
            length(max = 1024),
        )]
    pub label: String,

}


impl TextSearchResult {
    #[allow(clippy::new_without_default)]
    pub fn new(variant: models::TextSearchResultVariant, id: String, label: String, ) -> TextSearchResult {
        TextSearchResult {
            variant,
            id,
            label,
        }
    }
}

/// Converts the TextSearchResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TextSearchResult {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping variant in query parameter serialization


            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("label".to_string()),
            Some(self.label.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextSearchResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextSearchResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub variant: Vec<models::TextSearchResultVariant>,
            pub id: Vec<String>,
            pub label: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextSearchResult".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "variant" => intermediate_rep.variant.push(<models::TextSearchResultVariant as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextSearchResult".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextSearchResult {
            variant: intermediate_rep.variant.into_iter().next().ok_or_else(|| "variant missing in TextSearchResult".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in TextSearchResult".to_string())?,
            label: intermediate_rep.label.into_iter().next().ok_or_else(|| "label missing in TextSearchResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextSearchResult> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TextSearchResult>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextSearchResult>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TextSearchResult - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TextSearchResult> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextSearchResult as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TextSearchResult - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// Variant for text search result entry.
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TextSearchResultVariant {
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "organisation")]
    Organisation,
}

impl std::fmt::Display for TextSearchResultVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextSearchResultVariant::Product => write!(f, "product"),
            TextSearchResultVariant::Organisation => write!(f, "organisation"),
        }
    }
}

impl std::str::FromStr for TextSearchResultVariant {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "product" => std::result::Result::Ok(TextSearchResultVariant::Product),
            "organisation" => std::result::Result::Ok(TextSearchResultVariant::Organisation),
            _ => std::result::Result::Err(format!("Value not valid: {}", s)),
        }
    }
}

/// List of results of a text search.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextSearchResults {
    #[serde(rename = "results")]
    pub results: Vec<models::TextSearchResult>,

}


impl TextSearchResults {
    #[allow(clippy::new_without_default)]
    pub fn new(results: Vec<models::TextSearchResult>, ) -> TextSearchResults {
        TextSearchResults {
            results,
        }
    }
}

/// Converts the TextSearchResults value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TextSearchResults {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping results in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextSearchResults value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextSearchResults {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub results: Vec<Vec<models::TextSearchResult>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextSearchResults".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "results" => return std::result::Result::Err("Parsing a container in this style is not supported in TextSearchResults".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextSearchResults".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextSearchResults {
            results: intermediate_rep.results.into_iter().next().ok_or_else(|| "results missing in TextSearchResults".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextSearchResults> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TextSearchResults>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextSearchResults>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TextSearchResults - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TextSearchResults> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextSearchResults as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TextSearchResults - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

