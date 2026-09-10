mod convert;

use convert::Finvoice;
use flexpdf::builder::{document, text, view};
use flexpdf::{render_document, PageSize, Style};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = std::fs::read_to_string("finvoice_testi_2_01.xml")?;
    let finvoice: Finvoice = quick_xml::de::from_str(&xml)?;

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
                        text("INVOICE"),
                        text(format!("Invoice number: {}", finvoice.invoice.number)),
                        text(format!("Total: {} EUR", finvoice.invoice.total)),
                        text(format!("Due date: {}", finvoice.payment.instruction.due_date)),
                    ]),
            )
        })
        .build();

    let pdf = render_document(&doc)?;

    std::fs::write("test.pdf", pdf)?;

    Ok(())
}
