#define MOZ_UNIFIED_BUILD
#include "ServiceWorkerOpArgs.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ServiceWorkerOpArgs.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ServiceWorkerOpArgs.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "ServiceWorkerRegistrarTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "ServiceWorkerRegistrarTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "ServiceWorkerRegistrarTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "URIParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "URIParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "URIParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "WebRenderMessages.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "WebRenderMessages.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "WebRenderMessages.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "WebrtcProxyConfig.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "WebrtcProxyConfig.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "WebrtcProxyConfig.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "WindowGlobalTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "WindowGlobalTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "WindowGlobalTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif