#define MOZ_UNIFIED_BUILD
#include "/worker/build/hal/HalWakeLock.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/HalWakeLock.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/HalWakeLock.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/WindowIdentifier.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/WindowIdentifier.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/WindowIdentifier.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/fallback/FallbackNetwork.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/fallback/FallbackNetwork.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/fallback/FallbackNetwork.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/fallback/FallbackProcessPriority.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/fallback/FallbackProcessPriority.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/fallback/FallbackProcessPriority.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/fallback/FallbackScreenConfiguration.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/fallback/FallbackScreenConfiguration.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/fallback/FallbackScreenConfiguration.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/fallback/FallbackSensor.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/fallback/FallbackSensor.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/fallback/FallbackSensor.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/fallback/FallbackVibration.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/fallback/FallbackVibration.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/fallback/FallbackVibration.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/linux/UPowerClient.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/linux/UPowerClient.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/linux/UPowerClient.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/hal/sandbox/SandboxHal.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/hal/sandbox/SandboxHal.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/hal/sandbox/SandboxHal.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif