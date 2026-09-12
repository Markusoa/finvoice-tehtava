mod convert;

use convert::Finvoice;
use flexpdf::builder::{document, text, view};
use flexpdf::{render_document, PageSize, Style};
use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("finvoice_testi_2_01.xml")?;
    let finvoice: Finvoice = quick_xml::de::from_str(&xml)?;

    fn format_date(ccyymmdd: &str) -> String {
        if let Ok(date) = NaiveDate::parse_from_str(ccyymmdd, "%Y%m%d") {
            date.format("%-d.%-m.%Y").to_string()
        } else {
            ccyymmdd.to_string() 
        }
    }

    let row1 = text(format!("{:<30}       {:<20} {:<18} {:<18} {:<16} {:<18} {:<18}",
        finvoice.rows[0].name,
        finvoice.rows[0].quantity.value,
        finvoice.rows[0].quantity.unit,
        finvoice.rows[0].unit_price,
        finvoice.rows[0].vat_rate,
        finvoice.rows[0].vat_amount,
        finvoice.rows[0].amount
    )).style(Style {
        padding_bottom: Some(17.0),
        ..Style::default()
    });

    let row2 = text(format!("{:<30}           {:<20}  {:<18} {:<18} {:<16} {:<18} {:<18}",
        finvoice.rows[1].name,
        finvoice.rows[1].quantity.value,
        finvoice.rows[1].quantity.unit,
        finvoice.rows[1].unit_price,
        finvoice.rows[1].vat_rate,
        finvoice.rows[1].vat_amount,
        finvoice.rows[1].amount
    ));

    let doc = document()
        .title("Test invoice")
        .page_with(PageSize::A4, |page| {
            page.child(
                view()
                    .style(Style {
                        padding: Some(32.0),
                        ..Style::default()
                    })
                    .children([
                        text("LASKU").style(Style {
                            font_size: Some(22.0),
                            ..Style::default()
                        }),
                        text("\n"),
                        text(format!("Myyjä")).style(Style {
                            font_size: Some(16.0),
                            ..Style::default()
                        }),
                        text(format!("{}", finvoice.seller.name)),
                        text(format!("{}", finvoice.seller.address.streetname)),
                        text(format!("{}  {}  {}", finvoice.seller.address.postcode,
                        finvoice.seller.address.townname, 
                        finvoice.seller.address.country)),
                        text(format!("Y-tunnus: {}", finvoice.seller.identifier)),
                        text("\n"),
                        text(format!("Ostaja")).style(Style {
                            font_size: Some(16.0),
                            ..Style::default() 
                        }),
                        text(format!("{}", finvoice.buyer.name)),
                        text(format!("{}", finvoice.buyer.address.streetname)),
                        text(format!("{}  {}  {}", finvoice.buyer.address.postcode,
                        finvoice.buyer.address.townname,
                        finvoice.buyer.address.country)),
                        text("\n"),
                        text("Maksutiedot").style(Style {
                            font_size: Some(16.0),
                            ..Style::default()
                        }),
                        text(format!("{}", finvoice.seller.name)),
                        text(format!("IBAN:   {}", finvoice.sellerinfo.account.account_id)),
                        text(format!("BIC:   {}", finvoice.sellerinfo.account.bic)),
                        text(format!("laskunumero:   {}", finvoice.invoice.number)),
                        text(format!("Viitenumero:   {}", finvoice.payment.identification.reference)),
                        text(format!("Päiväys:   {}", format_date(&finvoice.payment.identification.date))),
                        text(format!("Eräpäivä:   {}", format_date(&finvoice.payment.instruction.due_date))),
                        text("\n"),
                        text("Kuvaus                          Määrä      Yksikkö    Y-Hinta     ALV %      ALV €      Yhteensä").style(Style {
                            font_size: Some(16.0),
                            ..Style::default()
                        }),
                        text("\n"),
                    ])
                        .children([row1, row2,])

                        .children([
                            text("\n"),
                            text("\n"),
                            text(format!("Veroton summa: {}", finvoice.invoice.vat_excluded)).style(Style {
                                font_size: Some(14.0),
                                padding_bottom: Some(20.0),
                                ..Style::default()
                            }),
                            text(format!("Arvonlisävero: {}", finvoice.invoice.vat)).style(Style {
                                font_size: Some(14.0),
                                padding_bottom: Some(20.0),
                                ..Style::default()
                            }),
                            text(format!("Loppusumma: {}", finvoice.invoice.total)).style(Style {
                                font_size: Some(14.0),
                                padding_bottom: Some(20.0),
                                ..Style::default()
                            }),
                        ])
            )
        })
        .build();

    let pdf = render_document(&doc)?;

    std::fs::write("test.pdf", pdf)?;

    Ok(())
}
