#include "../include/reader.hpp"
#include <fstream>
#include <iostream>
#include <ostream>
#include <string>

int main(int argc, char* argv[]) {
    if (argc != 2) {
        std::cerr << "Usage: " << argv[0] << " <source file>" << std::endl;
        return 1;
    }
    std::string filename = argv[1];

    std::ifstream inputFile(filename);

    if (!inputFile.is_open()) {
        std::cerr << "Error: Could not open the file " << filename << std::endl;
        return 1;
    }

    std::string input;
    std::string line;
    while (getline(inputFile, line)) {
        input += line;
    }

    inputFile.close();

    Reader reader(input);

    while (reader.peekNextChar() != '\0') {
        std::cout << reader.nextChar();
    }

    return 0;
}
