use library::lexeme::definition::{TokenKind, TokenType};

pub fn identify_token_type(tok: &Vec<char>) -> (TokenType, TokenKind) {
    let tok_value: String = tok.iter().collect();
    match tok_value.as_str() {
        "auto" => (TokenType::Auto, TokenKind::DataTypes),
        "int" => (TokenType::Integer, TokenKind::DataTypes),
        "long" => (TokenType::Long, TokenKind::DataTypes),
        "char" => (TokenType::Character, TokenKind::DataTypes),
        "float" => (TokenType::Float, TokenKind::DataTypes),
        "double" => (TokenType::Double, TokenKind::DataTypes),
        "short" => (TokenType::Short, TokenKind::DataTypes),
        "bool" => (TokenType::Boolean, TokenKind::DataTypes),
        "signed" => (TokenType::Signed, TokenKind::Modifiers),
        "unsigned" => (TokenType::Unsigned, TokenKind::Modifiers),
        "typedef" => (TokenType::Typedef, TokenKind::Typedef),
        "class" => (TokenType::KeywordClass, TokenKind::Keyword),
        "enum" => (TokenType::KeywordEnum, TokenKind::Keyword),
        "union" => (TokenType::KeywordUnion, TokenKind::Keyword),
        "break" => (TokenType::KeywordBreak, TokenKind::Keyword),
        "continue" => (TokenType::KeywordContinue, TokenKind::Keyword),
        "for" => (TokenType::KeywordFor, TokenKind::Keyword),
        "while" => (TokenType::KeywordWhile, TokenKind::Keyword),
        "switch" => (TokenType::KeywordSwitch, TokenKind::Keyword),
        "if" => (TokenType::KeywordIf, TokenKind::Keyword),
        "else" => (TokenType::KeywordElse, TokenKind::Keyword),
        "do" => (TokenType::KeywordDo, TokenKind::Keyword),
        "public" => (TokenType::KeywordPublic, TokenKind::Modifiers),
        "private" => (TokenType::keywordPrivate, TokenKind::Modifiers),
        "protected" => (TokenType::KeywordProtected, TokenKind::Modifiers),
        "case" => (TokenType::KeywordCase, TokenKind::Keyword),
        "static" => (TokenType::KeywordStatic, TokenKind::Modifiers),
        "const" => (TokenType::KeywordConst, TokenKind::Keyword),
        "default" => (TokenType::KeywordDefault, TokenKind::Keyword),
        "return" => (TokenType::KeywordReturn, TokenKind::Keyword),
        "true" => (TokenType::True, TokenKind::Values),
        "false" => (TokenType::False, TokenKind::Values),
        "new" => (TokenType::KeywordNew, TokenKind::Keyword),
        "main" => (TokenType::Main, TokenKind::Identifiers),
        "void" => (TokenType::Void, TokenKind::DataTypes),
        "struct" => (TokenType::KeywordStruct, TokenKind::Keyword),
        "string" => (TokenType::StringValue, TokenKind::DataTypes),
        "NULL" => (TokenType::Null, TokenKind::Keyword),
        "#include" => (TokenType::HeaderInclude, TokenKind::Preprocessors),
        "#define" => (TokenType::HeaderDefine, TokenKind::Preprocessors),
        "#ifdef" => (TokenType::HeaderIfDefineStart, TokenKind::Preprocessors),
        "#endif" => (TokenType::HeaderIfDefineEnd, TokenKind::Preprocessors),
        "sizeof" => (TokenType::SizeOf, TokenKind::UnaryOperators),
        _ => (TokenType::Identifier, TokenKind::Identifiers),
    }
}
