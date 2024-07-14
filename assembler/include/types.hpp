#ifndef TYPES_HPP
#define TYPES_HPP

#include <cstdint>

namespace poli_as {

/**
 * @brief DataProcessing struct represents the format of ARM Data Processing
 * instructions.
 */
struct DataProcessing {
    /**
     * @union operand2
     * @brief Operand2 can either be an immediate value or a shifted register.
     */
    union {
        /**
         * @struct reg_operand
         * @brief Structure for register operand when I = 0.
         */
        struct {
            uint32_t rm : 4;    /**< Bits 3-0: Register Rm (source register) */
            uint32_t shift : 1; /**< Bit 4: Shift by register (0 = immediate, 1
                                   = register) */
            uint32_t stype : 2; /**< Bits 6-5: Shift type (00 = LSL, 01 = LSR,
                                   10 = ASR, 11 = ROR) */
            union {
                /**
                 * @struct shift_by_reg
                 * @brief Shift by register.
                 */
                struct {
                    uint32_t z : 1;  /**< Bit 7: Zero (set to 0) */
                    uint32_t rs : 4; /**< Bits 11-8: Shift register */
                };
                uint32_t imm5 : 5; /**< Bits 11-7: Immediate shift value */
            };
        } reg_operand;       /**< Operand2 when I = 0 */
        uint32_t imm12 : 12; /**< Bits 11-0: Immediate value when I = 1 */
    } operand2; /**< Operand2 can be immediate or register shifted value */

    uint32_t rd : 4;          /**< Bits 15-12: Destination register */
    uint32_t rn : 4;          /**< Bits 19-16: First operand register */
    uint32_t s : 1;           /**< Bit 20: Set condition codes flag */
    uint32_t opcode : 4;      /**< Bits 24-21: Operation code */
    uint32_t i : 1;           /**< Bit 25: Immediate operand flag */
    uint32_t always_zero : 2; /**< Bits 27-26: Always 00 for Data Processing
                                 instructions */
    uint32_t cond : 4;        /**< Bits 31-28: Condition field */
};

/**
 * @brief LoadStore struct represents the format of ARM Load
 * and Store instructions.
 */
struct LoadStore {
    /**
     * @union offset
     * @brief Offset can either be an immediate value or a shifted register.
     */
    union {
        uint32_t
            imm12 : 12; /**< Bits 11-0: Immediate offset value when I = 1 */
        /**
         * @struct reg_offset
         * @brief Structure for register offset when I = 0.
         */
        struct {
            uint32_t rm : 4;    /**< Bits 3-0: Register Rm (source register for
                                   offset) */
            uint32_t stype : 2; /**< Bits 6-5: Shift type (00 = LSL, 01 = LSR,
                                   10 = ASR, 11 = ROR) */
            uint32_t imm5 : 5;  /**< Bits 11-7: Immediate shift value */
        } reg_offset;           /**< Offset when I = 0 */
    } offset; /**< Offset can be immediate or register shifted value */

    uint32_t rt : 4; /**< Bits 15-12: Destination (load) or source (store)
                        register */
    uint32_t rn : 4; /**< Bits 19-16: Base register */
    uint32_t p : 1;  /**< Bit 24: Pre-indexing flag */
    uint32_t u : 1;  /**< Bit 23: Up/Down bit (1 = add offset to base, 0 =
                        subtract offset) */
    uint32_t b : 1;  /**< Bit 22: Byte/Word bit (1 = unsigned byte, 0 = word) */
    uint32_t w : 1;  /**< Bit 21: Write-back flag */
    uint32_t l : 1;  /**< Bit 20: Load/Store bit (1 = load, 0 = store) */
    uint32_t i : 1;  /**< Bit 25: Immediate offset flag */
    uint32_t always_zero : 2; /**< Bits 27-26: Always 00 for Load/Store
                                 instructions */
    uint32_t cond : 4;        /**< Bits 31-28: Condition field */
};

/**
 * @brief Branch struct represents the format of ARM Branch
 * instructions.
 */
struct Branch {
    uint32_t imm24 : 24; /**< Bits 23-0: Signed 24-bit immediate value */
    uint32_t link : 1;   /**< Bit 24: Link bit (1 = branch with link) */
    uint32_t
        always_zero : 3; /**< Bits 27-25: Always 101 for Branch instructions */
    uint32_t cond : 4;   /**< Bits 31-28: Condition field */
};

/**
 * @brief BranchAndExchange struct represents the format of ARM Branch and
 * Exchange instructions.
 */
struct BranchAndExchange {
    uint32_t rm : 4; /**< Bits 3-0: Register Rm (source register for target
                        address) */
    uint32_t
        always_one : 4; /**< Bits 7-4: Always set to 0001 for BX instructions */
    uint32_t always_zero1 : 12; /**< Bits 19-8: Always set to 000000000000 for
                                   BX instructions */
    uint32_t always_one2 : 4;   /**< Bits 23-20: Always set to 0001 for BX
                                   instructions */
    uint32_t always_zero2 : 4;  /**< Bits 27-24: Always set to 0000 for BX
                                   instructions */
    uint32_t cond : 4;          /**< Bits 31-28: Condition field */
};

struct Multiply {
    uint32_t rm : 4;
    uint32_t padding1 : 4;
    uint32_t rs : 4;
    uint32_t rn : 4;
    uint32_t rd : 4;
    uint32_t s : 1;
    uint32_t a : 1;
    uint32_t padding2 : 1;
    uint32_t cond : 4;
};

struct SingleDataSwap {
    uint32_t rm : 4;
    uint32_t padding1 : 8;
    uint32_t rd : 4;
    uint32_t rn : 4;
    uint32_t b : 1;
    uint32_t padding2 : 7;
    uint32_t cond : 4;
};

struct HalfwordSignedTransfer {
    uint32_t rm : 4;
    uint32_t padding1 : 1;
    uint32_t h : 1;
    uint32_t s : 1;
    uint32_t offset_8 : 4;
    uint32_t rd : 4;
    uint32_t rn : 4;
    uint32_t l : 1;
    uint32_t w : 1;
    uint32_t b : 1;
    uint32_t u : 1;
    uint32_t p : 1;
    uint32_t i : 1;
    uint32_t cond : 4;
};

// Union of all instruction formats
union ArmInstruction {
    uint32_t               raw;
    DataProcessing         data_processing;
    LoadStore              load_store;
    Branch                 branch;
    BranchAndExchange      branch_and_exchange;
    Multiply               multiply;
    SingleDataSwap         single_data_swap;
    HalfwordSignedTransfer halfword_signed_transfer;
};

} // namespace poli_as

#endif // TYPES_HPP
