#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/components/processtools/ProcInfo_linux.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/processtools/ProcInfo_linux.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/processtools/ProcInfo_linux.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/processtools/ProcessToolsService.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/processtools/ProcessToolsService.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/processtools/ProcessToolsService.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif