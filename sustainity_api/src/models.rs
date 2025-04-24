#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a TryFrom or FromStr implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Accuracy"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Match accuracy.\","]
#[doc = "  \"type\": \"number\","]
#[doc = "  \"format\": \"double\","]
#[doc = "  \"maximum\": 1.0,"]
#[doc = "  \"minimum\": 0.0"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Accuracy(pub f64);
impl std::ops::Deref for Accuracy {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl From<Accuracy> for f64 {
    fn from(value: Accuracy) -> Self {
        value.0
    }
}
impl From<&Accuracy> for Accuracy {
    fn from(value: &Accuracy) -> Self {
        value.clone()
    }
}
impl From<f64> for Accuracy {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for Accuracy {
    type Err = <f64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for Accuracy {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Accuracy {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Accuracy {
    type Error = <f64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for Accuracy {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "Details of BCorp evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of BCorp evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"report_url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"report_url\": {"]
#[doc = "      \"$ref\": \"#/$defs/longString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BCorpMedallion {
    pub id: Id,
    pub report_url: LongString,
}
impl From<&BCorpMedallion> for BCorpMedallion {
    fn from(value: &BCorpMedallion) -> Self {
        value.clone()
    }
}
impl BCorpMedallion {
    pub fn builder() -> builder::BCorpMedallion {
        Default::default()
    }
}
#[doc = "Name of a badge."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of a badge.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bcorp\","]
#[doc = "    \"eu\","]
#[doc = "    \"tco\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BadgeName {
    #[serde(rename = "bcorp")]
    Bcorp,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "tco")]
    Tco,
}
impl From<&BadgeName> for BadgeName {
    fn from(value: &BadgeName) -> Self {
        value.clone()
    }
}
impl ToString for BadgeName {
    fn to_string(&self) -> String {
        match *self {
            Self::Bcorp => "bcorp".to_string(),
            Self::Eu => "eu".to_string(),
            Self::Tco => "tco".to_string(),
        }
    }
}
impl std::str::FromStr for BadgeName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bcorp" => Ok(Self::Bcorp),
            "eu" => Ok(Self::Eu),
            "tco" => Ok(Self::Tco),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "List of product alternative in the given category."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"List of product alternative in the given category.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alternatives\","]
#[doc = "    \"category\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternatives\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/productShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CategoryAlternatives {
    pub alternatives: Vec<ProductShort>,
    pub category: String,
}
impl From<&CategoryAlternatives> for CategoryAlternatives {
    fn from(value: &CategoryAlternatives) -> Self {
        value.clone()
    }
}
impl CategoryAlternatives {
    pub fn builder() -> builder::CategoryAlternatives {
        Default::default()
    }
}
#[doc = "Describes where the related data was retrieved from."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes where the related data was retrieved from.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"wiki\","]
#[doc = "    \"off\","]
#[doc = "    \"eu\","]
#[doc = "    \"b_corp\","]
#[doc = "    \"fti\","]
#[doc = "    \"tco\","]
#[doc = "    \"other\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DataSource {
    #[serde(rename = "wiki")]
    Wiki,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "b_corp")]
    BCorp,
    #[serde(rename = "fti")]
    Fti,
    #[serde(rename = "tco")]
    Tco,
    #[serde(rename = "other")]
    Other,
}
impl From<&DataSource> for DataSource {
    fn from(value: &DataSource) -> Self {
        value.clone()
    }
}
impl ToString for DataSource {
    fn to_string(&self) -> String {
        match *self {
            Self::Wiki => "wiki".to_string(),
            Self::Off => "off".to_string(),
            Self::Eu => "eu".to_string(),
            Self::BCorp => "b_corp".to_string(),
            Self::Fti => "fti".to_string(),
            Self::Tco => "tco".to_string(),
            Self::Other => "other".to_string(),
        }
    }
}
impl std::str::FromStr for DataSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "wiki" => Ok(Self::Wiki),
            "off" => Ok(Self::Off),
            "eu" => Ok(Self::Eu),
            "b_corp" => Ok(Self::BCorp),
            "fti" => Ok(Self::Fti),
            "tco" => Ok(Self::Tco),
            "other" => Ok(Self::Other),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of EU Ecolabel evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of EU Ecolabel evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"matchAccuracy\": {"]
#[doc = "      \"$ref\": \"#/$defs/accuracy\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EuEcolabelMedallion {
    #[serde(
        rename = "matchAccuracy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub match_accuracy: Option<Accuracy>,
}
impl From<&EuEcolabelMedallion> for EuEcolabelMedallion {
    fn from(value: &EuEcolabelMedallion) -> Self {
        value.clone()
    }
}
impl EuEcolabelMedallion {
    pub fn builder() -> builder::EuEcolabelMedallion {
        Default::default()
    }
}
#[doc = "Details of Fashion Transparency Index evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of Fashion Transparency Index evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"score\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"score\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FtiMedallion {
    pub score: i64,
}
impl From<&FtiMedallion> for FtiMedallion {
    fn from(value: &FtiMedallion) -> Self {
        value.clone()
    }
}
impl FtiMedallion {
    pub fn builder() -> builder::FtiMedallion {
        Default::default()
    }
}
#[doc = "ID of a resource."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ID of a resource.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Id(String);
impl std::ops::Deref for Id {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Id {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Image path/URL with its source."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Image path/URL with its source.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"image\","]
#[doc = "    \"source\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"image\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/dataSource\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Image {
    pub image: String,
    pub source: DataSource,
}
impl From<&Image> for Image {
    fn from(value: &Image) -> Self {
        value.clone()
    }
}
impl Image {
    pub fn builder() -> builder::Image {
        Default::default()
    }
}
#[doc = "List of all items in the library."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"List of all items in the library.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"items\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"items\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/libraryItemShort\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LibraryContents {
    pub items: Vec<LibraryItemShort>,
}
impl From<&LibraryContents> for LibraryContents {
    fn from(value: &LibraryContents) -> Self {
        value.clone()
    }
}
impl LibraryContents {
    pub fn builder() -> builder::LibraryContents {
        Default::default()
    }
}
#[doc = "Full contents of a library item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Full contents of a library item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"article\","]
#[doc = "    \"id\","]
#[doc = "    \"summary\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"article\": {"]
#[doc = "      \"$ref\": \"#/$defs/longString\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/libraryTopic\""]
#[doc = "    },"]
#[doc = "    \"presentation\": {"]
#[doc = "      \"$ref\": \"#/$defs/presentation\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LibraryItemFull {
    pub article: LongString,
    pub id: LibraryTopic,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presentation: Option<Presentation>,
    pub summary: ShortString,
    pub title: ShortString,
}
impl From<&LibraryItemFull> for LibraryItemFull {
    fn from(value: &LibraryItemFull) -> Self {
        value.clone()
    }
}
impl LibraryItemFull {
    pub fn builder() -> builder::LibraryItemFull {
        Default::default()
    }
}
#[doc = "Short summary of a library item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short summary of a library item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"summary\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/libraryTopic\""]
#[doc = "    },"]
#[doc = "    \"summary\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LibraryItemShort {
    pub id: LibraryTopic,
    pub summary: ShortString,
    pub title: ShortString,
}
impl From<&LibraryItemShort> for LibraryItemShort {
    fn from(value: &LibraryItemShort) -> Self {
        value.clone()
    }
}
impl LibraryItemShort {
    pub fn builder() -> builder::LibraryItemShort {
        Default::default()
    }
}
#[doc = "Name of a topic in the library."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of a topic in the library.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"info:main\","]
#[doc = "    \"info:for_producers\","]
#[doc = "    \"info:faq\","]
#[doc = "    \"data:wiki\","]
#[doc = "    \"data:open_food_facts\","]
#[doc = "    \"cert:bcorp\","]
#[doc = "    \"cert:eu_ecolabel\","]
#[doc = "    \"cert:tco\","]
#[doc = "    \"cert:fti\","]
#[doc = "    \"other:not_found\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum LibraryTopic {
    #[serde(rename = "info:main")]
    InfoMain,
    #[serde(rename = "info:for_producers")]
    InfoForProducers,
    #[serde(rename = "info:faq")]
    InfoFaq,
    #[serde(rename = "data:wiki")]
    DataWiki,
    #[serde(rename = "data:open_food_facts")]
    DataOpenFoodFacts,
    #[serde(rename = "cert:bcorp")]
    CertBcorp,
    #[serde(rename = "cert:eu_ecolabel")]
    CertEuEcolabel,
    #[serde(rename = "cert:tco")]
    CertTco,
    #[serde(rename = "cert:fti")]
    CertFti,
    #[serde(rename = "other:not_found")]
    OtherNotFound,
}
impl From<&LibraryTopic> for LibraryTopic {
    fn from(value: &LibraryTopic) -> Self {
        value.clone()
    }
}
impl ToString for LibraryTopic {
    fn to_string(&self) -> String {
        match *self {
            Self::InfoMain => "info:main".to_string(),
            Self::InfoForProducers => "info:for_producers".to_string(),
            Self::InfoFaq => "info:faq".to_string(),
            Self::DataWiki => "data:wiki".to_string(),
            Self::DataOpenFoodFacts => "data:open_food_facts".to_string(),
            Self::CertBcorp => "cert:bcorp".to_string(),
            Self::CertEuEcolabel => "cert:eu_ecolabel".to_string(),
            Self::CertTco => "cert:tco".to_string(),
            Self::CertFti => "cert:fti".to_string(),
            Self::OtherNotFound => "other:not_found".to_string(),
        }
    }
}
impl std::str::FromStr for LibraryTopic {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "info:main" => Ok(Self::InfoMain),
            "info:for_producers" => Ok(Self::InfoForProducers),
            "info:faq" => Ok(Self::InfoFaq),
            "data:wiki" => Ok(Self::DataWiki),
            "data:open_food_facts" => Ok(Self::DataOpenFoodFacts),
            "cert:bcorp" => Ok(Self::CertBcorp),
            "cert:eu_ecolabel" => Ok(Self::CertEuEcolabel),
            "cert:tco" => Ok(Self::CertTco),
            "cert:fti" => Ok(Self::CertFti),
            "other:not_found" => Ok(Self::OtherNotFound),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for LibraryTopic {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LibraryTopic {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LibraryTopic {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Long string for descriptions, articles..."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Long string for descriptions, articles...\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1048576"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct LongString(String);
impl std::ops::Deref for LongString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<LongString> for String {
    fn from(value: LongString) -> Self {
        value.0
    }
}
impl From<&LongString> for LongString {
    fn from(value: &LongString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for LongString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for LongString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Long text with its source."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Long text with its source.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/dataSource\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"$ref\": \"#/$defs/longString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LongText {
    pub source: DataSource,
    pub text: LongString,
}
impl From<&LongText> for LongText {
    fn from(value: &LongText) -> Self {
        value.clone()
    }
}
impl LongText {
    pub fn builder() -> builder::LongText {
        Default::default()
    }
}
#[doc = "Combines data from any medallion."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Combines data from any medallion.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"variant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bcorp\": {"]
#[doc = "      \"$ref\": \"#/$defs/bCorpMedallion\""]
#[doc = "    },"]
#[doc = "    \"euEcolabel\": {"]
#[doc = "      \"$ref\": \"#/$defs/euEcolabelMedallion\""]
#[doc = "    },"]
#[doc = "    \"fti\": {"]
#[doc = "      \"$ref\": \"#/$defs/ftiMedallion\""]
#[doc = "    },"]
#[doc = "    \"sustainity\": {"]
#[doc = "      \"$ref\": \"#/$defs/sustainityMedallion\""]
#[doc = "    },"]
#[doc = "    \"tco\": {"]
#[doc = "      \"$ref\": \"#/$defs/tcoMedallion\""]
#[doc = "    },"]
#[doc = "    \"variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/medallionVariant\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Medallion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcorp: Option<BCorpMedallion>,
    #[serde(
        rename = "euEcolabel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub eu_ecolabel: Option<EuEcolabelMedallion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fti: Option<FtiMedallion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sustainity: Option<SustainityMedallion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tco: Option<TcoMedallion>,
    pub variant: MedallionVariant,
}
impl From<&Medallion> for Medallion {
    fn from(value: &Medallion) -> Self {
        value.clone()
    }
}
impl Medallion {
    pub fn builder() -> builder::Medallion {
        Default::default()
    }
}
#[doc = "Medallion variant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Medallion variant.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bCorp\","]
#[doc = "    \"euEcolabel\","]
#[doc = "    \"fti\","]
#[doc = "    \"sustainity\","]
#[doc = "    \"tco\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MedallionVariant {
    #[serde(rename = "bCorp")]
    BCorp,
    #[serde(rename = "euEcolabel")]
    EuEcolabel,
    #[serde(rename = "fti")]
    Fti,
    #[serde(rename = "sustainity")]
    Sustainity,
    #[serde(rename = "tco")]
    Tco,
}
impl From<&MedallionVariant> for MedallionVariant {
    fn from(value: &MedallionVariant) -> Self {
        value.clone()
    }
}
impl ToString for MedallionVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::BCorp => "bCorp".to_string(),
            Self::EuEcolabel => "euEcolabel".to_string(),
            Self::Fti => "fti".to_string(),
            Self::Sustainity => "sustainity".to_string(),
            Self::Tco => "tco".to_string(),
        }
    }
}
impl std::str::FromStr for MedallionVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bCorp" => Ok(Self::BCorp),
            "euEcolabel" => Ok(Self::EuEcolabel),
            "fti" => Ok(Self::Fti),
            "sustainity" => Ok(Self::Sustainity),
            "tco" => Ok(Self::Tco),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Full organisation data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Full organisation data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"descriptions\","]
#[doc = "    \"images\","]
#[doc = "    \"medallions\","]
#[doc = "    \"names\","]
#[doc = "    \"organisationIds\","]
#[doc = "    \"products\","]
#[doc = "    \"websites\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"descriptions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/longText\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/image\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"medallions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/medallion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/shortText\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"organisationIds\": {"]
#[doc = "      \"$ref\": \"#/$defs/organisationIds\""]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/regionCode\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"products\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/productShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"websites\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/shortString\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrganisationFull {
    pub descriptions: Vec<LongText>,
    pub images: Vec<Image>,
    pub medallions: Vec<Medallion>,
    pub names: Vec<ShortText>,
    #[serde(rename = "organisationIds")]
    pub organisation_ids: OrganisationIds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<RegionCode>,
    pub products: Vec<ProductShort>,
    pub websites: Vec<ShortString>,
}
impl From<&OrganisationFull> for OrganisationFull {
    fn from(value: &OrganisationFull) -> Self {
        value.clone()
    }
}
impl OrganisationFull {
    pub fn builder() -> builder::OrganisationFull {
        Default::default()
    }
}
#[doc = "Enumerates organisation ID variants."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Enumerates organisation ID variants.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"wiki\","]
#[doc = "    \"vat\","]
#[doc = "    \"www\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrganisationIdVariant {
    #[serde(rename = "wiki")]
    Wiki,
    #[serde(rename = "vat")]
    Vat,
    #[serde(rename = "www")]
    Www,
}
impl From<&OrganisationIdVariant> for OrganisationIdVariant {
    fn from(value: &OrganisationIdVariant) -> Self {
        value.clone()
    }
}
impl ToString for OrganisationIdVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Wiki => "wiki".to_string(),
            Self::Vat => "vat".to_string(),
            Self::Www => "www".to_string(),
        }
    }
}
impl std::str::FromStr for OrganisationIdVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "wiki" => Ok(Self::Wiki),
            "vat" => Ok(Self::Vat),
            "www" => Ok(Self::Www),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IDs of an organisation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"IDs of an organisation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"domains\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"vat\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"wiki\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrganisationIds {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub domains: Vec<Id>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vat: Vec<Id>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wiki: Vec<Id>,
}
impl From<&OrganisationIds> for OrganisationIds {
    fn from(value: &OrganisationIds) -> Self {
        value.clone()
    }
}
impl OrganisationIds {
    pub fn builder() -> builder::OrganisationIds {
        Default::default()
    }
}
#[doc = "OrganisationLink"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"organisation_id_variant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"organisation_id_variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/organisationIdVariant\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrganisationLink {
    pub id: Id,
    pub organisation_id_variant: OrganisationIdVariant,
}
impl From<&OrganisationLink> for OrganisationLink {
    fn from(value: &OrganisationLink) -> Self {
        value.clone()
    }
}
impl OrganisationLink {
    pub fn builder() -> builder::OrganisationLink {
        Default::default()
    }
}
#[doc = "Extract from organisation data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Extract from organisation data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"badges\","]
#[doc = "    \"name\","]
#[doc = "    \"organisationIds\","]
#[doc = "    \"scores\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"badges\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/badgeName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/longText\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"organisationIds\": {"]
#[doc = "      \"$ref\": \"#/$defs/organisationIds\""]
#[doc = "    },"]
#[doc = "    \"scores\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/score\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrganisationShort {
    pub badges: Vec<BadgeName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LongText>,
    pub name: ShortString,
    #[serde(rename = "organisationIds")]
    pub organisation_ids: OrganisationIds,
    pub scores: Vec<Score>,
}
impl From<&OrganisationShort> for OrganisationShort {
    fn from(value: &OrganisationShort) -> Self {
        value.clone()
    }
}
impl OrganisationShort {
    pub fn builder() -> builder::OrganisationShort {
        Default::default()
    }
}
#[doc = "Additional data to present together with a library item."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Additional data to present together with a library item.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/presentationEntry\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Presentation {
    pub data: Vec<PresentationEntry>,
}
impl From<&Presentation> for Presentation {
    fn from(value: &Presentation) -> Self {
        value.clone()
    }
}
impl Presentation {
    pub fn builder() -> builder::Presentation {
        Default::default()
    }
}
#[doc = "An entry in a presentation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An entry in a presentation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"score\","]
#[doc = "    \"wiki_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"score\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"wiki_id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PresentationEntry {
    pub name: ShortString,
    pub score: i64,
    pub wiki_id: Id,
}
impl From<&PresentationEntry> for PresentationEntry {
    fn from(value: &PresentationEntry) -> Self {
        value.clone()
    }
}
impl PresentationEntry {
    pub fn builder() -> builder::PresentationEntry {
        Default::default()
    }
}
#[doc = "Full product data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Full product data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"alternatives\","]
#[doc = "    \"descriptions\","]
#[doc = "    \"images\","]
#[doc = "    \"manufacturers\","]
#[doc = "    \"medallions\","]
#[doc = "    \"names\","]
#[doc = "    \"productIds\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternatives\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/categoryAlternatives\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"descriptions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/longText\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"images\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/image\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"manufacturers\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/organisationShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"medallions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/medallion\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"names\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/shortText\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"origins\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/regionCode\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"productIds\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIds\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductFull {
    pub alternatives: Vec<CategoryAlternatives>,
    pub descriptions: Vec<LongText>,
    pub images: Vec<Image>,
    pub manufacturers: Vec<OrganisationShort>,
    pub medallions: Vec<Medallion>,
    pub names: Vec<ShortText>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub origins: Vec<RegionCode>,
    #[serde(rename = "productIds")]
    pub product_ids: ProductIds,
}
impl From<&ProductFull> for ProductFull {
    fn from(value: &ProductFull) -> Self {
        value.clone()
    }
}
impl ProductFull {
    pub fn builder() -> builder::ProductFull {
        Default::default()
    }
}
#[doc = "Enumerates product ID variants."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Enumerates product ID variants.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ean\","]
#[doc = "    \"gtin\","]
#[doc = "    \"wiki\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ProductIdVariant {
    #[serde(rename = "ean")]
    Ean,
    #[serde(rename = "gtin")]
    Gtin,
    #[serde(rename = "wiki")]
    Wiki,
}
impl From<&ProductIdVariant> for ProductIdVariant {
    fn from(value: &ProductIdVariant) -> Self {
        value.clone()
    }
}
impl ToString for ProductIdVariant {
    fn to_string(&self) -> String {
        match *self {
            Self::Ean => "ean".to_string(),
            Self::Gtin => "gtin".to_string(),
            Self::Wiki => "wiki".to_string(),
        }
    }
}
impl std::str::FromStr for ProductIdVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ean" => Ok(Self::Ean),
            "gtin" => Ok(Self::Gtin),
            "wiki" => Ok(Self::Wiki),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IDs of a product."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"IDs of a product.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"eans\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"gtins\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"wiki\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/id\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductIds {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub eans: Vec<Id>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gtins: Vec<Id>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wiki: Vec<Id>,
}
impl From<&ProductIds> for ProductIds {
    fn from(value: &ProductIds) -> Self {
        value.clone()
    }
}
impl ProductIds {
    pub fn builder() -> builder::ProductIds {
        Default::default()
    }
}
#[doc = "ProductLink"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"product_id_variant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"product_id_variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIdVariant\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductLink {
    pub id: Id,
    pub product_id_variant: ProductIdVariant,
}
impl From<&ProductLink> for ProductLink {
    fn from(value: &ProductLink) -> Self {
        value.clone()
    }
}
impl ProductLink {
    pub fn builder() -> builder::ProductLink {
        Default::default()
    }
}
#[doc = "Extract from product data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Extract from product data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"badges\","]
#[doc = "    \"name\","]
#[doc = "    \"productIds\","]
#[doc = "    \"scores\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"badges\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/badgeName\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/longText\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"productIds\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIds\""]
#[doc = "    },"]
#[doc = "    \"scores\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/score\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductShort {
    pub badges: Vec<BadgeName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<LongText>,
    pub name: ShortString,
    #[serde(rename = "productIds")]
    pub product_ids: ProductIds,
    pub scores: Vec<Score>,
}
impl From<&ProductShort> for ProductShort {
    fn from(value: &ProductShort) -> Self {
        value.clone()
    }
}
impl ProductShort {
    pub fn builder() -> builder::ProductShort {
        Default::default()
    }
}
#[doc = "Code of a region."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Code of a region.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 3,"]
#[doc = "  \"minLength\": 3"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RegionCode(String);
impl std::ops::Deref for RegionCode {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RegionCode> for String {
    fn from(value: RegionCode) -> Self {
        value.0
    }
}
impl From<&RegionCode> for RegionCode {
    fn from(value: &RegionCode) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RegionCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.len() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RegionCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "score."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"score.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"score\","]
#[doc = "    \"scorerName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"score\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"format\": \"int64\""]
#[doc = "    },"]
#[doc = "    \"scorerName\": {"]
#[doc = "      \"$ref\": \"#/$defs/scorerName\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Score {
    pub score: i64,
    #[serde(rename = "scorerName")]
    pub scorer_name: ScorerName,
}
impl From<&Score> for Score {
    fn from(value: &Score) -> Self {
        value.clone()
    }
}
impl Score {
    pub fn builder() -> builder::Score {
        Default::default()
    }
}
#[doc = "Name of a scorer."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of a scorer.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"fti\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ScorerName {
    #[serde(rename = "fti")]
    Fti,
}
impl From<&ScorerName> for ScorerName {
    fn from(value: &ScorerName) -> Self {
        value.clone()
    }
}
impl ToString for ScorerName {
    fn to_string(&self) -> String {
        match *self {
            Self::Fti => "fti".to_string(),
        }
    }
}
impl std::str::FromStr for ScorerName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "fti" => Ok(Self::Fti),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Short string for labels, titles, summaries..."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short string for labels, titles, summaries...\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1024"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ShortString(String);
impl std::ops::Deref for ShortString {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<ShortString> for String {
    fn from(value: ShortString) -> Self {
        value.0
    }
}
impl From<&ShortString> for ShortString {
    fn from(value: &ShortString) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ShortString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if value.len() > 1024usize {
            return Err("longer than 1024 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for ShortString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Short text with its source."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short text with its source.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source\": {"]
#[doc = "      \"$ref\": \"#/$defs/dataSource\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ShortText {
    pub source: DataSource,
    pub text: ShortString,
}
impl From<&ShortText> for ShortText {
    fn from(value: &ShortText) -> Self {
        value.clone()
    }
}
impl ShortText {
    pub fn builder() -> builder::ShortText {
        Default::default()
    }
}
#[doc = "Details of Sustainity evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of Sustainity evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"score\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"score\": {"]
#[doc = "      \"$ref\": \"#/$defs/sustainityScore\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SustainityMedallion {
    pub score: SustainityScore,
}
impl From<&SustainityMedallion> for SustainityMedallion {
    fn from(value: &SustainityMedallion) -> Self {
        value.clone()
    }
}
impl SustainityMedallion {
    pub fn builder() -> builder::SustainityMedallion {
        Default::default()
    }
}
#[doc = "Explanation of calculation of the Sustainity score."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Explanation of calculation of the Sustainity score.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"total\","]
#[doc = "    \"tree\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"total\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"tree\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/sustainityScoreBranch\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SustainityScore {
    pub total: f64,
    pub tree: Vec<SustainityScoreBranch>,
}
impl From<&SustainityScore> for SustainityScore {
    fn from(value: &SustainityScore) -> Self {
        value.clone()
    }
}
impl SustainityScore {
    pub fn builder() -> builder::SustainityScore {
        Default::default()
    }
}
#[doc = "Part of explanation of calculation of the Sustainity score."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Part of explanation of calculation of the Sustainity score.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"branches\","]
#[doc = "    \"category\","]
#[doc = "    \"score\","]
#[doc = "    \"weight\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"branches\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/sustainityScoreBranch\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"$ref\": \"#/$defs/sustainityScoreCategory\""]
#[doc = "    },"]
#[doc = "    \"score\": {"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\""]
#[doc = "    },"]
#[doc = "    \"weight\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SustainityScoreBranch {
    pub branches: Vec<SustainityScoreBranch>,
    pub category: SustainityScoreCategory,
    pub score: f64,
    pub weight: i64,
}
impl From<&SustainityScoreBranch> for SustainityScoreBranch {
    fn from(value: &SustainityScoreBranch) -> Self {
        value.clone()
    }
}
impl SustainityScoreBranch {
    pub fn builder() -> builder::SustainityScoreBranch {
        Default::default()
    }
}
#[doc = "Categories in the sustainity score"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Categories in the sustainity score\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"dataAvailability\","]
#[doc = "    \"producerKnown\","]
#[doc = "    \"productionPlaceKnown\","]
#[doc = "    \"idKnown\","]
#[doc = "    \"categoryAssigned\","]
#[doc = "    \"category\","]
#[doc = "    \"warrantyLength\","]
#[doc = "    \"numCerts\","]
#[doc = "    \"atLeastOneCert\","]
#[doc = "    \"atLeastTwoCerts\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SustainityScoreCategory {
    #[serde(rename = "dataAvailability")]
    DataAvailability,
    #[serde(rename = "producerKnown")]
    ProducerKnown,
    #[serde(rename = "productionPlaceKnown")]
    ProductionPlaceKnown,
    #[serde(rename = "idKnown")]
    IdKnown,
    #[serde(rename = "categoryAssigned")]
    CategoryAssigned,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "warrantyLength")]
    WarrantyLength,
    #[serde(rename = "numCerts")]
    NumCerts,
    #[serde(rename = "atLeastOneCert")]
    AtLeastOneCert,
    #[serde(rename = "atLeastTwoCerts")]
    AtLeastTwoCerts,
}
impl From<&SustainityScoreCategory> for SustainityScoreCategory {
    fn from(value: &SustainityScoreCategory) -> Self {
        value.clone()
    }
}
impl ToString for SustainityScoreCategory {
    fn to_string(&self) -> String {
        match *self {
            Self::DataAvailability => "dataAvailability".to_string(),
            Self::ProducerKnown => "producerKnown".to_string(),
            Self::ProductionPlaceKnown => "productionPlaceKnown".to_string(),
            Self::IdKnown => "idKnown".to_string(),
            Self::CategoryAssigned => "categoryAssigned".to_string(),
            Self::Category => "category".to_string(),
            Self::WarrantyLength => "warrantyLength".to_string(),
            Self::NumCerts => "numCerts".to_string(),
            Self::AtLeastOneCert => "atLeastOneCert".to_string(),
            Self::AtLeastTwoCerts => "atLeastTwoCerts".to_string(),
        }
    }
}
impl std::str::FromStr for SustainityScoreCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "dataAvailability" => Ok(Self::DataAvailability),
            "producerKnown" => Ok(Self::ProducerKnown),
            "productionPlaceKnown" => Ok(Self::ProductionPlaceKnown),
            "idKnown" => Ok(Self::IdKnown),
            "categoryAssigned" => Ok(Self::CategoryAssigned),
            "category" => Ok(Self::Category),
            "warrantyLength" => Ok(Self::WarrantyLength),
            "numCerts" => Ok(Self::NumCerts),
            "atLeastOneCert" => Ok(Self::AtLeastOneCert),
            "atLeastTwoCerts" => Ok(Self::AtLeastTwoCerts),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SustainityScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SustainityScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SustainityScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Details of TCO evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of TCO evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"brandName\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"brandName\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TcoMedallion {
    #[serde(rename = "brandName")]
    pub brand_name: ShortString,
}
impl From<&TcoMedallion> for TcoMedallion {
    fn from(value: &TcoMedallion) -> Self {
        value.clone()
    }
}
impl TcoMedallion {
    pub fn builder() -> builder::TcoMedallion {
        Default::default()
    }
}
#[doc = "Link to a text search result."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Link to a text search result.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/organisationLink\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/productLink\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TextSearchLink {
    OrganisationLink(OrganisationLink),
    ProductLink(ProductLink),
}
impl From<&TextSearchLink> for TextSearchLink {
    fn from(value: &TextSearchLink) -> Self {
        value.clone()
    }
}
impl From<OrganisationLink> for TextSearchLink {
    fn from(value: OrganisationLink) -> Self {
        Self::OrganisationLink(value)
    }
}
impl From<ProductLink> for TextSearchLink {
    fn from(value: ProductLink) -> Self {
        Self::ProductLink(value)
    }
}
#[doc = "Link to a text search result."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Link to a text search result.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"$ref\": \"#/$defs/id\""]
#[doc = "    },"]
#[doc = "    \"organisation_id_variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/organisationIdVariant\""]
#[doc = "    },"]
#[doc = "    \"product_id_variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/productIdVariant\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextSearchLinkHack {
    pub id: Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organisation_id_variant: Option<OrganisationIdVariant>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_id_variant: Option<ProductIdVariant>,
}
impl From<&TextSearchLinkHack> for TextSearchLinkHack {
    fn from(value: &TextSearchLinkHack) -> Self {
        value.clone()
    }
}
impl TextSearchLinkHack {
    pub fn builder() -> builder::TextSearchLinkHack {
        Default::default()
    }
}
#[doc = "An entry in text search results."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An entry in text search results.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"label\","]
#[doc = "    \"link\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"link\": {"]
#[doc = "      \"$ref\": \"#/$defs/textSearchLinkHack\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextSearchResult {
    pub label: ShortString,
    pub link: TextSearchLinkHack,
}
impl From<&TextSearchResult> for TextSearchResult {
    fn from(value: &TextSearchResult) -> Self {
        value.clone()
    }
}
impl TextSearchResult {
    pub fn builder() -> builder::TextSearchResult {
        Default::default()
    }
}
#[doc = "List of results of a text search."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"List of results of a text search.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"results\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"results\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/textSearchResult\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TextSearchResults {
    pub results: Vec<TextSearchResult>,
}
impl From<&TextSearchResults> for TextSearchResults {
    fn from(value: &TextSearchResults) -> Self {
        value.clone()
    }
}
impl TextSearchResults {
    pub fn builder() -> builder::TextSearchResults {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BCorpMedallion {
        id: Result<super::Id, String>,
        report_url: Result<super::LongString, String>,
    }
    impl Default for BCorpMedallion {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                report_url: Err("no value supplied for report_url".to_string()),
            }
        }
    }
    impl BCorpMedallion {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn report_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LongString>,
            T::Error: std::fmt::Display,
        {
            self.report_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for report_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BCorpMedallion> for super::BCorpMedallion {
        type Error = super::error::ConversionError;
        fn try_from(value: BCorpMedallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                report_url: value.report_url?,
            })
        }
    }
    impl From<super::BCorpMedallion> for BCorpMedallion {
        fn from(value: super::BCorpMedallion) -> Self {
            Self {
                id: Ok(value.id),
                report_url: Ok(value.report_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CategoryAlternatives {
        alternatives: Result<Vec<super::ProductShort>, String>,
        category: Result<String, String>,
    }
    impl Default for CategoryAlternatives {
        fn default() -> Self {
            Self {
                alternatives: Err("no value supplied for alternatives".to_string()),
                category: Err("no value supplied for category".to_string()),
            }
        }
    }
    impl CategoryAlternatives {
        pub fn alternatives<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProductShort>>,
            T::Error: std::fmt::Display,
        {
            self.alternatives = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alternatives: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<CategoryAlternatives> for super::CategoryAlternatives {
        type Error = super::error::ConversionError;
        fn try_from(value: CategoryAlternatives) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternatives: value.alternatives?,
                category: value.category?,
            })
        }
    }
    impl From<super::CategoryAlternatives> for CategoryAlternatives {
        fn from(value: super::CategoryAlternatives) -> Self {
            Self {
                alternatives: Ok(value.alternatives),
                category: Ok(value.category),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuEcolabelMedallion {
        match_accuracy: Result<Option<super::Accuracy>, String>,
    }
    impl Default for EuEcolabelMedallion {
        fn default() -> Self {
            Self {
                match_accuracy: Ok(Default::default()),
            }
        }
    }
    impl EuEcolabelMedallion {
        pub fn match_accuracy<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Accuracy>>,
            T::Error: std::fmt::Display,
        {
            self.match_accuracy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_accuracy: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<EuEcolabelMedallion> for super::EuEcolabelMedallion {
        type Error = super::error::ConversionError;
        fn try_from(value: EuEcolabelMedallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                match_accuracy: value.match_accuracy?,
            })
        }
    }
    impl From<super::EuEcolabelMedallion> for EuEcolabelMedallion {
        fn from(value: super::EuEcolabelMedallion) -> Self {
            Self {
                match_accuracy: Ok(value.match_accuracy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FtiMedallion {
        score: Result<i64, String>,
    }
    impl Default for FtiMedallion {
        fn default() -> Self {
            Self {
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl FtiMedallion {
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<FtiMedallion> for super::FtiMedallion {
        type Error = super::error::ConversionError;
        fn try_from(value: FtiMedallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
            })
        }
    }
    impl From<super::FtiMedallion> for FtiMedallion {
        fn from(value: super::FtiMedallion) -> Self {
            Self {
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Image {
        image: Result<String, String>,
        source: Result<super::DataSource, String>,
    }
    impl Default for Image {
        fn default() -> Self {
            Self {
                image: Err("no value supplied for image".to_string()),
                source: Err("no value supplied for source".to_string()),
            }
        }
    }
    impl Image {
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DataSource>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Image> for super::Image {
        type Error = super::error::ConversionError;
        fn try_from(value: Image) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                source: value.source?,
            })
        }
    }
    impl From<super::Image> for Image {
        fn from(value: super::Image) -> Self {
            Self {
                image: Ok(value.image),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryContents {
        items: Result<Vec<super::LibraryItemShort>, String>,
    }
    impl Default for LibraryContents {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl LibraryContents {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LibraryItemShort>>,
            T::Error: std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LibraryContents> for super::LibraryContents {
        type Error = super::error::ConversionError;
        fn try_from(value: LibraryContents) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl From<super::LibraryContents> for LibraryContents {
        fn from(value: super::LibraryContents) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryItemFull {
        article: Result<super::LongString, String>,
        id: Result<super::LibraryTopic, String>,
        presentation: Result<Option<super::Presentation>, String>,
        summary: Result<super::ShortString, String>,
        title: Result<super::ShortString, String>,
    }
    impl Default for LibraryItemFull {
        fn default() -> Self {
            Self {
                article: Err("no value supplied for article".to_string()),
                id: Err("no value supplied for id".to_string()),
                presentation: Ok(Default::default()),
                summary: Err("no value supplied for summary".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl LibraryItemFull {
        pub fn article<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LongString>,
            T::Error: std::fmt::Display,
        {
            self.article = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for article: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LibraryTopic>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn presentation<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Presentation>>,
            T::Error: std::fmt::Display,
        {
            self.presentation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for presentation: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LibraryItemFull> for super::LibraryItemFull {
        type Error = super::error::ConversionError;
        fn try_from(value: LibraryItemFull) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                article: value.article?,
                id: value.id?,
                presentation: value.presentation?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl From<super::LibraryItemFull> for LibraryItemFull {
        fn from(value: super::LibraryItemFull) -> Self {
            Self {
                article: Ok(value.article),
                id: Ok(value.id),
                presentation: Ok(value.presentation),
                summary: Ok(value.summary),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryItemShort {
        id: Result<super::LibraryTopic, String>,
        summary: Result<super::ShortString, String>,
        title: Result<super::ShortString, String>,
    }
    impl Default for LibraryItemShort {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                summary: Err("no value supplied for summary".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl LibraryItemShort {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LibraryTopic>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LibraryItemShort> for super::LibraryItemShort {
        type Error = super::error::ConversionError;
        fn try_from(value: LibraryItemShort) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl From<super::LibraryItemShort> for LibraryItemShort {
        fn from(value: super::LibraryItemShort) -> Self {
            Self {
                id: Ok(value.id),
                summary: Ok(value.summary),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LongText {
        source: Result<super::DataSource, String>,
        text: Result<super::LongString, String>,
    }
    impl Default for LongText {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl LongText {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DataSource>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::LongString>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<LongText> for super::LongText {
        type Error = super::error::ConversionError;
        fn try_from(value: LongText) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                text: value.text?,
            })
        }
    }
    impl From<super::LongText> for LongText {
        fn from(value: super::LongText) -> Self {
            Self {
                source: Ok(value.source),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Medallion {
        bcorp: Result<Option<super::BCorpMedallion>, String>,
        eu_ecolabel: Result<Option<super::EuEcolabelMedallion>, String>,
        fti: Result<Option<super::FtiMedallion>, String>,
        sustainity: Result<Option<super::SustainityMedallion>, String>,
        tco: Result<Option<super::TcoMedallion>, String>,
        variant: Result<super::MedallionVariant, String>,
    }
    impl Default for Medallion {
        fn default() -> Self {
            Self {
                bcorp: Ok(Default::default()),
                eu_ecolabel: Ok(Default::default()),
                fti: Ok(Default::default()),
                sustainity: Ok(Default::default()),
                tco: Ok(Default::default()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl Medallion {
        pub fn bcorp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BCorpMedallion>>,
            T::Error: std::fmt::Display,
        {
            self.bcorp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bcorp: {}", e));
            self
        }
        pub fn eu_ecolabel<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::EuEcolabelMedallion>>,
            T::Error: std::fmt::Display,
        {
            self.eu_ecolabel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eu_ecolabel: {}", e));
            self
        }
        pub fn fti<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::FtiMedallion>>,
            T::Error: std::fmt::Display,
        {
            self.fti = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fti: {}", e));
            self
        }
        pub fn sustainity<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::SustainityMedallion>>,
            T::Error: std::fmt::Display,
        {
            self.sustainity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sustainity: {}", e));
            self
        }
        pub fn tco<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TcoMedallion>>,
            T::Error: std::fmt::Display,
        {
            self.tco = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tco: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::MedallionVariant>,
            T::Error: std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Medallion> for super::Medallion {
        type Error = super::error::ConversionError;
        fn try_from(value: Medallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                bcorp: value.bcorp?,
                eu_ecolabel: value.eu_ecolabel?,
                fti: value.fti?,
                sustainity: value.sustainity?,
                tco: value.tco?,
                variant: value.variant?,
            })
        }
    }
    impl From<super::Medallion> for Medallion {
        fn from(value: super::Medallion) -> Self {
            Self {
                bcorp: Ok(value.bcorp),
                eu_ecolabel: Ok(value.eu_ecolabel),
                fti: Ok(value.fti),
                sustainity: Ok(value.sustainity),
                tco: Ok(value.tco),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationFull {
        descriptions: Result<Vec<super::LongText>, String>,
        images: Result<Vec<super::Image>, String>,
        medallions: Result<Vec<super::Medallion>, String>,
        names: Result<Vec<super::ShortText>, String>,
        organisation_ids: Result<super::OrganisationIds, String>,
        origins: Result<Vec<super::RegionCode>, String>,
        products: Result<Vec<super::ProductShort>, String>,
        websites: Result<Vec<super::ShortString>, String>,
    }
    impl Default for OrganisationFull {
        fn default() -> Self {
            Self {
                descriptions: Err("no value supplied for descriptions".to_string()),
                images: Err("no value supplied for images".to_string()),
                medallions: Err("no value supplied for medallions".to_string()),
                names: Err("no value supplied for names".to_string()),
                organisation_ids: Err("no value supplied for organisation_ids".to_string()),
                origins: Ok(Default::default()),
                products: Err("no value supplied for products".to_string()),
                websites: Err("no value supplied for websites".to_string()),
            }
        }
    }
    impl OrganisationFull {
        pub fn descriptions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LongText>>,
            T::Error: std::fmt::Display,
        {
            self.descriptions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for descriptions: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn medallions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Medallion>>,
            T::Error: std::fmt::Display,
        {
            self.medallions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medallions: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ShortText>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn organisation_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OrganisationIds>,
            T::Error: std::fmt::Display,
        {
            self.organisation_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organisation_ids: {}",
                    e
                )
            });
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::RegionCode>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ProductShort>>,
            T::Error: std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
        pub fn websites<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ShortString>>,
            T::Error: std::fmt::Display,
        {
            self.websites = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for websites: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganisationFull> for super::OrganisationFull {
        type Error = super::error::ConversionError;
        fn try_from(value: OrganisationFull) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                descriptions: value.descriptions?,
                images: value.images?,
                medallions: value.medallions?,
                names: value.names?,
                organisation_ids: value.organisation_ids?,
                origins: value.origins?,
                products: value.products?,
                websites: value.websites?,
            })
        }
    }
    impl From<super::OrganisationFull> for OrganisationFull {
        fn from(value: super::OrganisationFull) -> Self {
            Self {
                descriptions: Ok(value.descriptions),
                images: Ok(value.images),
                medallions: Ok(value.medallions),
                names: Ok(value.names),
                organisation_ids: Ok(value.organisation_ids),
                origins: Ok(value.origins),
                products: Ok(value.products),
                websites: Ok(value.websites),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationIds {
        domains: Result<Vec<super::Id>, String>,
        vat: Result<Vec<super::Id>, String>,
        wiki: Result<Vec<super::Id>, String>,
    }
    impl Default for OrganisationIds {
        fn default() -> Self {
            Self {
                domains: Ok(Default::default()),
                vat: Ok(Default::default()),
                wiki: Ok(Default::default()),
            }
        }
    }
    impl OrganisationIds {
        pub fn domains<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.domains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domains: {}", e));
            self
        }
        pub fn vat<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.vat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vat: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganisationIds> for super::OrganisationIds {
        type Error = super::error::ConversionError;
        fn try_from(value: OrganisationIds) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                domains: value.domains?,
                vat: value.vat?,
                wiki: value.wiki?,
            })
        }
    }
    impl From<super::OrganisationIds> for OrganisationIds {
        fn from(value: super::OrganisationIds) -> Self {
            Self {
                domains: Ok(value.domains),
                vat: Ok(value.vat),
                wiki: Ok(value.wiki),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationLink {
        id: Result<super::Id, String>,
        organisation_id_variant: Result<super::OrganisationIdVariant, String>,
    }
    impl Default for OrganisationLink {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                organisation_id_variant: Err(
                    "no value supplied for organisation_id_variant".to_string()
                ),
            }
        }
    }
    impl OrganisationLink {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn organisation_id_variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OrganisationIdVariant>,
            T::Error: std::fmt::Display,
        {
            self.organisation_id_variant = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organisation_id_variant: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<OrganisationLink> for super::OrganisationLink {
        type Error = super::error::ConversionError;
        fn try_from(value: OrganisationLink) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                organisation_id_variant: value.organisation_id_variant?,
            })
        }
    }
    impl From<super::OrganisationLink> for OrganisationLink {
        fn from(value: super::OrganisationLink) -> Self {
            Self {
                id: Ok(value.id),
                organisation_id_variant: Ok(value.organisation_id_variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationShort {
        badges: Result<Vec<super::BadgeName>, String>,
        description: Result<Option<super::LongText>, String>,
        name: Result<super::ShortString, String>,
        organisation_ids: Result<super::OrganisationIds, String>,
        scores: Result<Vec<super::Score>, String>,
    }
    impl Default for OrganisationShort {
        fn default() -> Self {
            Self {
                badges: Err("no value supplied for badges".to_string()),
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                organisation_ids: Err("no value supplied for organisation_ids".to_string()),
                scores: Err("no value supplied for scores".to_string()),
            }
        }
    }
    impl OrganisationShort {
        pub fn badges<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::BadgeName>>,
            T::Error: std::fmt::Display,
        {
            self.badges = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for badges: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LongText>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn organisation_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::OrganisationIds>,
            T::Error: std::fmt::Display,
        {
            self.organisation_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organisation_ids: {}",
                    e
                )
            });
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Score>>,
            T::Error: std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<OrganisationShort> for super::OrganisationShort {
        type Error = super::error::ConversionError;
        fn try_from(value: OrganisationShort) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                badges: value.badges?,
                description: value.description?,
                name: value.name?,
                organisation_ids: value.organisation_ids?,
                scores: value.scores?,
            })
        }
    }
    impl From<super::OrganisationShort> for OrganisationShort {
        fn from(value: super::OrganisationShort) -> Self {
            Self {
                badges: Ok(value.badges),
                description: Ok(value.description),
                name: Ok(value.name),
                organisation_ids: Ok(value.organisation_ids),
                scores: Ok(value.scores),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Presentation {
        data: Result<Vec<super::PresentationEntry>, String>,
    }
    impl Default for Presentation {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl Presentation {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::PresentationEntry>>,
            T::Error: std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Presentation> for super::Presentation {
        type Error = super::error::ConversionError;
        fn try_from(value: Presentation) -> Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl From<super::Presentation> for Presentation {
        fn from(value: super::Presentation) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PresentationEntry {
        name: Result<super::ShortString, String>,
        score: Result<i64, String>,
        wiki_id: Result<super::Id, String>,
    }
    impl Default for PresentationEntry {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                score: Err("no value supplied for score".to_string()),
                wiki_id: Err("no value supplied for wiki_id".to_string()),
            }
        }
    }
    impl PresentationEntry {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn wiki_id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.wiki_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki_id: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<PresentationEntry> for super::PresentationEntry {
        type Error = super::error::ConversionError;
        fn try_from(value: PresentationEntry) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                score: value.score?,
                wiki_id: value.wiki_id?,
            })
        }
    }
    impl From<super::PresentationEntry> for PresentationEntry {
        fn from(value: super::PresentationEntry) -> Self {
            Self {
                name: Ok(value.name),
                score: Ok(value.score),
                wiki_id: Ok(value.wiki_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductFull {
        alternatives: Result<Vec<super::CategoryAlternatives>, String>,
        descriptions: Result<Vec<super::LongText>, String>,
        images: Result<Vec<super::Image>, String>,
        manufacturers: Result<Vec<super::OrganisationShort>, String>,
        medallions: Result<Vec<super::Medallion>, String>,
        names: Result<Vec<super::ShortText>, String>,
        origins: Result<Vec<super::RegionCode>, String>,
        product_ids: Result<super::ProductIds, String>,
    }
    impl Default for ProductFull {
        fn default() -> Self {
            Self {
                alternatives: Err("no value supplied for alternatives".to_string()),
                descriptions: Err("no value supplied for descriptions".to_string()),
                images: Err("no value supplied for images".to_string()),
                manufacturers: Err("no value supplied for manufacturers".to_string()),
                medallions: Err("no value supplied for medallions".to_string()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                product_ids: Err("no value supplied for product_ids".to_string()),
            }
        }
    }
    impl ProductFull {
        pub fn alternatives<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::CategoryAlternatives>>,
            T::Error: std::fmt::Display,
        {
            self.alternatives = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alternatives: {}", e));
            self
        }
        pub fn descriptions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::LongText>>,
            T::Error: std::fmt::Display,
        {
            self.descriptions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for descriptions: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Image>>,
            T::Error: std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn manufacturers<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::OrganisationShort>>,
            T::Error: std::fmt::Display,
        {
            self.manufacturers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for manufacturers: {}", e));
            self
        }
        pub fn medallions<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Medallion>>,
            T::Error: std::fmt::Display,
        {
            self.medallions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medallions: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::ShortText>>,
            T::Error: std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::RegionCode>>,
            T::Error: std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIds>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductFull> for super::ProductFull {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductFull) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternatives: value.alternatives?,
                descriptions: value.descriptions?,
                images: value.images?,
                manufacturers: value.manufacturers?,
                medallions: value.medallions?,
                names: value.names?,
                origins: value.origins?,
                product_ids: value.product_ids?,
            })
        }
    }
    impl From<super::ProductFull> for ProductFull {
        fn from(value: super::ProductFull) -> Self {
            Self {
                alternatives: Ok(value.alternatives),
                descriptions: Ok(value.descriptions),
                images: Ok(value.images),
                manufacturers: Ok(value.manufacturers),
                medallions: Ok(value.medallions),
                names: Ok(value.names),
                origins: Ok(value.origins),
                product_ids: Ok(value.product_ids),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductIds {
        eans: Result<Vec<super::Id>, String>,
        gtins: Result<Vec<super::Id>, String>,
        wiki: Result<Vec<super::Id>, String>,
    }
    impl Default for ProductIds {
        fn default() -> Self {
            Self {
                eans: Ok(Default::default()),
                gtins: Ok(Default::default()),
                wiki: Ok(Default::default()),
            }
        }
    }
    impl ProductIds {
        pub fn eans<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.eans = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eans: {}", e));
            self
        }
        pub fn gtins<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.gtins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gtins: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Id>>,
            T::Error: std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductIds> for super::ProductIds {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductIds) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                eans: value.eans?,
                gtins: value.gtins?,
                wiki: value.wiki?,
            })
        }
    }
    impl From<super::ProductIds> for ProductIds {
        fn from(value: super::ProductIds) -> Self {
            Self {
                eans: Ok(value.eans),
                gtins: Ok(value.gtins),
                wiki: Ok(value.wiki),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductLink {
        id: Result<super::Id, String>,
        product_id_variant: Result<super::ProductIdVariant, String>,
    }
    impl Default for ProductLink {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                product_id_variant: Err("no value supplied for product_id_variant".to_string()),
            }
        }
    }
    impl ProductLink {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn product_id_variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIdVariant>,
            T::Error: std::fmt::Display,
        {
            self.product_id_variant = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for product_id_variant: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<ProductLink> for super::ProductLink {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductLink) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                product_id_variant: value.product_id_variant?,
            })
        }
    }
    impl From<super::ProductLink> for ProductLink {
        fn from(value: super::ProductLink) -> Self {
            Self {
                id: Ok(value.id),
                product_id_variant: Ok(value.product_id_variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductShort {
        badges: Result<Vec<super::BadgeName>, String>,
        description: Result<Option<super::LongText>, String>,
        name: Result<super::ShortString, String>,
        product_ids: Result<super::ProductIds, String>,
        scores: Result<Vec<super::Score>, String>,
    }
    impl Default for ProductShort {
        fn default() -> Self {
            Self {
                badges: Err("no value supplied for badges".to_string()),
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                product_ids: Err("no value supplied for product_ids".to_string()),
                scores: Err("no value supplied for scores".to_string()),
            }
        }
    }
    impl ProductShort {
        pub fn badges<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::BadgeName>>,
            T::Error: std::fmt::Display,
        {
            self.badges = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for badges: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::LongText>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ProductIds>,
            T::Error: std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Score>>,
            T::Error: std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ProductShort> for super::ProductShort {
        type Error = super::error::ConversionError;
        fn try_from(value: ProductShort) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                badges: value.badges?,
                description: value.description?,
                name: value.name?,
                product_ids: value.product_ids?,
                scores: value.scores?,
            })
        }
    }
    impl From<super::ProductShort> for ProductShort {
        fn from(value: super::ProductShort) -> Self {
            Self {
                badges: Ok(value.badges),
                description: Ok(value.description),
                name: Ok(value.name),
                product_ids: Ok(value.product_ids),
                scores: Ok(value.scores),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Score {
        score: Result<i64, String>,
        scorer_name: Result<super::ScorerName, String>,
    }
    impl Default for Score {
        fn default() -> Self {
            Self {
                score: Err("no value supplied for score".to_string()),
                scorer_name: Err("no value supplied for scorer_name".to_string()),
            }
        }
    }
    impl Score {
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn scorer_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ScorerName>,
            T::Error: std::fmt::Display,
        {
            self.scorer_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scorer_name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Score> for super::Score {
        type Error = super::error::ConversionError;
        fn try_from(value: Score) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
                scorer_name: value.scorer_name?,
            })
        }
    }
    impl From<super::Score> for Score {
        fn from(value: super::Score) -> Self {
            Self {
                score: Ok(value.score),
                scorer_name: Ok(value.scorer_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShortText {
        source: Result<super::DataSource, String>,
        text: Result<super::ShortString, String>,
    }
    impl Default for ShortText {
        fn default() -> Self {
            Self {
                source: Err("no value supplied for source".to_string()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl ShortText {
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DataSource>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ShortText> for super::ShortText {
        type Error = super::error::ConversionError;
        fn try_from(value: ShortText) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                text: value.text?,
            })
        }
    }
    impl From<super::ShortText> for ShortText {
        fn from(value: super::ShortText) -> Self {
            Self {
                source: Ok(value.source),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SustainityMedallion {
        score: Result<super::SustainityScore, String>,
    }
    impl Default for SustainityMedallion {
        fn default() -> Self {
            Self {
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl SustainityMedallion {
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SustainityScore>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SustainityMedallion> for super::SustainityMedallion {
        type Error = super::error::ConversionError;
        fn try_from(value: SustainityMedallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
            })
        }
    }
    impl From<super::SustainityMedallion> for SustainityMedallion {
        fn from(value: super::SustainityMedallion) -> Self {
            Self {
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SustainityScore {
        total: Result<f64, String>,
        tree: Result<Vec<super::SustainityScoreBranch>, String>,
    }
    impl Default for SustainityScore {
        fn default() -> Self {
            Self {
                total: Err("no value supplied for total".to_string()),
                tree: Err("no value supplied for tree".to_string()),
            }
        }
    }
    impl SustainityScore {
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {}", e));
            self
        }
        pub fn tree<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::SustainityScoreBranch>>,
            T::Error: std::fmt::Display,
        {
            self.tree = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tree: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SustainityScore> for super::SustainityScore {
        type Error = super::error::ConversionError;
        fn try_from(value: SustainityScore) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                total: value.total?,
                tree: value.tree?,
            })
        }
    }
    impl From<super::SustainityScore> for SustainityScore {
        fn from(value: super::SustainityScore) -> Self {
            Self {
                total: Ok(value.total),
                tree: Ok(value.tree),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SustainityScoreBranch {
        branches: Result<Vec<super::SustainityScoreBranch>, String>,
        category: Result<super::SustainityScoreCategory, String>,
        score: Result<f64, String>,
        weight: Result<i64, String>,
    }
    impl Default for SustainityScoreBranch {
        fn default() -> Self {
            Self {
                branches: Err("no value supplied for branches".to_string()),
                category: Err("no value supplied for category".to_string()),
                score: Err("no value supplied for score".to_string()),
                weight: Err("no value supplied for weight".to_string()),
            }
        }
    }
    impl SustainityScoreBranch {
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::SustainityScoreBranch>>,
            T::Error: std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::SustainityScoreCategory>,
            T::Error: std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<f64>,
            T::Error: std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<i64>,
            T::Error: std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<SustainityScoreBranch> for super::SustainityScoreBranch {
        type Error = super::error::ConversionError;
        fn try_from(value: SustainityScoreBranch) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                branches: value.branches?,
                category: value.category?,
                score: value.score?,
                weight: value.weight?,
            })
        }
    }
    impl From<super::SustainityScoreBranch> for SustainityScoreBranch {
        fn from(value: super::SustainityScoreBranch) -> Self {
            Self {
                branches: Ok(value.branches),
                category: Ok(value.category),
                score: Ok(value.score),
                weight: Ok(value.weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TcoMedallion {
        brand_name: Result<super::ShortString, String>,
    }
    impl Default for TcoMedallion {
        fn default() -> Self {
            Self {
                brand_name: Err("no value supplied for brand_name".to_string()),
            }
        }
    }
    impl TcoMedallion {
        pub fn brand_name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.brand_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for brand_name: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TcoMedallion> for super::TcoMedallion {
        type Error = super::error::ConversionError;
        fn try_from(value: TcoMedallion) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                brand_name: value.brand_name?,
            })
        }
    }
    impl From<super::TcoMedallion> for TcoMedallion {
        fn from(value: super::TcoMedallion) -> Self {
            Self {
                brand_name: Ok(value.brand_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextSearchLinkHack {
        id: Result<super::Id, String>,
        organisation_id_variant: Result<Option<super::OrganisationIdVariant>, String>,
        product_id_variant: Result<Option<super::ProductIdVariant>, String>,
    }
    impl Default for TextSearchLinkHack {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                organisation_id_variant: Ok(Default::default()),
                product_id_variant: Ok(Default::default()),
            }
        }
    }
    impl TextSearchLinkHack {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Id>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn organisation_id_variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::OrganisationIdVariant>>,
            T::Error: std::fmt::Display,
        {
            self.organisation_id_variant = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for organisation_id_variant: {}",
                    e
                )
            });
            self
        }
        pub fn product_id_variant<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ProductIdVariant>>,
            T::Error: std::fmt::Display,
        {
            self.product_id_variant = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for product_id_variant: {}",
                    e
                )
            });
            self
        }
    }
    impl std::convert::TryFrom<TextSearchLinkHack> for super::TextSearchLinkHack {
        type Error = super::error::ConversionError;
        fn try_from(value: TextSearchLinkHack) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                organisation_id_variant: value.organisation_id_variant?,
                product_id_variant: value.product_id_variant?,
            })
        }
    }
    impl From<super::TextSearchLinkHack> for TextSearchLinkHack {
        fn from(value: super::TextSearchLinkHack) -> Self {
            Self {
                id: Ok(value.id),
                organisation_id_variant: Ok(value.organisation_id_variant),
                product_id_variant: Ok(value.product_id_variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextSearchResult {
        label: Result<super::ShortString, String>,
        link: Result<super::TextSearchLinkHack, String>,
    }
    impl Default for TextSearchResult {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                link: Err("no value supplied for link".to_string()),
            }
        }
    }
    impl TextSearchResult {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ShortString>,
            T::Error: std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TextSearchLinkHack>,
            T::Error: std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextSearchResult> for super::TextSearchResult {
        type Error = super::error::ConversionError;
        fn try_from(value: TextSearchResult) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                link: value.link?,
            })
        }
    }
    impl From<super::TextSearchResult> for TextSearchResult {
        fn from(value: super::TextSearchResult) -> Self {
            Self {
                label: Ok(value.label),
                link: Ok(value.link),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextSearchResults {
        results: Result<Vec<super::TextSearchResult>, String>,
    }
    impl Default for TextSearchResults {
        fn default() -> Self {
            Self {
                results: Err("no value supplied for results".to_string()),
            }
        }
    }
    impl TextSearchResults {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::TextSearchResult>>,
            T::Error: std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TextSearchResults> for super::TextSearchResults {
        type Error = super::error::ConversionError;
        fn try_from(value: TextSearchResults) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                results: value.results?,
            })
        }
    }
    impl From<super::TextSearchResults> for TextSearchResults {
        fn from(value: super::TextSearchResults) -> Self {
            Self {
                results: Ok(value.results),
            }
        }
    }
}
