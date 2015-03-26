[htmler](https://github.com/oovm/zhihu-markdown/blob/dev/projects/htmler) is a html parsing library with css selectors.

This is a fork of [scraper](https://github.com/causal-agent/scraper), and provides higher-level encapsulation and a more consistent interface.

# Examples

## Parsing a document/fragment

```rs
use htmler::Html;

let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

let document = Html::parse_document(html);
let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
```

## Selecting elements

```rs
use htmler::{Html, Selector};

let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

let fragment = Html::parse_fragment(html);
let selector = Selector::new("li");

for element in fragment.select(&selector) {
    assert_eq!("li", element.value().name());
}
```

## Selecting descendent elements

```rs
use htmler::{Html, Selector};

let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

let fragment = Html::parse_fragment(html);
let ul_selector = Selector::new("ul");
let li_selector = Selector::new("li");

let ul = fragment.select(&ul_selector).next().unwrap();
for element in ul.select(&li_selector) {
    assert_eq!("li", element.value().name());
}
```

## Accessing element attributes

```rs
use htmler::{Html, Selector};

let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
let selector = Selector::new(r#"input[name="foo"]"#);

let input = fragment.select(&selector).next().unwrap();
assert_eq!("bar", input.get_attribute("value"));
```

## Serializing HTML and inner HTML

```rs
use htmler::{Html, Selector};

let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
let selector = Selector::new("h1");

let h1 = fragment.select(&selector).next().unwrap();

assert_eq!("<h1>Hello, <i>world!</i></h1>", h1.html());
assert_eq!("Hello, <i>world!</i>", h1.inner_html());
```

## Accessing descendent text

```rs
use htmler::{Html, Selector};

let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
let selector = Selector::new("h1");

let h1 = fragment.select(&selector).next().unwrap();
let text = h1.text().collect::<Vec<_>>();

assert_eq!(vec!["Hello, ", "world!"], text);
```