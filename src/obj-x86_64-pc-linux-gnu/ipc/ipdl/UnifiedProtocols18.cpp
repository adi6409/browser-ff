#define MOZ_UNIFIED_BUILD
#include "PGMPVideoEncoderParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPVideoEncoderParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPVideoEncoderParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGPU.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGPU.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGPU.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGPUChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGPUChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGPUChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGPUParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGPUParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGPUParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadEventChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadEventChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadEventChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadEventChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadEventChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadEventChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadEventChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadEventChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadEventChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadTestChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadTestChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadTestChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadTestChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadTestChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadTestChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGamepadTestChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGamepadTestChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGamepadTestChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHalChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHalChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHalChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHalParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHalParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHalParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHandlerService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHandlerService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHandlerService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHandlerServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHandlerServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHandlerServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHandlerServiceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHandlerServiceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHandlerServiceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif