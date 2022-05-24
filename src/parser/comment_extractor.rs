use super::error;

pub type Result<T> = std::result::Result<T, error::ParserError>;
fn extract_commands_from_comment(coment: &str) -> Option<Vec<String>> {
    unimplemented!()
}

struct CommentInterpreter<'a> {
    comment: &'a [u8],
    current_index: usize,
}

const COMMAND_BEGIN_DELIMITER_CHAR: u8 = b'%';
const NEW_LINE: u8 = b'\n';

impl<'a> CommentInterpreter<'a> {
    pub fn new(comment: &'a str) -> CommentInterpreter<'a> {
        CommentInterpreter {
            comment: comment.as_bytes(),
            current_index: 0,
        }
    }

    fn peek(&self) -> Option<&u8> {
        self.comment.get(self.current_index + 1)
    }

    fn peek2(&self) -> Option<&u8> {
        self.comment.get(self.current_index + 2)
    }

    fn chomp(&mut self) -> Option<&u8> {
        let current_char = self.comment.get(self.current_index);
        self.current_index += 1;
        current_char
    }

    fn chomp_until_line_end(&mut self) -> Option<&[u8]> {
        unimplemented!()
    }

    pub fn extact_command(&mut self) -> Result<Option<String>> {
        while let Some(peeked_char) = self.peek() {
            if *peeked_char == COMMAND_BEGIN_DELIMITER_CHAR {
                if let Some(peeked_char_2) = self.peek2() {
                    if *peeked_char_2 == COMMAND_BEGIN_DELIMITER_CHAR {
                        let _ = self.chomp();
                        let _ = self.chomp();
                        if let Some(command) = self.chomp_until_line_end() {
                            return Ok(Some(String::from_utf8(
                                command.into_iter().cloned().collect(),
                            )?));
                        }
                    }
                }
            }
            let _ = self.chomp();
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_peek() {}
}
