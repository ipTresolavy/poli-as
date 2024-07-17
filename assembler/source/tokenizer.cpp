#include "../include/tokenizer.hpp"
#include <cctype>
#include <string>

Tokenizer::Tokenizer(Reader reader) : reader(reader) { this->reader = reader; }

Token Tokenizer::nextToken() {
    this->reader.consumeWhitespace();
    std::string curr = this->reader.consumeToWhitespace();

    if (curr == "]") {
        return Token{TokenType::RPAREN, curr};
    }

    if (curr == "[") {
        return Token{TokenType::LPAREN, curr};
    }

    if (curr == ",") {
        return Token{TokenType::COMMA, curr};
    }

    if (curr == "!") {
        return Token{TokenType::BANG, curr};
    }

    if (curr[0] == '.') {
        return Token{TokenType::DIRECTIVE, curr};
    }

    if (curr[0] == 'r' && isdigit(curr[1])) {
        return Token{TokenType::REGISTER, curr};
    }

    return Token{TokenType::ILLEGAL, curr};
}
