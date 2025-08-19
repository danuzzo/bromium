use xee_xpath::error::Error;
use xee_xpath::context::StaticContextBuilder;
use xee_xpath::Itemable;
use xee_xpath::Query;

pub struct XpathResult {
    result_count: usize,
    result: Vec<String>,
}

impl XpathResult {
    fn new(result_count: usize, result: Vec<String>) -> Self {
        XpathResult { result_count, result }
    }
    
    pub fn get_result_count(&self) -> usize {
        self.result_count
    }
    
    pub fn get_result_items(&self) -> Vec<String> {
        self.result.clone()
    }
}



pub fn eval_xpath(expr: String, srcxml: String) -> XpathResult {
    
    let input_xml = srcxml.as_str();

    let mut documents = xee_xpath::Documents::new();
    let doc = documents.add_string_without_uri(&input_xml).unwrap();

    let static_context_builder = make_static_context_builder(
        None,
        &[],
    ).unwrap();

    let queries = xee_xpath::Queries::new(static_context_builder);
    let res = execute_query(expr.as_str(), &queries, &mut documents, Some(doc)).unwrap();
    res
}


fn execute_query(
    xpath: &str,
    queries: &xee_xpath::Queries<'_>,
    documents: &mut xee_xpath::Documents,
    doc: Option<xee_xpath::DocumentHandle>,
) -> Result<XpathResult, anyhow::Error> {

    let no_result = XpathResult::new(0, vec!["".to_string()]);

    let sequence_query = queries.sequence(xpath);
    let sequence_query = match sequence_query {
        Ok(sequence_query) => sequence_query,
        Err(e) => {
            render_error(xpath, e);
            return Ok(no_result);
        }
    };
    let mut context_builder = sequence_query.dynamic_context_builder(documents);
    if let Some(doc) = doc {
        context_builder.context_item(doc.to_item(documents)?);
    }
    let context = context_builder.build();

    let sequence = sequence_query.execute_with_context(documents, &context);
    let sequence = match sequence {
        Ok(sequence) => sequence,
        Err(e) => {
            render_error(xpath, e);
            return Ok(no_result);
        }
    };

    println!(
        "No of items found: {}\n{}", sequence.len(),
        sequence.display_representation(documents.xot(), &context)
    );

    let mut results: Vec<String> = Vec::new();
    for idx in 0..sequence.len() {
        results.push(sequence.get(idx).unwrap().display_representation(documents.xot(), &context).unwrap());
        println!("{}", sequence.get(idx).unwrap().display_representation(documents.xot(), &context).unwrap());
    }

    // construct the result
    let result = XpathResult::new(sequence.len(),results);

    Ok(result)
}


fn make_static_context_builder<'a>(
    default_namespace_uri: Option<&'a str>,
    namespaces: &'a [String],
) -> anyhow::Result<StaticContextBuilder<'a>> {
    let mut static_context_builder = xee_xpath::context::StaticContextBuilder::default();
    if let Some(default_namespace_uri) = default_namespace_uri {
        static_context_builder.default_element_namespace(default_namespace_uri);
    }
    let namespaces = namespaces
        .iter()
        .map(|declaration| {
            let mut parts = declaration.splitn(2, '=');
            let prefix = parts.next().ok_or(anyhow::anyhow!("missing prefix"))?;
            let uri = parts.next().ok_or(anyhow::anyhow!("missing uri"))?;
            Ok((prefix, uri))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    static_context_builder.namespaces(namespaces);
    Ok(static_context_builder)
}




fn render_error(src: &str, e: Error) {
    let red = ariadne::Color::Red;

    let mut report = ariadne::Report::build(ariadne::ReportKind::Error, ("source", (0..0)))
        .with_code(e.error.code());

    if let Some(span) = e.span {
        report = report.with_label(
            ariadne::Label::new(("source", span.range()))
                .with_message(e.error.message())
                .with_color(red),
        )
    }
    report
        .finish()
        .eprint(("source", ariadne::Source::from(src)))
        .unwrap();
    println!("{}", e.error.note());
}