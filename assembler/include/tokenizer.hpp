#include "reader.hpp"
#ifndef TOKENIZER_H
#define TOKENIZER_H

#include <string>
enum TokenType {
    REGISTER,
    INSTRUCTION,
    IMMEDIATE,
    LABEL,
    DIRECTIVE,
    LPAREN,
    RPAREN,
    COMMA,
    BANG,
    ILLEGAL,
};

struct Token {
    TokenType   type;
    std::string literal;
};

class Tokenizer {
  public:
    explicit Tokenizer(Reader reader);
    Token nextToken();

  private:
    Reader reader;
};

#endif
