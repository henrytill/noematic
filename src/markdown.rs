use pulldown_cmark::{Options, Parser};

#[derive(Debug)]
struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[allow(dead_code)]
fn parse_markdown(markdown: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::empty();
    let mut parser = Parser::new_ext(markdown, options);
    if parser.next().is_none() {
        return Err(Box::new(Error {}));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO_WORLD_MD: &str = r#"
    # Hello World

    This is a test
    "#;

    #[test]
    fn it_works() {
        assert!(parse_markdown(HELLO_WORLD_MD).is_ok());
    }
}
