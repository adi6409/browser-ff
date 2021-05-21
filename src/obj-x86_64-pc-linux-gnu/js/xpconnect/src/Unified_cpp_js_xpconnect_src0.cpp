#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/xpconnect/src/ExportHelpers.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/ExportHelpers.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/ExportHelpers.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/JSServices.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/JSServices.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/JSServices.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/Sandbox.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/Sandbox.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/Sandbox.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCCallContext.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCCallContext.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCCallContext.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCComponents.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCComponents.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCComponents.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCConvert.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCConvert.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCConvert.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCDebug.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCDebug.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCDebug.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCException.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCException.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCException.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCJSContext.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCJSContext.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCJSContext.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCJSID.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCJSID.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCJSID.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCJSRuntime.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCJSRuntime.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCJSRuntime.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCJSWeakReference.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCJSWeakReference.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCJSWeakReference.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCLocale.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCLocale.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCLocale.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCLog.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCLog.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCLog.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCMaps.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCMaps.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCMaps.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCModule.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCModule.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCModule.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif