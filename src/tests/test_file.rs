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
}