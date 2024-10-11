/* ----------------------------------------------------------------------------

    MIT License

    Copyright (c) 2024 MW

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

---------------------------------------------------------------------------- */

#[cfg(test)]

mod test_file {
    use std::io::{BufRead, BufReader};

    use crate::file::*;

    fn read_line<'a>(reader: &'a mut BufReader<BufferSlice>) -> Option<String> {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(count) if count > 0 => Some(String::from(line.trim())),
            _ => None
        }
    }

    #[test]
    fn readbuffer_from_string() {
        let buffer = ReadBuffer::from_string("hello, world!");

        assert_eq!(String::from_utf8(buffer.as_vec().clone()).unwrap(), String::from("hello, world!"));
    }

    #[test]
    fn readbuffer_from_string_read_line() {
        let buffer = ReadBuffer::from_string("this\nis\na\ntest");
        let mut reader = buffer.reader();

        assert_eq!(read_line(&mut reader).unwrap(), "this".to_string());
        assert_eq!(read_line(&mut reader).unwrap(), "is".to_string());
        assert_eq!(read_line(&mut reader).unwrap(), "a".to_string());
        assert_eq!(read_line(&mut reader).unwrap(), "test".to_string());
    }

    #[test]
    fn readbuffer_from_string_read_line_until_empty() {
        let buffer = ReadBuffer::from_string("this\nis\na\ntest");
        let mut reader = buffer.reader();

        read_line(&mut reader);
        read_line(&mut reader);
        read_line(&mut reader);
        read_line(&mut reader);

        assert!(read_line(&mut reader).is_none());
    }

    #[test]
    fn fileparser_init_with_path() {
        let parser = FileParser::init(ReadBuffer::from_string("abc\ndef\nxyz"), Some(String::from("one/two/three")));

        assert_eq!(String::from_utf8(parser.get_buffer().as_vec().clone()).unwrap(), String::from("abc\ndef\nxyz"));
    }

    #[test]
    fn fileparser_init_without_path() {
        let parser = FileParser::init(ReadBuffer::from_string("abc\ndef\nxyz"), None);

        assert_eq!(String::from_utf8(parser.get_buffer().as_vec().clone()).unwrap(), String::from("abc\ndef\nxyz"));
    }

    #[test]
    fn fileparser_get_path_some() {
        let parser = FileParser::init(ReadBuffer::from_string("abc\ndef\nxyz"), Some(String::from("abc\ndef\nxyz")));

        assert_eq!(parser.get_path().unwrap(), String::from("abc\ndef\nxyz"));
    }

    #[test]
    fn fileparser_get_path_none() {
        let parser = FileParser::init(ReadBuffer::from_string("abc\ndef\nxyz"), None);

        assert_eq!(parser.get_path(), None);
    }

    #[test]
    fn fileparser_from_string() {
        let parser = FileParser::from_string("1,3\n2,4\n3,5\n4,6\n5,7\n6,8");

        assert_eq!(String::from_utf8(parser.get_buffer().as_vec().clone()).unwrap(), String::from("1,3\n2,4\n3,5\n4,6\n5,7\n6,8"));
    }

    #[test]
    fn fileparser_read_numbers() {
        let mut file = FileParser::from_string("1,3\n2,4\n3,5\n4,6\n5,7\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (1, 3));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
    }

    #[test]
    fn fileparser_read_numbers_until_last() {
        let mut file = FileParser::from_string("1,3\n2,4\n3,5\n4,6\n5,7\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (1, 3));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));
    }

    #[test]
    fn fileparser_read_numbers_until_done() {
        let mut file = FileParser::from_string("1,3\n2,4\n3,5\n4,6\n5,7\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (1, 3));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fileparser_read_numbers_ignore_whitespace() {
        let mut file = FileParser::from_string("1, 3 \n\t2  ,4 \n  3,  5\n4, 6\n5  ,  7\t\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (1, 3));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));
    }

    #[test]
    fn fileparser_read_numbers_ignore_comments() {
        let mut file = FileParser::from_string("1,3\n#hello\n2,4\n3,5\n4,6\n#world\n5,7\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (1, 3));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));

        assert_eq!(iter.next(), None);
    }
}