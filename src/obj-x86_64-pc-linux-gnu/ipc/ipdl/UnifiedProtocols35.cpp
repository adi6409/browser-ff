#define MOZ_UNIFIED_BUILD
#include "PVsyncBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVsyncBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVsyncBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVsyncBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVsyncBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVsyncBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVsyncChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVsyncChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVsyncChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVsyncParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVsyncParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVsyncParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebAuthnTransaction.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebAuthnTransaction.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebAuthnTransaction.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebAuthnTransactionChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebAuthnTransactionChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebAuthnTransactionChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebAuthnTransactionParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebAuthnTransactionParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebAuthnTransactionParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistDocument.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistDocument.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistDocument.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistDocumentChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistDocumentChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistDocumentChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistDocumentParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistDocumentParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistDocumentParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistResources.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistResources.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistResources.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistResourcesChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistResourcesChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistResourcesChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistResourcesParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistResourcesParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistResourcesParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistSerialize.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistSerialize.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistSerialize.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistSerializeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistSerializeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistSerializeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PWebBrowserPersistSerializeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PWebBrowserPersistSerializeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PWebBrowserPersistSerializeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif