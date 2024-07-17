#include "../include/tokenizer.hpp"
#include <cctype>
#include <regex>
#include <string>

Tokenizer::Tokenizer(Reader reader) : reader(reader) { this->reader = reader; }

#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

const std::unordered_map<std::string, std::vector<std::string>>
    INSTRUCTION_GROUPS = {
        {"arithmetic",
         {"add", "adc", "sub", "sbc", "rsb", "mul", "mla", "mls", "umull",
          "umlal", "smull", "smlal", "udiv", "sdiv"}},
        {"bit_operations", {"and", "bic", "orr", "orn", "eor"}},
        {"tests", {"cmp", "cmn", "tst", "teq"}},
        {"register_moves", {"mov", "lsr", "asr", "lsl", "ror", "rrx"}},
        {"load_store", {"ldr", "str"}},
        {"branch", {"blx", "bl", "bx", "b", "cbnz", "cbz", "tbh", "tbb"}}};

const std::unordered_set<std::string> CONDITION_CODES = {
    "eq", "ne", "cs", "hs", "cc", "lo", "mi", "pl", "vs",
    "vc", "hi", "ls", "ge", "lt", "gt", "le", "al", ""};

const std::unordered_set<std::string> SAVE_REGISTER = {"s"};

struct InstructionToken {
    std::string instructionGroup;
    std::string instructionType;
    std::string condition;
    bool        saveRegister;
};

std::string extractSaveRegister(const std::smatch& match) {
    if (match[3].matched) {
        return match[3];
    }
    return "";
}

std::string extractCondition(const std::smatch& match) {
    if (!match[2].matched || match[2].str().empty()) {
        return "al";
    }
    if (CONDITION_CODES.find(match[2]) != CONDITION_CODES.end()) {
        return match[2];
    }
    throw std::runtime_error("Unknown condition: " + match[2].str());
}

std::pair<std::string, std::string>
extractInstruction(const std::string& opcode) {
    for (const auto& group : INSTRUCTION_GROUPS) {
        if (std::find(group.second.begin(), group.second.end(), opcode) !=
            group.second.end()) {
            return {group.first, opcode};
        }
    }
    throw std::runtime_error("Unknown instruction: " + opcode);
}

InstructionToken parse(const std::string& instructionStr) {
    std::regex  instructionRegex(R"((\w+)(\w+)?\s*(\w+)?)");
    std::smatch match;

    if (!std::regex_match(instructionStr, match, instructionRegex)) {
        throw std::runtime_error("Invalid instruction format");
    }

    auto [group, opcode]     = extractInstruction(match[1]);
    std::string condition    = extractCondition(match);
    std::string saveRegister = extractSaveRegister(match);

    return {group, opcode, condition, !saveRegister.empty()};
}

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

    if (curr[0] == '#') {
        return Token{TokenType::IMMEDIATE, curr};
    }

    if (curr[0] == 'r' && isdigit(curr[1])) {
        return Token{TokenType::REGISTER, curr};
    }

    return Token{TokenType::ILLEGAL, curr};
}
