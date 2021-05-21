#define MOZ_UNIFIED_BUILD
#include "/worker/build/media/gmp-clearkey/0.1/gtest/TestClearKeyUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/gmp-clearkey/0.1/gtest/TestClearKeyUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/gmp-clearkey/0.1/gtest/TestClearKeyUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif