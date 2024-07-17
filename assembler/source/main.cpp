#include "../include/reader.hpp"
#include <iostream>
#include <ostream>
#include <string>

int main() {
    std::string input = "Hello, World!";
    Reader      reader(input);
    std::cout << reader.nextChar() << std::endl;
    return 0;
}
