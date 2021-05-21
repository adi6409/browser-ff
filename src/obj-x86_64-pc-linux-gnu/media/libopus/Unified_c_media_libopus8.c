#define MOZ_UNIFIED_BUILD
#include "/worker/build/media/libopus/src/opus_projection_decoder.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libopus/src/opus_projection_decoder.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libopus/src/opus_projection_decoder.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libopus/src/opus_projection_encoder.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libopus/src/opus_projection_encoder.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libopus/src/opus_projection_encoder.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/media/libopus/src/repacketizer.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/media/libopus/src/repacketizer.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/media/libopus/src/repacketizer.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif