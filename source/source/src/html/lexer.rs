#[derive(Debug)]
pub enum Token {
    LessThan,
    GreaterThan,
    Slash,
    Equals,
    Identifier(String),
    String(String),
    Text(String),
    
}


pub fn tokenize(html: &str) -> Vec<Token> {
    let chars: Vec<char> = html.chars().collect();
    let mut tokens = Vec::new();

    let mut inside_tag = false;
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {

            // <
            '<' => {
                tokens.push(Token::LessThan);
                inside_tag = true;
                i += 1;
            }

            // >
            '>' => {
                tokens.push(Token::GreaterThan);
                inside_tag = false;
                i += 1;
            }

            // /
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }

            // =
            '=' => {
                tokens.push(Token::Equals);
                i += 1;
            }

            // Tag
            c if c.is_alphanumeric() => {

                let mut word = String::new();

                while i < chars.len() && chars[i].is_alphanumeric() {
                    word.push(chars[i]);
                    i += 1;
                }

                if inside_tag {
                    tokens.push(Token::Identifier(word));
                } else {
                    tokens.push(Token::Text(word));
                }
            }

            // String
            c if c == '"' && inside_tag => {
                i += 1;
                let mut string_content = String::new();

                while i < chars.len() && chars[i] != '"' {
                    string_content.push(chars[i]);
                    i += 1;
                }

                tokens.push(Token::String(string_content));
                i += 1;
            }

            // Whitespace
            c if c.is_whitespace() && inside_tag => {
                i += 1;
            }

            // Other characters (text)
            _ => {

                let mut text = String::new();

                while i < chars.len() && chars[i] != '<' {
                    text.push(chars[i]);
                    i += 1;
                }

                let text = text.trim();

                if !text.is_empty() {
                    tokens.push(Token::Text(text.to_string()));
                }
            }
        }
    }

    tokens
}