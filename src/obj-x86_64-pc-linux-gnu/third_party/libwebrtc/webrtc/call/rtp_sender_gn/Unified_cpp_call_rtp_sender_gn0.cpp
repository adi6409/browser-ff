#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libwebrtc/webrtc/call/rtp_transport_controller_send.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/call/rtp_transport_controller_send.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/call/rtp_transport_controller_send.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif