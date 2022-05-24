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

    fn peek(&self, n: usize) -> Option<&u8> {
        self.comment.get(self.current_index + n)
    }

    fn shift(&mut self, n: usize) {
        self.current_index += n;
    }

    fn at_end(&self) -> bool {
        self.current_index >= self.comment.len()
    }
    fn chomp_until_line_end(&mut self) -> Option<&[u8]> {
        if self.at_end() {
            return None;
        }
        let start_index = self.current_index;
        while let Some(each_char) = self.comment.get(self.current_index) {
            if *each_char == NEW_LINE {
                break;
            }
            self.current_index += 1;
        }
        let result = Some(&self.comment[start_index..self.current_index]);
        self.shift(1);
        result
    }

    pub fn parse_command(&mut self) -> Result<Option<String>> {
        while let Some(peeked_char) = self.peek(1) {
            if *peeked_char == COMMAND_BEGIN_DELIMITER_CHAR {
                if let Some(peeked_char_2) = self.peek(2) {
                    if *peeked_char_2 == COMMAND_BEGIN_DELIMITER_CHAR {
                        self.shift(3);
                        if let Some(command) = self.chomp_until_line_end() {
                            return Ok(Some(String::from_utf8(
                                command.into_iter().cloned().collect(),
                            )?));
                        }
                    }
                }
            }
            let _ = self.shift(1);
        }
        Ok(None)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_chomp_until_line_end_1() {
        let data = "abcdef\nghi";
        let mut interpreter = CommentInterpreter::new(data);
        {
            let chomped = interpreter.chomp_until_line_end();
            assert_eq!(
                Some("abcdef".chars().into_iter().map(|c| c as u8).collect()) as Option<Vec<u8>>,
                chomped.map(|v| v.into_iter().cloned().collect())
            );
        }

        {
            let chomped = interpreter.chomp_until_line_end();
            assert_eq!(
                Some("ghi".chars().into_iter().map(|c| c as u8).collect()) as Option<Vec<u8>>,
                chomped.map(|v| v.into_iter().cloned().collect())
            );
        }

        {
            let chomped = interpreter.chomp_until_line_end();
            assert_eq!(None, chomped);
        }
    }

    #[test]
    fn test_chomp_until_line_end_2() {
        let data = "abcdefghi";
        let mut interpreter = CommentInterpreter::new(data);
        {
            let chomped = interpreter.chomp_until_line_end();
            assert_eq!(
                Some("abcdefghi".chars().into_iter().map(|c| c as u8).collect()) as Option<Vec<u8>>,
                chomped.map(|v| v.into_iter().cloned().collect())
            );
        }

        {
            let chomped = interpreter.chomp_until_line_end();
            assert_eq!(None, chomped);
        }
    }

    #[test]
    fn test_extact_command_1() {
        let data = r#"
            %% this is command
            aeiou
            %%  this another is command

            this is not command abcdefghi"#;
        let mut interpreter = CommentInterpreter::new(data);
        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(Some(" this is command".to_string()), parsed);
        }

        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(Some("  this another is command".to_string()), parsed);
        }

        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(None, parsed);
        }
    }

    #[test]
    fn test_extact_command_2() {
        let data = r#"
            %% this is command
            this is not command abcdefghi %% aaa
            %% "#;
        let mut interpreter = CommentInterpreter::new(data);
        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(Some(" this is command".to_string()), parsed);
        }

        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(Some(" aaa".to_string()), parsed);
        }

        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(Some(" ".to_string()), parsed);
        }

        {
            let parsed = interpreter.parse_command().unwrap();
            assert_eq!(None, parsed);
        }
    }
}
