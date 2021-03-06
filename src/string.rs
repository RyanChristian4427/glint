use std::ops::Range;
use unic_segment::{Graphemes, WordBounds};

pub fn len(s: &str) -> usize {
    Graphemes::new(s).count()
}

pub fn to_byte_offset(s: &'_ str, grapheme_offset: usize) -> usize {
    let mut byte_offset = 0;

    for item in Graphemes::new(s).take(grapheme_offset) {
        byte_offset += item.len();
    }

    byte_offset
}

pub fn to_byte_range(s: &'_ str, grapheme_offset: usize) -> Range<usize> {
    let mut start = 0;
    let mut end = 0;

    for item in Graphemes::new(s).take(grapheme_offset + 1) {
        start = end;
        end = start + item.len();
    }

    start..end
}

pub fn to_byte_offset_end(s: &'_ str, grapheme_offset: usize) -> usize {
    let mut byte_offset = 0;

    for item in Graphemes::new(s).take(grapheme_offset) {
        byte_offset += item.len();
    }

    byte_offset
}

// pub fn get_at(s: &'_ str, grapheme_offset: usize) -> Option<&str> {
//     Graphemes::new(s).nth(grapheme_offset)
// }

pub fn split_at(s: &str, grapheme_offset: usize) -> (&str, &str) {
    let mut byte_offset = 0;

    for item in Graphemes::new(s).take(grapheme_offset) {
        byte_offset += item.len();
    }

    (&s[0..byte_offset], &s[byte_offset..])
}

pub fn prev_word_grapheme(s: &str, current_offset: usize) -> usize {
    let mut grapheme_offset = 0;

    for word in WordBounds::new(s) {
        let next = grapheme_offset + len(word);
        if next >= current_offset {
            break;
        }

        grapheme_offset = next;
    }

    grapheme_offset
}

pub fn next_word_grapheme(s: &str, current_offset: usize) -> usize {
    let mut grapheme_offset = 0;

    for word in WordBounds::new(s) {
        let next = grapheme_offset + len(word);

        grapheme_offset = next;

        if next > current_offset {
            break;
        }
    }

    grapheme_offset
}
