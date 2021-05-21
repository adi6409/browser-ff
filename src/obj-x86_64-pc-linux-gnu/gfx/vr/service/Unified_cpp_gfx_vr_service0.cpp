#define MOZ_UNIFIED_BUILD
#include "/worker/build/gfx/vr/service/OSVRSession.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/vr/service/OSVRSession.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/vr/service/OSVRSession.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/vr/service/VRService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/vr/service/VRService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/vr/service/VRService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/gfx/vr/service/VRSession.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/gfx/vr/service/VRSession.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/gfx/vr/service/VRSession.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif