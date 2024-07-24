# High-Level Design Document for Winclip

## Introduction
Winclip is a command-line utility for accessing and manipulating the Windows clipboard from a console or MS-DOS window. It supports copying to and pasting from the clipboard, and querying clipboard data formats.

## Features
- **Copy to Clipboard**: Copies data from standard input or a file to the clipboard.
- **Paste from Clipboard**: Outputs clipboard contents to standard output or a specified file.
- **Clipboard Information**: Displays available data formats in the clipboard.
- **Text Format Options**: Supports various text formats (ANSI, OEM, Unicode, UTF-8, RTF).
- **Graphics Support**: Copies graphic images to the clipboard using built-in and OLE interfaces.
- **Language and Sublanguage Identification**: Specifies language and sublanguage for clipboard text data.
- **Version and Help**: Displays version information and help text.

## Architecture

### Components
1. **Command-Line Interface (CLI)**: Handles user input and displays output.
2. **Clipboard Manager**: Core functionality for clipboard operations.
3. **Format Handler**: Manages text and graphics formats and their conversions.
4. **File I/O**: Handles reading from and writing to files.
5. **Error Handling**: Manages errors and edge cases, such as unsupported clipboard formats.

### Workflow
1. **CLI Input Parsing**: Parses command-line arguments to determine the operation mode (copy, paste, info).
2. **Clipboard Operation Execution**:
   - **Copy Mode**: Reads input data, converts it to the specified format, and copies it to the clipboard.
   - **Paste Mode**: Reads clipboard data, converts it if necessary, and outputs it.
   - **Info Mode**: Queries and lists all available clipboard formats.
3. **Format Conversion**: Converts data between various text formats (ANSI, OEM, Unicode, UTF-8) and handles RTF and graphics.
4. **File Operations**: Manages optional file input/output if specified.
5. **Output Handling**: Directs the output to standard output or specified file.

### Error Handling
- **Unsupported Formats**: Provides user feedback for unsupported clipboard content types.
- **Locale and Encoding Issues**: Handles potential issues with console and clipboard code pages, providing guidance on necessary adjustments.

## User Interface
- **Commands**:
  - `-c`: Copy input to clipboard.
  - `-p`: Paste clipboard contents.
  - `-i`: Display clipboard data formats.
- **Options**:
  - Text format: `-w`, `-u`, `-m`, `-r`
  - Byte Order Mark: `-b`
  - Language and Sublanguage: `-l`, `-s`
  - Graphics: `-g`
  - Version and Help: `-v`, `-h`

## Example Usage
- **Copy Directory List**:
  ```sh
  dir /b | winclip -c
  ```
- **Save Clipboard to File**:
  ```sh
  winclip -p file.txt
  ```
- **Convert Clipboard to Lowercase**:
  ```sh
  winclip -p | perl -pe "tr/[A-Z]/[a-z]/" | winclip -c
  ```

## Dependencies
- Windows OS (NT, 95, 98, Me, 2000, XP)
- Intel architecture

## Limitations and Future Enhancements
- **Additional Clipboard Content Types**: Extend support to more formats.
- **Improved Locale Handling**: Enhanced handling of different locale and encoding settings.

## Conclusion
Winclip provides a robust command-line interface for clipboard operations on Windows, supporting a variety of text and graphic formats, and offering flexibility in data handling through various options and commands.

## Acknowledgments

Winclip was created by Diomidis Spinellis.  This is merely a knockoff.  For more information, visit the [official website](https://www.spinellis.gr/sw/outwit/winclip.html).
