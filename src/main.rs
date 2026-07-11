use image::{ImageReader, Rgba};
use std::env;
use std::fmt::Write;
use std::fs::File;
use std::io::Write as IoWrite;

const ARG_ASCII: &str = "--ascii";

fn parse_command_line() -> Result<(String, String, bool), String> {
    let mut args: Vec<String> = env::args().collect();
    assert!(args.len() > 0);
    let file_name = args.remove(0);
    let mut files: Vec<String> = Vec::new();
    let mut is_ascii_output = false;
    for arg in args {
        if arg == ARG_ASCII {
            is_ascii_output = true;
            continue;
        }

        files.push(arg);
    }

    if files.len() != 2 {
        return Err(format!(
            "{file_name} [{ARG_ASCII}] <image-file> <output-file>\n
{ARG_ASCII}: the output file will be generated as ASCII with 16 hexadecimal values per line"
        ));
    }

    Ok((files.remove(0), files.remove(0), is_ascii_output))
}

fn main() {
    let (input_file_name, output_file_name, ascii) = match parse_command_line() {
        Ok(a) => a,
        Err(msg) => {
            println!("{}", msg);
            return;
        }
    };

    let reader = match ImageReader::open(&input_file_name) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to open file {input_file_name}: {e:?}");
            return;
        }
    };
    let image = reader.decode().unwrap().to_rgba8();

    let mut output_file = match File::create(&output_file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file {output_file_name}: {e:?}");
            return;
        }
    };

    let mut i = 0;
    for row in 0..6 {
        for column in 0..0x10 {
            for row_pixel in 0..8 {
                let mut value = 0;
                for column_pixel in 0..8 {
                    let pixel = image.get_pixel(column * 8 + column_pixel, row * 8 + row_pixel);
                    let gray = if to_gray(pixel) < 128 { 1 } else { 0 };
                    value = (value << 1) | gray;
                }

                if let Err(e) = write_value(value, ascii, i, &mut output_file) {
                    eprintln!("Failed to write to file {output_file_name}: {e:?}");
                    return;
                }

                i = if i < 15 { i + 1 } else { 0 }
            }
        }
    }
}

fn write_value(value: u8, ascii: bool, i: u8, file: &mut File) -> std::io::Result<usize> {
    if ascii {
        let mut buffer = String::new();
        if i > 0 {
            buffer.push(' ');
            if i == 8 {
                buffer.push(' ');
            }
        }
        let _ = write!(buffer, "{value:02x}");
        if i == 15 {
            buffer.push('\n');
        }
        file.write(buffer.as_bytes())
    } else {
        file.write(&[value])
    }
}

fn to_gray(pixel: &Rgba<u8>) -> u8 {
    let [r, g, b, _a] = pixel.0;
    ((r as u16 + g as u16 + b as u16) / 3) as u8
}
