#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/doctor/test/gtest/TestMultiWriterQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/doctor/test/gtest/TestMultiWriterQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/doctor/test/gtest/TestMultiWriterQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/doctor/test/gtest/TestRollingNumber.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/doctor/test/gtest/TestRollingNumber.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/doctor/test/gtest/TestRollingNumber.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif