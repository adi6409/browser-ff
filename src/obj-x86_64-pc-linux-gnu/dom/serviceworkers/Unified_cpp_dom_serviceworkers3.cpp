#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/serviceworkers/ServiceWorkerUpdaterParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/serviceworkers/ServiceWorkerUpdaterParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/serviceworkers/ServiceWorkerUpdaterParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/serviceworkers/ServiceWorkerUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/serviceworkers/ServiceWorkerUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/serviceworkers/ServiceWorkerUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif