#define MOZ_UNIFIED_BUILD
#include "/worker/build/tools/profiler/tests/gtest/GeckoProfiler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/tests/gtest/GeckoProfiler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/tests/gtest/GeckoProfiler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/tests/gtest/LulTest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/tests/gtest/LulTest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/tests/gtest/LulTest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/tests/gtest/LulTestDwarf.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/tests/gtest/LulTestDwarf.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/tests/gtest/LulTestDwarf.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/tests/gtest/LulTestInfrastructure.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/tests/gtest/LulTestInfrastructure.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/tests/gtest/LulTestInfrastructure.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/tests/gtest/ThreadProfileTest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/tests/gtest/ThreadProfileTest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/tests/gtest/ThreadProfileTest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif