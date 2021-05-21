#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/skia/skia/src/utils/win/SkIStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/skia/skia/src/utils/win/SkIStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/skia/skia/src/utils/win/SkIStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif