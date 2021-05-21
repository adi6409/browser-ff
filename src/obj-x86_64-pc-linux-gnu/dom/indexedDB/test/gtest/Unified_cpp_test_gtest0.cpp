#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/indexedDB/test/gtest/TestIDBResult.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/indexedDB/test/gtest/TestIDBResult.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/indexedDB/test/gtest/TestIDBResult.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif