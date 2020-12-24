use std::io::{BufRead, Write};
use std::num::ParseFloatError;

pub fn prompt<R, W>(mut reader: R, mut writer: W, question: &str) -> String
where
    R: BufRead,
    W: Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    writer.flush().expect("Unable to flush");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

pub fn get_value(s: &String) -> Result<f32, ParseFloatError> {
    s.trim().parse::<f32>()
}

#[cfg(test)]
mod tests {
    use super::{get_value, prompt};

    #[test]
    fn it_should_get_input() {
        let input = b"100.0";
        let mut output = Vec::new();

        let answer = prompt(&input[..], &mut output, "Enter your weight (kg): ");

        let output = String::from_utf8(output).expect("Not UTF-8");

        assert_eq!("Enter your weight (kg): ", output);
        assert_eq!("100.0", answer);
    }

    #[test]
    fn it_should_get_value() {
        let result = get_value(&String::from("100.0"));
        let expected = Ok(100.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_should_not_get_value() {
        let result = get_value(&String::from("test"));
        assert!(result.is_err());
    }
}
