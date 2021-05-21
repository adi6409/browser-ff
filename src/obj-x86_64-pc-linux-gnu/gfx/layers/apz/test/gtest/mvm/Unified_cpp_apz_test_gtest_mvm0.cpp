#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/layers/apz/test/gtest/mvm/TestMobileViewportManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/layers/apz/test/gtest/mvm/TestMobileViewportManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/layers/apz/test/gtest/mvm/TestMobileViewportManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif