# WORK IN PROGRESS

While functional, this is a work in progress.  See the [ROADMAP](ROADMAP.md) for which features have been implemented thus far.  Only items with a checkmark (✓) have been implemented.

# Winclip

Winclip is a command-line utility designed for accessing and manipulating the Windows clipboard. It allows you to copy data to the clipboard, paste data from the clipboard, and query clipboard data formats directly from a console or MS-DOS window.

## Features

- ✓ **Paste from Clipboard**: Output clipboard contents to standard output or a specified file.
- **Copy to Clipboard**: Copy data from standard input or a file to the clipboard.
- **Clipboard Information**: Display available data formats in the clipboard.
- **Text Format Options**: Support for various text formats including ANSI, OEM, Unicode, and UTF-8.
- **Graphics Support**: Copy graphic images to the clipboard using built-in and OLE interfaces.
- **Language and Sublanguage Identification**: Specify language and sublanguage for clipboard text data.
- ✓ **Version and Help**: Display version information and help text.

## Installation

To install Winclip, follow these steps:

1. Download the latest release from the [... Not Yet ...]().
2. Extract the downloaded archive.
3. Add the directory containing `winclip.exe` to your system's PATH.

## Usage

### Copy to Clipboard

Copy the output of a command to the clipboard:
```sh
dir /b | winclip -c
```

Copy the contents of a file to the clipboard:
```sh
winclip -c < file.txt
```

### Paste from Clipboard

Paste the clipboard contents to standard output:
```sh
winclip -p
```

Paste the clipboard contents to a file:
```sh
winclip -p > output.txt
```

### Display Clipboard Information

List all available clipboard formats:
```sh
winclip -i
```

### Format Options

Specify the text format for copying:
```sh
winclip -c -w < file.txt  # Unicode
winclip -c -u < file.txt  # UTF-8
winclip -c -m < file.txt  # OEM
winclip -c -r < file.txt  # RTF
```

### Additional Options

Specify the language and sublanguage:
```sh
winclip -c -l LANG -s SUBLANG < file.txt
```

Handle Byte Order Mark (BOM) for Unicode and UTF-8 formats:
```sh
winclip -c -b < file.txt
```

### Version and Help

Display version information:
```sh
winclip -v
```

Display help text:
```sh
winclip -h
```

## Contributing

Contributions are welcome! Please submit pull requests or issues on the [GitHub repository](https://github.com/yourusername/winclip).

## License

Winclip is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

Winclip was created by Diomidis Spinellis.  This is merely a knockoff.  For more information, visit the [official website](https://www.spinellis.gr/sw/outwit/winclip.html).
