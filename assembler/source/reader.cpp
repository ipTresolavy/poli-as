#include "../include/reader.hpp"
#include <string>

Reader::Reader(std::string& input) : position(0) { this->input = input; }

char Reader::nextChar() {
    char curr = this->input[this->position];
    this->position++;
    return curr;
}

char Reader::peekNextChar() { return this->input[this->position + 1]; }

bool isWhitespace(char c) { return c == ' ' || c == '\n' || c == '\t'; }

void Reader::consumeWhitespace() {
    while (isWhitespace(this->input[this->position])) {
        this->position++;
    }
}
