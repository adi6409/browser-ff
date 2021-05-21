#define MOZ_UNIFIED_BUILD
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_audio_proc.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_audio_proc.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_audio_proc.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_circular_buffer.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_circular_buffer.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/vad_circular_buffer.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/voice_activity_detector.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/voice_activity_detector.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/vad/voice_activity_detector.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/voice_detection_impl.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/voice_detection_impl.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/third_party/libwebrtc/webrtc/modules/audio_processing/voice_detection_impl.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif