#define MOZ_UNIFIED_BUILD
#include "/worker/build/media/libnestegg/src/nestegg.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libnestegg/src/nestegg.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libnestegg/src/nestegg.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif