#define MOZ_UNIFIED_BUILD
#include "/worker/build/tools/profiler/lul/LulCommon.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/LulCommon.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/LulCommon.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/lul/LulDwarf.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/LulDwarf.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/LulDwarf.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/lul/LulDwarfSummariser.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/LulDwarfSummariser.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/LulDwarfSummariser.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/lul/LulElf.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/LulElf.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/LulElf.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/lul/LulMain.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/LulMain.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/LulMain.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/tools/profiler/lul/platform-linux-lul.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/tools/profiler/lul/platform-linux-lul.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/tools/profiler/lul/platform-linux-lul.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif