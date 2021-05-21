#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/cairo/cairo/src/cairo-wideint.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/cairo/cairo/src/cairo-wideint.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/cairo/cairo/src/cairo-wideint.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/cairo/cairo/src/cairo.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/cairo/cairo/src/cairo.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/cairo/cairo/src/cairo.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif