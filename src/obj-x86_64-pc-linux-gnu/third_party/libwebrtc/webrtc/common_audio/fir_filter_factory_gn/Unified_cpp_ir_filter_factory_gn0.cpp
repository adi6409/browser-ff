#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_c.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_c.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_c.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_factory.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_factory.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/fir_filter_factory.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif