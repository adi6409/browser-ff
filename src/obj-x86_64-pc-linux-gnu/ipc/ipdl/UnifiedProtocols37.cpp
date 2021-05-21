#define MOZ_UNIFIED_BUILD
#include "PWebrtcGlobalChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcGlobalChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcGlobalChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebrtcGlobalParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcGlobalParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcGlobalParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebrtcTCPSocket.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcTCPSocket.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcTCPSocket.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebrtcTCPSocketChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcTCPSocketChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcTCPSocketChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebrtcTCPSocketParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebrtcTCPSocketParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebrtcTCPSocketParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWindowGlobal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWindowGlobal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWindowGlobal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWindowGlobalChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWindowGlobalChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWindowGlobalChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWindowGlobalParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWindowGlobalParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWindowGlobalParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PlatformWidgetTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PlatformWidgetTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PlatformWidgetTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PluginTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PluginTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PluginTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PrefsTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PrefsTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PrefsTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PresState.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PresState.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PresState.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ProfilerTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ProfilerTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ProfilerTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ProtocolTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ProtocolTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ProtocolTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "RemoteWorkerTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "RemoteWorkerTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "RemoteWorkerTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ServiceWorkerConfiguration.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ServiceWorkerConfiguration.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ServiceWorkerConfiguration.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif