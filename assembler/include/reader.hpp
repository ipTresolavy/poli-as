#ifndef READER_H
#define READER_H

#include <string>
class Reader {
  public:
    explicit Reader(std::string& input);
    char        nextChar();
    char        peekNextChar();
    void        consumeWhitespace();
    std::string consumeToWhitespace();

  private:
    int         position = 0;
    std::string input;
};

#endif
