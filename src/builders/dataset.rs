use crate::resource_clients::dataset::DatasetClient;
use std::marker::PhantomData;
use crate::error::ApifyClientError;
use crate::generic_types::{BaseBuilder, PaginationList};

#[derive(Debug)]
pub enum Format {
    Json,
    Jsonl,
    Xml,
    Html,
    Csv,
    Xlsx,
    Rss,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let string_repr = match self {
            Format::Json => "json",
            Format::Jsonl => "jsonl",
            Format::Xml => "xml",
            Format::Html => "html",
            Format::Csv => "csv",
            Format::Xlsx => "xlsx",
            Format::Rss => "ss",
        };
        write!(f, "{}", string_repr)
    }
}

#[derive(Default, QueryParams)]
#[allow(non_snake_case)]
pub struct GetItemsParams {
    format: Option<Format>,
    clean: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
    // Just string so QueryParams work, we parse it ourselves
    fields: Option<String>,
    // Just string so QueryParams work, we parse it ourselves
    omit: Option<String>,
    unwind: Option<String>,
    desc: Option<bool>, 
    attachment: Option<bool>,
    delimiter: Option<String>,
    bom: Option<bool>,
    xmlRoot: Option<String>,
    xmlRow: Option<String>,
    skipHeaderRow: Option<bool>,
    skipHidden: Option<bool>, 
    skipEmpty: Option<bool>, 
    simplified: Option<bool>,
    skipFailedPages: Option<bool>,
}

pub struct GetItemsBuilder<'a, T> {
    dataset_client: &'a DatasetClient<'a>,
    options: GetItemsParams,
    _phantom: PhantomData<T>,
}

impl <'a, T: serde::de::DeserializeOwned> GetItemsBuilder<'a, T> {
    pub fn new(dataset_client: &'a DatasetClient<'a>) -> Self {
        GetItemsBuilder {
            dataset_client,
            options: Default::default(),
            _phantom: PhantomData,
        }
    }

    pub async fn send(self) -> Result<PaginationList<T>, ApifyClientError> {
        let mut base_builder: BaseBuilder<'_, PaginationList<T>> = BaseBuilder::new(
            self.dataset_client.apify_client,
            self.dataset_client.url_segment.clone(),
            self.dataset_client.identifier.clone(),
            reqwest::Method::GET,
        );
        base_builder.query_string(self.options.to_query_params());
        Ok(base_builder.send().await?)
    }

    pub fn clean(& mut self, clean: bool) -> &'_ mut Self {
        self.options.clean = Some(clean);
        self
    }
    pub fn offset(& mut self, offset: u64) -> &'_ mut Self {
        self.options.offset = Some(offset);
        self
    }
    pub fn limit(& mut self, limit: u64) -> &'_ mut Self {
        self.options.limit = Some(limit);
        self
    }
    pub fn fields(& mut self, fields: Vec<String>) -> &'_ mut Self {
        self.options.fields = Some(fields.join(","));
        self
    }
    pub fn omit(& mut self, omit: Vec<String>) -> &'_ mut Self {
        self.options.omit = Some(omit.join(","));
        self
    }
    pub fn unwind(& mut self, unwind: String) -> &'_ mut Self {
        self.options.unwind = Some(unwind);
        self
    }
    pub fn desc(& mut self, desc: bool) -> &'_ mut Self {
        self.options.desc = Some(desc);
        self
    }
    pub fn attachment(& mut self, attachment: bool) -> &'_ mut Self {
        self.options.attachment = Some(attachment);
        self
    }
    pub fn delimiter(& mut self, delimiter: String) -> &'_ mut Self {
        self.options.delimiter = Some(delimiter);
        self
    }
    pub fn bom(& mut self, bom: bool) -> &'_ mut Self {
        self.options.bom = Some(bom);
        self
    }
    pub fn xml_root(& mut self, xml_root: String) -> &'_ mut Self {
        self.options.xmlRoot = Some(xml_root);
        self
    }
    pub fn xml_row(& mut self, xml_row: String) -> &'_ mut Self {
        self.options.xmlRow = Some(xml_row);
        self
    }
    pub fn skip_header_row(& mut self, skip_header_row: bool) -> &'_ mut Self {
        self.options.skipHeaderRow = Some(skip_header_row);
        self
    }
    pub fn skip_hidden(& mut self, skip_hidden: bool) -> &'_ mut Self {
        self.options.skipHidden = Some(skip_hidden);
        self
    }
    pub fn skip_empty(& mut self, skip_empty: bool) -> &'_ mut Self {
        self.options.skipEmpty = Some(skip_empty);
        self
    }
    pub fn simplified(& mut self, simplified: bool) -> &'_ mut Self {
        self.options.simplified = Some(simplified);
        self
    }
    pub fn skip_failed_pages(& mut self, skip_failed_pages: bool) -> &'_ mut Self {
        self.options.skipFailedPages = Some(skip_failed_pages);
        self
    }
}