
use ::arbitrary::{Arbitrary, Unstructured, Result};
use super::*;

impl<'a> Arbitrary<'a> for Span {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        if <bool as Arbitrary>::arbitrary(u)? {
            Ok(Span::call_site())
        } else {
            Ok(Span::mixed_site())
        }
    }
}

impl<'a> Arbitrary<'a> for Spacing {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        if <bool as Arbitrary>::arbitrary(u)? {
            Ok(Spacing::Alone)
        } else {
            Ok(Spacing::Joint)
        }
    }
}

impl<'a> Arbitrary<'a> for Ident {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Ident::new(<&str as Arbitrary>::arbitrary(u)?, <Span as Arbitrary>::arbitrary(u)?))
    }
}

impl<'a> Arbitrary<'a> for Delimiter {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(match <u8 as Arbitrary>::arbitrary(u)? % 4 {
            0 => Delimiter::Brace,
            1 => Delimiter::Bracket,
            2 => Delimiter::Parenthesis,
            3 => Delimiter::None,
            _ => unreachable!(),
        })
    }
}

impl<'a> Arbitrary<'a> for Punct {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Punct::new(<char as Arbitrary>::arbitrary(u)?, <Spacing as Arbitrary>::arbitrary(u)?))
    }
}

impl<'a> Arbitrary<'a> for Literal {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(match <u8 as Arbitrary>::arbitrary(u)? % 31 {
            0 => Literal::u8_suffixed(<u8 as Arbitrary>::arbitrary(u)?),
            1 => Literal::u16_suffixed(<u16 as Arbitrary>::arbitrary(u)?),
            2 => Literal::u32_suffixed(<u32 as Arbitrary>::arbitrary(u)?),
            3 => Literal::u64_suffixed(<u64 as Arbitrary>::arbitrary(u)?),
            4 => Literal::u128_suffixed(<u128 as Arbitrary>::arbitrary(u)?),
            5 => Literal::usize_suffixed(<usize as Arbitrary>::arbitrary(u)?),
            6 => Literal::i8_suffixed(<i8 as Arbitrary>::arbitrary(u)?),
            7 => Literal::i16_suffixed(<i16 as Arbitrary>::arbitrary(u)?),
            8 => Literal::i32_suffixed(<i32 as Arbitrary>::arbitrary(u)?),
            9 => Literal::i64_suffixed(<i64 as Arbitrary>::arbitrary(u)?),
            10 => Literal::i128_suffixed(<i128 as Arbitrary>::arbitrary(u)?),
            11 => Literal::isize_suffixed(<isize as Arbitrary>::arbitrary(u)?),
            12 => Literal::u8_unsuffixed(<u8 as Arbitrary>::arbitrary(u)?),
            13 => Literal::u16_unsuffixed(<u16 as Arbitrary>::arbitrary(u)?),
            14 => Literal::u32_unsuffixed(<u32 as Arbitrary>::arbitrary(u)?),
            15 => Literal::u64_unsuffixed(<u64 as Arbitrary>::arbitrary(u)?),
            16 => Literal::u128_unsuffixed(<u128 as Arbitrary>::arbitrary(u)?),
            17 => Literal::usize_unsuffixed(<usize as Arbitrary>::arbitrary(u)?),
            18 => Literal::i8_unsuffixed(<i8 as Arbitrary>::arbitrary(u)?),
            19 => Literal::i16_unsuffixed(<i16 as Arbitrary>::arbitrary(u)?),
            20 => Literal::i32_unsuffixed(<i32 as Arbitrary>::arbitrary(u)?),
            21 => Literal::i64_unsuffixed(<i64 as Arbitrary>::arbitrary(u)?),
            22 => Literal::i128_unsuffixed(<i128 as Arbitrary>::arbitrary(u)?),
            23 => Literal::isize_unsuffixed(<isize as Arbitrary>::arbitrary(u)?),
            24 => Literal::f32_suffixed(<f32 as Arbitrary>::arbitrary(u)?),
            25 => Literal::f32_unsuffixed(<f32 as Arbitrary>::arbitrary(u)?),
            26 => Literal::f64_suffixed(<f64 as Arbitrary>::arbitrary(u)?),
            27 => Literal::f64_unsuffixed(<f64 as Arbitrary>::arbitrary(u)?),
            28 => Literal::character(<char as Arbitrary>::arbitrary(u)?),
            29 => Literal::string(<&str as Arbitrary>::arbitrary(u)?),
            30 => Literal::byte_string(<&[u8] as Arbitrary>::arbitrary(u)?),
            _ => unreachable!()
        })
    }
}

impl<'a> Arbitrary<'a> for Group {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(Group::new(<Delimiter as Arbitrary>::arbitrary(u)?, <TokenStream as Arbitrary>::arbitrary(u)?))
    }
}

impl<'a> Arbitrary<'a> for TokenTree {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(match <u8 as Arbitrary>::arbitrary(u)? % 4 {
            0 => TokenTree::Group(<Group as Arbitrary>::arbitrary(u)?),
            1 => TokenTree::Ident(<Ident as Arbitrary>::arbitrary(u)?),
            2 => TokenTree::Punct(<Punct as Arbitrary>::arbitrary(u)?),
            3 => TokenTree::Literal(<Literal as Arbitrary>::arbitrary(u)?),
            _ => unreachable!()
        })
    }
}

impl<'a> Arbitrary<'a> for TokenStream {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Ok(<Vec<TokenTree> as Arbitrary>::arbitrary(u)?.into_iter().collect())
    }
}
