#define MOZ_UNIFIED_BUILD
#include "PServiceWorkerManagerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerManagerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerManagerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerRegistration.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerRegistration.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerRegistration.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerRegistrationChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerRegistrationChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerRegistrationChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerRegistrationParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerRegistrationParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerRegistrationParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerUpdater.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerUpdater.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerUpdater.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerUpdaterChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerUpdaterChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerUpdaterChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PServiceWorkerUpdaterParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PServiceWorkerUpdaterParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PServiceWorkerUpdaterParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSessionStorageObserver.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSessionStorageObserver.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSessionStorageObserver.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSessionStorageObserverChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSessionStorageObserverChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSessionStorageObserverChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSessionStorageObserverParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSessionStorageObserverParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSessionStorageObserverParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSharedWorker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSharedWorker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSharedWorker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSharedWorkerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSharedWorkerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSharedWorkerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSharedWorkerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSharedWorkerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSharedWorkerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSimpleChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSimpleChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSimpleChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PSimpleChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PSimpleChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PSimpleChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif