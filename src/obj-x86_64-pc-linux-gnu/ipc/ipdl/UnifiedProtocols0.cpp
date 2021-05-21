#define MOZ_UNIFIED_BUILD
#include "BlobTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "BlobTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "BlobTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "CacheTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "CacheTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "CacheTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ChannelInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ChannelInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ChannelInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ClientIPCTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ClientIPCTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ClientIPCTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "DOMTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "DOMTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "DOMTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "DocAccessibleTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "DocAccessibleTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "DocAccessibleTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "FetchTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "FetchTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "FetchTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "GMPTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "GMPTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "GMPTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "GamepadEventTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "GamepadEventTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "GamepadEventTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "GraphicsMessages.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "GraphicsMessages.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "GraphicsMessages.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "HangTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "HangTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "HangTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "HeadlessWidgetTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "HeadlessWidgetTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "HeadlessWidgetTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "HttpChannelParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "HttpChannelParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "HttpChannelParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "IPCBlob.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "IPCBlob.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "IPCBlob.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "IPCServiceWorkerDescriptor.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "IPCServiceWorkerDescriptor.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "IPCServiceWorkerDescriptor.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "IPCServiceWorkerRegistrationDescriptor.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "IPCServiceWorkerRegistrationDescriptor.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "IPCServiceWorkerRegistrationDescriptor.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif