#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/xz-embedded/src/xz_crc32.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/xz-embedded/src/xz_crc32.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/xz-embedded/src/xz_crc32.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/xz-embedded/src/xz_crc64.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/xz-embedded/src/xz_crc64.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/xz-embedded/src/xz_crc64.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/xz-embedded/src/xz_dec_bcj.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/xz-embedded/src/xz_dec_bcj.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/xz-embedded/src/xz_dec_bcj.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/xz-embedded/src/xz_dec_lzma2.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/xz-embedded/src/xz_dec_lzma2.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/xz-embedded/src/xz_dec_lzma2.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/xz-embedded/src/xz_dec_stream.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/xz-embedded/src/xz_dec_stream.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/xz-embedded/src/xz_dec_stream.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif