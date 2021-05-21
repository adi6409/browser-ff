#define MOZ_UNIFIED_BUILD
#include "/worker/build/image/decoders/iccjpeg.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/image/decoders/iccjpeg.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/image/decoders/iccjpeg.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif