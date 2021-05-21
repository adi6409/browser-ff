#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/mediasink/AudioSink.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mediasink/AudioSink.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mediasink/AudioSink.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/mediasink/AudioSinkWrapper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mediasink/AudioSinkWrapper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mediasink/AudioSinkWrapper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/mediasink/DecodedStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mediasink/DecodedStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mediasink/DecodedStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/mediasink/VideoSink.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mediasink/VideoSink.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mediasink/VideoSink.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif