#define MOZ_UNIFIED_BUILD
#include "/worker/build/intl/lwbrk/LineBreaker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/lwbrk/LineBreaker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/lwbrk/LineBreaker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/lwbrk/WordBreaker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/lwbrk/WordBreaker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/lwbrk/WordBreaker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif