#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/webrtc/jsep/JsepSessionImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/jsep/JsepSessionImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/jsep/JsepSessionImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/jsep/JsepTrack.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/jsep/JsepTrack.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/jsep/JsepTrack.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/jsep/SsrcGenerator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/jsep/SsrcGenerator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/jsep/SsrcGenerator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif