#[cfg(test)]
mod tests {
    use parity_scale_codec::{Decode, Encode, HasCompact};

    #[derive(Encode)]
    struct Example {
        number: u8,
        is_cool: bool,
        optional: Option<u32>,
    }

    fn output(value: &Vec<u8>) -> String {
        format!(
            "0x{}",
            value.iter().map(|b| format!("{:02x?}", b)).fold(
                String::with_capacity(value.len() * 2),
                |mut r, b| {
                    r.push_str(&b);
                    r
                }
            )
        )
    }

    #[test]
    fn little_vs_big_endian() {
        assert_eq!("1000101", format!("{:b}", 69i8));
        assert_eq!("[45]", format!("{:02x?}", 69i8.to_le_bytes()));
        assert_eq!("101010", format!("{:b}", 42u16));
        assert_eq!("[2a, 00]", format!("{:02x?}", 42u16.to_le_bytes()));
        assert_eq!("[00, 2a]", format!("{:02x?}", 42u16.to_be_bytes()));
        assert_eq!("111111111111111111111111", format!("{:b}", 16777215u32));
        assert_eq!(
            "[ff, ff, ff, 00]",
            format!("{:02x?}", 16777215u32.to_le_bytes())
        );
        assert_eq!(
            "[00, ff, ff, ff]",
            format!("{:02x?}", 16777215u32.to_be_bytes())
        );
    }

    #[test]
    fn fixed_width_integers() {
        assert_eq!("[45]", format!("{:02x?}", 69i8.encode()));
        assert_eq!("[2a, 00]", format!("{:02x?}", 42u16.encode()));
        assert_eq!("[ff, ff, ff, 00]", format!("{:02x?}", 16777215u32.encode()));
    }

    #[test]
    fn compact() {
        #[derive(Encode)]
        struct AsCompact<T: HasCompact>(#[codec(compact)] T);

        // 0
        assert_eq!("[00]", format!("{:02x?}", 0u8.encode()));
        assert_eq!("[00, 00, 00, 00]", format!("{:02x?}", 0u32.encode()));
        assert_eq!("[00]", format!("{:02x?}", AsCompact(0u8).encode()));
        assert_eq!("[00]", format!("{:02x?}", AsCompact(0u32).encode()));

        // 42 as binary: 0b101010 = [0x2a]
        // Add 00 to the least significant bits
        // 0b10101000 = [0xa8] = 168 as decimal.
        assert_eq!("[2a]", format!("{:02x?}", 42u8.encode()));
        assert_eq!("[2a, 00, 00, 00]", format!("{:02x?}", 42u32.encode()));
        assert_eq!("[a8]", format!("{:02x?}", AsCompact(42u8).encode()));
        assert_eq!("[a8]", format!("{:02x?}", AsCompact(42u32).encode()));

        // 69 as binary: 0b1000101 = [0x45]
        // Add 01 to the least significant bits
        // 0b100010101 = [0x15, 0x01] = 277 as decimal
        assert_eq!("[45]", format!("{:02x?}", 69u8.encode()));
        assert_eq!("[45, 00, 00, 00]", format!("{:02x?}", 69u32.encode()));
        assert_eq!("[15, 01]", format!("{:02x?}", AsCompact(69u8).encode()));
        assert_eq!("[15, 01]", format!("{:02x?}", AsCompact(69u32).encode()));

        // 65535 as binary: 0b1111111111111111 = [0xff, 0xff]
        // Add 10 to the least significant bits
        // 0b111111111111111110 = [0xfe, 0xff, 0x03, 0x00]: 262142 as decimal
        assert_eq!("[ff, ff]", format!("{:02x?}", 65535u16.encode()));
        assert_eq!("[ff, ff, 00, 00]", format!("{:02x?}", 65535u32.encode()));
        assert_eq!(
            "[fe, ff, 03, 00]",
            format!("{:02x?}", AsCompact(65535u16).encode())
        );
        assert_eq!(
            "[fe, ff, 03, 00]",
            format!("{:02x?}", AsCompact(65535u32).encode())
        );
    }

    #[test]
    fn encode_unit() {
        let encoded = ().encode();
        assert!(encoded.is_empty());
        assert_eq!("[]", format!("{:02x?}", encoded));
        assert_eq!("0x", output(&encoded));
    }

    #[test]
    fn encode_boolean() {
        let encoded = true.encode();
        assert_eq!("[01]", format!("{:02x?}", encoded));
        assert_eq!("0x01", output(&encoded));

        let encoded = false.encode();
        assert_eq!("[00]", format!("{:02x?}", encoded));
        assert_eq!("0x00", output(&encoded));
    }

    #[test]
    fn encode_ok_err() {
        let encoded = Ok::<u32, ()>(42).encode();
        assert_eq!("[00, 2a, 00, 00, 00]", format!("{:02x?}", encoded));
        assert_eq!("0x002a000000", output(&encoded));

        let encoded = Err::<u32, ()>(()).encode();
        assert_eq!("[01]", format!("{:02x?}", encoded));
        assert_eq!("0x01", output(&encoded));
    }

    #[test]
    fn some_none() {
        let encoded = Some(42u32).encode();
        assert_eq!("[01, 2a, 00, 00, 00]", format!("{:02x?}", encoded));
        assert_eq!("0x012a000000", output(&encoded));

        let encoded = None::<u32>.encode();
        assert_eq!("[00]", format!("{:02x?}", encoded));
        assert_eq!("0x00", output(&encoded));
    }

    // Arrays: Just concatenate the items.
    #[test]
    fn encode_array() {
        let encoded = [1u8, 2, 3].encode();
        assert_eq!("[01, 02, 03]", format!("{:02x?}", encoded));
        assert_eq!("0x010203", output(&encoded));
    }

    // Vectors: Also prefix with length (compact encoded).
    #[test]
    fn encode_vector() {
        let encoded = vec![0u8, 1, 2, 3, 4].encode();
        assert_eq!("[14, 00, 01, 02, 03, 04]", format!("{:02x?}", encoded));
        assert_eq!("0x140001020304", output(&encoded));
    }

    // String: Just Vec<u8> as utf-8 characters.
    #[test]
    fn encode_string() {
        let encoded = "hello".encode();
        assert_eq!("[14, 68, 65, 6c, 6c, 6f]", format!("{:02x?}", encoded));
        assert_eq!("0x1468656c6c6f", output(&encoded));
    }

    #[test]
    fn tuple_struct() {
        #[derive(Encode)]
        struct Example {
            number: u8,
            is_cool: bool,
            optional: Option<u32>,
        }

        let encoded = (0u8, true, Some(69u32)).encode();
        assert_eq!("[00, 01, 01, 45, 00, 00, 00]", format!("{:02x?}", encoded));
        assert_eq!("0x00010145000000", output(&encoded));

        let encoded = Example {
            number: 0,
            is_cool: true,
            optional: Some(69),
        }
        .encode();
        assert_eq!("[00, 01, 01, 45, 00, 00, 00]", format!("{:02x?}", encoded));
        assert_eq!("0x00010145000000", output(&encoded));
    }

    #[test]
    fn enumueration() {
        #[derive(Encode)]
        enum Example {
            First,
            Second(u8),
            Third(Vec<u8>),
            Fourth,
        }

        let encoded = Example::First.encode();
        assert_eq!("[00]", format!("{:02x?}", encoded));
        assert_eq!("0x00", output(&encoded));

        let encoded = Example::Second(2).encode();
        assert_eq!("[01, 02]", format!("{:02x?}", encoded));
        assert_eq!("0x0102", output(&encoded));

        let encoded = Example::Third(vec![0, 1, 2, 3, 4]).encode();
        assert_eq!("[02, 14, 00, 01, 02, 03, 04]", format!("{:02x?}", encoded));
        assert_eq!("0x02140001020304", output(&encoded));

        let encoded = Example::Fourth.encode();
        assert_eq!("[03]", format!("{:02x?}", encoded));
        assert_eq!("0x03", output(&encoded));
    }

    #[test]
    fn decode() {
        let array = [0u8, 1u8, 2u8, 3u8];
        let value: u32 = 50462976;

        let mut encoded = array.encode();
        assert_eq!("[00, 01, 02, 03]", format!("{:02x?}", encoded));
        assert_eq!("0x00010203", output(&encoded));
        assert_eq!(Ok(50462976), u32::decode(&mut &*encoded));

        // println!("{:02x?}", value.encode());
        // println!("{:?}", u32::decode(&mut &array.encode()[..]));
        // println!("{:?}", u16::decode(&mut &array.encode()[..]));
        // println!("{:?}", u16::decode_all(&mut &array.encode()[..]));
        // println!("{:?}", u64::decode(&mut &array.encode()[..]));
        //
        // [00, 01, 02, 03]
        // [00, 01, 02, 03]
        // Ok(50462976)
        // Ok(256)
        // Err(Error { cause: None, desc: "Input buffer has still data left after decoding!" })
        // Err(Error { cause: None, desc: "Not enough data to fill buffer" })
    }
}
