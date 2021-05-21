#define MOZ_UNIFIED_BUILD
#include "/worker/build/xpcom/rust/gtest/bench-collections/Bench.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/rust/gtest/bench-collections/Bench.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/rust/gtest/bench-collections/Bench.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/rust/gtest/moz_task/Test.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/rust/gtest/moz_task/Test.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/rust/gtest/moz_task/Test.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/rust/gtest/nsstring/Test.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/rust/gtest/nsstring/Test.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/rust/gtest/nsstring/Test.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/rust/gtest/xpcom/Test.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/rust/gtest/xpcom/Test.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/rust/gtest/xpcom/Test.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif