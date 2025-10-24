// c_vendor/swisseph/safe_stubs.c
#include <stdarg.h>
#include <stddef.h>

// 替换所有 printf 变体
int printf(const char *fmt, ...) { return 0; }
int fprintf(void *stream, const char *fmt, ...) { return 0; }
int sprintf(char *str, const char *fmt, ...) { return 0; }
int snprintf(char *str, size_t size, const char *fmt, ...) { return 0; }

// 替换 exit
void exit(int status) {
    // 在 Wasm/移动端，exit 会导致崩溃，静默忽略
    return;
}