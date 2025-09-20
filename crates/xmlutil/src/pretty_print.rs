use xot::{Xot, output};

pub(crate) fn pretty_print_xml(xmlsrc: &str) -> String {

    let xml: String;
    let mut xot = Xot::new();
    if let Ok(root) = xot.parse(xmlsrc) {
        xml = xot.serialize_xml_string(output::xml::Parameters {
            indentation: Some(Default::default()),
            ..Default::default()
        }, root).unwrap_or(xmlsrc.to_owned());

    } else {
        return xmlsrc.to_owned();
    }

    xml
}