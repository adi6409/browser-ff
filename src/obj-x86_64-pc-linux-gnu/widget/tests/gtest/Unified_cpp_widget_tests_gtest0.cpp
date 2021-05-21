#define MOZ_UNIFIED_BUILD
#include "/worker/build/widget/tests/gtest/TestTimeConverter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/widget/tests/gtest/TestTimeConverter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/widget/tests/gtest/TestTimeConverter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/widget/tests/gtest/TestTouchResampler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/widget/tests/gtest/TestTouchResampler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/widget/tests/gtest/TestTouchResampler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif