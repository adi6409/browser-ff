#define MOZ_UNIFIED_BUILD
#include "/worker/build/memory/build/mozjemalloc.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/memory/build/mozjemalloc.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/memory/build/mozjemalloc.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/memory/build/mozmemory_wrap.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/memory/build/mozmemory_wrap.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/memory/build/mozmemory_wrap.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif