#define MOZ_UNIFIED_BUILD
#include "/worker/build/mozglue/baseprofiler/lul/LulMain.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mozglue/baseprofiler/lul/LulMain.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mozglue/baseprofiler/lul/LulMain.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/mozglue/baseprofiler/lul/platform-linux-lul.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/mozglue/baseprofiler/lul/platform-linux-lul.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/mozglue/baseprofiler/lul/platform-linux-lul.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif