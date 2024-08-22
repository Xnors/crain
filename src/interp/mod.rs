use crate::bfir::{BFCell, BFCode};
use crate::error::ParseError;

#[derive(Debug)]
pub struct BFFrame<T: BFCell> {
    codes: Vec<BFCode<T>>,
    pc: usize,  // problem counter
}

impl<T> BFFrame<T>
    where T: BFCell {
    pub fn new(source_codes: impl std::io::BufRead) -> Result<BFFrame<T>, ParseError> {
        Ok(Self{ codes: BFCode::parse(source_codes)?, pc: 0 })
    }

    pub fn codes(&self) -> &Vec<BFCode<T>> { &(self.codes) }
}


#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use crate::bfir::BFCode::*;
    use super::*;

    #[test]
    fn test_bfframe_new() {
        let error_func = |e| panic!("Parse Error: {}", e);
        let ok_func    = |v| v;

        // test the parse function
        let codes = BufReader::new("+-<>,.[]".as_bytes());
        let frame = BFFrame::<u8>::new(codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            *(frame.codes()),
            vec![
                AddCell(1),
                SubCell(1),
                LeftShift(1),
                RightShift(1),
                Input,
                Output,
                Jz(7),
                Jnz(6),
            ]
        );

        // test the merge function
        let codes = BufReader::new("-<<<+++--><++>>>".as_bytes());
        let frame = BFFrame::<u8>::new(codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            *(frame.codes()),
            vec![
                SubCell(1),
                LeftShift(3),
                AddCell(3),
                SubCell(2),
                RightShift(1),
                LeftShift(1),
                AddCell(2),
                RightShift(3),
            ]
        );

        // test the brackets match function
        let codes = BufReader::new("[[][[[][[]][]]]]".as_bytes());
        let frame = BFFrame::<u8>::new(codes).map_or_else(
            error_func,
            ok_func
        );

        assert_eq!(
            *(frame.codes()),
            vec![
                Jz(15),   // 0
                Jz(2),    // 1
                Jnz(1),   // 2
                Jz(14),   // 3
                Jz(13),   // 4
                Jz(6),    // 5
                Jnz(5),   // 6
                Jz(10),   // 7
                Jz(9),    // 8
                Jnz(8),   // 9
                Jnz(7),   // 10
                Jz(12),   // 11
                Jnz(11),  // 12
                Jnz(4),   // 13
                Jnz(3),   // 14
                Jnz(0),   // 15
            ]
        );
    }
}
