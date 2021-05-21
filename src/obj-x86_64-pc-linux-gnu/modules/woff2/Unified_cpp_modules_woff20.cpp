#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/woff2/src/table_tags.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/woff2/src/table_tags.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/woff2/src/table_tags.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/woff2/src/variable_length.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/woff2/src/variable_length.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/woff2/src/variable_length.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/woff2/src/woff2_common.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/woff2/src/woff2_common.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/woff2/src/woff2_common.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/woff2/src/woff2_dec.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/woff2/src/woff2_dec.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/woff2/src/woff2_dec.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/woff2/src/woff2_out.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/woff2/src/woff2_out.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/woff2/src/woff2_out.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif