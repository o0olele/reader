use super::{JsContext, JsValue, QuickJsRuntime};

fn run(script: &str, html: &str) -> Result<JsValue, crate::error::AppError> {
    QuickJsRuntime::default().execute_blocking(
        script,
        JsContext {
            result: html.into(),
            ..Default::default()
        },
    )
}

#[test]
fn jsoup_parse_executes_the_corpus_catalog_sort_chain() {
    // source[519].ruleToc.chapterList, with a nonempty deterministic page.
    let value = run(
        "org.jsoup.Jsoup.parse(result.replace(/\\.html/g,\"\")).select('.list li a').toArray().sort((a,b)=>a.attr('href')-b.attr('href')).map(x=>({n:x.text(),u:x.attr('href')+\".html\"}))",
        r#"<ul class="list"><li><a href="20.html">第二十章</a></li><li><a href="3.html">第三章</a></li></ul>"#,
    ).unwrap();
    assert_eq!(
        value,
        JsValue::Json(serde_json::json!([
            {"n":"第三章","u":"3.html"}, {"n":"第二十章","u":"20.html"}
        ]))
    );
}

#[test]
fn jsoup_parse_keeps_document_head_and_nested_element_inputs() {
    let value = run(
        "var d=org.jsoup.Jsoup.parse(result); d.select('head meta').attr('content')+'|'+org.jsoup.Jsoup.parse(d.select('a')[0]).select('a').text()",
        r#"<html><head><meta name="author" content="作者"></head><body><a href="/1">章节</a></body></html>"#,
    ).unwrap();
    assert_eq!(value, JsValue::String("作者|章节".into()));
}

#[test]
fn jsoup_html_is_inner_markup_while_string_conversion_keeps_the_node() {
    let value = run(
        "var a=org.jsoup.Jsoup.parse(result).select('a'); [a.html(),a[0].html(),a[0].outerHtml(),String(a[0])].join('|')",
        "<a><b>Chapter</b></a>",
    ).unwrap();
    assert_eq!(
        value,
        JsValue::String(
            "<b>Chapter</b>|<b>Chapter</b>|<a><b>Chapter</b></a>|<a><b>Chapter</b></a>".into()
        )
    );
}

#[test]
fn jsoup_invalid_css_is_not_silently_an_empty_match() {
    for script in [
        "org.jsoup.Jsoup.parse(result).select('[')",
        "org.jsoup.Jsoup.parse(result).select('a')[0].select('[')",
        "org.jsoup.Jsoup.parse(result).select('.absent').select('[')",
    ] {
        assert!(run(script, "<a>Book</a>")
            .unwrap_err()
            .to_string()
            .contains("JSoup selector"));
    }
}

#[test]
fn jsoup_empty_match_is_empty_and_unsupported_mutation_still_fails() {
    assert_eq!(
        run(
            "org.jsoup.Jsoup.parse(result).select('.absent').size()",
            "<p>Body</p>"
        )
        .unwrap(),
        JsValue::Number(0.0)
    );
    assert!(run(
        "org.jsoup.Jsoup.parse(result).select('p').remove()",
        "<p>Body</p>"
    )
    .is_err());
}

#[test]
fn jsoup_namespace_does_not_pollute_strings_or_break_the_org_alias() {
    assert_eq!(
        run("org.trim()+'|'+org+'|'+typeof ''.jsoup", " body ").unwrap(),
        JsValue::String("body| body |undefined".into())
    );
}
