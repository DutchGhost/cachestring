#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod cachebuf;
use cachebuf::CacheBuf;

use core::{
    fmt::{self, Debug, Display},
    hash::{Hash, Hasher},
    iter::{Extend, FromIterator},
    ops::{Deref, DerefMut, Index, RangeFull},
};

#[derive(Clone)]
pub struct CacheString(CacheBuf);

impl CacheString {
    /// Creates a new `CacheString`, capable of holding 63 bytes.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(CacheBuf::new())
    }

    /// Returns the length of the `CacheString`
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns the capacity of the `CacheString`.
    /// This will be 63.
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline(always)]
    pub const fn remaining_capacity(&self) -> usize {
        self.0.remaining_capacity()
    }

    pub const fn is_full(&self) -> bool {
        self.0.is_full()
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_str(&self) -> &str {
        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Pushes `s` onto `self`
    pub fn push_str(&mut self, s: &str) {
        self.0.extend_from_slice(s.as_bytes())
    }

    /// Pushes `ch` onto `self`
    pub fn push(&mut self, ch: char) {
        match ch.len_utf8() {
            1 => self.0.push(ch as u8),
            _ => self
                .0
                .extend_from_slice(ch.encode_utf8(&mut [0; 4]).as_bytes()),
        }
    }

    pub fn truncate(&mut self, new_len: usize) {
        if new_len <= self.len() {
            assert!(self.is_char_boundary(new_len));
            self.0.truncate(new_len);
        }
    }

    pub fn clear(&mut self) {
        self.truncate(0)
    }
}

impl Default for CacheString {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for CacheString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl Display for CacheString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl Hash for CacheString {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (**self).hash(hasher)
    }
}

impl Eq for CacheString {}

impl PartialEq for CacheString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self[..] == other[..]
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        self[..] != other[..]
    }
}

impl PartialEq<str> for CacheString {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self[..] == other[..]
    }

    #[inline(always)]
    fn ne(&self, other: &str) -> bool {
        self[..] != other[..]
    }
}

impl<'a> PartialEq<&'a str> for CacheString {
    #[inline(always)]
    fn eq(&self, other: &&'a str) -> bool {
        self[..] == other[..]
    }

    #[inline(always)]
    fn ne(&self, other: &&'a str) -> bool {
        self[..] != other[..]
    }
}

impl Ord for CacheString {
    #[inline]
    fn cmp(&self, other: &CacheString) -> core::cmp::Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl PartialOrd for CacheString {
    #[inline]
    fn partial_cmp(&self, other: &CacheString) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for CacheString {
    fn from(s: &str) -> Self {
        let mut buf = Self::new();
        buf.push_str(s);
        buf
    }
}

impl Deref for CacheString {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl DerefMut for CacheString {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::str::from_utf8_unchecked_mut(self.as_bytes_mut()) }
    }
}

impl Extend<char> for CacheString {
    fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        iterator.for_each(|c| self.push(c));
    }
}

impl<'a> Extend<&'a char> for CacheString {
    fn extend<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned())
    }
}

impl<'a> Extend<&'a str> for CacheString {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        iter.into_iter().for_each(|s| self.push_str(s))
    }
}

impl FromIterator<char> for CacheString {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut buf = CacheString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a char> for CacheString {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        let mut buf = CacheString::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> FromIterator<&'a str> for CacheString {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut buf = CacheString::new();
        buf.extend(iter);
        buf
    }
}

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::CacheString;

    use core::iter::FromIterator;

    use alloc::{borrow::Cow, string::String};

    impl From<String> for CacheString {
        fn from(s: String) -> CacheString {
            CacheString::from(&*s)
        }
    }

    impl<'a> From<Cow<'a, str>> for CacheString {
        fn from(s: Cow<'a, str>) -> CacheString {
            CacheString::from(&*s)
        }
    }

    impl PartialEq<String> for CacheString {
        #[inline(always)]
        fn eq(&self, other: &String) -> bool {
            self.eq(&**other)
        }

        #[inline(always)]
        fn ne(&self, other: &String) -> bool {
            self.ne(&**other)
        }
    }

    impl<'a> PartialEq<Cow<'a, str>> for CacheString {
        #[inline(always)]
        fn eq(&self, other: &Cow<'a, str>) -> bool {
            self.eq(&**other)
        }

        #[inline(always)]
        fn ne(&self, other: &Cow<'a, str>) -> bool {
            self.ne(&**other)
        }
    }

    impl Extend<String> for CacheString {
        fn extend<I: IntoIterator<Item = String>>(&mut self, iter: I) {
            iter.into_iter().for_each(|s| self.push_str(&s))
        }
    }

    impl<'a> Extend<Cow<'a, str>> for CacheString {
        fn extend<I: IntoIterator<Item = Cow<'a, str>>>(&mut self, iter: I) {
            iter.into_iter().for_each(|s| self.push_str(&s))
        }
    }

    impl FromIterator<String> for CacheString {
        fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
            let mut buf = CacheString::new();
            buf.extend(iter);
            buf
        }
    }

    impl<'a> FromIterator<Cow<'a, str>> for CacheString {
        fn from_iter<I: IntoIterator<Item = Cow<'a, str>>>(iter: I) -> Self {
            let mut buf = CacheString::new();
            buf.extend(iter);
            buf
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cachestr_new() {
        let mut c = CacheString::new();
        c.push_str("Hello, sailor!");
        assert_eq!(c, "Hello, sailor!");
    }

    #[test]
    fn cache_str_len() {
        let mut c = CacheString::from("Hello!");
        assert_eq!(c.len(), 6);
        assert_eq!(c.remaining_capacity(), 63 - 6);

        c.clear();
        assert_eq!(c.len(), 0);
        assert!(c.is_empty());
    }

    #[test]
    fn cache_str_push() {
        let mut c = CacheString::from("Hello, world");
        c.push('!');
        assert_eq!(c, "Hello, world!");
    }

    #[test]
    fn cache_str_truncate() {
        let mut c = CacheString::from("Hello, world!");
        assert_eq!(c, "Hello, world!");
        c.truncate(1);
        assert_eq!(c, "H");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_string() {
        let mut c = CacheString::from(alloc::string::String::from("Hello world!"));
        assert_eq!(c, "Hello world!");
    }
}
