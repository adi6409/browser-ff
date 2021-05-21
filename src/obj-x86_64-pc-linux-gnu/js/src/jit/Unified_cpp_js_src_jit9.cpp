#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/src/jit/RangeAnalysis.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/RangeAnalysis.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/RangeAnalysis.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/Recover.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/Recover.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/Recover.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/RegisterAllocator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/RegisterAllocator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/RegisterAllocator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/RematerializedFrame.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/RematerializedFrame.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/RematerializedFrame.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/SafepointIndex.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/SafepointIndex.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/SafepointIndex.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/src/jit/Safepoints.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/src/jit/Safepoints.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/src/jit/Safepoints.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif