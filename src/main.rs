mod convert;

use convert::Finvoice;
use flexpdf::builder::{document, text, view};
use flexpdf::{render_document, PageSize, Style};
use flexpdf::style::FlexDirection;
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

    let cell = |value: String, width: f32, font_size: f32| {
        view()
            .style(Style {
                width: Some(flexpdf::style::Dimension::Points(width)),
                ..Style::default()
            })
            .children([text(value).style(Style {
                font_size: Some(font_size),
                ..Style::default()
            })])
    };

    let rows: Vec<_> = finvoice
        .rows
        .iter()
        .map(|row| {
            view()
                .style(Style {
                    flex_direction: Some(FlexDirection::Row),
                    padding_bottom: Some(12.0),
                    ..Style::default()
                })
                .children([
                    cell(row.name.clone(), 190.0, 12.0),
                    cell(row.quantity.value.clone(), 75.0, 12.0),
                    cell(row.quantity.unit.clone(), 70.0, 12.0),
                    cell(row.unit_price.clone(), 70.0, 12.0),
                    cell(row.vat_rate.clone(), 65.0, 12.0),
                    cell(row.vat_amount.clone(), 80.0, 12.0),
                    cell(row.amount.clone(), 80.0, 12.0),
                ])
        })
        .collect();

    let doc = document()
        .title("Test invoice")
        .page_with(PageSize::A4, |page| {
            page.child(
                view()
                    .style(Style {
                        padding: Some(32.0),
                        gap: Some(40.0),
                        ..Style::default()
                    })
                    .children([
                        view().children([
                            text("LASKU").style(Style {
                                font_size: Some(22.0),
                                ..Style::default()
                            }),
                        ]).style(Style {
                            padding_bottom: Some(20.0),
                            ..Style::default()
                        }),
                        view()
                            .style(Style {
                                flex_direction: Some(FlexDirection::Row),
                                ..Style::default()
                            })
                            .children([
                                view().style(Style {
                                    padding_bottom: Some(20.0),
                                    gap: Some(3.0),
                                    ..Style::default()
                                }).children([
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
                                ]),
                                view()
                                    .style(Style {
                                        flex_grow: Some(1.0),
                                        padding_left: Some(175.0),
                                        gap: Some(3.0),
                                        ..Style::default()
                                    })
                                    .children([
                                        text("Maksutiedot").style(Style {
                                            font_size: Some(16.0),
                                            ..Style::default()
                                        }),
                                        text(format!("{}", finvoice.seller.name)),
                                        text(format!("IBAN:   {}", finvoice.sellerinfo.account.account_id)),
                                        text(format!("BIC:   {}", finvoice.sellerinfo.account.bic)),
                                        text(format!("Laskunumero:   {}", finvoice.invoice.number)),
                                        text(format!("Viitenumero:   {}", finvoice.payment.identification.reference)),
                                        text(format!("Päiväys:   {}", format_date(&finvoice.payment.identification.date))),
                                        text(format!("Eräpäivä:   {}", format_date(&finvoice.payment.instruction.due_date))),
                                    ]),
                            ]),
                        view()
                            .style(Style {
                                flex_direction: Some(FlexDirection::Column),
                                ..Style::default()
                            })
                            .children([
                                view().children([
                                    view()
                                        .style(Style {
                                            flex_direction: Some(FlexDirection::Row),
                                            ..Style::default()
                                        })
                                        .children([
                                            cell("Kuvaus".into(), 160.0, 14.0),
                                            cell("Määrä".into(), 70.0, 14.0),
                                            cell("Yksikkö".into(), 70.0, 14.0),
                                            cell("Y-Hinta".into(), 70.0, 14.0),
                                            cell("ALV %".into(), 65.0, 14.0),
                                            cell("ALV €".into(), 70.0, 14.0),
                                            cell("Yhteensä".into(), 80.0, 14.0),
                                        ]),
                                    view()
                                        .style(Style {
                                            padding_top: Some(25.0),
                                            gap: Some(12.0),
                                            ..Style::default()
                                        })
                                        .children(rows),
                                ]),

                                view().children([
                                    text(".").style(Style {
                                        font_size: Some(0.1),
                                        padding_bottom: Some(30.0),
                                        ..Style::default()
                                    }),
                                ]),
                                view()
                                    .style(Style {
                                        position: Some(flexpdf::style::Position::Absolute),
                                        top: Some(flexpdf::style::Dimension::Points(150.0)),
                                        gap: Some(12.0),
                                        ..Style::default()
                                    })
                                    .children([
                                        text(format!("Veroton summa: {}", finvoice.invoice.vat_excluded)).style(Style {
                                            font_size: Some(14.0),
                                            ..Style::default()
                                        }),
                                        text(format!("Arvonlisävero: {}", finvoice.invoice.vat)).style(Style {
                                            font_size: Some(14.0),
                                            ..Style::default()
                                        }),
                                        text(format!("Loppusumma: {}", finvoice.invoice.total)).style(Style {
                                            font_size: Some(14.0),
                                            ..Style::default()
                                        }),
                                    ]),
                            ])
                    ])
                )
        })
        .build();

    let pdf = render_document(&doc)?;

    std::fs::write("lasku.pdf", pdf)?;

    Ok(())
}

