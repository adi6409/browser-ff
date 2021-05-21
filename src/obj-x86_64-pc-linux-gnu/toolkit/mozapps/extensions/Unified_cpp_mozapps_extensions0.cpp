#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/mozapps/extensions/AddonContentPolicy.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/mozapps/extensions/AddonContentPolicy.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/mozapps/extensions/AddonContentPolicy.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/mozapps/extensions/AddonManagerStartup.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/mozapps/extensions/AddonManagerStartup.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/mozapps/extensions/AddonManagerStartup.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/mozapps/extensions/AddonManagerWebAPI.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/mozapps/extensions/AddonManagerWebAPI.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/mozapps/extensions/AddonManagerWebAPI.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif