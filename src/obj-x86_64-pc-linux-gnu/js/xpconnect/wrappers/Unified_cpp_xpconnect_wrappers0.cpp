#define MOZ_UNIFIED_BUILD
#include "/worker/build/js/xpconnect/wrappers/AccessCheck.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/wrappers/AccessCheck.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/wrappers/AccessCheck.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/wrappers/ChromeObjectWrapper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/wrappers/ChromeObjectWrapper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/wrappers/ChromeObjectWrapper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/wrappers/FilteringWrapper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/wrappers/FilteringWrapper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/wrappers/FilteringWrapper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/wrappers/WaiveXrayWrapper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/wrappers/WaiveXrayWrapper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/wrappers/WaiveXrayWrapper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/js/xpconnect/wrappers/WrapperFactory.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/js/xpconnect/wrappers/WrapperFactory.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/js/xpconnect/wrappers/WrapperFactory.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif