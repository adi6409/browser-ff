#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/xpconnect/loader/AutoMemMap.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/AutoMemMap.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/AutoMemMap.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/ChromeScriptLoader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/ChromeScriptLoader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/ChromeScriptLoader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/ScriptCacheActors.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/ScriptCacheActors.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/ScriptCacheActors.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/ScriptPreloader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/ScriptPreloader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/ScriptPreloader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/URLPreloader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/URLPreloader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/URLPreloader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/mozJSLoaderUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/mozJSLoaderUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/mozJSLoaderUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/mozJSSubScriptLoader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/mozJSSubScriptLoader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/mozJSSubScriptLoader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/loader/nsImportModule.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/loader/nsImportModule.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/loader/nsImportModule.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif