use crate::prelude::*;

/// A purchase order number associated with this invoice, e.g. `"PO-12345"`
/// Typically agreed upon between the vendor and client before the
/// invoice is issued.
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
#[from(String, &'static str)]
#[serde(transparent)]
pub struct PurchaseOrder(String);

impl PurchaseOrder {
    pub fn sample() -> Self {
        Self::from("PO-12345")
    }
}
