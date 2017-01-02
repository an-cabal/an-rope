use std::str;

use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

/// The index of a Unicode grapheme
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct GraphemeIndex(usize);

/// The index of a byte in a string
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct ByteIndex(usize);

/// The index of a Rust `char` in a string
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct CharIndex(usize);

impl GraphemeIndex {

    /// Returns the byte index in `text` corresponding to this grapheme index.
    ///
    /// # Arguments
    /// - `text`: the `&str` to find the byte index for this grapheme index in
    ///
    /// # Panics
    /// - If the index is to a grapheme that would lie outside of the length of
    ///   `text`.
    pub fn to_byte_index<'a>(&self, text: &'a str) -> ByteIndex {

        if text.grapheme_len() == self.0 {
            ByteIndex(text.len())
        } else {
            text.grapheme_indices(true)
                .find(|&(offset, _)| offset == self.0)
                .map( |(offset, _)| ByteIndex(offset))
                .expect(&format!( "grapheme index {} is greater than the length\
                                   of text {:?}"
                                 , self.0, text))
        }

    }

    /// Returns the Rust `char` index in `text` corresponding to this grapheme
    /// index.
    ///
    /// # Arguments
    /// - `text`: the `&str` to find the `char` index for this grapheme index in
    ///
    /// # Panics
    /// - If the index is to a grapheme that would lie outside of the length of
    ///   `text`.
    pub fn to_char_index<'a>(&self, text: &'a str) -> CharIndex  {

        text.graphemes(true)
            .scan(0usize, |char_count, grapheme| {
                    *char_count += grapheme.char_len();
                    Some(CharIndex(*char_count))
                })
            .nth(self.0)
            .expect(&format!( "grapheme index {} is greater than the length\
                               of text {:?}"
                             , self.0, text))
    }
}

impl CharIndex {
    fn to_byte_index<'a>(&self, into: &'a str) -> ByteIndex  {

        unimplemented!()
    }

    fn to_grapheme_index<'a>(&self, into: &'a str) -> GraphemeIndex {

        unimplemented!()
    }
}

impl ByteIndex {
    fn to_char_index<'a>(&self, into: &'a str) -> CharIndex {

        unimplemented!()
    }

    fn to_grapheme_index<'a>(&self, into: &'a str) -> GraphemeIndex  {

        unimplemented!()
    }
}

pub trait Unicode<'a> {
    // type GraphemeIndices: Iterator<Item = (usize, &'a str)>;
    // type CharIndices: Iterator<Item = (usize, char)>;
    // type ByteIndices: Iterator<Item = (usize, u8)>;
    //
    // fn grapheme_indices(&'a self) -> Self::GraphemeIndices;
    // fn char_indices(&'a self) -> Self::CharIndices;
    // fn byte_indices(&'a self) -> Self::ByteIndices;

    fn char_len(&self) -> usize;
    fn grapheme_len(&self) -> usize;
    fn byte_len(&self) -> usize;
}

impl<'a> Unicode<'a> for str {
    // type GraphemeIndices = USGraphemeIndices<'a>;
    // type CharIndices = str::CharIndices<'a>;
    // type ByteIndices = iter::Enumerate<str::Bytes<'a>>;
    //
    // #[inline]
    // fn grapheme_indices(&'a self) -> Self::GraphemeIndices {
    //     UnicodeSegmentation::grapheme_indices(self, true)
    // }
    //
    // #[inline]
    // fn char_indices(&'a self) -> Self::CharIndices { self.char_indices() }
    //
    // #[inline]
    // fn byte_indices(&'a self) -> Self::ByteIndices { self.bytes().enumerate() }

    #[inline]
    fn byte_len(&self) -> usize { self.len() }

    #[inline]
    fn char_len(&self) -> usize { self.chars().count() }

    #[inline]
    fn grapheme_len(&self) -> usize { self.graphemes(true).count() }
}
