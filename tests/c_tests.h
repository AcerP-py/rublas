#define assert_eq(a, b) do { \
    if (a != b) { \
        printf("assertion failed: %.2f != %.2f", a, b); \
        exit(1); \
    } \
} while (0)
