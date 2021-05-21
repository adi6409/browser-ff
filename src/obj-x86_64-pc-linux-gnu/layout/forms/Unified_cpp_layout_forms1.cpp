#define MOZ_UNIFIED_BUILD
#include "/worker/build/layout/forms/nsSelectsAreaFrame.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/forms/nsSelectsAreaFrame.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/forms/nsSelectsAreaFrame.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/layout/forms/nsTextControlFrame.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/layout/forms/nsTextControlFrame.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/layout/forms/nsTextControlFrame.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif