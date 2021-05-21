#define MOZ_UNIFIED_BUILD
#include "/worker/build/media/libcubeb/gtest/test_audio.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_audio.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_audio.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libcubeb/gtest/test_latency.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_latency.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_latency.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libcubeb/gtest/test_resampler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_resampler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_resampler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libcubeb/gtest/test_sanity.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_sanity.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_sanity.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libcubeb/gtest/test_tone.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_tone.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_tone.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libcubeb/gtest/test_utils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libcubeb/gtest/test_utils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libcubeb/gtest/test_utils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif