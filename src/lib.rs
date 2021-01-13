use std::fmt;

use serde::de::{Deserializer, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

/// Wrapper type for rug integer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer(rug::Integer);

struct IVisitor();

impl<'de> Visitor<'de> for IVisitor {
    type Value = Integer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of bytes in little-endian format")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut digits: Vec<u8> = Vec::with_capacity(seq.size_hint().unwrap_or(64));
        while let Some(digit) = seq.next_element()? {
            digits.push(digit);
        }
        Ok(Integer(rug::Integer::from_digits(
            &digits[..],
            rug::integer::Order::Lsf,
        )))
    }
}

impl<'de> Deserialize<'de> for Integer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(IVisitor())
    }
}

impl From<rug::Integer> for Integer {
    fn from(i: rug::Integer) -> Self {
        Integer(i)
    }
}

impl From<Integer> for rug::Integer {
    fn from(i: Integer) -> rug::Integer {
        i.0
    }
}

impl AsRef<rug::Integer> for Integer {
    fn as_ref(&self) -> &rug::Integer {
        &self.0
    }
}

impl AsMut<rug::Integer> for Integer {
    fn as_mut(&mut self) -> &mut rug::Integer {
        &mut self.0
    }
}

impl Serialize for Integer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let digits: Vec<u8> = self.0.to_digits(rug::integer::Order::Lsf);
        let mut seq = serializer.serialize_seq(Some(digits.len()))?;
        for e in digits.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use rug::rand::RandState;

    use super::*;

    fn lengths() -> Vec<u32> {
        vec![
            0, 1, 2, 4, 7, 8, 9, 15, 16, 17, 30, 31, 32, 32, 60, 61, 62, 64, 127, 128, 129, 130,
        ]
    }

    #[test]
    fn test_serialize_bincode() {
        let mut rand = RandState::new();

        // check specific lengths
        for len in lengths().into_iter() {
            for _ in 0..10 {
                let bound = rug::Integer::from(rug::Integer::u_pow_u(2, len));
                let number: Integer = bound.random_below(&mut rand).into();
                let bs = bincode::serialize(&number).unwrap();
                let number_new = bincode::deserialize(&bs[..]).unwrap();
                assert_eq!(number, number_new);
            }
        }

        // check random lengths
        for _ in 0..100 {
            let bits = rug::Integer::from(1_000_000);
            let bound = rug::Integer::from(rug::Integer::u_pow_u(2, bits.to_u32().unwrap()));
            let number: Integer = bound.random_below(&mut rand).into();
            let bs = bincode::serialize(&number).unwrap();
            let number_new = bincode::deserialize(&bs[..]).unwrap();
            assert_eq!(number, number_new);
        }
    }

    #[test]
    fn test_serialize_json() {
        let mut rand = RandState::new();

        // check specific lengths
        for len in lengths().into_iter() {
            for _ in 0..10 {
                let bound = rug::Integer::from(rug::Integer::u_pow_u(2, len));
                let number: Integer = bound.random_below(&mut rand).into();
                let bs = serde_json::to_string(&number).unwrap();
                let number_new = serde_json::from_str(&bs[..]).unwrap();
                assert_eq!(number, number_new);
            }
        }

        for _ in 0..100 {
            let bits = rug::Integer::from(1_000_000);
            let bound = rug::Integer::from(rug::Integer::u_pow_u(2, bits.to_u32().unwrap()));
            let number: Integer = bound.random_below(&mut rand).into();
            let bs = serde_json::to_string(&number).unwrap();
            let number_new = serde_json::from_str(&bs[..]).unwrap();
            assert_eq!(number, number_new);
        }
    }
}
