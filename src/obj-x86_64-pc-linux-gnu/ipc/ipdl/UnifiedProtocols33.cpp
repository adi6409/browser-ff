#define MOZ_UNIFIED_BUILD
#include "PUDPSocketChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUDPSocketChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUDPSocketChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PUDPSocketParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUDPSocketParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUDPSocketParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifier.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifier.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifier.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierLocal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierLocal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierLocal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierLocalChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierLocalChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierLocalChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierLocalParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierLocalParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierLocalParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PURLClassifierParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PURLClassifierParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PURLClassifierParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PUiCompositorController.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUiCompositorController.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUiCompositorController.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PUiCompositorControllerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUiCompositorControllerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUiCompositorControllerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PUiCompositorControllerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PUiCompositorControllerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PUiCompositorControllerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVR.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVR.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVR.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVRChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVRChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVRChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVRGPU.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVRGPU.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVRGPU.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PVRGPUChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PVRGPUChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PVRGPUChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif