pub struct SmartSplit<'a> {
    string: &'a str,
    cur_pos: usize,

}

impl<'a> SmartSplit<'a> {
    pub fn new(string: &str) -> SmartSplit{
        SmartSplit {string: string, cur_pos: 0}
    }
}

impl<'a> Iterator for SmartSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        // TODO: support brackets containing a single instr, or perhaps make commas supersede
        // spaces?
        let mut start_pos = self.cur_pos;
        for c in self.string[start_pos..].chars() {
            if c.is_whitespace() || c == ',' {
                start_pos += 1;
            } else {
                break
            }
        }

        let mut end_pos = start_pos;
        let mut escaped = false;
        let mut quote_char: char = '\0';


        for c in self.string[start_pos..].chars() {
            if escaped {
                escaped = false;
            } else if quote_char != '\0' {
                if c == quote_char {
                    quote_char = '\0';
                } else if c == '\\' {
                    escaped = true;
                }
            } else if c == '\\' {
                escaped = true;
            } else if c == '\'' || c == '"' {
                quote_char = c;
            } else if c.is_whitespace() || c == ',' {
                break
            } 
            
            end_pos += 1;
        }

        if quote_char != '\0' {
            panic!("During parsing, an unterminated quote string was found!")
        }
        if escaped {
            panic!("During parsing, the last character was an escape character!")
        }
        if start_pos == end_pos {
            return None
        }
        self.cur_pos = end_pos;
        
        Some(&self.string[start_pos..end_pos])

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn compare_iterator_to_vec(s: &str, v: Vec<&str>) {
        for (vec_item, iter_item) in  SmartSplit::new(s) .zip(v.iter()) {
            assert_eq!(vec_item, *iter_item);
        }
    }

    #[test]
    fn test_smart_split() {
        compare_iterator_to_vec("a b c", vec!["a", "b", "c"]);
        compare_iterator_to_vec("a     b    c", vec!["a", "b", "c"]);
        compare_iterator_to_vec("a 'b c'", vec!["a", "'b c'"]);
        compare_iterator_to_vec("'a b' c", vec!["'a b'", "c"]);
        compare_iterator_to_vec("a '\\'b c'", vec!["a", "'\\'b c'"]);
        compare_iterator_to_vec("'a b\\'' c", vec!["'a b\\''", "c"]);
        compare_iterator_to_vec("'\"a\" ' b c", vec!["'\"a\" '", "b", "c"]);
    }

}
