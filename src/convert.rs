use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename = "Finvoice")]
pub struct Finvoice {
    #[serde(rename = "SellerPartyDetails")]
    pub seller: SellerPartyDetails,

    #[serde(rename = "BuyerPartyDetails")]
    pub buyer: BuyerPartyDetails,

    #[serde(rename = "InvoiceDetails")]
    pub invoice: InvoiceDetails,

    #[serde(rename = "InvoiceRow")]
    pub rows: Vec<InvoiceRow>,

    #[serde(rename = "EpiDetails")]
    pub payment: EpiDetails,
}

#[derive(Debug, Deserialize)]
pub struct SellerPartyDetails {
    #[serde(rename = "SellerOrganisationName")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct BuyerPartyDetails {
    #[serde(rename = "BuyerOrganisationName")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceDetails {
    #[serde(rename = "InvoiceNumber")]
    pub number: String,

    #[serde(rename = "InvoiceDate")]
    pub date: String,

    #[serde(rename = "InvoiceTotalVatExcludedAmount")]
    pub vat_excluded: String,

    #[serde(rename = "InvoiceTotalVatAmount")]
    pub vat: String,

    #[serde(rename = "InvoiceTotalVatIncludedAmount")]
    pub total: String,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceRow {
    #[serde(rename = "ArticleName")]
    pub name: String,

    #[serde(rename = "InvoicedQuantity")]
    pub quantity: String,

    #[serde(rename = "UnitPriceAmount")]
    pub unit_price: String,

    #[serde(rename = "RowAmount")]
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct EpiDetails {
    #[serde(rename = "EpiPaymentInstructionDetails")]
    pub instruction: PaymentInstruction,
}

#[derive(Debug, Deserialize)]
pub struct PaymentInstruction {
    #[serde(rename = "EpiReference")]
    pub reference: Option<String>,

    #[serde(rename = "EpiInstructedAmount")]
    pub amount: String,

    #[serde(rename = "EpiDateOptionDate")]
    pub due_date: String,
}
