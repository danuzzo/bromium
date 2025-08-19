pub mod xpath_gen;
pub mod xpath_eval;
pub mod xml;

pub use xpath_gen::*;
pub use xpath_eval::*;
pub use xml::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_xpath() {

        
        
        let xpath = "/doc/section1/para";
        let xml = r#"<doc>
                                    <section1>
                                        <heading>A Level-1 Heading</heading>
                                        <para>The first paragraph.</para>
                                        <para>The second paragraph.</para>
                                        <section2>
                                        <heading type="myHeading">A Level-2 Heading</heading>
                                        <para>The first paragraph after the sub-heading.</para>
                                        </section2>
                                    </section1>
                                    </doc>
                                    "#;

        let res: XpathResult = xpath_eval::eval_xpath(xpath.to_owned(), xml.to_owned());
        
        assert_eq!(res.get_result_count(), 2);
        assert_eq!(res.get_result_items().get(1).unwrap_or(&"".to_string()), &"<para>The second paragraph.</para>".to_string());

    }
}
