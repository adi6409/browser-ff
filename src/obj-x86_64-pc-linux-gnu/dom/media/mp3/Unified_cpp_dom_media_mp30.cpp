#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/mp3/MP3Decoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mp3/MP3Decoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mp3/MP3Decoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/mp3/MP3Demuxer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mp3/MP3Demuxer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mp3/MP3Demuxer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/mp3/MP3FrameParser.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/mp3/MP3FrameParser.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/mp3/MP3FrameParser.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif