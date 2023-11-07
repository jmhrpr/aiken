use chumsky::prelude::*;
use uplc::machine::runtime::Compressable;

use crate::{
    ast,
    parser::{error::ParseError, expr::UntypedExpr, literal::bytearray, token::Token},
};

pub fn parser() -> impl Parser<Token, UntypedExpr, Error = ParseError> {
    bytearray(
        |bytes, preferred_format, curve, location, emit| match curve {
            Some(curve @ ast::CurveType::Bls12_381(point)) => {
                let point = match point {
                    ast::Bls12_381PointType::G1 => {
                        blst::blst_p1::uncompress(&bytes).map(ast::Bls12_381Point::G1)
                    }
                    ast::Bls12_381PointType::G2 => {
                        blst::blst_p2::uncompress(&bytes).map(ast::Bls12_381Point::G2)
                    }
                };

                let point = point.unwrap_or_else(|_err| {
                    emit(ParseError::point_not_on_curve(curve, location));

                    ast::Bls12_381Point::default()
                });

                UntypedExpr::CurvePoint {
                    location,
                    point: ast::Curve::Bls12_381(point).into(),
                    preferred_format,
                }
            }
            None => UntypedExpr::ByteArray {
                location,
                bytes,
                preferred_format,
            },
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::assert_expr;

    #[test]
    fn bytearray_basic() {
        assert_expr!("#[0, 170, 255]");
    }

    #[test]
    fn bytearray_base16() {
        assert_expr!("#\"00aaff\"");
    }

    #[test]
    fn bytearray_utf8_encoded() {
        assert_expr!("\"aiken\"");
    }

    #[test]
    fn bytearray_utf8_escaped() {
        assert_expr!("\"\\\"aiken\\\"\"");
    }
}
