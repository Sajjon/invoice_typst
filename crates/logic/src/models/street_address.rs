use crate::prelude::*;

/// Street address information
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct StreetAddress {
    /// The street address line 1, of the company, e.g. `"10 West Smithfield"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_1: String,
    /// The street address line 2, of the company, e.g. `"C/o Other company"`.
    #[builder(setter(into), default = "".to_owned())]
    #[getset(get = "pub")]
    line_2: String,
    /// The street address line 3, of the company, e.g. `"2nd floor"`.
    #[builder(setter(into), default = "".to_owned())]
    #[getset(get = "pub")]
    line_3: String,
}
