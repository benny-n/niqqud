//! Lightweight library for removing hebrew diacritics ("Niqqud", Hebrew: נִקּוּד) from a string.
//!
//! reference: https://www.unicode.org/charts/PDF/U0590.pdf
//!
//! Examples
//! --------
//! ```
//! let diacrited_quoted = "״שָׁלוֹם עוֹלָם״";
//!
//! // Remove only the diacritics (keep '״' chars)
//! let quoted = niqqud::remove(diacrited_quoted);
//! assert_eq!("״שלום עולם״", quoted);
//!
//! // Remove diacritics and hebrew quotes (double '״' and single '׳')
//! let unquoted = niqqud::remove_thorough(diacrited_quoted);
//! assert_eq!("שלום עולם", unquoted);
//! ```

use std::borrow::Cow;
/// ```
///
/// let word = niqqud::remove("נִקּוּד");
/// assert_eq!("נקוד", word);
/// ```
///
pub fn remove<'s>(string: &'s str) -> Cow<'s, str> {
    string.chars().filter(|&c| !is_diacritic(c)).collect()
}
/// ```
///
/// let word = niqqud::remove_thorough("״גֵּרְשַׁיִם״");
/// assert_eq!("גרשים", word);
/// ```
///
pub fn remove_thorough<'s>(string: &'s str) -> Cow<'s, str> {
    string
        .chars()
        .filter(|&c| !is_diacritic(c) && !is_special(c))
        .collect()
}

/// Returns true if the character is a diacritic
fn is_diacritic(c: char) -> bool {
    matches!(c, '\u{0590}'..='\u{05CF}')
}

/// Returns true if the character is a special (HEB) character, such as hebrew quotes '״' ("Gershayim", Hebrew: גֵּרְשַׁיִם)
fn is_special(c: char) -> bool {
    matches!(c, '\u{05EB}'..='\u{05FF}')
}

#[cfg(test)]
mod tests {
    use crate::remove;

    #[test]
    fn test_normal_remove() {
        let string = remove("שָׁלוֹם עוֹלָם");

        assert_eq!("שלום עולם", string);
    }
}
