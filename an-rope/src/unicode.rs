use std::convert::Into;
use unicode_segmentation::UnicodeSegmentation;

pub struct GraphemeIndex<'a> { index: usize
                             , into_str: &'a str
                             }
pub struct ByteIndex<'a> { index: usize
                         , into_str: &'a str
                         }
pub struct CharIndex<'a> { index: usize
                         , into_str: &'a str
                         }

impl<'a> Into<ByteIndex<'a>> for GraphemeIndex<'a> {
    // TODO is this reasonable?
    fn into(self) -> ByteIndex<'a> {
        self.into_str.grapheme_indices(true)
            .find(|&(i, _)| i == self.index)
            .map(|(i, _)| ByteIndex { index: i, into_str: self.into_str})
            .expect(&format!( "Grapheme index {} off the end of string {}!"
                             , self.index
                             , self.into_str))
    }
}

impl<'a> Into<CharIndex<'a>> for GraphemeIndex<'a> {
    fn into(self) -> CharIndex<'a> {
        unimplemented!()
    }
}
