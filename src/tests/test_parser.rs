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
    use crate::file::*;

    type Cell = (usize, usize);

    static CELL_DATA: &[Cell] = &[(50,25),(2,4),(3,5),(4,6),(5,7),(6,8)];
    static CELL_DATA_STRING: &str = "50,25\n2,4\n3,5\n4,6\n5,7\n6,8\n";

    fn create_fileparser(contents: &str) -> FileParser {
        let mut parser = FileParser::from_string(contents);
        parser.set_test();  // <--- This causes a panic instead a process exit.
                            //      In that case the test harness will remain
                            //      running.

        parser
    }

    fn create_fileparser_from(contents: &[Cell]) -> FileParser {
        let mut arg = String::new();
        for (x, y) in contents {
            arg += format!("{},{}\n", x, y).as_str();
        }

        let mut parser = FileParser::from_string(arg.as_str());
        parser.set_test();  // <--- This causes a panic instead of process exit.
                            //      In that case the test harness will remain
                            //      running.

        parser
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
        let parser = FileParser::from_string(CELL_DATA_STRING);

        assert_eq!(String::from_utf8(parser.get_buffer().as_vec().clone()).unwrap(), String::from(CELL_DATA_STRING));
    }

    #[test]
    #[should_panic]
    fn fileparser_bad_char() {
        let contents = "50,25\n2,4\n3,5\n4,6\n!\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    fn fileparser_read_numbers_in_bounds() {
        let cells = &[(20, 10), (5,5), (6,6)];
        let mut file = create_fileparser_from(cells);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), cells[0]);
        assert_eq!(iter.next().unwrap(), cells[1]);
        assert_eq!(iter.next().unwrap(), cells[2]);
    }

    #[test]
    #[should_panic]
    fn fileparser_read_numbers_out_bounds() {
        let cells = &[(20, 10), (21,5), (6,6)];
        let mut file = create_fileparser_from(cells);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), cells[0]);
        assert_eq!(iter.next().unwrap(), cells[1]);
        assert_eq!(iter.next().unwrap(), cells[2]);
    }

    #[test]
    fn fileparser_read_numbers() {
        let mut file = create_fileparser_from(CELL_DATA);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
    }

    #[test]
    fn fileparser_read_numbers_until_last() {
        let mut file = create_fileparser_from(CELL_DATA);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[3]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[4]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[5]);
    }

    #[test]
    fn fileparser_read_numbers_until_done() {
        let mut file = create_fileparser_from(CELL_DATA);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[3]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[4]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[5]);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fileparser_read_numbers_ignore_whitespace() {
        let mut file = create_fileparser("99, 88 \n\t2  ,4 \n  3,  5\n4, 6\n5  ,  7\t\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (99, 88));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));
    }

    #[test]
    fn fileparser_read_numbers_ignore_comments() {
        let mut file = create_fileparser("66,77\n#hello\n2,4\n3,5\n4,6\n#world\n5,7\n6,8");
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), (66, 77));
        assert_eq!(iter.next().unwrap(), (2, 4));
        assert_eq!(iter.next().unwrap(), (3, 5));
        assert_eq!(iter.next().unwrap(), (4, 6));
        assert_eq!(iter.next().unwrap(), (5, 7));
        assert_eq!(iter.next().unwrap(), (6, 8));

        assert_eq!(iter.next(), None);
    }

    //
    // Symbol definition tests...
    //

    #[test]
    fn fileparser_define_symbol_good() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n2,1\n3,1\n;\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[3]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[4]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[5]);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fileparser_define_symbol_after_size() {
        let contents = "50,25\n:FIRST\n0,1\n2,1\n3,1\n;\n2,4\n3,5\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[3]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[4]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[5]);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fileparser_define_symbol_before_size() {
        let contents = ":FIRST\n0,1\n2,1\n3,1\n;\n50,25\n2,4\n3,5\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut iter = file.iter();

        assert_eq!(iter.next().unwrap(), CELL_DATA[0]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[1]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[2]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[3]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[4]);
        assert_eq!(iter.next().unwrap(), CELL_DATA[5]);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fileparser_use_symbol_good_with_offset() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\nFIRST 20,15\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&CELL_DATA[0]));
        assert!(cells.contains(&CELL_DATA[1]));
        assert!(cells.contains(&CELL_DATA[2]));
        assert!(cells.contains(&CELL_DATA[3]));
        assert!(cells.contains(&CELL_DATA[4]));
        assert!(cells.contains(&CELL_DATA[5]));

        assert!(cells.contains(&(20, 16)));
        assert!(cells.contains(&(21, 16)));
        assert!(cells.contains(&(22, 16)));
    }

    #[test]
    fn fileparser_use_symbol_bad_no_use_offset() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\nFIRST 20,15\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(!cells.contains(&(0, 1)));
        assert!(!cells.contains(&(1, 1)));
        assert!(!cells.contains(&(2, 1)));
    }

    #[test]
    #[should_panic]
    fn fileparser_use_symbol_bad_offset_out_of_range() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\nFIRST 50,15\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    #[should_panic]
    fn fileparser_use_symbol_bad_before_size() {
        let contents = "FIRST 50,15\n20,15\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    fn fileparser_use_symbol_good_after_size() {
        let contents = "50,25\n:FIRST\n0,1\n1,1\n2,1\n;\nFIRST 20,15\n2,4\n3,5\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&(21, 16)));
    }

    #[test]
    fn fileparser_use_symbol_good_last() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\n6,8\nFIRST 20,15\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&(21, 16)));
    }

    #[test]
    #[should_panic]
    fn fileparser_use_symbol_bad_before_define() {
        let contents = "50,25\nFIRST 20,15\n:FIRST\n0,1\n1,1\n2,1\n;\n2,4\n3,5\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    #[should_panic]
    fn fileparser_use_symbol_bad_unknown() {
        let contents = "50,25\nFIRST 20,15\n2,4\n3,5\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    fn fileparser_use_symbol_good_twice() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n1,1\n2,1\n;\n4,6\n5,7\n6,8\nFIRST 30,15\nFIRST 20,15\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&(21, 16)));
        assert!(cells.contains(&(31, 16)));
    }

    #[test]
    fn fileparser_define_symbol_nested() {
        let contents = "50,25\n2,4\n:FIRST\n0,1\n1,1\n2,1\n;\n3,5\n:SECOND\nFIRST 0,0\nFIRST 5,5\n;\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&CELL_DATA[1]));
        assert!(cells.contains(&CELL_DATA[5]));
    }

    #[test]
    fn fileparser_use_symbol_nested_in_bounds() {
        let contents = "50,25\n2,4\n:FIRST\n0,1\n1,1\n2,1\n;\n3,5\n:SECOND\nFIRST 0,0\nFIRST 5,5\n;\n4,6\n5,7\nSECOND 10, 3\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();

        assert!(cells.contains(&(10, 4)));
        assert!(cells.contains(&(15, 9)));
    }

    #[test]
    #[should_panic]
    fn fileparser_use_symbol_nested_out_bounds() {
        let contents = "50,25\n2,4\n:FIRST\n0,1\n1,1\n2,1\n;\n3,5\n:SECOND\nFIRST 0,0\nFIRST 5,5\n;\n4,6\n5,7\nSECOND 60, 3\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    #[should_panic]
    fn fileparser_define_symbol_bad_terminator() {
        let contents = "50,25\n2,4\n3,5\n4,6\n;\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }

    #[test]
    fn fileparser_define_symbol_good_no_terminator() {
        let contents = "50,25\n2,4\n3,5\n:FIRST\n0,1\n2,1\n3,1\n4,6\n5,7\n6,8\n";
        let mut file = create_fileparser(&contents);
        let mut cells: Vec<Cell> = file.iter().collect();
    }
}
