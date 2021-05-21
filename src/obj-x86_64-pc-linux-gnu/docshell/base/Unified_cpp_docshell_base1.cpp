#define MOZ_UNIFIED_BUILD
#include "/worker/build/docshell/base/nsDocShellTreeOwner.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/base/nsDocShellTreeOwner.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/base/nsDocShellTreeOwner.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/base/nsPingListener.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/base/nsPingListener.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/base/nsPingListener.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/base/nsRefreshTimer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/base/nsRefreshTimer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/base/nsRefreshTimer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/base/nsWebNavigationInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/base/nsWebNavigationInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/base/nsWebNavigationInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif