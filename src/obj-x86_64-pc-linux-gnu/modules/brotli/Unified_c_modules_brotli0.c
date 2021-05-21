#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/brotli/common/constants.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/common/constants.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/common/constants.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/common/context.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/common/context.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/common/context.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/common/dictionary.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/common/dictionary.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/common/dictionary.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/common/platform.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/common/platform.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/common/platform.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/common/transform.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/common/transform.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/common/transform.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/dec/bit_reader.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/dec/bit_reader.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/dec/bit_reader.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/dec/decode.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/dec/decode.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/dec/decode.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/dec/huffman.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/dec/huffman.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/dec/huffman.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/brotli/dec/state.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/brotli/dec/state.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/brotli/dec/state.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif