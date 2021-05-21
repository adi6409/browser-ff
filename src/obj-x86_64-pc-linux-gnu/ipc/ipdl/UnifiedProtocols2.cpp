#define MOZ_UNIFIED_BUILD
#include "PAPZParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAPZParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAPZParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltDataOutputStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltDataOutputStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltDataOutputStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltDataOutputStreamChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltDataOutputStreamChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltDataOutputStreamChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltDataOutputStreamParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltDataOutputStreamParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltDataOutputStreamParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltServiceParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltServiceParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltServiceParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltSvcTransaction.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltSvcTransaction.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltSvcTransaction.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltSvcTransactionChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltSvcTransactionChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltSvcTransactionChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PAltSvcTransactionParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PAltSvcTransactionParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PAltSvcTransactionParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackground.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackground.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackground.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundDataBridge.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundDataBridge.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundDataBridge.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundDataBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundDataBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundDataBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundDataBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundDataBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundDataBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PBackgroundFileHandle.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PBackgroundFileHandle.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PBackgroundFileHandle.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif