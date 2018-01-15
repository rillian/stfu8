/* Copyright (c) 2018 Garrett Berg, vitiral@gmail.com
 *
 * Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
 * http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
 * http://opensource.org/licenses/MIT>, at your option. This file may not be
 * copied, modified, or distributed except according to those terms.
 */

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod decode;
mod encode;

pub use decode::{decode, DecodeError};

/// Encode text as stfu8, escaping all non-printable characters.
///
/// # Examples
/// ```rust
/// # extern crate stfu8;
///
/// # fn main() {
/// let encoded = stfu8::encode(b"foo\xFF\nbar");
/// assert_eq!(
///     encoded,
///     r"foo\xFF\nbar" // notice the `r` == raw string
/// );
/// # }
/// ```
pub fn encode(v: &[u8]) -> String {
    let encoder = Encoder::new();
    encode::encode(&encoder, v)
}

/// Decode stfu8 text as binary, escaping all non-printable characters EXCEPT:
/// - `\t`: tab
/// - `\n`: line feed
/// - `\r`: cariage return
///
/// This will allow the encoded text to print "pretilly" while still escaping invalid unicode and
/// other non-printable characters.
///
/// # Examples
/// ```rust
/// # extern crate stfu8;
///
/// # fn main() {
/// let encoded = stfu8::encode_pretty(b"foo\xFF\nbar");
/// assert_eq!(
///     encoded,
///     "foo\\xFF\nbar"
/// );
/// # }
/// ```
pub fn encode_pretty(v: &[u8]) -> String {
    let encoder = Encoder::pretty();
    encode::encode(&encoder, v)
}

// NOT YET STABILIZED

/// Settings for encoding binary data.
///
/// TODO: make this public eventually
pub(crate) struct Encoder {
    pub(crate) encode_tab: bool,          // \t \x09
    pub(crate) encode_line_feed: bool,    // \n \x0A
    pub(crate) encode_cariage: bool,      // \r \x0D
}


impl Encoder {
    /// Create a new "non pretty" `Encoder`.
    ///
    /// ALL non-printable characters will be escaped
    pub fn new() -> Encoder {
        Encoder {
            encode_tab: true,
            encode_line_feed: true,
            encode_cariage: true,
        }
    }

    /// Create a "pretty" `Encoder`.
    ///
    /// The following non-printable characters will not be escaped:
    /// - `\t`: tab
    /// - `\n`: line feed
    /// - `\r`: cariage return
    pub fn pretty() -> Encoder {
        Encoder {
            encode_tab: false,
            encode_line_feed: false,
            encode_cariage: false,
        }
    }
}
