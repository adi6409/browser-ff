#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/audiochannel/AudioChannelAgent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/audiochannel/AudioChannelAgent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/audiochannel/AudioChannelAgent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/audiochannel/AudioChannelService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/audiochannel/AudioChannelService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/audiochannel/AudioChannelService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif