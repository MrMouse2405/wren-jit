use logos::Logos;

/**
 *
 * Lexer
 *
 * Uses Logos for lexing tokens.
 *
 * Remember when lexing:
 *  Longer beats shorter.
 *  Specific beats generic.
 *
 * Priorities:
 *  Keyword = 10
 *  Identifier = 4,3
 *  Hexadecimal = 2
 *  Number = 1
 *
 *  
 *
 */

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] //skip whitespace,newlines,etc.
pub enum Token {
    /*

        Comments

    */
    #[token("//")]
    LineComment,

    #[token("/*")]
    BlockCommentPrefix,

    #[token("*/")]
    BlockCommentSuffix,

    /*

        Groupings

    */
    #[token("(")]
    OpenParenthesis,

    #[token(")")]
    CloseParenthesis,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    /*

       Arithmetic Operators

    */
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    /*

       Logical Operators

    */
    #[token("!")]
    Negate,

    #[token("==")]
    Equals,

    #[token("!=")]
    NotEqual,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LesserThan,

    #[token(">=")]
    GreaterThanEqualTo,

    #[token("<=")]
    LesserThanEqualTo,

    #[token("&&")]
    LogicalAnd,

    #[token("||")]
    LogicalOr,

    #[token("is", priority = 10)]
    Is,

    #[token("?")]
    Ternary,

    /*

        Bitwise

    */
    #[token("^")]
    BitwiseXOR,

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[token("<<")]
    BitwiseLeftShift,

    #[token(">>")]
    BitwiseRightShift,

    /*

        Quotation

    */
    #[token("'")]
    SingleQuote,

    #[token("\"")]
    DoubleQuote,

    /*

        Other

    */
    #[token("=")]
    Assignment,

    #[token(".")]
    Dot,

    #[token("..")]
    InclusiveRange,

    #[token("...")]
    ExclusiveRange,

    #[token(",")]
    Comma,

    #[token("\\")]
    BackSlash,

    #[token(":")]
    Colon,

    #[token("#")]
    HashTag,
    /*

       Keywords

    */
    // Control Flow
    #[token("for", priority = 10)]
    For,

    #[token("while", priority = 10)]
    While,

    #[token("break", priority = 10)]
    Break,

    #[token("continue", priority = 10)]
    Continue,

    #[token("return", priority = 10)]
    Return,

    #[token("if", priority = 10)]
    If,

    #[token("else", priority = 10)]
    Else,

    // OOP
    #[token("class", priority = 10)]
    Class,

    #[token("construct", priority = 10)]
    Construct,

    #[token("static", priority = 10)]
    Static,

    #[token("super", priority = 10)]
    Super,

    #[token("this", priority = 10)]
    This,

    // Constants
    #[token("true", priority = 10)]
    True,

    #[token("false", priority = 10)]
    False,

    #[token("null", priority = 10)]
    Null,

    // Modules
    #[token("import", priority = 10)]
    Import,

    #[token("as", priority = 10)]
    As,

    // Variable Declaration
    #[token("var", priority = 10)]
    Var,

    // Iterator Protocol
    #[token("in", priority = 10)]
    In,

    // Foreign API
    #[token("foreign", priority = 10)]
    Foreign,

    /**
     * Literals
     *
     * Can be Identifiers,
     * Numbers, or even Strings.
     *
     * Rust does not support
     * look around asserrtions
     * in regex.
     *
     * So we will make the parser
     * do it.
     */

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 3)]
    Identifier,

    #[regex(r"0[xX][0-9a-fA-F]+", priority = 2)]
    Hexadecimal,

    #[regex(r"[+-]?\d+", priority = 1)]
    #[regex(r"[+-]?\d+\.", priority = 1)]
    #[regex(r"[+-]?\.\d+", priority = 1)]
    #[regex(r"[+-]?\d+\.\d+", priority = 1)]
    Number,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn identifiers() {
        for result in super::Token::lexer(
            "hi
        camelCase
        PascalCase
        _under_score
        ALL_CAPS
        breakreturn
        iflalala
        _continue
        _
        a
        X
        __python_loves_this__
        _12345
        _AB_123_C_2
        A1234
        ",
        ) {
            assert_eq!(result, Ok(Token::Identifier));
        }
    }

    #[test]
    fn numbers() {
        for result in super::Token::lexer(
            "123
        456
        123434
        1234.5
        12.
        12345.6789
        .45
        +2
        -2
        +.1
        +2.1
        -.5
        ",
        ) {
            assert_eq!(result, Ok(Token::Number));
        }
    }

    #[test]
    fn hexadecimal() {
        for result in super::Token::lexer(
            "0x123
        0XABFE
        0x60CD
        ",
        ) {
            assert_eq!(result, Ok(Token::Hexadecimal));
        }
    }

    #[test]
    fn various() {
        let mut lex = super::Token::lexer(
            "
            ok
            Nuh_uh
            123why
            JesusNo
            valid1234
            _123_WOW
            LOL123.456LOL
            Why.brother.why
            wtf.123
            NAHHH+
            NAHHHHH.+
            +.+
            :)
            class something {stopIamTiredOfWritingTests}
            0x1234
            0x_1234
            _0x_AB
        ",
        );

        //ok
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        // Nuh_uh
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        // 123why
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        //JesusNo
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        //valid1234
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        //_123_WOW
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        //LOL123.456LOL
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        //Why.brother.why
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        // wtf.123
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        // NAHHH+
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        // NAHHHHH.+
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        // +.+
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        assert_eq!(lex.next(), Some(Ok(Token::Dot)));
        assert_eq!(lex.next(), Some(Ok(Token::Plus)));
        // :)
        assert_eq!(lex.next(), Some(Ok(Token::Colon)));
        assert_eq!(lex.next(), Some(Ok(Token::CloseParenthesis)));
        // class something {stopIamTiredOfWritingTests}
        assert_eq!(lex.next(), Some(Ok(Token::Class)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::OpenBrace)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        assert_eq!(lex.next(), Some(Ok(Token::CloseBrace)));
        // 0x1234
        assert_eq!(lex.next(), Some(Ok(Token::Hexadecimal)));
        // 0x_1234
        assert_eq!(lex.next(), Some(Ok(Token::Number)));
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
        // _0x_AB
        assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
    }
}

/*

    Refer to this for operator precedence table
    https://wren.io/syntax.html#precedence-and-associativity

*/
