#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libwebrtc/webrtc/common_audio/signal_processing/dot_product_with_scale.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/signal_processing/dot_product_with_scale.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/common_audio/signal_processing/dot_product_with_scale.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif