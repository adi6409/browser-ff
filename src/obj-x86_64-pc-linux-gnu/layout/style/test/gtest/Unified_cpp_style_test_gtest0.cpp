#define MOZ_UNIFIED_BUILD
#include "/worker/build/layout/style/test/gtest/ImportScannerTest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/style/test/gtest/ImportScannerTest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/style/test/gtest/ImportScannerTest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/style/test/gtest/StyloParsingBench.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/style/test/gtest/StyloParsingBench.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/style/test/gtest/StyloParsingBench.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif