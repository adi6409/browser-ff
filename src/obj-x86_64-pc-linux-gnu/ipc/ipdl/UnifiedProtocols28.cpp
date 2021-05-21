#define MOZ_UNIFIED_BUILD
#include "PRemoteWorkerControllerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerControllerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerControllerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PRemoteWorkerServiceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PRemoteWorkerServiceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PRemoteWorkerServiceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSMIPCTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSMIPCTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSMIPCTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PScriptCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PScriptCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PScriptCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PScriptCacheChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PScriptCacheChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PScriptCacheChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PScriptCacheParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PScriptCacheParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PScriptCacheParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerContainer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerContainer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerContainer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerContainerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerContainerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerContainerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerContainerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerContainerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerContainerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerManagerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerManagerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerManagerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif