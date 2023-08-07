use super::*;

impl Display for HTMLElement {
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
            f.write_str(&self.classes.join(" "))?;
            f.write_str("\"")?;
        }
        for (key, value) in &self.attributes {
            f.write_str(" ")?;
            f.write_str(key)?;
            f.write_str("=\"")?;
            f.write_str(value)?;
            f.write_str("\"")?;
        }
        match &self.children {
            [] => f.write_str("/>"),
            nodes => {
                f.write_str(">")?;
                for node in nodes {
                    f.write_str(&node.to_string())?;
                }
                f.write_str("</")?;
                f.write_str(&self.tag)?;
                f.write_str(">")
            }
        }
    }
}
