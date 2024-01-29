//! VCF header format key.

mod v4_3;
mod v4_4;

use crate::header::{record::value::map::format::Type, FileFormat, Number};

pub(crate) fn definition(
    file_format: FileFormat,
    key: &str,
) -> Option<(Number, Type, &'static str)> {
    match (file_format.major(), file_format.minor()) {
        (4, 4) => v4_4::definition(key),
        (4, 3) => v4_3::definition(key),
        _ => None,
    }
}
