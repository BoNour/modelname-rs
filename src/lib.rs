//! Parse AI model IDs and display names into parts you can compare.
//!
//! Every lab names things a little differently. Anthropic used to write
//! `claude-3-5-sonnet` and now writes `claude-sonnet-4-5`. OpenAI uses dots,
//! OpenRouter puts the vendor in front, Mistral and DeepSeek tack a date code
//! on the end. [`parse`] reads all of those and gives back the same shape, so
//! `claude-3-5-sonnet-20241022` and `Claude Sonnet 4.5` both land in the
//! `claude-sonnet` family.
//!
//! ```
//! let m = modelname::parse("anthropic/claude-sonnet-4-5");
//! assert_eq!(m.vendor.as_deref(), Some("anthropic"));
//! assert_eq!(m.family, "claude-sonnet");
//! assert_eq!(m.version.as_deref(), Some("4.5"));
//! ```
//!
//! It's heuristic, not a registry. Most of the test cases are names listed at
//! <https://whizi.io/models/>, which is a handy place to see what's out there
//! right now.
//!
//! This is a port of the Go package of the same name,
//! [github.com/BoNour/modelname](https://github.com/BoNour/modelname).

use std::cmp::Ordering;
use std::fmt;

mod vendor;

/// A parsed model name. Anything that wasn't in the name is `None` or empty.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Model {
    /// The input, unchanged.
    pub id: String,
    /// `"anthropic"`, `"openai"`, ...
    pub vendor: Option<String>,
    /// Name words joined with `-`, e.g. `"gemini-flash-lite"`.
    pub family: String,
    /// `"4.5"`, `"4o"`, `"k2.7"`.
    pub version: Option<String>,
    /// Parameter count if given: `"70b"`, `"35b-a3b"`.
    pub size: Option<String>,
    /// Only set for full dates like `20241022`.
    pub date: Option<Date>,
    /// Short date codes like `"0731"` or `"2512"`.
    pub snapshot: Option<String>,
    /// `"preview"`, `"latest"`, `"thinking"`, ...
    pub tags: Vec<String>,
}

/// A calendar date pulled out of a model ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    fn from_digits(s: &str) -> Option<Date> {
        if s.len() != 8 || !s.starts_with("20") {
            return None;
        }
        let year = s[0..4].parse().ok()?;
        let month = s[4..6].parse().ok()?;
        let day = s[6..8].parse().ok()?;
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return None;
        }
        Some(Date { year, month, day })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

const TAGS: &[&str] = &[
    "preview",
    "latest",
    "exp",
    "experimental",
    "instruct",
    "thinking",
    "beta",
    "free",
];

/// Splits a model ID or display name into its parts. It never fails; anything
/// it doesn't recognise ends up in `family`.
pub fn parse(id: &str) -> Model {
    let mut m = Model {
        id: id.to_string(),
        ..Model::default()
    };
    let mut s = id.trim().to_lowercase();

    // OpenRouter style suffixes, e.g. ":free" or ":thinking".
    if let Some(i) = s.find(':') {
        m.tags.extend(
            s[i + 1..]
                .split(':')
                .filter(|t| !t.is_empty())
                .map(String::from),
        );
        s.truncate(i);
    }

    let mut prefix = None;
    if let Some(i) = s.rfind('/') {
        prefix = Some(s[..i].to_string());
        s = s[i + 1..].to_string();
    }

    let toks: Vec<&str> = s
        .split(['-', '_', ' ', '(', ')'])
        .map(|t| t.trim_matches(['.', ',']))
        .filter(|t| !t.is_empty())
        .collect();

    let mut words: Vec<String> = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i];
        if i + 2 < toks.len()
            && is_year(t)
            && is_digits(toks[i + 1], &[2])
            && is_digits(toks[i + 2], &[2])
        {
            // 2024-08-06
            match Date::from_digits(&format!("{}{}{}", t, toks[i + 1], toks[i + 2])) {
                Some(d) => {
                    m.date = Some(d);
                    i += 2;
                }
                None => m.snapshot = Some(t.to_string()),
            }
        } else if is_digits(t, &[8]) {
            match Date::from_digits(t) {
                Some(d) => m.date = Some(d),
                None => m.snapshot = Some(t.to_string()),
            }
        } else if is_size(t) {
            let mut size = t.to_string();
            if i + 1 < toks.len() && is_active(toks[i + 1]) {
                size = format!("{}-{}", size, toks[i + 1]);
                i += 1;
            }
            m.size = Some(size);
        } else if TAGS.contains(&t) {
            m.tags.push(t.to_string());
        } else if m.version.is_none() && starts_with_digit(t) && !is_long_number(t) {
            m.version = Some(t.to_string());
            i = absorb(&mut m, &toks, i);
        } else if m.version.is_none() && i == 0 && split_glued(t).is_some() {
            // qwen3.5, hy3
            let v = split_glued(t).unwrap();
            words.push(t[..t.len() - v.len()].to_string());
            m.version = Some(v.to_string());
            i = absorb(&mut m, &toks, i);
        } else if m.version.is_none() && i > 0 && is_letter_version(t) {
            // v3.2, k2, m2, mk1
            let v = match t.strip_prefix('v') {
                Some(rest) if starts_with_digit(rest) => rest,
                _ => t,
            };
            m.version = Some(v.to_string());
            i = absorb(&mut m, &toks, i);
        } else if is_long_number(t) {
            m.snapshot = Some(t.to_string());
        } else if m.version.is_some() && is_digits(t, &[1, 2]) {
            // stray numbers after the name, like the 03-25 in gemini-2.5-pro-exp-03-25
            match &mut m.snapshot {
                Some(s) => {
                    s.push('-');
                    s.push_str(t);
                }
                None => m.snapshot = Some(t.to_string()),
            }
        } else {
            words.push(t.to_string());
        }
        i += 1;
    }

    m.family = words.join("-");
    m.vendor = vendor::lookup(prefix.as_deref(), words.first().map(String::as_str));
    m
}

impl std::str::FromStr for Model {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse(s))
    }
}

/// Orders two models from the same family, oldest first.
///
/// Version wins, then the full date, then the short date code. Comparing
/// across families works but doesn't mean much: `gpt-5` vs
/// `claude-sonnet-4-5` just compares 5 with 4.5.
pub fn compare(a: &Model, b: &Model) -> Ordering {
    natural(
        a.version.as_deref().unwrap_or(""),
        b.version.as_deref().unwrap_or(""),
    )
    .then(a.date.cmp(&b.date))
    .then_with(|| {
        natural(
            a.snapshot.as_deref().unwrap_or(""),
            b.snapshot.as_deref().unwrap_or(""),
        )
    })
}

/// Returns the newest ID in `ids` whose family matches. Handy for picking a
/// default out of a provider's `/models` list.
pub fn latest<'a, I>(ids: I, family: &str) -> Option<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    ids.into_iter()
        .map(|id| (id, parse(id)))
        .filter(|(_, m)| m.family == family)
        .max_by(|(_, a), (_, b)| compare(a, b))
        .map(|(id, _)| id)
}

// Joins dash-separated version parts, so claude-opus-4-1 reads as 4.1.
fn absorb(m: &mut Model, toks: &[&str], mut i: usize) -> usize {
    while i + 1 < toks.len() && is_digits(toks[i + 1], &[1, 2]) {
        let v = m.version.get_or_insert_with(String::new);
        v.push('.');
        v.push_str(toks[i + 1]);
        i += 1;
    }
    i
}

fn is_digits(t: &str, lengths: &[usize]) -> bool {
    !t.is_empty() && t.bytes().all(|c| c.is_ascii_digit()) && lengths.contains(&t.len())
}

fn is_year(t: &str) -> bool {
    is_digits(t, &[4]) && t.starts_with("20")
}

// Three or more plain digits is a date code, not a version.
fn is_long_number(t: &str) -> bool {
    is_digits(t, &[3, 4, 5, 6, 7])
}

fn starts_with_digit(t: &str) -> bool {
    t.bytes().next().is_some_and(|c| c.is_ascii_digit())
}

// 8b, 70b, 2.4t, 135m
fn is_size(t: &str) -> bool {
    match t.strip_suffix(['b', 'm', 't']) {
        Some(n) => is_number(n),
        None => false,
    }
}

// The active-parameter half of a MoE size, like a3b.
fn is_active(t: &str) -> bool {
    t.strip_prefix('a')
        .and_then(|r| r.strip_suffix('b'))
        .is_some_and(is_number)
}

fn is_number(t: &str) -> bool {
    !t.is_empty()
        && !t.starts_with('.')
        && !t.ends_with('.')
        && t.bytes().all(|c| c.is_ascii_digit() || c == b'.')
        && t.matches('.').count() <= 1
}

fn is_version(t: &str) -> bool {
    !t.is_empty() && !t.ends_with('.') && t.bytes().all(|c| c.is_ascii_digit() || c == b'.')
}

// The trailing version of a name like qwen3.5 or hy3. Single letters are
// left alone so o3 and r1 stay names.
fn split_glued(t: &str) -> Option<&str> {
    let i = t.find(|c: char| c.is_ascii_digit())?;
    if i < 2 || !t[..i].bytes().all(|c| c.is_ascii_lowercase()) || !is_version(&t[i..]) {
        return None;
    }
    Some(&t[i..])
}

// v2.5, k2, m2, n2, mk1
fn is_letter_version(t: &str) -> bool {
    match t.find(|c: char| c.is_ascii_digit()) {
        Some(i) if (1..=2).contains(&i) => {
            t[..i].bytes().all(|c| c.is_ascii_lowercase()) && is_version(&t[i..])
        }
        _ => false,
    }
}

// Compares strings with runs of digits compared as numbers, so "3.10" sorts
// after "3.9" and "k3" after "k2.7".
fn natural(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a, b);
    while !a.is_empty() && !b.is_empty() {
        let (ca, ra) = chunk(a);
        let (cb, rb) = chunk(b);
        let ord = if starts_with_digit(ca) && starts_with_digit(cb) {
            let (x, y) = (ca.trim_start_matches('0'), cb.trim_start_matches('0'));
            x.len().cmp(&y.len()).then(x.cmp(y))
        } else {
            ca.cmp(cb)
        };
        if ord != Ordering::Equal {
            return ord;
        }
        a = ra;
        b = rb;
    }
    a.len().cmp(&b.len())
}

fn chunk(s: &str) -> (&str, &str) {
    let digit = s.as_bytes()[0].is_ascii_digit();
    let end = s
        .bytes()
        .position(|c| c.is_ascii_digit() != digit)
        .unwrap_or(s.len());
    s.split_at(end)
}
