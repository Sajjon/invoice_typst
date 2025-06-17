use crate::prelude::*;

use chrono::FixedOffset;
use typst::{
    Library, World,
    foundations::{Bytes, Datetime},
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};

/// A typst context that contains the necessary
/// sources and environment to render an invoice.
#[derive(Debug, Getters)]
pub struct TypstContext {
    /// The typst source files used to render the invoices layout and data.
    #[getset(get = "pub")]
    content: Content,

    /// The environment containing the library, font book, and current time.
    #[getset(get = "pub")]
    environment: Environment,
}

impl TypstContext {
    fn new(layout: Source, l18n: Source, data: Source) -> Self {
        let content = Content::builder()
            .data(data)
            .layout(layout)
            .l18n(l18n)
            .build();
        let environment = Environment::default();

        Self {
            content,
            environment,
        }
    }

    pub fn with_path(
        layout_path: impl AsRef<Path>,
        l18n_inline: String,
        data_inline: String,
    ) -> Result<Self> {
        Ok(Self::new(
            Source::load_source_at(layout_path)?,
            // Virtual path MUST match the first lines inside `invoice.typ` file.
            Source::inline(l18n_inline, "/crates/render/src/l18n.typ")?,
            // Virtual path MUST match the first lines inside `invoice.typ` file.
            Source::inline(data_inline, "/crates/render/src/input.typ")?,
        ))
    }
}

impl World for TypstContext {
    fn library(&self) -> &LazyHash<Library> {
        self.environment().library()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        self.environment().book()
    }

    fn main(&self) -> FileId {
        self.content().layout().id()
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == self.content().layout().id() {
            let source = self.content().layout().clone();
            Ok(source)
        } else if id == self.content.l18n().id() {
            let source = self.content().l18n().clone();
            Ok(source)
        } else if id == self.content.data().id() {
            let source = self.content().data().clone();
            Ok(source)
        } else {
            panic!("Unknown typst resource requested: '{:?}'", id);
        }
    }

    fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
        panic!("File access not implemented in this minimal example")
    }

    fn font(&self, index: usize) -> Option<Font> {
        if let Some(font) = self.environment().fonts().get(index)?.get() {
            trace!("Using font @{} => {:?}", index, font.info().family);
            Some(font)
        } else {
            panic!("Font not found")
        }
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = self.environment().now();
        let with_offset = match offset {
            None => now.with_timezone(now.offset()).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                let fixed = FixedOffset::east_opt(seconds)?;
                now.with_timezone(&fixed)
            }
        };

        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
}
