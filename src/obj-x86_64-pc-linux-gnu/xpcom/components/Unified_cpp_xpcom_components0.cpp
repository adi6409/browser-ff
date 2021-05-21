#define MOZ_UNIFIED_BUILD
#include "/worker/build/xpcom/components/GenericFactory.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/GenericFactory.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/GenericFactory.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/components/ManifestParser.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/ManifestParser.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/ManifestParser.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/components/nsCategoryCache.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/nsCategoryCache.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/nsCategoryCache.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/components/nsCategoryManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/nsCategoryManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/nsCategoryManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/components/nsComponentManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/nsComponentManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/nsComponentManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/components/nsComponentManagerUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/components/nsComponentManagerUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/components/nsComponentManagerUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif