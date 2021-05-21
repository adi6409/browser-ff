#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/xpconnect/src/XPCRuntimeService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCRuntimeService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCRuntimeService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCShellImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCShellImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCShellImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCString.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCString.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCString.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCThrower.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCThrower.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCThrower.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCVariant.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCVariant.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCVariant.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedJS.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedJS.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedJS.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedJSClass.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedJSClass.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedJSClass.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedJSIterator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedJSIterator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedJSIterator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedNative.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedNative.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedNative.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedNativeInfo.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeInfo.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeInfo.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedNativeJSOps.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeJSOps.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeJSOps.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedNativeProto.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeProto.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeProto.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrappedNativeScope.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeScope.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrappedNativeScope.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/XPCWrapper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/XPCWrapper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/XPCWrapper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/src/nsXPConnect.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/src/nsXPConnect.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/src/nsXPConnect.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif