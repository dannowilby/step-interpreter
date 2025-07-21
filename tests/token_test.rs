use step_compiler::compilation::token::{Token, TokenSpan, tokenize};

#[test]
fn test_empty_string() {
    // Test with empty input
    let result = tokenize(String::from(""));
    assert_eq!(result.len(), 0, "Empty string should produce no tokens");
}

#[test]
fn test_long_string() {
    let input = String::from("(1 + 2) * 3 + 4 * (778 * 99) / 45.5 + 33 * 2");
    let result = tokenize(input);

    assert_eq!(result.len(), 23);

    // check each token has been parsed correctly
    assert_eq!(result[0].span, 0);
    assert!(matches!(result[0].token, Token::LParen));

    assert_eq!(result[1].span, 1);
    assert!(matches!(result[1].token, Token::Const(1)));

    assert_eq!(result[2].span, 3);
    assert!(matches!(result[2].token, Token::Operator('+')));

    assert_eq!(result[3].span, 5);
    assert!(matches!(result[3].token, Token::Const(2)));

    assert_eq!(result[4].span, 6);
    assert!(matches!(result[4].token, Token::RParen));

    assert_eq!(result[5].span, 8);
    assert!(matches!(result[5].token, Token::Operator('*')));

    assert_eq!(result[6].span, 10);
    assert!(matches!(result[6].token, Token::Const(3)));

    assert_eq!(result[7].span, 12);
    assert!(matches!(result[7].token, Token::Operator('+')));

    assert_eq!(result[8].span, 14);
    assert!(matches!(result[8].token, Token::Const(4)));

    assert_eq!(result[9].span, 16);
    assert!(matches!(result[9].token, Token::Operator('*')));

    assert_eq!(result[10].span, 18);
    assert!(matches!(result[10].token, Token::LParen));

    assert_eq!(result[11].span, 19);
    assert!(matches!(result[11].token, Token::Const(778)));

    assert_eq!(result[12].span, 23);
    assert!(matches!(result[12].token, Token::Operator('*')));

    assert_eq!(result[13].span, 25);
    assert!(matches!(result[13].token, Token::Const(99)));

    assert_eq!(result[14].span, 27);
    assert!(matches!(result[14].token, Token::RParen));

    assert_eq!(result[15].span, 29);
    assert!(matches!(result[15].token, Token::UnIdent('/')));

    assert_eq!(result[16].span, 31);
    assert!(matches!(result[16].token, Token::Const(45)));

    assert_eq!(result[17].span, 33);
    assert!(matches!(result[17].token, Token::UnIdent('.')));

    assert_eq!(result[18].span, 34);
    assert!(matches!(result[18].token, Token::Const(5)));

    assert_eq!(result[19].span, 36);
    assert!(matches!(result[19].token, Token::Operator('+')));

    assert_eq!(result[20].span, 38);
    assert!(matches!(result[20].token, Token::Const(33)));

    assert_eq!(result[21].span, 41);
    assert!(matches!(result[21].token, Token::Operator('*')));

    assert_eq!(result[22].span, 43);
    assert!(matches!(result[22].token, Token::Const(2)));
}

#[test]
fn test_single_character_tokens() {
    // Test opening parenthesis
    let result = tokenize(String::from("("));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::LParen));
    assert_eq!(result[0].span, 0);

    // Test closing parenthesis
    let result = tokenize(String::from(")"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::RParen));
    assert_eq!(result[0].span, 0);

    // Test plus operator
    let result = tokenize(String::from("+"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::Operator('+')));
    assert_eq!(result[0].span, 0);

    // Test multiplication operator
    let result = tokenize(String::from("*"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::Operator('*')));
    assert_eq!(result[0].span, 0);
}

#[test]
fn test_number_tokens() {
    // Test single digit number
    let result = tokenize(String::from("5"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::Const(5)));
    assert_eq!(result[0].span, 0);

    // Test multi-digit number
    let result = tokenize(String::from("123"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::Const(123)));
    assert_eq!(result[0].span, 0);

    // Test large number
    let result = tokenize(String::from("4294967295")); // Max u32 value
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::Const(4294967295)));
    assert_eq!(result[0].span, 0);
}

#[test]
#[should_panic]
fn test_too_large_number_tokens() {
    // Test number larger than max u32
    let result = tokenize(String::from("42949672950")); // Larger than max u32 value

    // if numbers larger than the max u32 were able to be parsed, then the
    // following code would execute and pass
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].span, 0);
}

#[test]
fn test_unidentified_tokens() {
    // Test a character that doesn't match any defined token
    let result = tokenize(String::from("x"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::UnIdent('x')));
    assert_eq!(result[0].span, 0);

    // Test special characters
    let result = tokenize(String::from("#"));
    assert_eq!(result.len(), 1);
    assert!(matches!(result[0].token, Token::UnIdent('#')));
    assert_eq!(result[0].span, 0);
}

#[test]
fn test_whitespace_handling() {
    // Test single space
    let result = tokenize(String::from(" "));
    assert_eq!(result.len(), 0, "Single space should be ignored");

    // Test multiple spaces
    let result = tokenize(String::from("   "));
    assert_eq!(result.len(), 0, "Multiple spaces should be ignored");

    // Test space between tokens
    let result = tokenize(String::from("( )"));
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].token, Token::LParen));
    assert!(matches!(result[1].token, Token::RParen));

    // Test leading, trailing and middle spaces
    let result = tokenize(String::from("  (  123  )  "));
    assert_eq!(result.len(), 3);
    assert!(matches!(result[0].token, Token::LParen));
    assert!(matches!(result[1].token, Token::Const(123)));
    assert!(matches!(result[2].token, Token::RParen));
}

#[test]
fn test_mixed_tokens() {
    // Test a simple expression
    let result = tokenize(String::from("(1+2)"));
    assert_eq!(result.len(), 5);

    assert!(matches!(result[0].token, Token::LParen));
    assert!(matches!(result[1].token, Token::Const(1)));
    assert!(matches!(result[2].token, Token::Operator('+')));
    assert!(matches!(result[3].token, Token::Const(2)));
    assert!(matches!(result[4].token, Token::RParen));

    // Test a more complex expression with spaces
    let result = tokenize(String::from("( 12 + 34 * 56 )"));
    assert_eq!(result.len(), 7);
    assert!(matches!(result[0].token, Token::LParen));
    assert!(matches!(result[1].token, Token::Const(12)));
    assert!(matches!(result[2].token, Token::Operator('+')));
    assert!(matches!(result[3].token, Token::Const(34)));
    assert!(matches!(result[4].token, Token::Operator('*')));
    assert!(matches!(result[5].token, Token::Const(56)));
    assert!(matches!(result[6].token, Token::RParen));
}

#[test]
fn test_numbers_followed_by_other_tokens() {
    // Test number followed by parenthesis
    let result = tokenize(String::from("123)"));
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].token, Token::Const(123)));
    assert!(matches!(result[1].token, Token::RParen));
    assert_eq!(result[0].span, 0);
    assert_eq!(result[1].span, 3);

    // Test number followed by operator
    let result = tokenize(String::from("456+"));
    assert_eq!(result.len(), 2);
    assert!(matches!(result[0].token, Token::Const(456)));
    assert!(matches!(result[1].token, Token::Operator('+')));
    assert_eq!(result[0].span, 0);
    assert_eq!(result[1].span, 3);
}

#[test]
fn test_span_tracking() {
    // Test span tracking in a complex expression
    let result = tokenize(String::from("(12 + 3)"));
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].span, 0); // '('
    assert_eq!(result[1].span, 1); // '12'
    assert_eq!(result[2].span, 4); // '+'
    assert_eq!(result[3].span, 6); // '3'
    assert_eq!(result[4].span, 7); // ')'

    // Test span with longer numbers
    let result = tokenize(String::from("(123+456)"));
    assert_eq!(result.len(), 5);
    assert_eq!(result[0].span, 0); // '('
    assert_eq!(result[1].span, 1); // '123'
    assert_eq!(result[2].span, 4); // '+'
    assert_eq!(result[3].span, 5); // '456'
    assert_eq!(result[4].span, 8); // ')'
}

#[test]
fn test_edge_cases() {
    // Test a string with only unidentified tokens
    let result = tokenize(String::from("xyz"));
    assert_eq!(result.len(), 3);
    for (i, c) in "xyz".chars().enumerate() {
        assert!(matches!(result[i].token, Token::UnIdent(c)));
    }

    // Test a mix of valid and invalid tokens
    let result = tokenize(String::from("a(1+b)c"));
    assert_eq!(result.len(), 7);
    assert!(matches!(result[0].token, Token::UnIdent('a')));
    assert!(matches!(result[1].token, Token::LParen));
    assert!(matches!(result[2].token, Token::Const(1)));
    assert!(matches!(result[3].token, Token::Operator('+')));
    assert!(matches!(result[4].token, Token::UnIdent('b')));
    assert!(matches!(result[5].token, Token::RParen));
    assert!(matches!(result[6].token, Token::UnIdent('c')));
}
