#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/u2f/U2F.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/u2f/U2F.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/u2f/U2F.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif