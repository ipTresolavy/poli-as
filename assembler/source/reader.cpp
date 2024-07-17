#include "../include/reader.hpp"
#include <string>

Reader::Reader(std::string& input) : position(-1) { this->input = input; }

char Reader::nextChar() {
    this->position++;
    return this->input[this->position];
}

char Reader::peekNextChar() { return this->input[this->position + 1]; }

void Reader::consumeWhitespace() {}
