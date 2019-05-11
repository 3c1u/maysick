/*
 * maysick
 *
 * 2018 - murueka
 */

use std::io::{stdin, Read};

pub fn get_utf8_char() -> Option<u32> {
    let s = stdin();
    let s = s.lock();

    let mut b = s.bytes();

    if let Some(Ok(c)) = b.next() {
        let c_ = c;
        let c = u32::from(c);

        if 0 < c && c <= 0x7F {
            // ASCII文字
            Some(c)
        } else {
            // UTF-8符号化文字
            let cnt = (!c_).leading_zeros() - 1;

            let mut v = c & (0x3F >> cnt);

            assert!(cnt > 0, "Fragmented UTF-8 character.");
            assert!(
                cnt < 5,
                "Unicode code point that cannot be represented by a single 32-bit value."
            );

            for _ in 0..cnt {
                let c = u32::from(
                    b.next()
                        .expect("Invalid UTF-8 character.")
                        .expect("Failed to read from standard input."),
                );

                v = (v << 6) | (c & 0x3F);
            }

            Some(v)
        }
    } else {
        None
    }
}

pub fn getchar() -> i64 {
    if let Some(c) = get_utf8_char() {
        // surrogate-pair check
        if 0xD800 <= c && c <= 0xDBFF {
            eprintln!("Warning: surrogate pairs should not be used in UTF-8 encoding.");
            if let Some(lo) = get_utf8_char() {
                return i64::from(0x10000 + (c - 0xD800) * 0x400 + (lo - 0xDC00));
            } else {
                panic!("Invalid UTF-8 character. Missing LO value.");
            }
        } else if 0xDC00 <= c && c <= 0xDFFF {
            panic!("Invalid UTF-8 character. Missing HI value.");
        }

        i64::from(c)
    } else {
        -1
    }
}
