#define MOZ_UNIFIED_BUILD
#include "PCacheOpChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheOpChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheOpChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheOpParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheOpParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheOpParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStorage.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStorage.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStorage.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStorageChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStorageChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStorageChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStorageParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStorageParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStorageParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStreamControl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStreamControl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStreamControl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStreamControlChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStreamControlChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStreamControlChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCacheStreamControlParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCacheStreamControlParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCacheStreamControlParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCameras.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCameras.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCameras.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCamerasChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCamerasChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCamerasChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCamerasParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCamerasParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCamerasParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCanvas.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCanvas.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCanvas.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCanvasChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCanvasChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCanvasChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PCanvasParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PCanvasParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PCanvasParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PChildToParentStream.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PChildToParentStream.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PChildToParentStream.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif