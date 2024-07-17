#include "../include/reader.hpp"
#include <iostream>
#include <ostream>
#include <string>

int main() {
    std::string input = "Hello, \n World!";
    Reader      reader(input);
    reader.consumeWhitespace();
    for (int i = 0; i < 15; i++) {
        std::cout << reader.nextChar() << std::endl;
        reader.consumeWhitespace();
    }

    return 0;
}
