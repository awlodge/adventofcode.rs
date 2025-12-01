use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
    let mut output: Vec<T> = Vec::new();
    for x in input.split('\n') {
        match x.trim().parse::<T>() {
            Ok(r) => output.push(r),
            Err(e) => return Err(e),
        }
    }
    Ok(output)
}
