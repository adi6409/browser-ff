#define MOZ_UNIFIED_BUILD
#include "PCompositorBridge.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorBridge.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorBridge.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorBridgeTypes.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorBridgeTypes.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorBridgeTypes.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorManagerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorManagerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorManagerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorManagerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorManagerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorManagerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorWidget.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorWidget.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorWidget.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorWidgetChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorWidgetChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorWidgetChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCompositorWidgetParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCompositorWidgetParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCompositorWidgetParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContentChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContentChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContentChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContentParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContentParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContentParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContentPermission.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContentPermission.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContentPermission.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContentPermissionRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContentPermissionRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContentPermissionRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PContentPermissionRequestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PContentPermissionRequestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PContentPermissionRequestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif