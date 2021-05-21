#define MOZ_UNIFIED_BUILD
#include "/worker/build/modules/libpref/Preferences.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libpref/Preferences.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libpref/Preferences.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/modules/libpref/SharedPrefMap.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/modules/libpref/SharedPrefMap.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/modules/libpref/SharedPrefMap.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif