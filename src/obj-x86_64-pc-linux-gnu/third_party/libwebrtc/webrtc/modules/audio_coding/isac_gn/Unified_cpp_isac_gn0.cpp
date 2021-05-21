#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_decoder_isac.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_decoder_isac.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_decoder_isac.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_encoder_isac.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_encoder_isac.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_coding/codecs/isac/main/source/audio_encoder_isac.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif