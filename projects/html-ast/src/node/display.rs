use super::*;

impl Default for HtmlNode {
    fn default() -> Self {
        Self::Element(HtmlElement::default())
    }
}
impl Display for HtmlNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlNode::Doctype(_) => write!(f, "<!DOCTYPE html>"),
            HtmlNode::Comment(comment) => write!(f, "<!-- {} -->", comment),
            HtmlNode::Text(text) => {
                write!(f, "{}", text)
            }
            HtmlNode::Element(element) => write!(f, "{}", element),
            HtmlNode::ProcessingInstruction(pi) => write!(f, "<?{} {}?>", pi.target, pi.data),
        }
    }
}
