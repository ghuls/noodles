//! Indexed VCF reader.

mod builder;

pub use self::builder::Builder;

use std::io::{self, Read, Seek};

use noodles_bgzf as bgzf;
use noodles_core::Region;
use noodles_csi::BinningIndex;

use super::{
    reader::{Query, Records},
    Reader,
};
use crate::{variant::RecordBuf, Header, Record};

/// An indexed VCF reader.
pub struct IndexedReader<R> {
    inner: Reader<bgzf::Reader<R>>,
    index: Box<dyn BinningIndex>,
}

impl<R> IndexedReader<R>
where
    R: Read,
{
    /// Creates an indexed VCF reader.
    pub fn new<I>(inner: R, index: I) -> Self
    where
        I: BinningIndex + 'static,
    {
        Self {
            inner: Reader::new(bgzf::Reader::new(inner)),
            index: Box::new(index),
        }
    }

    /// Returns a reference to the underlying reader.
    pub fn get_ref(&self) -> &bgzf::Reader<R> {
        self.inner.get_ref()
    }

    /// Returns a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut bgzf::Reader<R> {
        self.inner.get_mut()
    }

    /// Returns the underlying reader.
    pub fn into_inner(self) -> bgzf::Reader<R> {
        self.inner.into_inner()
    }

    /// Reads the VCF header.
    pub fn read_header(&mut self) -> io::Result<Header> {
        self.inner.read_header()
    }

    /// Reads a single raw VCF record.
    pub fn read_record(&mut self, header: &Header, record: &mut RecordBuf) -> io::Result<usize> {
        self.inner.read_record(header, record)
    }

    /// Returns an iterator over records starting from the current stream position.
    pub fn records<'r, 'h: 'r>(
        &'r mut self,
        header: &'h Header,
    ) -> Records<'r, 'h, bgzf::Reader<R>> {
        self.inner.records(header)
    }

    /// Reads a single record without eagerly parsing its fields.
    pub fn read_lazy_record(&mut self, record: &mut Record) -> io::Result<usize> {
        self.inner.read_lazy_record(record)
    }

    /// Returns the associated index.
    pub fn index(&self) -> &dyn BinningIndex {
        &self.index
    }
}

impl<R> IndexedReader<R>
where
    R: Read + Seek,
{
    /// Returns an iterator over records that intersects the given region.
    pub fn query<'r, 'h>(
        &'r mut self,
        header: &'h Header,
        region: &Region,
    ) -> io::Result<Query<'r, 'h, R>> {
        self.inner.query(header, &self.index, region)
    }
}
