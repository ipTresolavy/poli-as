use regex::Regex;

#[derive(Debug)]
pub enum InstructionName {
    ADC,
    ADCS,
    ADD,
    ADDS,
    ADR,
    AND,
    ANDS,
    ASR,
    ASRS,
    B,
    BFC,
    BFI,
    BIC,
    BICS,
    BKPT,
    BL,
    BLX,
    BX,
    BXJ,
    CBNZ,
    CBZ,
    CLRBHB,
    CLREX,
    CLZ,
    CMN,
    CMP,
    CPS,
    CPSID,
    CPSIE,
    CRC32,
    CRC32C,
    CSDB,
    DBG,
    DCPS1,
    DCPS2,
    DCPS3,
    DMB,
    DSB,
    EOR,
    EORS,
    ERET,
    ESB,
    HLT,
    HVC,
    ISB,
    IT,
    LDA,
    LDAB,
    LDAEX,
    LDAEXB,
    LDAEXD,
    LDAEXH,
    LDAH,
    LDC,
    LDM,
    LDMIA,
    LDMFD,
    LDMDA,
    LDMFA,
    LDMDB,
    LDMEA,
    LDMIB,
    LDMED,
    LDR,
    LDRB,
    LDRBT,
    LDRD,
    LDREX,
    LDREXB,
    LDREXD,
    LDREXH,
    LDRH,
    LDRHT,
    LDRSB,
    LDRSBT,
    LDRSH,
    LDRSHT,
    LDRT,
    LSL,
    LSLS,
    LSR,
    LSRS,
    MCR,
    MCRR,
    MLA,
    MLAS,
    MLS,
    MOV,
    MOVS,
    MOVT,
    MRC,
    MRRC,
    MRS,
    MSR,
    MUL,
    MULS,
    MVN,
    MVNS,
    NOP,
    ORN,
    ORNS,
    ORR,
    ORRS,
    PKHBT,
    PKHTB,
    PLD,
    PLDW,
    PLI,
    POP,
    PSSBB,
    PUSH,
    QADD,
    QADD16,
    QADD8,
    QASX,
    QDADD,
    QDSUB,
    QSAX,
    QSUB,
    QSUB16,
    QSUB8,
    RBIT,
    REV,
    REV16,
    REVSH,
    RFE,
    RFEDA,
    RFEDB,
    RFEIA,
    RFEIB,
    ROR,
    RORS,
    RRX,
    RRXS,
    RSB,
    RSBS,
    RSC,
    RSCS,
    SADD16,
    SADD8,
    SASX,
    SB,
    SBC,
    SBCS,
    SBFX,
    SDIV,
    SEL,
    SETEND,
    SETPAN,
    SEV,
    SEVL,
    SHADD16,
    SHADD8,
    SHASX,
    SHSAX,
    SHSUB16,
    SHSUB8,
    SMC,
    SMLABB,
    SMLABT,
    SMLATB,
    SMLATT,
    SMLAD,
    SMLADX,
    SMLAL,
    SMLALS,
    SMLALBB,
    SMLALBT,
    SMLALTB,
    SMLALTT,
    SMLALD,
    SMLALDX,
    SMLAWB,
    SMLAWT,
    SMLSD,
    SMLSDX,
    SMLSLD,
    SMLSLDX,
    SMMLA,
    SMMLAR,
    SMMLS,
    SMMLSR,
    SMMUL,
    SMMULR,
    SMUAD,
    SMUADX,
    SMULBB,
    SMULBT,
    SMULTB,
    SMULTT,
    SMULL,
    SMULLS,
    SMULWB,
    SMULWT,
    SMUSD,
    SMUSDX,
    SRS,
    SRSDA,
    SRSDB,
    SRSIA,
    SRSIB,
    SSAT,
    SSAT16,
    SSAX,
    SSBB,
    SSUB16,
    SSUB8,
    STC,
    STL,
    STLB,
    STLEX,
    STLEXB,
    STLEXD,
    STLEXH,
    STLH,
    STM,
    STMIA,
    STMEA,
    STMDA,
    STMED,
    STMDB,
    STMFD,
    STMIB,
    STMFA,
    STR,
    STRB,
    STRBT,
    STRD,
    STREX,
    STREXB,
    STREXD,
    STREXH,
    STRH,
    STRHT,
    STRT,
    SUB,
    SUBS,
    SVC,
    SXTAB,
    SXTAB16,
    SXTAH,
    SXTB,
    SXTB16,
    SXTH,
    TBB,
    TBH,
    TEQ,
    TSB,
    TST,
    UADD16,
    UADD8,
    UASX,
    UBFX,
    UDF,
    UDIV,
    UHADD16,
    UHADD8,
    UHASX,
    UHSAX,
    UHSUB16,
    UHSUB8,
    UMAAL,
    UMLAL,
    UMLALS,
    UMULL,
    UMULLS,
    UQADD16,
    UQADD8,
    UQASX,
    UQSAX,
    UQSUB16,
    UQSUB8,
    USAD8,
    USADA8,
    USAT,
    USAT16,
    USAX,
    USUB16,
    USUB8,
    UXTAB,
    UXTAB16,
    UXTAH,
    UXTB,
    UXTB16,
    UXTH,
    WFE,
    WFI,
    YIELD,
}
// Get ready for the most cursed Regex you will ever see
// Basically all istrs + condition + save register
// Do not try and modify this as you will not be able to :thumbs_up:
pub fn get_istr_regex() -> regex::Regex {
    Regex::new("^(adc|adcs|add|adds|adr|and|ands|asr|asrs|b|bfc|bfi|bic|bics|bkpt|bl|blx|bx|bxj|cbnz|cbz|clrbhb|clrex|clz|cmn|cmp|cps|cpsid|cpsie|crc32|crc32c|csdb|dbg|dcps1|dcps2|dcps3|dmb|dsb|eor|eors|eret|esb|hlt|hvc|isb|it|lda|ldab|ldaex|ldaexb|ldaexd|ldaexh|ldah|ldc|ldm|ldmia|ldmfd|ldmda|ldmfa|ldmdb|ldmea|ldmib|ldmed|ldr|ldrb|ldrbt|ldrd|ldrex|ldrexb|ldrexd|ldrexh|ldrh|ldrht|ldrsb|ldrsbt|ldrsh|ldrsht|ldrt|lsl|lsls|lsr|lsrs|mcr|mcrr|mla|mlas|mls|mov|movs|movt|mrc|mrrc|mrs|msr|mul|muls|mvn|mvns|nop|orn|orns|orr|orrs|pkhbt|pkhtb|pld|pldw|pli|pop|pssbb|push|qadd|qadd16|qadd8|qasx|qdadd|qdsub|qsax|qsub|qsub16|qsub8|rbit|rev|rev16|revsh|rfe|rfeda|rfedb|rfeia|rfeib|ror|rors|rrx|rrxs|rsb|rsbs|rsc|rscs|sadd16|sadd8|sasx|sb|sbc|sbcs|sbfx|sdiv|sel|setend|setpan|sev|sevl|shadd16|shadd8|shasx|shsax|shsub16|shsub8|smc|smlabb|smlabt|smlatb|smlatt|smlad|smladx|smlal|smlals|smlalbb|smlalbt|smlaltb|smlaltt|smlald|smlaldx|smlawb|smlawt|smlsd|smlsdx|smlsld|smlsldx|smmla|smmlar|smmls|smmlsr|smmul|smmulr|smuad|smuadx|smulbb|smulbt|smultb|smultt|smull|smulls|smulwb|smulwt|smusd|smusdx|srs|srsda|srsdb|srsia|srsib|ssat|ssat16|ssax|ssbb|ssub16|ssub8|stc|stl|stlb|stlex|stlexb|stlexd|stlexh|stlh|stm|stmia|stmea|stmda|stmed|stmdb|stmfd|stmib|stmfa|str|strb|strbt|strd|strex|strexb|strexd|strexh|strh|strht|strt|sub|subs|svc|sxtab|sxtab16|sxtah|sxtb|sxtb16|sxth|tbb|tbh|teq|tsb|tst|uadd16|uadd8|uasx|ubfx|udf|udiv|uhadd16|uhadd8|uhasx|uhsax|uhsub16|uhsub8|umaal|umlal|umlals|umull|umulls|uqadd16|uqadd8|uqasx|uqsax|uqsub16|uqsub8|usad8|usada8|usat|usat16|usax|usub16|usub8|uxtab|uxtab16|uxtah|uxtb|uxtb16|uxth|wfe|wfi|yield)(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)?(s)?$").expect("the regex should always be valid")
}

impl InstructionName {
    pub fn from_name(name: &str) -> Option<InstructionName> {
        match name.to_lowercase().as_str() {
            "adc" => Some(InstructionName::ADC),
            "add" => Some(InstructionName::ADD),
            "adr" => Some(InstructionName::ADR),
            "and" => Some(InstructionName::AND),
            "asr" => Some(InstructionName::ASR),
            "b" => Some(InstructionName::B),
            "bfc" => Some(InstructionName::BFC),
            "bfi" => Some(InstructionName::BFI),
            "bic" => Some(InstructionName::BIC),
            "bics" => Some(InstructionName::BICS),
            "bkpt" => Some(InstructionName::BKPT),
            "bl" => Some(InstructionName::BL),
            "blx" => Some(InstructionName::BLX),
            "bx" => Some(InstructionName::BX),
            "bxj" => Some(InstructionName::BXJ),
            "cbnz" => Some(InstructionName::CBNZ),
            "cbz" => Some(InstructionName::CBZ),
            "clrbhb" => Some(InstructionName::CLRBHB),
            "clrex" => Some(InstructionName::CLREX),
            "clz" => Some(InstructionName::CLZ),
            "cmn" => Some(InstructionName::CMN),
            "cmp" => Some(InstructionName::CMP),
            "cps" => Some(InstructionName::CPS),
            "cpsid" => Some(InstructionName::CPSID),
            "cpsie" => Some(InstructionName::CPSIE),
            "crc32" => Some(InstructionName::CRC32),
            "crc32c" => Some(InstructionName::CRC32C),
            "csdb" => Some(InstructionName::CSDB),
            "dbg" => Some(InstructionName::DBG),
            "dcps1" => Some(InstructionName::DCPS1),
            "dcps2" => Some(InstructionName::DCPS2),
            "dcps3" => Some(InstructionName::DCPS3),
            "dmb" => Some(InstructionName::DMB),
            "dsb" => Some(InstructionName::DSB),
            "eor" => Some(InstructionName::EOR),
            "eors" => Some(InstructionName::EORS),
            "eret" => Some(InstructionName::ERET),
            "esb" => Some(InstructionName::ESB),
            "hlt" => Some(InstructionName::HLT),
            "hvc" => Some(InstructionName::HVC),
            "isb" => Some(InstructionName::ISB),
            "it" => Some(InstructionName::IT),
            "lda" => Some(InstructionName::LDA),
            "ldab" => Some(InstructionName::LDAB),
            "ldaex" => Some(InstructionName::LDAEX),
            "ldaexb" => Some(InstructionName::LDAEXB),
            "ldaexd" => Some(InstructionName::LDAEXD),
            "ldaexh" => Some(InstructionName::LDAEXH),
            "ldah" => Some(InstructionName::LDAH),
            "ldc" => Some(InstructionName::LDC),
            "ldm" => Some(InstructionName::LDM),
            "ldmia" => Some(InstructionName::LDMIA),
            "ldmfd" => Some(InstructionName::LDMFD),
            "ldmda" => Some(InstructionName::LDMDA),
            "ldmfa" => Some(InstructionName::LDMFA),
            "ldmdb" => Some(InstructionName::LDMDB),
            "ldmea" => Some(InstructionName::LDMEA),
            "ldmib" => Some(InstructionName::LDMIB),
            "ldmed" => Some(InstructionName::LDMED),
            "ldr" => Some(InstructionName::LDR),
            "ldrb" => Some(InstructionName::LDRB),
            "ldrbt" => Some(InstructionName::LDRBT),
            "ldrd" => Some(InstructionName::LDRD),
            "ldrex" => Some(InstructionName::LDREX),
            "ldrexb" => Some(InstructionName::LDREXB),
            "ldrexd" => Some(InstructionName::LDREXD),
            "ldrexh" => Some(InstructionName::LDREXH),
            "ldrh" => Some(InstructionName::LDRH),
            "ldrht" => Some(InstructionName::LDRHT),
            "ldrsb" => Some(InstructionName::LDRSB),
            "ldrsbt" => Some(InstructionName::LDRSBT),
            "ldrsh" => Some(InstructionName::LDRSH),
            "ldrsht" => Some(InstructionName::LDRSHT),
            "ldrt" => Some(InstructionName::LDRT),
            "lsl" => Some(InstructionName::LSL),
            "lsr" => Some(InstructionName::LSR),
            "mcr" => Some(InstructionName::MCR),
            "mcrr" => Some(InstructionName::MCRR),
            "mla" => Some(InstructionName::MLA),
            "mlas" => Some(InstructionName::MLAS),
            "mls" => Some(InstructionName::MLS),
            "mov" => Some(InstructionName::MOV),
            "movs" => Some(InstructionName::MOVS),
            "movt" => Some(InstructionName::MOVT),
            "mrc" => Some(InstructionName::MRC),
            "mrrc" => Some(InstructionName::MRRC),
            "mrs" => Some(InstructionName::MRS),
            "msr" => Some(InstructionName::MSR),
            "mul" => Some(InstructionName::MUL),
            "muls" => Some(InstructionName::MULS),
            "mvn" => Some(InstructionName::MVN),
            "mvns" => Some(InstructionName::MVNS),
            "nop" => Some(InstructionName::NOP),
            "orn" => Some(InstructionName::ORN),
            "orns" => Some(InstructionName::ORNS),
            "orr" => Some(InstructionName::ORR),
            "orrs" => Some(InstructionName::ORRS),
            "pkhbt" => Some(InstructionName::PKHBT),
            "pkhtb" => Some(InstructionName::PKHTB),
            "pld" => Some(InstructionName::PLD),
            "pldw" => Some(InstructionName::PLDW),
            "pli" => Some(InstructionName::PLI),
            "pop" => Some(InstructionName::POP),
            "pssbb" => Some(InstructionName::PSSBB),
            "push" => Some(InstructionName::PUSH),
            "qadd" => Some(InstructionName::QADD),
            "qadd16" => Some(InstructionName::QADD16),
            "qadd8" => Some(InstructionName::QADD8),
            "qasx" => Some(InstructionName::QASX),
            "qdadd" => Some(InstructionName::QDADD),
            "qdsub" => Some(InstructionName::QDSUB),
            "qsax" => Some(InstructionName::QSAX),
            "qsub" => Some(InstructionName::QSUB),
            "qsub16" => Some(InstructionName::QSUB16),
            "qsub8" => Some(InstructionName::QSUB8),
            "rbit" => Some(InstructionName::RBIT),
            "rev" => Some(InstructionName::REV),
            "rev16" => Some(InstructionName::REV16),
            "revsh" => Some(InstructionName::REVSH),
            "rfe" => Some(InstructionName::RFE),
            "rfeda" => Some(InstructionName::RFEDA),
            "rfedb" => Some(InstructionName::RFEDB),
            "rfeia" => Some(InstructionName::RFEIA),
            "rfeib" => Some(InstructionName::RFEIB),
            "ror" => Some(InstructionName::ROR),
            "rors" => Some(InstructionName::RORS),
            "rrx" => Some(InstructionName::RRX),
            "rrxs" => Some(InstructionName::RRXS),
            "rsb" => Some(InstructionName::RSB),
            "rsbs" => Some(InstructionName::RSBS),
            "rsc" => Some(InstructionName::RSC),
            "rscs" => Some(InstructionName::RSCS),
            "sadd16" => Some(InstructionName::SADD16),
            "sadd8" => Some(InstructionName::SADD8),
            "sasx" => Some(InstructionName::SASX),
            "sb" => Some(InstructionName::SB),
            "sbc" => Some(InstructionName::SBC),
            "sbcs" => Some(InstructionName::SBCS),
            "sbfx" => Some(InstructionName::SBFX),
            "sdiv" => Some(InstructionName::SDIV),
            "sel" => Some(InstructionName::SEL),
            "setend" => Some(InstructionName::SETEND),
            "setpan" => Some(InstructionName::SETPAN),
            "sev" => Some(InstructionName::SEV),
            "sevl" => Some(InstructionName::SEVL),
            "shadd16" => Some(InstructionName::SHADD16),
            "shadd8" => Some(InstructionName::SHADD8),
            "shasx" => Some(InstructionName::SHASX),
            "shsax" => Some(InstructionName::SHSAX),
            "shsub16" => Some(InstructionName::SHSUB16),
            "shsub8" => Some(InstructionName::SHSUB8),
            "smc" => Some(InstructionName::SMC),
            "smlabb" => Some(InstructionName::SMLABB),
            "smlabt" => Some(InstructionName::SMLABT),
            "smlatb" => Some(InstructionName::SMLATB),
            "smlatt" => Some(InstructionName::SMLATT),
            "smlad" => Some(InstructionName::SMLAD),
            "smladx" => Some(InstructionName::SMLADX),
            "smlal" => Some(InstructionName::SMLAL),
            "smlals" => Some(InstructionName::SMLALS),
            "smlalbb" => Some(InstructionName::SMLALBB),
            "smlalbt" => Some(InstructionName::SMLALBT),
            "smlaltb" => Some(InstructionName::SMLALTB),
            "smlaltt" => Some(InstructionName::SMLALTT),
            "smlald" => Some(InstructionName::SMLALD),
            "smlaldx" => Some(InstructionName::SMLALDX),
            "smlawb" => Some(InstructionName::SMLAWB),
            "smlawt" => Some(InstructionName::SMLAWT),
            "smlsd" => Some(InstructionName::SMLSD),
            "smlsdx" => Some(InstructionName::SMLSDX),
            "smlsld" => Some(InstructionName::SMLSLD),
            "smlsldx" => Some(InstructionName::SMLSLDX),
            "smmla" => Some(InstructionName::SMMLA),
            "smmlar" => Some(InstructionName::SMMLAR),
            "smmls" => Some(InstructionName::SMMLS),
            "smmlsr" => Some(InstructionName::SMMLSR),
            "smmul" => Some(InstructionName::SMMUL),
            "smmulr" => Some(InstructionName::SMMULR),
            "smuad" => Some(InstructionName::SMUAD),
            "smuadx" => Some(InstructionName::SMUADX),
            "smulbb" => Some(InstructionName::SMULBB),
            "smulbt" => Some(InstructionName::SMULBT),
            "smultb" => Some(InstructionName::SMULTB),
            "smultt" => Some(InstructionName::SMULTT),
            "smull" => Some(InstructionName::SMULL),
            "smulls" => Some(InstructionName::SMULLS),
            "smulwb" => Some(InstructionName::SMULWB),
            "smulwt" => Some(InstructionName::SMULWT),
            "smusd" => Some(InstructionName::SMUSD),
            "smusdx" => Some(InstructionName::SMUSDX),
            "srs" => Some(InstructionName::SRS),
            "srsda" => Some(InstructionName::SRSDA),
            "srsdb" => Some(InstructionName::SRSDB),
            "srsia" => Some(InstructionName::SRSIA),
            "srsib" => Some(InstructionName::SRSIB),
            "ssat" => Some(InstructionName::SSAT),
            "ssat16" => Some(InstructionName::SSAT16),
            "ssax" => Some(InstructionName::SSAX),
            "ssbb" => Some(InstructionName::SSBB),
            "ssub16" => Some(InstructionName::SSUB16),
            "ssub8" => Some(InstructionName::SSUB8),
            "stc" => Some(InstructionName::STC),
            "stl" => Some(InstructionName::STL),
            "stlb" => Some(InstructionName::STLB),
            "stlex" => Some(InstructionName::STLEX),
            "stlexb" => Some(InstructionName::STLEXB),
            "stlexd" => Some(InstructionName::STLEXD),
            "stlexh" => Some(InstructionName::STLEXH),
            "stlh" => Some(InstructionName::STLH),
            "stm" => Some(InstructionName::STM),
            "stmia" => Some(InstructionName::STMIA),
            "stmea" => Some(InstructionName::STMEA),
            "stmda" => Some(InstructionName::STMDA),
            "stmed" => Some(InstructionName::STMED),
            "stmdb" => Some(InstructionName::STMDB),
            "stmfd" => Some(InstructionName::STMFD),
            "stmib" => Some(InstructionName::STMIB),
            "stmfa" => Some(InstructionName::STMFA),
            "str" => Some(InstructionName::STR),
            "strb" => Some(InstructionName::STRB),
            "strbt" => Some(InstructionName::STRBT),
            "strd" => Some(InstructionName::STRD),
            "strex" => Some(InstructionName::STREX),
            "strexb" => Some(InstructionName::STREXB),
            "strexd" => Some(InstructionName::STREXD),
            "strexh" => Some(InstructionName::STREXH),
            "strh" => Some(InstructionName::STRH),
            "strht" => Some(InstructionName::STRHT),
            "strt" => Some(InstructionName::STRT),
            "sub" => Some(InstructionName::SUB),
            "subs" => Some(InstructionName::SUBS),
            "svc" => Some(InstructionName::SVC),
            "sxtab" => Some(InstructionName::SXTAB),
            "sxtab16" => Some(InstructionName::SXTAB16),
            "sxtah" => Some(InstructionName::SXTAH),
            "sxtb" => Some(InstructionName::SXTB),
            "sxtb16" => Some(InstructionName::SXTB16),
            "sxth" => Some(InstructionName::SXTH),
            "tbb" => Some(InstructionName::TBB),
            "tbh" => Some(InstructionName::TBH),
            "teq" => Some(InstructionName::TEQ),
            "tsb" => Some(InstructionName::TSB),
            "tst" => Some(InstructionName::TST),
            "uadd16" => Some(InstructionName::UADD16),
            "uadd8" => Some(InstructionName::UADD8),
            "uasx" => Some(InstructionName::UASX),
            "ubfx" => Some(InstructionName::UBFX),
            "udf" => Some(InstructionName::UDF),
            "udiv" => Some(InstructionName::UDIV),
            "uhadd16" => Some(InstructionName::UHADD16),
            "uhadd8" => Some(InstructionName::UHADD8),
            "uhasx" => Some(InstructionName::UHASX),
            "uhsax" => Some(InstructionName::UHSAX),
            "uhsub16" => Some(InstructionName::UHSUB16),
            "uhsub8" => Some(InstructionName::UHSUB8),
            "umaal" => Some(InstructionName::UMAAL),
            "umlal" => Some(InstructionName::UMLAL),
            "umlals" => Some(InstructionName::UMLALS),
            "umull" => Some(InstructionName::UMULL),
            "umulls" => Some(InstructionName::UMULLS),
            "uqadd16" => Some(InstructionName::UQADD16),
            "uqadd8" => Some(InstructionName::UQADD8),
            "uqasx" => Some(InstructionName::UQASX),
            "uqsax" => Some(InstructionName::UQSAX),
            "uqsub16" => Some(InstructionName::UQSUB16),
            "uqsub8" => Some(InstructionName::UQSUB8),
            "usad8" => Some(InstructionName::USAD8),
            "usada8" => Some(InstructionName::USADA8),
            "usat" => Some(InstructionName::USAT),
            "usat16" => Some(InstructionName::USAT16),
            "usax" => Some(InstructionName::USAX),
            "usub16" => Some(InstructionName::USUB16),
            "usub8" => Some(InstructionName::USUB8),
            "uxtab" => Some(InstructionName::UXTAB),
            "uxtab16" => Some(InstructionName::UXTAB16),
            "uxtah" => Some(InstructionName::UXTAH),
            "uxtb" => Some(InstructionName::UXTB),
            "uxtb16" => Some(InstructionName::UXTB16),
            "uxth" => Some(InstructionName::UXTH),
            "wfe" => Some(InstructionName::WFE),
            "wfi" => Some(InstructionName::WFI),
            "yield" => Some(InstructionName::YIELD),
            _ => None,
        }
    }
}
