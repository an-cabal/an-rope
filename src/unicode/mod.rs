use std::str;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod test;

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
