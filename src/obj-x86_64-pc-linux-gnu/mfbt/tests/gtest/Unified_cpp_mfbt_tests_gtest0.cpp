#define MOZ_UNIFIED_BUILD
#include "/worker/build/mfbt/tests/gtest/TestBuffer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestBuffer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestBuffer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mfbt/tests/gtest/TestLinkedList.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestLinkedList.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestLinkedList.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mfbt/tests/gtest/TestMozDbg.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestMozDbg.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestMozDbg.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mfbt/tests/gtest/TestReverseIterator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestReverseIterator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestReverseIterator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mfbt/tests/gtest/TestSpan.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestSpan.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestSpan.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mfbt/tests/gtest/TestTainting.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mfbt/tests/gtest/TestTainting.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mfbt/tests/gtest/TestTainting.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif