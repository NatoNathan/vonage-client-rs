use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct PageMeta {
    page_size: Option<usize>,
    _links: Links,
}
#[derive(Debug, Deserialize)]
pub struct Links {
    first: Option<Link>,
    #[serde(rename = "self")]
    me: Option<Link>,
    next: Option<Link>,
    prev: Option<Link>,
}
#[derive(Debug, Deserialize)]
pub struct Link {
    href: String,
}