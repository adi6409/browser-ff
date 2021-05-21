#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/libmar/src/mar_create.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libmar/src/mar_create.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libmar/src/mar_create.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libmar/src/mar_extract.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libmar/src/mar_extract.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libmar/src/mar_extract.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libmar/src/mar_read.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libmar/src/mar_read.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libmar/src/mar_read.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif