// src/lib.rs

use std::io::{self, BufReader, BufWriter, Read, Write, Stderr, Stdin, Stdout};
// use std::fmt;
use std::fs;
use std::process::Command;
// use std::process;
use docopt::Docopt;
use clipboard_win::{formats, get_clipboard, set_clipboard};
use serde::Deserialize;

// winclip [-h|-v] [-w|-u|-m|-g|-r] [-b] [-l lang] [-s sublang] -c|-p|-i [file]
pub const USAGE: &str = "
WinClip

Usage:
  winclip [-h|-v] (-c|-p|-i) [file]
  winclip [-h|-v] --copy [<file>]
  winclip [-h|-v] --paste [<file>]
  winclip --help
  winclip --version

Options:
  -c, --copy         Copy STDIN to clipboard.
  -p, --paste        Paste STDOUT from clipboard.
  -h, --help         Show this screen.
  -h, --version      Show version.
";
//   winclip --copy <text>
//   winclip --interactive
//     -i, --interactive  Interactive mode.

#[derive(Debug, Deserialize)]
pub struct Args {
    // pub arg_text: Option<String>,
    pub flag_copy: bool,
    pub flag_paste: bool,
    // pub flag_interactive: bool,
    pub flag_version: bool,
    pub arg_file : Option<String>,
}

pub fn lib_main() {
    std::process::exit( inner_main(io::stdin(), io::stdout(), io::stderr()) );
}

pub fn inner_main(mut inp : impl Read, mut out: impl Write, mut err: impl Write) -> i32 {
    let version = "WinClip 1.0.0";
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        write!(out, "{}", version);
        return 0;
    }
/*
    if args.flag_copy {
        if let Some(text) = args.arg_text {
            return copy_text(&text, out, err);
        }
        write!(err, "Error: No text provided for copy command.");
        return 1;
    }
 */
    if args.flag_paste {
        if let Some(file_path) = args.arg_file {
            let mut dst = fs::File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(file_path)
                .expect("Unable to open, for writing, file '{file_path}'");
            return paste_text(dst, err);
        } else {
            return paste_text(out, err);
        }
    }
    /*
    if args.flag_interactive {
        return interactive_mode(inp, out, err);
    }*/
    write!(err, "Error: No recognized action [internal error]");
    return 1;
}

/*
pub fn copy_text(mut out: impl Write, mut err: impl Write) -> i32 {
    if let Err(e) = set_clipboard(formats::Unicode, text) {
        write!(err, "Failed to copy text to clipboard: {}", e)
            .expect("writen to <err>");
        return 1;
    }
    write!(out, "Text copied to clipboard.").expect("writen to <out>");
    return 0;
}
*/

pub fn paste_text(mut out: impl Write, mut err: impl Write) -> i32 {
    match get_clipboard::<String, _>(formats::Unicode) {
        Ok(text) => {
            write!(out, "{}", text).expect("writen to <out>");
            0
        }
        Err(e) => {
            write!(err, "Error: Failed to get text from clipboard: {}", e)
                .expect("writen to <err>");
            1
        }
    }
}

/*
#[allow(dead_code)]
pub fn interactive_mode(mut out: impl Write, mut err: impl Write) -> i32 {
    loop {
        // ? Loop Termination CONDITION ?
        if paste_text(out, err) == 0 {
            return 1;
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader, BufWriter, Read, Write, Stderr, Stdin, Stdout};
    use clipboard_win::{formats, get_clipboard, set_clipboard};
    use serial_test::serial;
    use assert_fs::TempDir;
    use std::fmt;

    // to buffer
    #[test]
    #[serial]
    fn test_paste_text() {
        let input_text = "Test paste\n";
        set_clipboard(formats::Unicode, input_text).expect("Failed to set clipboard text");
        // let writer = BufWriter::new("");
        let mut writer : Vec<u8> = Vec::with_capacity(128);
        paste_text(&mut writer, io::stderr());
        let pasted_text = String::from_utf8(writer).unwrap();
        assert_eq!(pasted_text, input_text);
    }

    // OUTPUT only, file may or may exist, it is created if necessary, overwritten otherwise
    #[test]
    #[allow(dead_code)]
    #[serial]
    fn output_to_file() -> Result<(), Box<dyn std::error::Error>> {
        let fname = "output_to_file";
        let text_clipboard = format!("{fname}():\nActual content\nfor test\n");
        set_clipboard(formats::Unicode, &text_clipboard);

        let tmp_dir = TempDir::new().unwrap();
        let target_path = tmp_dir.path().join(format!("{fname}.txt"));
        let target_file = target_path.to_str().expect("Get String from PathBuf");

        let mut outf = fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(target_file)
            .expect("Unable to open/create, for writing, '{target_file}'.");

        paste_text(outf, io::stderr());

        let contents = fs::read_to_string(&target_file)
            .expect("Unable to read contents of file '{&target_file}'");

        assert_eq!(text_clipboard, contents);

        tmp_dir.close().unwrap();
        Ok(())
    }

    /*
    #[test]
    #[serial]
    fn test_copy_text() {
        let input_text = "Test copy\n";
        io::BufWriter("");
        copy_text(input_text);
        let clipboard_content = get_clipboard::<String, _>(formats::Unicode).unwrap();
        assert_eq!(clipboard_content, input_text);
    }
    */
}
