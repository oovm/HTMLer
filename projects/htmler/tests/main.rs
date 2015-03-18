#[test]
fn ready() {
    println!("it works!")
}

use htmler::{Html, Selector};

#[test]
fn tag_with_newline() {
    let selector = Selector::try_parse("a").unwrap();

    let document = Html::parse_fragment(
        r#"
        <a
                            href="https://github.com/causal-agent/scraper">

                            </a>
        "#,
    );

    let mut iter = document.select(&selector);
    let a = iter.next().unwrap();
    assert_eq!(
        a.value().get_attribute("href"),
        Some("https://github.com/causal-agent/scraper")
    );
}
