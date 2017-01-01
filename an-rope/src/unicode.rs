pub struct GraphemeIndex(usize);
pub struct ByteIndex(usize);
pub struct CharIndex(usize);


#[cfg(feature = "unstable")]
trait UnicodeIndexing {
    fn graphemes<'a>(&'a self) -> impl Iterator<Item = &'a str> + 'a;
    fn grapheme_indices<'a>(&'a self)
                            -> impl Iterator<Item = (usize, &'a str)> + 'a;

    #[inline]
    fn grapheme_count(&self) -> usize {
        self.graphemes().count()
    }

    /// Returns the byte index corresponding to a given grapheme index
    /// or none, if it is off the end of the string.
    // TODO: newtypes for usize
    fn grapheme_to_byte_index(&self, idx: GraphemeIndex) -> Option<ByteIndex> {
        self.grapheme_indices()
            .find(|(i, _)| i == idx.0)
            .map(ByteIndex)
        // TODO: what about the case where g_idx is the last grapheme
    }

    fn grapheme_to_char_index(&self, idx: GraphemeIndex) -> Option<CharIndex> {
        unimplemented!()
    }

    fn char_to_grapheme_index(&self, idx: CharIndex) -> Option<GraphemeIndex> {
        unimplemented!()
    }

    fn char_to_byte_index(&self, idx: CharIndex) -> Option<ByteIndex> {
        unimplemented!()
    }

    fn byte_to_char_index(&self, idx: ByteIndex) -> Option<CharIndex> {
        unimplemented!()
    }

    fn byte_to_grapheme_index(&self, idx: ByteIndex) -> Option<GraphemeIndex> {
        unimplemented!()
    }

}
