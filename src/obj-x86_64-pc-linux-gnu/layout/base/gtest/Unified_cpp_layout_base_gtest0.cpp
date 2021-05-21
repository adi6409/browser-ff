#define MOZ_UNIFIED_BUILD
#include "/worker/build/layout/base/gtest/TestAccessibleCaretEventHub.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/gtest/TestAccessibleCaretEventHub.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/gtest/TestAccessibleCaretEventHub.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/base/gtest/TestAccessibleCaretManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/base/gtest/TestAccessibleCaretManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/base/gtest/TestAccessibleCaretManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif