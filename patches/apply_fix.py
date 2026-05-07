#!/usr/bin/env python3
import re

file_path = "quickjs-hook/src/hook_engine_mem.c"

with open(file_path, 'r') as f:
    content = f.read()

# Fix 1: Replace hook_flush_cache function
old_func = r'void hook_flush_cache\(void\* start, size_t size\) \{\s*__builtin___clear_cache\(\(char\*\)start, \(char\*\)start \+ size\);\s*\}'

new_func = '''void hook_flush_cache(void* start, size_t size) {
    /* ARM64 manual cache flush - avoid __clear_cache symbol dependency */
    char* begin = (char*)start;
    char* end = (char*)start + size;
    
    /* DC CVAU - Data Cache Clean by VA to PoU */
    for (char* p = begin; p < end; p += 64) {
        __asm__ __volatile__("dc cvau, %0" : : "r"(p) : "memory");
    }
    __asm__ __volatile__("dsb ish" : : : "memory");
    
    /* IC IVAU - Instruction Cache Invalidate by VA to PoU */
    for (char* p = begin; p < end; p += 64) {
        __asm__ __volatile__("ic ivau, %0" : : "r"(p) : "memory");
    }
    __asm__ __volatile__("dsb ish" : : : "memory");
    __asm__ __volatile__("isb" : : : "memory");
}'''

content = re.sub(old_func, new_func, content, flags=re.DOTALL)

# Fix 2: Replace the two __builtin___clear_cache calls in patch_target
old_pattern = r'memcpy\(writable, jump_buf, \(size_t\)jump_result\);\s*/\* flush icache[^*]*\*/\s*__builtin___clear_cache\(\(char\*\)target, \(char\*\)target \+ jump_result\);\s*__builtin___clear_cache\(\(char\*\)writable, \(char\*\)writable \+ jump_result\);'

new_code = '''memcpy(writable, jump_buf, (size_t)jump_result);
            
            /* Manual cache flush for both target and writable addresses */
            char* addrs[] = {(char*)target, (char*)writable};
            for (int i = 0; i < 2; i++) {
                char* begin = addrs[i];
                char* end = begin + jump_result;
                for (char* p = begin; p < end; p += 64) {
                    __asm__ __volatile__("dc cvau, %0" : : "r"(p) : "memory");
                }
                __asm__ __volatile__("dsb ish" : : : "memory");
                for (char* p = begin; p < end; p += 64) {
                    __asm__ __volatile__("ic ivau, %0" : : "r"(p) : "memory");
                }
            }
            __asm__ __volatile__("dsb ish" : : : "memory");
            __asm__ __volatile__("isb" : : : "memory");'''

content = re.sub(old_pattern, new_code, content, flags=re.DOTALL)

with open(file_path, 'w') as f:
    f.write(content)

print("✓ Applied __clear_cache fix successfully")
