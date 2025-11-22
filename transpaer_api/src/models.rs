#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "`Accuracy`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Accuracy(pub f64);
impl ::std::ops::Deref for Accuracy {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Accuracy> for f64 {
    fn from(value: Accuracy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Accuracy> for Accuracy {
    fn from(value: &Accuracy) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for Accuracy {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Accuracy {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Accuracy {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Accuracy {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Accuracy {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Accuracy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct BCorpMedallion {
    pub id: Id,
    pub report_url: LongString,
}
impl ::std::convert::From<&BCorpMedallion> for BCorpMedallion {
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BadgeName {
    #[serde(rename = "bcorp")]
    Bcorp,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "tco")]
    Tco,
}
impl ::std::convert::From<&Self> for BadgeName {
    fn from(value: &BadgeName) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BadgeName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bcorp => f.write_str("bcorp"),
            Self::Eu => f.write_str("eu"),
            Self::Tco => f.write_str("tco"),
        }
    }
}
impl ::std::str::FromStr for BadgeName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bcorp" => Ok(Self::Bcorp),
            "eu" => Ok(Self::Eu),
            "tco" => Ok(Self::Tco),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BadgeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "ID of a category."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ID of a category.\","]
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct Category(pub ::std::string::String);
impl ::std::ops::Deref for Category {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Category> for ::std::string::String {
    fn from(value: Category) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Category> for Category {
    fn from(value: &Category) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for Category {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Category {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Category {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[doc = "    \"category_id\","]
#[doc = "    \"category_label\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"alternatives\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/productShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"category_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"category_label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CategoryAlternatives {
    pub alternatives: ::std::vec::Vec<ProductShort>,
    pub category_id: ::std::string::String,
    pub category_label: ::std::string::String,
}
impl ::std::convert::From<&CategoryAlternatives> for CategoryAlternatives {
    fn from(value: &CategoryAlternatives) -> Self {
        value.clone()
    }
}
impl CategoryAlternatives {
    pub fn builder() -> builder::CategoryAlternatives {
        Default::default()
    }
}
#[doc = "Full category data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Full category data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"label\","]
#[doc = "    \"products\","]
#[doc = "    \"status\","]
#[doc = "    \"subcategories\","]
#[doc = "    \"supercategories\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"products\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/productShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$ref\": \"#/$defs/categoryStatus\""]
#[doc = "    },"]
#[doc = "    \"subcategories\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/categoryShort\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"supercategories\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/categoryShort\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CategoryFull {
    pub label: ::std::string::String,
    pub products: ::std::vec::Vec<ProductShort>,
    pub status: CategoryStatus,
    pub subcategories: ::std::vec::Vec<CategoryShort>,
    pub supercategories: ::std::vec::Vec<CategoryShort>,
}
impl ::std::convert::From<&CategoryFull> for CategoryFull {
    fn from(value: &CategoryFull) -> Self {
        value.clone()
    }
}
impl CategoryFull {
    pub fn builder() -> builder::CategoryFull {
        Default::default()
    }
}
#[doc = "Short category data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short category data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"label\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct CategoryShort {
    pub id: ::std::string::String,
    pub label: ::std::string::String,
}
impl ::std::convert::From<&CategoryShort> for CategoryShort {
    fn from(value: &CategoryShort) -> Self {
        value.clone()
    }
}
impl CategoryShort {
    pub fn builder() -> builder::CategoryShort {
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
#[doc = "    \"exploratory\","]
#[doc = "    \"incomplete\","]
#[doc = "    \"satisfactory\","]
#[doc = "    \"complete\","]
#[doc = "    \"broad\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CategoryStatus {
    #[serde(rename = "exploratory")]
    Exploratory,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "satisfactory")]
    Satisfactory,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "broad")]
    Broad,
}
impl ::std::convert::From<&Self> for CategoryStatus {
    fn from(value: &CategoryStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CategoryStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Exploratory => f.write_str("exploratory"),
            Self::Incomplete => f.write_str("incomplete"),
            Self::Satisfactory => f.write_str("satisfactory"),
            Self::Complete => f.write_str("complete"),
            Self::Broad => f.write_str("broad"),
        }
    }
}
impl ::std::str::FromStr for CategoryStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "exploratory" => Ok(Self::Exploratory),
            "incomplete" => Ok(Self::Incomplete),
            "satisfactory" => Ok(Self::Satisfactory),
            "complete" => Ok(Self::Complete),
            "broad" => Ok(Self::Broad),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CategoryStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CategoryStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CategoryStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "    \"transpaer\","]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DataSource {
    #[serde(rename = "transpaer")]
    Transpaer,
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
impl ::std::convert::From<&Self> for DataSource {
    fn from(value: &DataSource) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DataSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Transpaer => f.write_str("transpaer"),
            Self::Wiki => f.write_str("wiki"),
            Self::Off => f.write_str("off"),
            Self::Eu => f.write_str("eu"),
            Self::BCorp => f.write_str("b_corp"),
            Self::Fti => f.write_str("fti"),
            Self::Tco => f.write_str("tco"),
            Self::Other => f.write_str("other"),
        }
    }
}
impl ::std::str::FromStr for DataSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "transpaer" => Ok(Self::Transpaer),
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
impl ::std::convert::TryFrom<&str> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DataSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct EuEcolabelMedallion {
    #[serde(
        rename = "matchAccuracy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub match_accuracy: ::std::option::Option<Accuracy>,
}
impl ::std::convert::From<&EuEcolabelMedallion> for EuEcolabelMedallion {
    fn from(value: &EuEcolabelMedallion) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for EuEcolabelMedallion {
    fn default() -> Self {
        Self {
            match_accuracy: Default::default(),
        }
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct FtiMedallion {
    pub score: i64,
}
impl ::std::convert::From<&FtiMedallion> for FtiMedallion {
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
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Id(::std::string::String);
impl ::std::ops::Deref for Id {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Id> for ::std::string::String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Id> for Id {
    fn from(value: &Id) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Id {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Image {
    pub image: ::std::string::String,
    pub source: DataSource,
}
impl ::std::convert::From<&Image> for Image {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LibraryContents {
    pub items: ::std::vec::Vec<LibraryItemShort>,
}
impl ::std::convert::From<&LibraryContents> for LibraryContents {
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
#[doc = "    \"links\","]
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
#[doc = "    \"links\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/referenceLink\""]
#[doc = "      }"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LibraryItemFull {
    pub article: LongString,
    pub id: LibraryTopic,
    pub links: ::std::vec::Vec<ReferenceLink>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub presentation: ::std::option::Option<Presentation>,
    pub summary: ShortString,
    pub title: ShortString,
}
impl ::std::convert::From<&LibraryItemFull> for LibraryItemFull {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LibraryItemShort {
    pub id: LibraryTopic,
    pub summary: ShortString,
    pub title: ShortString,
}
impl ::std::convert::From<&LibraryItemShort> for LibraryItemShort {
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
#[doc = "  \"type\": \"string\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[serde(transparent)]
pub struct LibraryTopic(pub ::std::string::String);
impl ::std::ops::Deref for LibraryTopic {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LibraryTopic> for ::std::string::String {
    fn from(value: LibraryTopic) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LibraryTopic> for LibraryTopic {
    fn from(value: &LibraryTopic) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::string::String> for LibraryTopic {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for LibraryTopic {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for LibraryTopic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct LongString(::std::string::String);
impl ::std::ops::Deref for LongString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<LongString> for ::std::string::String {
    fn from(value: LongString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LongString> for LongString {
    fn from(value: &LongString) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for LongString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1048576usize {
            return Err("longer than 1048576 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LongString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for LongString {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct LongText {
    pub source: DataSource,
    pub text: LongString,
}
impl ::std::convert::From<&LongText> for LongText {
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
#[doc = "    \"tco\": {"]
#[doc = "      \"$ref\": \"#/$defs/tcoMedallion\""]
#[doc = "    },"]
#[doc = "    \"transpaer\": {"]
#[doc = "      \"$ref\": \"#/$defs/transpaerMedallion\""]
#[doc = "    },"]
#[doc = "    \"variant\": {"]
#[doc = "      \"$ref\": \"#/$defs/medallionVariant\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Medallion {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bcorp: ::std::option::Option<BCorpMedallion>,
    #[serde(
        rename = "euEcolabel",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub eu_ecolabel: ::std::option::Option<EuEcolabelMedallion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fti: ::std::option::Option<FtiMedallion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tco: ::std::option::Option<TcoMedallion>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transpaer: ::std::option::Option<TranspaerMedallion>,
    pub variant: MedallionVariant,
}
impl ::std::convert::From<&Medallion> for Medallion {
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
#[doc = "    \"transpaer\","]
#[doc = "    \"tco\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MedallionVariant {
    #[serde(rename = "bCorp")]
    BCorp,
    #[serde(rename = "euEcolabel")]
    EuEcolabel,
    #[serde(rename = "fti")]
    Fti,
    #[serde(rename = "transpaer")]
    Transpaer,
    #[serde(rename = "tco")]
    Tco,
}
impl ::std::convert::From<&Self> for MedallionVariant {
    fn from(value: &MedallionVariant) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MedallionVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BCorp => f.write_str("bCorp"),
            Self::EuEcolabel => f.write_str("euEcolabel"),
            Self::Fti => f.write_str("fti"),
            Self::Transpaer => f.write_str("transpaer"),
            Self::Tco => f.write_str("tco"),
        }
    }
}
impl ::std::str::FromStr for MedallionVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bCorp" => Ok(Self::BCorp),
            "euEcolabel" => Ok(Self::EuEcolabel),
            "fti" => Ok(Self::Fti),
            "transpaer" => Ok(Self::Transpaer),
            "tco" => Ok(Self::Tco),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MedallionVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Medium`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"mentions\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"icon\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mentions\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/mention\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Medium {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub icon: ::std::option::Option<::std::string::String>,
    pub mentions: ::std::vec::Vec<Mention>,
}
impl ::std::convert::From<&Medium> for Medium {
    fn from(value: &Medium) -> Self {
        value.clone()
    }
}
impl Medium {
    pub fn builder() -> builder::Medium {
        Default::default()
    }
}
#[doc = "`Mention`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"link\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"link\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Mention {
    pub link: ::std::string::String,
    pub title: ::std::string::String,
}
impl ::std::convert::From<&Mention> for Mention {
    fn from(value: &Mention) -> Self {
        value.clone()
    }
}
impl Mention {
    pub fn builder() -> builder::Mention {
        Default::default()
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
#[doc = "    \"media\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/medium\""]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganisationFull {
    pub descriptions: ::std::vec::Vec<LongText>,
    pub images: ::std::vec::Vec<Image>,
    pub medallions: ::std::vec::Vec<Medallion>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub media: ::std::vec::Vec<Medium>,
    pub names: ::std::vec::Vec<ShortText>,
    #[serde(rename = "organisationIds")]
    pub organisation_ids: OrganisationIds,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub origins: ::std::vec::Vec<RegionCode>,
    pub products: ::std::vec::Vec<ProductShort>,
    pub websites: ::std::vec::Vec<ShortString>,
}
impl ::std::convert::From<&OrganisationFull> for OrganisationFull {
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum OrganisationIdVariant {
    #[serde(rename = "wiki")]
    Wiki,
    #[serde(rename = "vat")]
    Vat,
    #[serde(rename = "www")]
    Www,
}
impl ::std::convert::From<&Self> for OrganisationIdVariant {
    fn from(value: &OrganisationIdVariant) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for OrganisationIdVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wiki => f.write_str("wiki"),
            Self::Vat => f.write_str("vat"),
            Self::Www => f.write_str("www"),
        }
    }
}
impl ::std::str::FromStr for OrganisationIdVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "wiki" => Ok(Self::Wiki),
            "vat" => Ok(Self::Vat),
            "www" => Ok(Self::Www),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrganisationIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganisationIds {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub domains: ::std::vec::Vec<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub vat: ::std::vec::Vec<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wiki: ::std::vec::Vec<Id>,
}
impl ::std::convert::From<&OrganisationIds> for OrganisationIds {
    fn from(value: &OrganisationIds) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OrganisationIds {
    fn default() -> Self {
        Self {
            domains: Default::default(),
            vat: Default::default(),
            wiki: Default::default(),
        }
    }
}
impl OrganisationIds {
    pub fn builder() -> builder::OrganisationIds {
        Default::default()
    }
}
#[doc = "`OrganisationLink`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganisationLink {
    pub id: Id,
    pub organisation_id_variant: OrganisationIdVariant,
}
impl ::std::convert::From<&OrganisationLink> for OrganisationLink {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct OrganisationShort {
    pub badges: ::std::vec::Vec<BadgeName>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<LongText>,
    pub name: ShortString,
    #[serde(rename = "organisationIds")]
    pub organisation_ids: OrganisationIds,
    pub scores: ::std::vec::Vec<Score>,
}
impl ::std::convert::From<&OrganisationShort> for OrganisationShort {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Presentation {
    pub data: ::std::vec::Vec<PresentationEntry>,
}
impl ::std::convert::From<&Presentation> for Presentation {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct PresentationEntry {
    pub name: ShortString,
    pub score: i64,
    pub wiki_id: Id,
}
impl ::std::convert::From<&PresentationEntry> for PresentationEntry {
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
#[doc = "    \"media\","]
#[doc = "    \"names\","]
#[doc = "    \"productIds\","]
#[doc = "    \"shopping\""]
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
#[doc = "    \"media\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/medium\""]
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
#[doc = "    },"]
#[doc = "    \"shopping\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/shoppingEntry\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProductFull {
    pub alternatives: ::std::vec::Vec<CategoryAlternatives>,
    pub descriptions: ::std::vec::Vec<LongText>,
    pub images: ::std::vec::Vec<Image>,
    pub manufacturers: ::std::vec::Vec<OrganisationShort>,
    pub medallions: ::std::vec::Vec<Medallion>,
    pub media: ::std::vec::Vec<Medium>,
    pub names: ::std::vec::Vec<ShortText>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub origins: ::std::vec::Vec<RegionCode>,
    #[serde(rename = "productIds")]
    pub product_ids: ProductIds,
    pub shopping: ::std::vec::Vec<ShoppingEntry>,
}
impl ::std::convert::From<&ProductFull> for ProductFull {
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProductIdVariant {
    #[serde(rename = "ean")]
    Ean,
    #[serde(rename = "gtin")]
    Gtin,
    #[serde(rename = "wiki")]
    Wiki,
}
impl ::std::convert::From<&Self> for ProductIdVariant {
    fn from(value: &ProductIdVariant) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ProductIdVariant {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ean => f.write_str("ean"),
            Self::Gtin => f.write_str("gtin"),
            Self::Wiki => f.write_str("wiki"),
        }
    }
}
impl ::std::str::FromStr for ProductIdVariant {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ean" => Ok(Self::Ean),
            "gtin" => Ok(Self::Gtin),
            "wiki" => Ok(Self::Wiki),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProductIdVariant {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProductIds {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub eans: ::std::vec::Vec<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub gtins: ::std::vec::Vec<Id>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wiki: ::std::vec::Vec<Id>,
}
impl ::std::convert::From<&ProductIds> for ProductIds {
    fn from(value: &ProductIds) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ProductIds {
    fn default() -> Self {
        Self {
            eans: Default::default(),
            gtins: Default::default(),
            wiki: Default::default(),
        }
    }
}
impl ProductIds {
    pub fn builder() -> builder::ProductIds {
        Default::default()
    }
}
#[doc = "`ProductLink`"]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProductLink {
    pub id: Id,
    pub product_id_variant: ProductIdVariant,
}
impl ::std::convert::From<&ProductLink> for ProductLink {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ProductShort {
    pub badges: ::std::vec::Vec<BadgeName>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<LongText>,
    pub name: ShortString,
    #[serde(rename = "productIds")]
    pub product_ids: ProductIds,
    pub scores: ::std::vec::Vec<Score>,
}
impl ::std::convert::From<&ProductShort> for ProductShort {
    fn from(value: &ProductShort) -> Self {
        value.clone()
    }
}
impl ProductShort {
    pub fn builder() -> builder::ProductShort {
        Default::default()
    }
}
#[doc = "A reference link"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A reference link\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"link\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"link\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ReferenceLink {
    pub link: ::std::string::String,
    pub title: ::std::string::String,
}
impl ::std::convert::From<&ReferenceLink> for ReferenceLink {
    fn from(value: &ReferenceLink) -> Self {
        value.clone()
    }
}
impl ReferenceLink {
    pub fn builder() -> builder::ReferenceLink {
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
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RegionCode(::std::string::String);
impl ::std::ops::Deref for RegionCode {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RegionCode> for ::std::string::String {
    fn from(value: RegionCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RegionCode> for RegionCode {
    fn from(value: &RegionCode) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for RegionCode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 3usize {
            return Err("longer than 3 characters".into());
        }
        if value.chars().count() < 3usize {
            return Err("shorter than 3 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RegionCode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RegionCode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct Score {
    pub score: i64,
    #[serde(rename = "scorerName")]
    pub scorer_name: ScorerName,
}
impl ::std::convert::From<&Score> for Score {
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ScorerName {
    #[serde(rename = "fti")]
    Fti,
}
impl ::std::convert::From<&Self> for ScorerName {
    fn from(value: &ScorerName) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ScorerName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fti => f.write_str("fti"),
        }
    }
}
impl ::std::str::FromStr for ScorerName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fti" => Ok(Self::Fti),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScorerName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Describes a link to a shop"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Describes a link to a shop\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"link\","]
#[doc = "    \"shop\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$ref\": \"#/$defs/shortString\""]
#[doc = "    },"]
#[doc = "    \"link\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"shop\": {"]
#[doc = "      \"$ref\": \"#/$defs/verifiedShop\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ShoppingEntry {
    pub description: ShortString,
    pub link: ::std::string::String,
    pub shop: VerifiedShop,
}
impl ::std::convert::From<&ShoppingEntry> for ShoppingEntry {
    fn from(value: &ShoppingEntry) -> Self {
        value.clone()
    }
}
impl ShoppingEntry {
    pub fn builder() -> builder::ShoppingEntry {
        Default::default()
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
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ShortString(::std::string::String);
impl ::std::ops::Deref for ShortString {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ShortString> for ::std::string::String {
    fn from(value: ShortString) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ShortString> for ShortString {
    fn from(value: &ShortString) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ShortString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1024usize {
            return Err("longer than 1024 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ShortString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ShortString {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct ShortText {
    pub source: DataSource,
    pub text: ShortString,
}
impl ::std::convert::From<&ShortText> for ShortText {
    fn from(value: &ShortText) -> Self {
        value.clone()
    }
}
impl ShortText {
    pub fn builder() -> builder::ShortText {
        Default::default()
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TcoMedallion {
    #[serde(rename = "brandName")]
    pub brand_name: ShortString,
}
impl ::std::convert::From<&TcoMedallion> for TcoMedallion {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TextSearchLink {
    OrganisationLink(OrganisationLink),
    ProductLink(ProductLink),
}
impl ::std::convert::From<&Self> for TextSearchLink {
    fn from(value: &TextSearchLink) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<OrganisationLink> for TextSearchLink {
    fn from(value: OrganisationLink) -> Self {
        Self::OrganisationLink(value)
    }
}
impl ::std::convert::From<ProductLink> for TextSearchLink {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TextSearchLinkHack {
    pub id: Id,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub organisation_id_variant: ::std::option::Option<OrganisationIdVariant>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub product_id_variant: ::std::option::Option<ProductIdVariant>,
}
impl ::std::convert::From<&TextSearchLinkHack> for TextSearchLinkHack {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TextSearchResult {
    pub label: ShortString,
    pub link: TextSearchLinkHack,
}
impl ::std::convert::From<&TextSearchResult> for TextSearchResult {
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TextSearchResults {
    pub results: ::std::vec::Vec<TextSearchResult>,
}
impl ::std::convert::From<&TextSearchResults> for TextSearchResults {
    fn from(value: &TextSearchResults) -> Self {
        value.clone()
    }
}
impl TextSearchResults {
    pub fn builder() -> builder::TextSearchResults {
        Default::default()
    }
}
#[doc = "Details of Transpaer evaluation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Details of Transpaer evaluation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"score\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"score\": {"]
#[doc = "      \"$ref\": \"#/$defs/transpaerScore\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranspaerMedallion {
    pub score: TranspaerScore,
}
impl ::std::convert::From<&TranspaerMedallion> for TranspaerMedallion {
    fn from(value: &TranspaerMedallion) -> Self {
        value.clone()
    }
}
impl TranspaerMedallion {
    pub fn builder() -> builder::TranspaerMedallion {
        Default::default()
    }
}
#[doc = "Explanation of calculation of the Transpaer score."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Explanation of calculation of the Transpaer score.\","]
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
#[doc = "        \"$ref\": \"#/$defs/transpaerScoreBranch\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranspaerScore {
    pub total: f64,
    pub tree: ::std::vec::Vec<TranspaerScoreBranch>,
}
impl ::std::convert::From<&TranspaerScore> for TranspaerScore {
    fn from(value: &TranspaerScore) -> Self {
        value.clone()
    }
}
impl TranspaerScore {
    pub fn builder() -> builder::TranspaerScore {
        Default::default()
    }
}
#[doc = "Part of explanation of calculation of the Transpaer score."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Part of explanation of calculation of the Transpaer score.\","]
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
#[doc = "        \"$ref\": \"#/$defs/transpaerScoreBranch\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"category\": {"]
#[doc = "      \"$ref\": \"#/$defs/transpaerScoreCategory\""]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
pub struct TranspaerScoreBranch {
    pub branches: ::std::vec::Vec<TranspaerScoreBranch>,
    pub category: TranspaerScoreCategory,
    pub score: f64,
    pub weight: i64,
}
impl ::std::convert::From<&TranspaerScoreBranch> for TranspaerScoreBranch {
    fn from(value: &TranspaerScoreBranch) -> Self {
        value.clone()
    }
}
impl TranspaerScoreBranch {
    pub fn builder() -> builder::TranspaerScoreBranch {
        Default::default()
    }
}
#[doc = "Categories in the transpaer score"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Categories in the transpaer score\","]
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
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TranspaerScoreCategory {
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
impl ::std::convert::From<&Self> for TranspaerScoreCategory {
    fn from(value: &TranspaerScoreCategory) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TranspaerScoreCategory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DataAvailability => f.write_str("dataAvailability"),
            Self::ProducerKnown => f.write_str("producerKnown"),
            Self::ProductionPlaceKnown => f.write_str("productionPlaceKnown"),
            Self::IdKnown => f.write_str("idKnown"),
            Self::CategoryAssigned => f.write_str("categoryAssigned"),
            Self::Category => f.write_str("category"),
            Self::WarrantyLength => f.write_str("warrantyLength"),
            Self::NumCerts => f.write_str("numCerts"),
            Self::AtLeastOneCert => f.write_str("atLeastOneCert"),
            Self::AtLeastTwoCerts => f.write_str("atLeastTwoCerts"),
        }
    }
}
impl ::std::str::FromStr for TranspaerScoreCategory {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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
impl ::std::convert::TryFrom<&str> for TranspaerScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TranspaerScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TranspaerScoreCategory {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Enumerates verified shops."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Enumerates verified shops.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"fairphone\","]
#[doc = "    \"amazon\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VerifiedShop {
    #[serde(rename = "fairphone")]
    Fairphone,
    #[serde(rename = "amazon")]
    Amazon,
}
impl ::std::convert::From<&Self> for VerifiedShop {
    fn from(value: &VerifiedShop) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for VerifiedShop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fairphone => f.write_str("fairphone"),
            Self::Amazon => f.write_str("amazon"),
        }
    }
}
impl ::std::str::FromStr for VerifiedShop {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "fairphone" => Ok(Self::Fairphone),
            "amazon" => Ok(Self::Amazon),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VerifiedShop {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerifiedShop {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerifiedShop {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BCorpMedallion {
        id: ::std::result::Result<super::Id, ::std::string::String>,
        report_url: ::std::result::Result<super::LongString, ::std::string::String>,
    }
    impl ::std::default::Default for BCorpMedallion {
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
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn report_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LongString>,
            T::Error: ::std::fmt::Display,
        {
            self.report_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for report_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BCorpMedallion> for super::BCorpMedallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BCorpMedallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                report_url: value.report_url?,
            })
        }
    }
    impl ::std::convert::From<super::BCorpMedallion> for BCorpMedallion {
        fn from(value: super::BCorpMedallion) -> Self {
            Self {
                id: Ok(value.id),
                report_url: Ok(value.report_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CategoryAlternatives {
        alternatives:
            ::std::result::Result<::std::vec::Vec<super::ProductShort>, ::std::string::String>,
        category_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        category_label: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CategoryAlternatives {
        fn default() -> Self {
            Self {
                alternatives: Err("no value supplied for alternatives".to_string()),
                category_id: Err("no value supplied for category_id".to_string()),
                category_label: Err("no value supplied for category_label".to_string()),
            }
        }
    }
    impl CategoryAlternatives {
        pub fn alternatives<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProductShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.alternatives = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alternatives: {}", e));
            self
        }
        pub fn category_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.category_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category_id: {}", e));
            self
        }
        pub fn category_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.category_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category_label: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CategoryAlternatives> for super::CategoryAlternatives {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CategoryAlternatives,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternatives: value.alternatives?,
                category_id: value.category_id?,
                category_label: value.category_label?,
            })
        }
    }
    impl ::std::convert::From<super::CategoryAlternatives> for CategoryAlternatives {
        fn from(value: super::CategoryAlternatives) -> Self {
            Self {
                alternatives: Ok(value.alternatives),
                category_id: Ok(value.category_id),
                category_label: Ok(value.category_label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CategoryFull {
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        products:
            ::std::result::Result<::std::vec::Vec<super::ProductShort>, ::std::string::String>,
        status: ::std::result::Result<super::CategoryStatus, ::std::string::String>,
        subcategories:
            ::std::result::Result<::std::vec::Vec<super::CategoryShort>, ::std::string::String>,
        supercategories:
            ::std::result::Result<::std::vec::Vec<super::CategoryShort>, ::std::string::String>,
    }
    impl ::std::default::Default for CategoryFull {
        fn default() -> Self {
            Self {
                label: Err("no value supplied for label".to_string()),
                products: Err("no value supplied for products".to_string()),
                status: Err("no value supplied for status".to_string()),
                subcategories: Err("no value supplied for subcategories".to_string()),
                supercategories: Err("no value supplied for supercategories".to_string()),
            }
        }
    }
    impl CategoryFull {
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProductShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CategoryStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn subcategories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CategoryShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.subcategories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subcategories: {}", e));
            self
        }
        pub fn supercategories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CategoryShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.supercategories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for supercategories: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CategoryFull> for super::CategoryFull {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CategoryFull,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                products: value.products?,
                status: value.status?,
                subcategories: value.subcategories?,
                supercategories: value.supercategories?,
            })
        }
    }
    impl ::std::convert::From<super::CategoryFull> for CategoryFull {
        fn from(value: super::CategoryFull) -> Self {
            Self {
                label: Ok(value.label),
                products: Ok(value.products),
                status: Ok(value.status),
                subcategories: Ok(value.subcategories),
                supercategories: Ok(value.supercategories),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CategoryShort {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CategoryShort {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                label: Err("no value supplied for label".to_string()),
            }
        }
    }
    impl CategoryShort {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CategoryShort> for super::CategoryShort {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CategoryShort,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::CategoryShort> for CategoryShort {
        fn from(value: super::CategoryShort) -> Self {
            Self {
                id: Ok(value.id),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EuEcolabelMedallion {
        match_accuracy:
            ::std::result::Result<::std::option::Option<super::Accuracy>, ::std::string::String>,
    }
    impl ::std::default::Default for EuEcolabelMedallion {
        fn default() -> Self {
            Self {
                match_accuracy: Ok(Default::default()),
            }
        }
    }
    impl EuEcolabelMedallion {
        pub fn match_accuracy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Accuracy>>,
            T::Error: ::std::fmt::Display,
        {
            self.match_accuracy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_accuracy: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<EuEcolabelMedallion> for super::EuEcolabelMedallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EuEcolabelMedallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                match_accuracy: value.match_accuracy?,
            })
        }
    }
    impl ::std::convert::From<super::EuEcolabelMedallion> for EuEcolabelMedallion {
        fn from(value: super::EuEcolabelMedallion) -> Self {
            Self {
                match_accuracy: Ok(value.match_accuracy),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FtiMedallion {
        score: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for FtiMedallion {
        fn default() -> Self {
            Self {
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl FtiMedallion {
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FtiMedallion> for super::FtiMedallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FtiMedallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
            })
        }
    }
    impl ::std::convert::From<super::FtiMedallion> for FtiMedallion {
        fn from(value: super::FtiMedallion) -> Self {
            Self {
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Image {
        image: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<super::DataSource, ::std::string::String>,
    }
    impl ::std::default::Default for Image {
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DataSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Image> for super::Image {
        type Error = super::error::ConversionError;
        fn try_from(value: Image) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                image: value.image?,
                source: value.source?,
            })
        }
    }
    impl ::std::convert::From<super::Image> for Image {
        fn from(value: super::Image) -> Self {
            Self {
                image: Ok(value.image),
                source: Ok(value.source),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryContents {
        items:
            ::std::result::Result<::std::vec::Vec<super::LibraryItemShort>, ::std::string::String>,
    }
    impl ::std::default::Default for LibraryContents {
        fn default() -> Self {
            Self {
                items: Err("no value supplied for items".to_string()),
            }
        }
    }
    impl LibraryContents {
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LibraryItemShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for items: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LibraryContents> for super::LibraryContents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LibraryContents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                items: value.items?,
            })
        }
    }
    impl ::std::convert::From<super::LibraryContents> for LibraryContents {
        fn from(value: super::LibraryContents) -> Self {
            Self {
                items: Ok(value.items),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryItemFull {
        article: ::std::result::Result<super::LongString, ::std::string::String>,
        id: ::std::result::Result<super::LibraryTopic, ::std::string::String>,
        links: ::std::result::Result<::std::vec::Vec<super::ReferenceLink>, ::std::string::String>,
        presentation: ::std::result::Result<
            ::std::option::Option<super::Presentation>,
            ::std::string::String,
        >,
        summary: ::std::result::Result<super::ShortString, ::std::string::String>,
        title: ::std::result::Result<super::ShortString, ::std::string::String>,
    }
    impl ::std::default::Default for LibraryItemFull {
        fn default() -> Self {
            Self {
                article: Err("no value supplied for article".to_string()),
                id: Err("no value supplied for id".to_string()),
                links: Err("no value supplied for links".to_string()),
                presentation: Ok(Default::default()),
                summary: Err("no value supplied for summary".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl LibraryItemFull {
        pub fn article<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LongString>,
            T::Error: ::std::fmt::Display,
        {
            self.article = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for article: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LibraryTopic>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn links<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ReferenceLink>>,
            T::Error: ::std::fmt::Display,
        {
            self.links = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for links: {}", e));
            self
        }
        pub fn presentation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Presentation>>,
            T::Error: ::std::fmt::Display,
        {
            self.presentation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for presentation: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LibraryItemFull> for super::LibraryItemFull {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LibraryItemFull,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                article: value.article?,
                id: value.id?,
                links: value.links?,
                presentation: value.presentation?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::LibraryItemFull> for LibraryItemFull {
        fn from(value: super::LibraryItemFull) -> Self {
            Self {
                article: Ok(value.article),
                id: Ok(value.id),
                links: Ok(value.links),
                presentation: Ok(value.presentation),
                summary: Ok(value.summary),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LibraryItemShort {
        id: ::std::result::Result<super::LibraryTopic, ::std::string::String>,
        summary: ::std::result::Result<super::ShortString, ::std::string::String>,
        title: ::std::result::Result<super::ShortString, ::std::string::String>,
    }
    impl ::std::default::Default for LibraryItemShort {
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
            T: ::std::convert::TryInto<super::LibraryTopic>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn summary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.summary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for summary: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LibraryItemShort> for super::LibraryItemShort {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LibraryItemShort,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                summary: value.summary?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::LibraryItemShort> for LibraryItemShort {
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
        source: ::std::result::Result<super::DataSource, ::std::string::String>,
        text: ::std::result::Result<super::LongString, ::std::string::String>,
    }
    impl ::std::default::Default for LongText {
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
            T: ::std::convert::TryInto<super::DataSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LongString>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LongText> for super::LongText {
        type Error = super::error::ConversionError;
        fn try_from(value: LongText) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::LongText> for LongText {
        fn from(value: super::LongText) -> Self {
            Self {
                source: Ok(value.source),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Medallion {
        bcorp: ::std::result::Result<
            ::std::option::Option<super::BCorpMedallion>,
            ::std::string::String,
        >,
        eu_ecolabel: ::std::result::Result<
            ::std::option::Option<super::EuEcolabelMedallion>,
            ::std::string::String,
        >,
        fti: ::std::result::Result<
            ::std::option::Option<super::FtiMedallion>,
            ::std::string::String,
        >,
        tco: ::std::result::Result<
            ::std::option::Option<super::TcoMedallion>,
            ::std::string::String,
        >,
        transpaer: ::std::result::Result<
            ::std::option::Option<super::TranspaerMedallion>,
            ::std::string::String,
        >,
        variant: ::std::result::Result<super::MedallionVariant, ::std::string::String>,
    }
    impl ::std::default::Default for Medallion {
        fn default() -> Self {
            Self {
                bcorp: Ok(Default::default()),
                eu_ecolabel: Ok(Default::default()),
                fti: Ok(Default::default()),
                tco: Ok(Default::default()),
                transpaer: Ok(Default::default()),
                variant: Err("no value supplied for variant".to_string()),
            }
        }
    }
    impl Medallion {
        pub fn bcorp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BCorpMedallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.bcorp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bcorp: {}", e));
            self
        }
        pub fn eu_ecolabel<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EuEcolabelMedallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.eu_ecolabel = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eu_ecolabel: {}", e));
            self
        }
        pub fn fti<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FtiMedallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.fti = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fti: {}", e));
            self
        }
        pub fn tco<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TcoMedallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.tco = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tco: {}", e));
            self
        }
        pub fn transpaer<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TranspaerMedallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.transpaer = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transpaer: {}", e));
            self
        }
        pub fn variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MedallionVariant>,
            T::Error: ::std::fmt::Display,
        {
            self.variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for variant: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Medallion> for super::Medallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Medallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bcorp: value.bcorp?,
                eu_ecolabel: value.eu_ecolabel?,
                fti: value.fti?,
                tco: value.tco?,
                transpaer: value.transpaer?,
                variant: value.variant?,
            })
        }
    }
    impl ::std::convert::From<super::Medallion> for Medallion {
        fn from(value: super::Medallion) -> Self {
            Self {
                bcorp: Ok(value.bcorp),
                eu_ecolabel: Ok(value.eu_ecolabel),
                fti: Ok(value.fti),
                tco: Ok(value.tco),
                transpaer: Ok(value.transpaer),
                variant: Ok(value.variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Medium {
        icon: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        mentions: ::std::result::Result<::std::vec::Vec<super::Mention>, ::std::string::String>,
    }
    impl ::std::default::Default for Medium {
        fn default() -> Self {
            Self {
                icon: Ok(Default::default()),
                mentions: Err("no value supplied for mentions".to_string()),
            }
        }
    }
    impl Medium {
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {}", e));
            self
        }
        pub fn mentions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Mention>>,
            T::Error: ::std::fmt::Display,
        {
            self.mentions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mentions: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Medium> for super::Medium {
        type Error = super::error::ConversionError;
        fn try_from(value: Medium) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                icon: value.icon?,
                mentions: value.mentions?,
            })
        }
    }
    impl ::std::convert::From<super::Medium> for Medium {
        fn from(value: super::Medium) -> Self {
            Self {
                icon: Ok(value.icon),
                mentions: Ok(value.mentions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Mention {
        link: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for Mention {
        fn default() -> Self {
            Self {
                link: Err("no value supplied for link".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl Mention {
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Mention> for super::Mention {
        type Error = super::error::ConversionError;
        fn try_from(value: Mention) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                link: value.link?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Mention> for Mention {
        fn from(value: super::Mention) -> Self {
            Self {
                link: Ok(value.link),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationFull {
        descriptions:
            ::std::result::Result<::std::vec::Vec<super::LongText>, ::std::string::String>,
        images: ::std::result::Result<::std::vec::Vec<super::Image>, ::std::string::String>,
        medallions: ::std::result::Result<::std::vec::Vec<super::Medallion>, ::std::string::String>,
        media: ::std::result::Result<::std::vec::Vec<super::Medium>, ::std::string::String>,
        names: ::std::result::Result<::std::vec::Vec<super::ShortText>, ::std::string::String>,
        organisation_ids: ::std::result::Result<super::OrganisationIds, ::std::string::String>,
        origins: ::std::result::Result<::std::vec::Vec<super::RegionCode>, ::std::string::String>,
        products:
            ::std::result::Result<::std::vec::Vec<super::ProductShort>, ::std::string::String>,
        websites: ::std::result::Result<::std::vec::Vec<super::ShortString>, ::std::string::String>,
    }
    impl ::std::default::Default for OrganisationFull {
        fn default() -> Self {
            Self {
                descriptions: Err("no value supplied for descriptions".to_string()),
                images: Err("no value supplied for images".to_string()),
                medallions: Err("no value supplied for medallions".to_string()),
                media: Ok(Default::default()),
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::LongText>>,
            T::Error: ::std::fmt::Display,
        {
            self.descriptions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for descriptions: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Image>>,
            T::Error: ::std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn medallions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Medallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.medallions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medallions: {}", e));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Medium>>,
            T::Error: ::std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ShortText>>,
            T::Error: ::std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn organisation_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganisationIds>,
            T::Error: ::std::fmt::Display,
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::RegionCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn products<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ProductShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.products = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for products: {}", e));
            self
        }
        pub fn websites<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ShortString>>,
            T::Error: ::std::fmt::Display,
        {
            self.websites = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for websites: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganisationFull> for super::OrganisationFull {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationFull,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                descriptions: value.descriptions?,
                images: value.images?,
                medallions: value.medallions?,
                media: value.media?,
                names: value.names?,
                organisation_ids: value.organisation_ids?,
                origins: value.origins?,
                products: value.products?,
                websites: value.websites?,
            })
        }
    }
    impl ::std::convert::From<super::OrganisationFull> for OrganisationFull {
        fn from(value: super::OrganisationFull) -> Self {
            Self {
                descriptions: Ok(value.descriptions),
                images: Ok(value.images),
                medallions: Ok(value.medallions),
                media: Ok(value.media),
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
        domains: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
        vat: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
        wiki: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
    }
    impl ::std::default::Default for OrganisationIds {
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.domains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for domains: {}", e));
            self
        }
        pub fn vat<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.vat = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vat: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganisationIds> for super::OrganisationIds {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationIds,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                domains: value.domains?,
                vat: value.vat?,
                wiki: value.wiki?,
            })
        }
    }
    impl ::std::convert::From<super::OrganisationIds> for OrganisationIds {
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
        id: ::std::result::Result<super::Id, ::std::string::String>,
        organisation_id_variant:
            ::std::result::Result<super::OrganisationIdVariant, ::std::string::String>,
    }
    impl ::std::default::Default for OrganisationLink {
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
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn organisation_id_variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganisationIdVariant>,
            T::Error: ::std::fmt::Display,
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
    impl ::std::convert::TryFrom<OrganisationLink> for super::OrganisationLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                organisation_id_variant: value.organisation_id_variant?,
            })
        }
    }
    impl ::std::convert::From<super::OrganisationLink> for OrganisationLink {
        fn from(value: super::OrganisationLink) -> Self {
            Self {
                id: Ok(value.id),
                organisation_id_variant: Ok(value.organisation_id_variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OrganisationShort {
        badges: ::std::result::Result<::std::vec::Vec<super::BadgeName>, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::LongText>, ::std::string::String>,
        name: ::std::result::Result<super::ShortString, ::std::string::String>,
        organisation_ids: ::std::result::Result<super::OrganisationIds, ::std::string::String>,
        scores: ::std::result::Result<::std::vec::Vec<super::Score>, ::std::string::String>,
    }
    impl ::std::default::Default for OrganisationShort {
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::BadgeName>>,
            T::Error: ::std::fmt::Display,
        {
            self.badges = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for badges: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LongText>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn organisation_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OrganisationIds>,
            T::Error: ::std::fmt::Display,
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::Score>>,
            T::Error: ::std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OrganisationShort> for super::OrganisationShort {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OrganisationShort,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                badges: value.badges?,
                description: value.description?,
                name: value.name?,
                organisation_ids: value.organisation_ids?,
                scores: value.scores?,
            })
        }
    }
    impl ::std::convert::From<super::OrganisationShort> for OrganisationShort {
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
        data:
            ::std::result::Result<::std::vec::Vec<super::PresentationEntry>, ::std::string::String>,
    }
    impl ::std::default::Default for Presentation {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl Presentation {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PresentationEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Presentation> for super::Presentation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Presentation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::Presentation> for Presentation {
        fn from(value: super::Presentation) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PresentationEntry {
        name: ::std::result::Result<super::ShortString, ::std::string::String>,
        score: ::std::result::Result<i64, ::std::string::String>,
        wiki_id: ::std::result::Result<super::Id, ::std::string::String>,
    }
    impl ::std::default::Default for PresentationEntry {
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
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn wiki_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.wiki_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PresentationEntry> for super::PresentationEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PresentationEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                score: value.score?,
                wiki_id: value.wiki_id?,
            })
        }
    }
    impl ::std::convert::From<super::PresentationEntry> for PresentationEntry {
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
        alternatives: ::std::result::Result<
            ::std::vec::Vec<super::CategoryAlternatives>,
            ::std::string::String,
        >,
        descriptions:
            ::std::result::Result<::std::vec::Vec<super::LongText>, ::std::string::String>,
        images: ::std::result::Result<::std::vec::Vec<super::Image>, ::std::string::String>,
        manufacturers:
            ::std::result::Result<::std::vec::Vec<super::OrganisationShort>, ::std::string::String>,
        medallions: ::std::result::Result<::std::vec::Vec<super::Medallion>, ::std::string::String>,
        media: ::std::result::Result<::std::vec::Vec<super::Medium>, ::std::string::String>,
        names: ::std::result::Result<::std::vec::Vec<super::ShortText>, ::std::string::String>,
        origins: ::std::result::Result<::std::vec::Vec<super::RegionCode>, ::std::string::String>,
        product_ids: ::std::result::Result<super::ProductIds, ::std::string::String>,
        shopping:
            ::std::result::Result<::std::vec::Vec<super::ShoppingEntry>, ::std::string::String>,
    }
    impl ::std::default::Default for ProductFull {
        fn default() -> Self {
            Self {
                alternatives: Err("no value supplied for alternatives".to_string()),
                descriptions: Err("no value supplied for descriptions".to_string()),
                images: Err("no value supplied for images".to_string()),
                manufacturers: Err("no value supplied for manufacturers".to_string()),
                medallions: Err("no value supplied for medallions".to_string()),
                media: Err("no value supplied for media".to_string()),
                names: Err("no value supplied for names".to_string()),
                origins: Ok(Default::default()),
                product_ids: Err("no value supplied for product_ids".to_string()),
                shopping: Err("no value supplied for shopping".to_string()),
            }
        }
    }
    impl ProductFull {
        pub fn alternatives<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CategoryAlternatives>>,
            T::Error: ::std::fmt::Display,
        {
            self.alternatives = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for alternatives: {}", e));
            self
        }
        pub fn descriptions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::LongText>>,
            T::Error: ::std::fmt::Display,
        {
            self.descriptions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for descriptions: {}", e));
            self
        }
        pub fn images<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Image>>,
            T::Error: ::std::fmt::Display,
        {
            self.images = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for images: {}", e));
            self
        }
        pub fn manufacturers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::OrganisationShort>>,
            T::Error: ::std::fmt::Display,
        {
            self.manufacturers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for manufacturers: {}", e));
            self
        }
        pub fn medallions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Medallion>>,
            T::Error: ::std::fmt::Display,
        {
            self.medallions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for medallions: {}", e));
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Medium>>,
            T::Error: ::std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media: {}", e));
            self
        }
        pub fn names<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ShortText>>,
            T::Error: ::std::fmt::Display,
        {
            self.names = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for names: {}", e));
            self
        }
        pub fn origins<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::RegionCode>>,
            T::Error: ::std::fmt::Display,
        {
            self.origins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for origins: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProductIds>,
            T::Error: ::std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
        pub fn shopping<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ShoppingEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.shopping = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shopping: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProductFull> for super::ProductFull {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProductFull,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alternatives: value.alternatives?,
                descriptions: value.descriptions?,
                images: value.images?,
                manufacturers: value.manufacturers?,
                medallions: value.medallions?,
                media: value.media?,
                names: value.names?,
                origins: value.origins?,
                product_ids: value.product_ids?,
                shopping: value.shopping?,
            })
        }
    }
    impl ::std::convert::From<super::ProductFull> for ProductFull {
        fn from(value: super::ProductFull) -> Self {
            Self {
                alternatives: Ok(value.alternatives),
                descriptions: Ok(value.descriptions),
                images: Ok(value.images),
                manufacturers: Ok(value.manufacturers),
                medallions: Ok(value.medallions),
                media: Ok(value.media),
                names: Ok(value.names),
                origins: Ok(value.origins),
                product_ids: Ok(value.product_ids),
                shopping: Ok(value.shopping),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductIds {
        eans: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
        gtins: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
        wiki: ::std::result::Result<::std::vec::Vec<super::Id>, ::std::string::String>,
    }
    impl ::std::default::Default for ProductIds {
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.eans = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for eans: {}", e));
            self
        }
        pub fn gtins<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.gtins = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gtins: {}", e));
            self
        }
        pub fn wiki<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.wiki = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wiki: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProductIds> for super::ProductIds {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProductIds,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                eans: value.eans?,
                gtins: value.gtins?,
                wiki: value.wiki?,
            })
        }
    }
    impl ::std::convert::From<super::ProductIds> for ProductIds {
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
        id: ::std::result::Result<super::Id, ::std::string::String>,
        product_id_variant: ::std::result::Result<super::ProductIdVariant, ::std::string::String>,
    }
    impl ::std::default::Default for ProductLink {
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
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn product_id_variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProductIdVariant>,
            T::Error: ::std::fmt::Display,
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
    impl ::std::convert::TryFrom<ProductLink> for super::ProductLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProductLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                product_id_variant: value.product_id_variant?,
            })
        }
    }
    impl ::std::convert::From<super::ProductLink> for ProductLink {
        fn from(value: super::ProductLink) -> Self {
            Self {
                id: Ok(value.id),
                product_id_variant: Ok(value.product_id_variant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProductShort {
        badges: ::std::result::Result<::std::vec::Vec<super::BadgeName>, ::std::string::String>,
        description:
            ::std::result::Result<::std::option::Option<super::LongText>, ::std::string::String>,
        name: ::std::result::Result<super::ShortString, ::std::string::String>,
        product_ids: ::std::result::Result<super::ProductIds, ::std::string::String>,
        scores: ::std::result::Result<::std::vec::Vec<super::Score>, ::std::string::String>,
    }
    impl ::std::default::Default for ProductShort {
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
            T: ::std::convert::TryInto<::std::vec::Vec<super::BadgeName>>,
            T::Error: ::std::fmt::Display,
        {
            self.badges = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for badges: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::LongText>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn product_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ProductIds>,
            T::Error: ::std::fmt::Display,
        {
            self.product_ids = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for product_ids: {}", e));
            self
        }
        pub fn scores<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Score>>,
            T::Error: ::std::fmt::Display,
        {
            self.scores = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scores: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ProductShort> for super::ProductShort {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProductShort,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                badges: value.badges?,
                description: value.description?,
                name: value.name?,
                product_ids: value.product_ids?,
                scores: value.scores?,
            })
        }
    }
    impl ::std::convert::From<super::ProductShort> for ProductShort {
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
    pub struct ReferenceLink {
        link: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ReferenceLink {
        fn default() -> Self {
            Self {
                link: Err("no value supplied for link".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl ReferenceLink {
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ReferenceLink> for super::ReferenceLink {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferenceLink,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                link: value.link?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ReferenceLink> for ReferenceLink {
        fn from(value: super::ReferenceLink) -> Self {
            Self {
                link: Ok(value.link),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Score {
        score: ::std::result::Result<i64, ::std::string::String>,
        scorer_name: ::std::result::Result<super::ScorerName, ::std::string::String>,
    }
    impl ::std::default::Default for Score {
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
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn scorer_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ScorerName>,
            T::Error: ::std::fmt::Display,
        {
            self.scorer_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scorer_name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Score> for super::Score {
        type Error = super::error::ConversionError;
        fn try_from(value: Score) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
                scorer_name: value.scorer_name?,
            })
        }
    }
    impl ::std::convert::From<super::Score> for Score {
        fn from(value: super::Score) -> Self {
            Self {
                score: Ok(value.score),
                scorer_name: Ok(value.scorer_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShoppingEntry {
        description: ::std::result::Result<super::ShortString, ::std::string::String>,
        link: ::std::result::Result<::std::string::String, ::std::string::String>,
        shop: ::std::result::Result<super::VerifiedShop, ::std::string::String>,
    }
    impl ::std::default::Default for ShoppingEntry {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                link: Err("no value supplied for link".to_string()),
                shop: Err("no value supplied for shop".to_string()),
            }
        }
    }
    impl ShoppingEntry {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
        pub fn shop<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerifiedShop>,
            T::Error: ::std::fmt::Display,
        {
            self.shop = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for shop: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ShoppingEntry> for super::ShoppingEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ShoppingEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                link: value.link?,
                shop: value.shop?,
            })
        }
    }
    impl ::std::convert::From<super::ShoppingEntry> for ShoppingEntry {
        fn from(value: super::ShoppingEntry) -> Self {
            Self {
                description: Ok(value.description),
                link: Ok(value.link),
                shop: Ok(value.shop),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ShortText {
        source: ::std::result::Result<super::DataSource, ::std::string::String>,
        text: ::std::result::Result<super::ShortString, ::std::string::String>,
    }
    impl ::std::default::Default for ShortText {
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
            T: ::std::convert::TryInto<super::DataSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ShortText> for super::ShortText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ShortText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source: value.source?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::ShortText> for ShortText {
        fn from(value: super::ShortText) -> Self {
            Self {
                source: Ok(value.source),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TcoMedallion {
        brand_name: ::std::result::Result<super::ShortString, ::std::string::String>,
    }
    impl ::std::default::Default for TcoMedallion {
        fn default() -> Self {
            Self {
                brand_name: Err("no value supplied for brand_name".to_string()),
            }
        }
    }
    impl TcoMedallion {
        pub fn brand_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.brand_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for brand_name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TcoMedallion> for super::TcoMedallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TcoMedallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                brand_name: value.brand_name?,
            })
        }
    }
    impl ::std::convert::From<super::TcoMedallion> for TcoMedallion {
        fn from(value: super::TcoMedallion) -> Self {
            Self {
                brand_name: Ok(value.brand_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextSearchLinkHack {
        id: ::std::result::Result<super::Id, ::std::string::String>,
        organisation_id_variant: ::std::result::Result<
            ::std::option::Option<super::OrganisationIdVariant>,
            ::std::string::String,
        >,
        product_id_variant: ::std::result::Result<
            ::std::option::Option<super::ProductIdVariant>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TextSearchLinkHack {
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
            T: ::std::convert::TryInto<super::Id>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn organisation_id_variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OrganisationIdVariant>>,
            T::Error: ::std::fmt::Display,
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
            T: ::std::convert::TryInto<::std::option::Option<super::ProductIdVariant>>,
            T::Error: ::std::fmt::Display,
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
    impl ::std::convert::TryFrom<TextSearchLinkHack> for super::TextSearchLinkHack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextSearchLinkHack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                organisation_id_variant: value.organisation_id_variant?,
                product_id_variant: value.product_id_variant?,
            })
        }
    }
    impl ::std::convert::From<super::TextSearchLinkHack> for TextSearchLinkHack {
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
        label: ::std::result::Result<super::ShortString, ::std::string::String>,
        link: ::std::result::Result<super::TextSearchLinkHack, ::std::string::String>,
    }
    impl ::std::default::Default for TextSearchResult {
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
            T: ::std::convert::TryInto<super::ShortString>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn link<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TextSearchLinkHack>,
            T::Error: ::std::fmt::Display,
        {
            self.link = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TextSearchResult> for super::TextSearchResult {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextSearchResult,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                label: value.label?,
                link: value.link?,
            })
        }
    }
    impl ::std::convert::From<super::TextSearchResult> for TextSearchResult {
        fn from(value: super::TextSearchResult) -> Self {
            Self {
                label: Ok(value.label),
                link: Ok(value.link),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextSearchResults {
        results:
            ::std::result::Result<::std::vec::Vec<super::TextSearchResult>, ::std::string::String>,
    }
    impl ::std::default::Default for TextSearchResults {
        fn default() -> Self {
            Self {
                results: Err("no value supplied for results".to_string()),
            }
        }
    }
    impl TextSearchResults {
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TextSearchResult>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TextSearchResults> for super::TextSearchResults {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TextSearchResults,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                results: value.results?,
            })
        }
    }
    impl ::std::convert::From<super::TextSearchResults> for TextSearchResults {
        fn from(value: super::TextSearchResults) -> Self {
            Self {
                results: Ok(value.results),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranspaerMedallion {
        score: ::std::result::Result<super::TranspaerScore, ::std::string::String>,
    }
    impl ::std::default::Default for TranspaerMedallion {
        fn default() -> Self {
            Self {
                score: Err("no value supplied for score".to_string()),
            }
        }
    }
    impl TranspaerMedallion {
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranspaerScore>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TranspaerMedallion> for super::TranspaerMedallion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranspaerMedallion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                score: value.score?,
            })
        }
    }
    impl ::std::convert::From<super::TranspaerMedallion> for TranspaerMedallion {
        fn from(value: super::TranspaerMedallion) -> Self {
            Self {
                score: Ok(value.score),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranspaerScore {
        total: ::std::result::Result<f64, ::std::string::String>,
        tree: ::std::result::Result<
            ::std::vec::Vec<super::TranspaerScoreBranch>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TranspaerScore {
        fn default() -> Self {
            Self {
                total: Err("no value supplied for total".to_string()),
                tree: Err("no value supplied for tree".to_string()),
            }
        }
    }
    impl TranspaerScore {
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {}", e));
            self
        }
        pub fn tree<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranspaerScoreBranch>>,
            T::Error: ::std::fmt::Display,
        {
            self.tree = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tree: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TranspaerScore> for super::TranspaerScore {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranspaerScore,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                total: value.total?,
                tree: value.tree?,
            })
        }
    }
    impl ::std::convert::From<super::TranspaerScore> for TranspaerScore {
        fn from(value: super::TranspaerScore) -> Self {
            Self {
                total: Ok(value.total),
                tree: Ok(value.tree),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TranspaerScoreBranch {
        branches: ::std::result::Result<
            ::std::vec::Vec<super::TranspaerScoreBranch>,
            ::std::string::String,
        >,
        category: ::std::result::Result<super::TranspaerScoreCategory, ::std::string::String>,
        score: ::std::result::Result<f64, ::std::string::String>,
        weight: ::std::result::Result<i64, ::std::string::String>,
    }
    impl ::std::default::Default for TranspaerScoreBranch {
        fn default() -> Self {
            Self {
                branches: Err("no value supplied for branches".to_string()),
                category: Err("no value supplied for category".to_string()),
                score: Err("no value supplied for score".to_string()),
                weight: Err("no value supplied for weight".to_string()),
            }
        }
    }
    impl TranspaerScoreBranch {
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TranspaerScoreBranch>>,
            T::Error: ::std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TranspaerScoreCategory>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for category: {}", e));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for weight: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TranspaerScoreBranch> for super::TranspaerScoreBranch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TranspaerScoreBranch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branches: value.branches?,
                category: value.category?,
                score: value.score?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::TranspaerScoreBranch> for TranspaerScoreBranch {
        fn from(value: super::TranspaerScoreBranch) -> Self {
            Self {
                branches: Ok(value.branches),
                category: Ok(value.category),
                score: Ok(value.score),
                weight: Ok(value.weight),
            }
        }
    }
}
