//! https://docs.rs/symbol/0.1.9/src/symbol/lib.rs.html#84-86

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::Hash;
use std::mem::{forget, transmute};
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::Mutex;

static mut SYMBOL_HEAP: Mutex<BTreeSet<&'static str>> = Mutex::new(BTreeSet::new());

/// An interned string with O(1) equality.
#[derive(Clone, Copy, Eq)]
pub struct Symbol {
    s: &'static str,
}

impl Symbol {
    /// Retrieves the address of the backing string.
    #[inline(always)]
    pub fn addr(self) -> usize {
        self.s.as_ptr() as usize
    }

    /// Retrieves the string from the Symbol.
    pub fn as_str(self) -> &'static str {
        self.s
    }

    /// Generates a new symbol with a name of the form `G#n`, where `n` is some positive integer.
    pub fn gensym() -> Symbol {
        static mut N: AtomicUsize = AtomicUsize::new(0);

        unsafe {
            if let Ok(mut heap) = SYMBOL_HEAP.lock() {
                let n = loop {
                    let n = leak_string(format!("G#{}", N.fetch_add(1, AtomicOrdering::SeqCst)));
                    if heap.insert(n) {
                        break n;
                    }
                };
                Symbol::from(n)
            } else {
                unreachable!("failed to lock symbol heap")
            }
        }
    }
}

impl Debug for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Debug::fmt(self.s, fmt)
    }
}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &str {
        self.s
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.write_str(self.s)
    }
}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.addr().hash(state)
    }
}

impl<S: AsRef<str>> From<S> for Symbol {
    fn from(s: S) -> Symbol {
        let s = s.as_ref();
        {
            if let Ok(mut heap) = unsafe { SYMBOL_HEAP.lock() } {
                match heap.get(s) {
                    Some(s) => Symbol { s },
                    None => {
                        let s = leak_string(s.to_owned());
                        heap.insert(s);
                        return Symbol { s };
                    }
                }
            } else {
                unreachable!("failed to lock symbol heap")
            }
        }
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        let l = self.addr();
        let r = other.addr();
        l.cmp(&r)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S: AsRef<str>> PartialEq<S> for Symbol {
    fn eq(&self, other: &S) -> bool {
        self.partial_cmp(&other.as_ref()) == Some(Ordering::Equal)
    }
}

impl<S: AsRef<str>> PartialOrd<S> for Symbol {
    fn partial_cmp(&self, other: &S) -> Option<Ordering> {
        self.s.partial_cmp(other.as_ref())
    }
}

fn leak_string(s: String) -> &'static str {
    let out = unsafe { transmute(&s as &str) };
    forget(s);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        let s1 = Symbol::from("foo");
        let s2 = Symbol::from("foo");

        assert_eq!(s1, s2);

        let s3 = Symbol::from("bar".to_string());
        let s4 = Symbol::from(String::from("bar"));

        assert_eq!(s3, s4);

        let s5 = Symbol::from("ba".to_string() + "z");
        let s6 = Symbol::from("baz");

        assert_eq!(s5, s6);

        let s7 = Symbol::from("qux");
        let s8 = Symbol::from(s7);

        assert_eq!(s7, s8);
    }
}
