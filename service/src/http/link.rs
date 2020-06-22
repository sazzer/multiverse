use rocket::http::Header;
use std::fmt::{Display, Formatter};

/// Representation of a Link Relation
#[derive(Debug, PartialEq)]
pub struct LinkRel(&'static str);

impl LinkRel {
    pub const AUTHOR: LinkRel = LinkRel("author");
    pub const RELATED: LinkRel = LinkRel("related");
    pub const SELF: LinkRel = LinkRel("self");
}

/// Representation of a Link header
#[derive(Debug)]
pub struct Link {
    /// The target of the link
    target: String,
    /// The link relation
    rel: LinkRel,
    /// The title of the link
    title: Option<String>,
    /// The anchor of the link
    anchor: Option<String>,
}

/// Representation of a set of link headers
pub struct Links(pub Vec<Link>);

impl Link {
    /// Create a new Link header
    ///
    /// # Parameters
    /// - `target` - The target of the link
    /// - `rel` - The link relation
    ///
    /// # Returns
    /// The link header
    pub fn new<T>(target: T, rel: LinkRel) -> Self
    where
        T: Into<String>,
    {
        Self {
            target: target.into(),
            rel,
            title: None,
            anchor: None,
        }
    }

    /// Specify the Title of the Link
    ///
    /// # Parameters
    /// - `title` - The title of the link
    ///
    /// # Returns
    /// The link
    pub fn title<T>(self, title: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    /// Specify the Anchor of the Link
    ///
    /// # Parameters
    /// - `anchor` - The anchor of the link
    ///
    /// # Returns
    /// The link
    pub fn anchor<T>(self, anchor: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            anchor: Some(anchor.into()),
            ..self
        }
    }
}

impl Display for LinkRel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rel=\"{}\"", self.0)
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.target)?;
        write!(f, "; {}", self.rel)?;
        if let Some(title) = &self.title {
            write!(f, "; title=\"{}\"", title)?;
        }
        if let Some(anchor) = &self.anchor {
            write!(f, "; anchor=\"{}\"", anchor)?;
        }
        Ok(())
    }
}

impl<'h> From<Link> for Header<'h> {
    fn from(link: Link) -> Self {
        Header::new("Link", format!("{}", link))
    }
}

impl<'h> From<Links> for Header<'h> {
    fn from(links: Links) -> Self {
        let output: Vec<String> = links.0.iter().map(|link| format!("{}", link)).collect();
        Header::new("Link", output.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use galvanic_assert::{assert_that, matchers::*};

    #[test]
    pub fn test_format_self_link() {
        let link = Link::new("/example", LinkRel::SELF);
        let formatted = format!("{}", link);

        assert_that!(&formatted, eq("</example>; rel=\"self\"".to_owned()));
    }

    #[test]
    pub fn test_format_custom_link() {
        let link = Link::new("/example", LinkRel("somethingElse"));
        let formatted = format!("{}", link);

        assert_that!(
            &formatted,
            eq("</example>; rel=\"somethingElse\"".to_owned())
        );
    }

    #[test]
    pub fn test_format_link_with_title() {
        let link = Link::new("/example", LinkRel::SELF).title("Some Title");
        let formatted = format!("{}", link);

        assert_that!(
            &formatted,
            eq("</example>; rel=\"self\"; title=\"Some Title\"".to_owned())
        );
    }

    #[test]
    pub fn test_format_link_with_anchor() {
        let link = Link::new("/example", LinkRel::SELF).anchor("#2");
        let formatted = format!("{}", link);

        assert_that!(
            &formatted,
            eq("</example>; rel=\"self\"; anchor=\"#2\"".to_owned())
        );
    }

    #[test]
    pub fn test_format_link_with_title_and_anchor() {
        let link = Link::new("/example", LinkRel::SELF)
            .anchor("#2")
            .title("Some Title");
        let formatted = format!("{}", link);

        assert_that!(
            &formatted,
            eq("</example>; rel=\"self\"; title=\"Some Title\"; anchor=\"#2\"".to_owned())
        );
    }

    #[test]
    pub fn test_build_single_link_header() {
        let link = Link::new("/example", LinkRel::SELF);
        let header: Header<'_> = link.into();
        let formatted = format!("{}", header);

        assert_that!(&formatted, eq("Link: </example>; rel=\"self\"".to_owned()));
    }

    #[test]
    pub fn test_build_multiple_link_header() {
        let links = Links(vec![
            Link::new("/example1", LinkRel::SELF),
            Link::new("/example2", LinkRel::SELF),
        ]);

        let header: Header<'_> = links.into();
        let formatted = format!("{}", header);

        assert_that!(
            &formatted,
            eq("Link: </example1>; rel=\"self\",</example2>; rel=\"self\"".to_owned())
        );
    }
}
