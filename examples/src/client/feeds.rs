use anyhow::{Result, bail};
use std::collections::HashSet;
use std::slice::Iter;

pub struct Feeds {
    feeds: Vec<u32>,
}

impl Feeds {
    /// # Errors
    /// Returns an error if any argument is invalid.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(arg: &str) -> Result<Self> {
        let args = arg
            .split(' ')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() { None } else { Some(s) }
            })
            .collect::<Vec<_>>();
        Self::from_strs(&args)
    }

    /// Pass `Vec<String>` as `.iter().map(String::as_ref).collect()`.
    /// # Errors
    /// Returns an error if any argument is invalid.
    pub fn from_strs(args: &[&str]) -> Result<Self> {
        fn parse_number(a: &str) -> Option<u32> {
            a.parse::<u32>().ok()
        }

        fn parse_range(a: &str) -> Option<(u32, u32)> {
            let mut parts = a.splitn(2, '-');
            let start = parse_number(parts.next()?)?;
            let end = parse_number(parts.next()?)?;
            Some((start, end))
        }

        fn parse_and_push(map: &mut HashSet<u32>, arg: &str) -> Result<()> {
            if let Some(feed_id) = parse_number(arg) {
                map.insert(feed_id);
            } else if let Some((start, end)) = parse_range(arg) {
                for i in start..=end {
                    map.insert(i);
                }
            } else {
                bail!("Invalid argument: {arg}");
            }
            Ok(())
        }

        let mut ids_set: HashSet<u32> = HashSet::new();
        let mut ids_ignore_set: HashSet<u32> = HashSet::new();

        for arg in args {
            if (*arg).starts_with('!') {
                let arg = &arg[1..];
                parse_and_push(&mut ids_ignore_set, arg)?;
            } else {
                parse_and_push(&mut ids_set, arg)?;
            }
        }

        let mut feeds = Vec::with_capacity(ids_set.len());
        for feed_id in ids_set {
            if ids_ignore_set.contains(&feed_id) {
                continue;
            }
            feeds.push(feed_id);
        }
        feeds.sort_unstable();

        Ok(Self { feeds })
    }

    pub fn iter(&self) -> Iter<'_, u32> {
        self.feeds.iter()
    }
}

impl IntoIterator for Feeds {
    type Item = u32;
    type IntoIter = <Vec<u32> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.feeds.into_iter()
    }
}

impl<'a> IntoIterator for &'a Feeds {
    type Item = &'a u32;
    type IntoIter = Iter<'a, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.feeds.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_feeds() {
        let f = Feeds::from_strs(&[]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn some_feeds() {
        let f = Feeds::from_strs(&["7", "13"]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn feeds_range() {
        let f = Feeds::from_strs(&["7-9"]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn feed_ranges() {
        let f = Feeds::from_strs(&["1-3", "7-8"]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn bad_feeds() {
        assert!(Feeds::from_strs(&["0", "abc", "3-1"]).is_err());
    }

    #[test]
    fn ignore_feed() {
        let f = Feeds::from_strs(&["1-3", "!2"]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn ignore_feeds_range() {
        let f = Feeds::from_strs(&["1-5", "!2-3"]).unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn init_from_str() {
        let f = Feeds::from_str("1-5 !2 !4").unwrap();
        let mut iter = f.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
}
