#define MOZ_UNIFIED_BUILD
#include "/worker/build/extensions/pref/autoconfig/src/nsAutoConfig.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/pref/autoconfig/src/nsAutoConfig.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/pref/autoconfig/src/nsAutoConfig.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/pref/autoconfig/src/nsJSConfigTriggers.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/pref/autoconfig/src/nsJSConfigTriggers.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/pref/autoconfig/src/nsJSConfigTriggers.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/pref/autoconfig/src/nsReadConfig.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/pref/autoconfig/src/nsReadConfig.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/pref/autoconfig/src/nsReadConfig.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif