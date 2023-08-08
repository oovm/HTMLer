use super::*;
use std::borrow::Cow;

impl Display for HtmlElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<")?;
        f.write_str(&self.tag)?;
        if !self.id.is_empty() {
            f.write_str(" id=\"")?;
            f.write_str(&self.id)?;
            f.write_str("\"")?;
        }
        if !self.classes.is_empty() {
            f.write_str(" class=\"")?;
            for (i, class) in self.classes.iter().enumerate() {
                if i > 0 {
                    f.write_str(" ")?;
                }
                f.write_str(class)?;
            }
            f.write_str("\"")?;
        }
        for (key, value) in &self.attributes {
            f.write_str(" ")?;
            f.write_str(key)?;
            f.write_str("=\"")?;
            f.write_str(value)?;
            f.write_str("\"")?;
        }
        if self.children.is_empty() {
            f.write_str("/>")
        }
        else {
            f.write_str(">")?;
            for node in &self.children {
                f.write_str(&node.to_string())?;
            }
            f.write_str("</")?;
            f.write_str(&self.tag)?;
            f.write_str(">")
        }
    }
}

impl From<HtmlElement> for HtmlNode {
    fn from(value: HtmlElement) -> Self {
        Self::Element(value)
    }
}

impl From<String> for HtmlNode {
    fn from(text: String) -> Self {
        Self::Text(Cow::Owned(text))
    }
}
impl From<&'static str> for HtmlNode {
    fn from(text: &'static str) -> Self {
        Self::Text(Cow::Borrowed(text))
    }
}
