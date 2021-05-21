#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/webrtc/common/YuvStamper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/common/YuvStamper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/common/YuvStamper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/common/browser_logging/CSFLog.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/common/browser_logging/CSFLog.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/common/browser_logging/CSFLog.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/common/browser_logging/WebRtcLog.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/common/browser_logging/WebRtcLog.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/common/browser_logging/WebRtcLog.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif