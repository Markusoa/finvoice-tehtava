use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename = "Finvoice")]
pub struct Finvoice {
    #[serde(rename = "SellerPartyDetails")]
    pub seller: SellerPartyDetails,

    #[serde(rename = "SellerInformationDetails")]
    pub sellerinfo: SellerInformationDetails,

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

    #[serde(rename = "SellerPartyIdentifier")]
    pub identifier: String,

    #[serde(rename = "SellerPostalAddressDetails")]
    pub address: SellerPostalAddressDetails,
}

#[derive(Debug, Deserialize)]
pub struct SellerInformationDetails {
    #[serde(rename = "SellerAccountDetails")]
    pub account: SellerAccountDetails,
}

#[derive(Debug, Deserialize)]
pub struct SellerAccountDetails {
    #[serde(rename = "SellerAccountID")]
    pub account_id: String,

    #[serde(rename = "SellerBic")]
    pub bic: String,

}

#[derive(Debug, Deserialize)]
pub struct SellerPostalAddressDetails {
    #[serde(rename = "SellerStreetName")]
    pub streetname: String,

    #[serde(rename = "SellerTownName")]
    pub townname: String,

    #[serde(rename = "SellerPostCodeIdentifier")]
    pub postcode: String,

    #[serde(rename = "CountryName")]
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct BuyerPartyDetails {
    #[serde(rename = "BuyerOrganisationName")]
    pub name: String,

    #[serde(rename = "BuyerPostalAddressDetails")]
    pub address: BuyerPostalAddressDetails,
}

#[derive(Debug, Deserialize)]
pub struct BuyerPostalAddressDetails {
    #[serde(rename = "BuyerStreetName")]
    pub streetname: String,

    #[serde(rename = "BuyerTownName")]
    pub townname: String,

    #[serde(rename = "BuyerPostCodeIdentifier")]
    pub postcode: String,

    #[serde(rename = "CountryName")]
    pub country: String,
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
    pub quantity: Quantity,

    #[serde(rename = "UnitPriceAmount")]
    pub unit_price: String,

    #[serde(rename = "RowVatRatePercent")]
    pub vat_rate: String,

    #[serde(rename = "RowVatAmount")]
    pub vat_amount: String,

    #[serde(rename = "RowAmount")]
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct Quantity {
    #[serde(rename = "$value")]
    pub value: String,

    #[serde(rename = "@QuantityUnitCode")]
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct EpiDetails {
    #[serde(rename = "EpiPaymentInstructionDetails")]
    pub instruction: PaymentInstruction,

    #[serde(rename = "EpiIdentificationDetails")]
    pub identification: EpiIdentificationDetails,
}

#[derive(Debug, Deserialize)]
pub struct EpiIdentificationDetails {
    #[serde(rename = "EpiDate")]
    pub date: String,

    #[serde(rename = "EpiReference")]
    pub reference: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentInstruction {
    #[serde(rename = "EpiInstructedAmount")]
    pub amount: String,

    #[serde(rename = "EpiDateOptionDate")]
    pub due_date: String,
}
