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

mod test_env {
    use crate::env::*;

    #[test]
    fn unwrap_exit_some() {
        let opt = Some(99);

        assert_eq!(opt.unwrap_or_exit(String::from("hello, world!")), 99);
    }

    #[test]
    fn unwrap_exit_none() {
        let opt: Option<i32> = None;

        // Can't test this as it kills the test process...
        //assert_eq!(opt.unwrap_or_exit(String::from("hello, world!")), 99);
    }

    #[test]
    fn unwrap_exit_ok() {
        let res: Result<i32,()> = Ok(99);

        assert_eq!(res.unwrap_or_exit(String::from("hello, world!")), 99);
    }

    #[test]
    fn unwrap_exit_err() {
        let res: Result<i32,String> = Err(String::from("bang"));

        // Can't test this as it kills the test process...
        //assert_eq!(res.unwrap_or_exit(String::from("hello, world!")), 99);
    }

    #[test]
    fn unwrap_display_some() {
        let opt = Some(99);

        assert_eq!(opt.unwrap_display(), "99");
    }

    #[test]
    fn unwrap_display_none() {
        let opt: Option<i32> = None;

        assert_eq!(opt.unwrap_display(), "None");
    }

    #[test]
    fn unwrap_display_or_some() {
        let opt = Some(99);

        assert_eq!(opt.unwrap_display_or("hello, world!"), "99");
    }

    #[test]
    fn unwrap_display_or_none() {
        let opt: Option<i32> = None;

        assert_eq!(opt.unwrap_display_or("hello, world!"), "hello, world!");
    }
}