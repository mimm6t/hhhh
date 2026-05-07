# 修复 rustFrida __clear_cache 符号问题

## 问题
rustFrida 在某些 Android 设备上注入失败：
```
dlopen failed: cannot locate symbol "__clear_cache" referenced by "/memfd:wwb_so (deleted)"
```

## 原因
rustFrida 使用 `__builtin___clear_cache()` 来刷新 CPU 缓存，但这个符号在某些设备的 libc 中不存在。

## 解决方案
用 ARM64 汇编指令直接实现缓存刷新，避免依赖 `__clear_cache` 符号。

## 需要修改的文件
`rustFrida-master/quickjs-hook/src/hook_engine_mem.c`

## 修改内容

### 1. 替换 `hook_flush_cache` 函数（约第 118 行）

**原代码：**
```c
void hook_flush_cache(void* start, size_t size) {
    __builtin___clear_cache((char*)start, (char*)start + size);
}
```

**新代码：**
```c
void hook_flush_cache(void* start, size_t size) {
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
}
```

### 2. 替换 `patch_target` 中的缓存刷新（约第 1240 行）

**原代码：**
```c
        void* writable = find_rw_sibling(target, (size_t)jump_result);
        if (writable) {
            memcpy(writable, jump_buf, (size_t)jump_result);
            /* flush icache 在 target 侧 (CPU 执行地址) — 虚拟地址不同但物理页同 */
            __builtin___clear_cache((char*)target, (char*)target + jump_result);
            __builtin___clear_cache((char*)writable, (char*)writable + jump_result);
            entry->stealth = 0;
```

**新代码：**
```c
        void* writable = find_rw_sibling(target, (size_t)jump_result);
        if (writable) {
            memcpy(writable, jump_buf, (size_t)jump_result);
            
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
            __asm__ __volatile__("isb" : : : "memory");
            
            entry->stealth = 0;
```

## 技术说明

ARM64 缓存刷新指令：
- `DC CVAU` - Data Cache Clean by VA to Point of Unification
- `IC IVAU` - Instruction Cache Invalidate by VA to Point of Unification
- `DSB ISH` - Data Synchronization Barrier (Inner Shareable)
- `ISB` - Instruction Synchronization Barrier

缓存行大小：64 字节（ARM64 标准）

## 下一步
这个修改需要在 rustFrida 源码中进行，然后在 GitHub Actions 中重新编译。
