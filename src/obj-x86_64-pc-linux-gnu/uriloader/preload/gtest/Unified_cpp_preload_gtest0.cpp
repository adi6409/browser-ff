#define MOZ_UNIFIED_BUILD
#include "/worker/build/uriloader/preload/gtest/TestFetchPreloader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/uriloader/preload/gtest/TestFetchPreloader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/uriloader/preload/gtest/TestFetchPreloader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif