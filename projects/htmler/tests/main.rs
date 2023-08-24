#[test]
fn ready() {
    println!("it works!")
}

use htmler::{Html, Selector};

#[test]
fn tag_with_newline() {
    let selector = Selector::try_from("a").unwrap();

    let document = Html::parse_fragment(
        r#"
        <a
                            href="https://github.com/causal-agent/scraper">

                            </a>
        "#,
    );

    let mut iter = document.select(&selector);
    let a = iter.next().unwrap();
    assert_eq!(a.get_attribute("href"), "https://github.com/causal-agent/scraper");
}

#[test]
fn main() {
    let fragment = Html::parse_fragment("<h1><script>Hello, world!</script></h1>");
    for node in fragment.root_node().descendants() {
        match node.as_element() {
            None => {}
            Some(s) => {
                println!("{:#?}", s);
            }
        }
    }
}
