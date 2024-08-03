use crate::bfir::{BFCell, BFCode};
use crate::error::ParseError;

#[derive(Debug)]
pub struct BFFrame<T: BFCell> {
    codes: Vec<BFCode::<T>>,
    pc: usize,  // problem counter
}

impl<T> BFFrame<T>
    where T: BFCell {
    pub fn new(source_codes: impl std::io::BufRead) -> Result<BFFrame::<T>, ParseError> {

        let mut frame = BFFrame::<T>{ codes: vec![], pc: 0 };
        let mut stack: Vec<(usize, usize, usize)> = vec![];

        let mut line = 1usize;
        let mut row = 1usize;

        macro_rules! wrapping_push {
            ($vector:expr, $code:path) => {
                if let Some($code(n)) = $vector.last() {
                    // Pre dereference reference "n" to end the lifetime of immutable
                    // reference and avoid conflicts with mutable reference "last"
                    let n = *n;

                    let last = $vector.last_mut().unwrap();
                    *last = $code(n + (T::from(1)));
                } else {
                    $vector.push($code(T::from(1)));
                }
            };
        }

        macro_rules! unwrapping_push {
            ($vector:expr, $code:path) => {
                if let Some($code(n)) = $vector.last() {
                    // Pre dereference reference "n" to end the lifetime of immutable
                    // reference and avoid conflicts with mutable reference "last"
                    let n = *n;

                    if n == usize::MAX {
                        $vector.push($code(1usize));
                    } else {
                        let last = $vector.last_mut().unwrap();
                        *last = $code(n + 1);
                    }
                } else {
                    $vector.push($code(1usize));
                }
            };
        }

        for byte in source_codes.bytes() {
            let byte = byte.map_err(|e| {
                ParseError::IO{source: e}
            })?;

            if byte == b'\n' { line += 1; row = 1; continue; }
            row += 1;

            match byte {
                b'+' => { wrapping_push!(frame.codes, BFCode::<T>::AddCell) },
                b'-' => { wrapping_push!(frame.codes, BFCode::<T>::SubCell) },
                b'<' => { unwrapping_push!(frame.codes, BFCode::<T>::LeftShift) },
                b'>' => { unwrapping_push!(frame.codes, BFCode::<T>::RightShift) },
                b',' => { frame.codes.push(BFCode::<T>::Input) },
                b'.' => { frame.codes.push(BFCode::<T>::Output) },
                b'[' => { stack.push( (frame.codes.len(), line, row) );
                          frame.codes.push( BFCode::<T>::Jz(usize::MAX) ); },
                b']' => {
                    if let Some(left_bracket_data) = stack.pop() {
                        frame.codes[left_bracket_data.0] =
                            BFCode::<T>::Jz(frame.codes.len());
                        frame.codes.push(BFCode::<T>::Jnz(left_bracket_data.0));
                    } else {
                        return Err(ParseError::MismatchedBracket{
                            bracket: ']',
                            line,
                            row,
                        });
                    }
                },
                _ => {}
            }
        }

        if let Some(left_bracket_data) = stack.pop() {
            return Err(ParseError::MismatchedBracket{
                bracket: '[',
                line: left_bracket_data.1,
                row: left_bracket_data.2,
            });
        }

        Ok(frame)
    }

}

