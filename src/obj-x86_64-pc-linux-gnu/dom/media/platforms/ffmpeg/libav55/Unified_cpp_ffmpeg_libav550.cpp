#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/media/platforms/ffmpeg/FFmpegAudioDecoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegAudioDecoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegAudioDecoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/platforms/ffmpeg/FFmpegDataDecoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegDataDecoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegDataDecoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/platforms/ffmpeg/FFmpegDecoderModule.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegDecoderModule.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegDecoderModule.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/media/platforms/ffmpeg/FFmpegVideoDecoder.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegVideoDecoder.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/media/platforms/ffmpeg/FFmpegVideoDecoder.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif