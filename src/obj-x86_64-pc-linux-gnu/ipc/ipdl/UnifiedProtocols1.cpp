#define MOZ_UNIFIED_BUILD
#include "IPCStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "IPCStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "IPCStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "InputStreamParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "InputStreamParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "InputStreamParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "LayersMessages.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "LayersMessages.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "LayersMessages.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "LayersSurfaces.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "LayersSurfaces.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "LayersSurfaces.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "LookAndFeelTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "LookAndFeelTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "LookAndFeelTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "MIDITypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "MIDITypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "MIDITypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "MemoryReportTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "MemoryReportTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "MemoryReportTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "NeckoChannelParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "NeckoChannelParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "NeckoChannelParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZ.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZ.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZ.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZCTreeManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZCTreeManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZCTreeManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZCTreeManagerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZCTreeManagerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZCTreeManagerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZCTreeManagerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZCTreeManagerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZCTreeManagerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZInputBridge.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZInputBridge.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZInputBridge.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZInputBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZInputBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZInputBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAPZInputBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZInputBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZInputBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif