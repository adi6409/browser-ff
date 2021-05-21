#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/webrtc/transportbridge/MediaPipeline.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/transportbridge/MediaPipeline.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/transportbridge/MediaPipeline.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/transportbridge/MediaPipelineFilter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/transportbridge/MediaPipelineFilter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/transportbridge/MediaPipelineFilter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/webrtc/transportbridge/RtpLogger.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/webrtc/transportbridge/RtpLogger.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/webrtc/transportbridge/RtpLogger.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif