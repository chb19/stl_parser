# Rust STL Parser and Generator

This is a Rust library and command-line tool for reading and writing STL (Stereolithography) files in both binary and ASCII formats. It also includes functions for generating simple 3D cube and cone shapes and exporting them to STL files.

## Features

- Reading binary and ASCII STL files.
- Writing binary and ASCII STL files.
- Generating 3D cube and cone shapes.

## Usage

### Command-Line Tool

You can use the command-line tool `stl_parser` to perform various operations on STL files.

#### Parsing an STL File

```cmd
C:\> stl_parser.exe --output-format <OUTPUT_FORMAT> parse <INPUT>
```

#### Generating cube shape
```cmd
C:\> stl_parser.exe --output-format <OUTPUT_FORMAT> cube <A> <ORIGIN>
```

#### Generating cone shape
```cmd
C:\> stl_parser.exe --output-format <OUTPUT_FORMAT> cone <N> <R> <H> <ORIGIN>
```
