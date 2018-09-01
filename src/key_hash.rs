enum Key {
    // uint64_t address_ : 48;
    // uint64_t tag_ : 14;
    // uint64_t not_used_ : 2;
    Used(u64),
    
    // uint64_t control_;
    Unused(u64), 
}

struct KeyHash {
    key :Key,
}