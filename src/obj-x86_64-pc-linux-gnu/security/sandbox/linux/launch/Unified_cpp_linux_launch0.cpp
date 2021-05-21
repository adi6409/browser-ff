#define MOZ_UNIFIED_BUILD
#include "/worker/build/security/sandbox/linux/launch/LinuxCapabilities.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/launch/LinuxCapabilities.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/launch/LinuxCapabilities.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/security/sandbox/linux/launch/SandboxLaunch.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/security/sandbox/linux/launch/SandboxLaunch.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/security/sandbox/linux/launch/SandboxLaunch.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif