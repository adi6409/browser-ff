#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/config/gfxConfig.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/config/gfxConfig.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/config/gfxConfig.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/config/gfxConfigManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/config/gfxConfigManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/config/gfxConfigManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/config/gfxFeature.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/config/gfxFeature.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/config/gfxFeature.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/config/gfxVars.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/config/gfxVars.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/config/gfxVars.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif