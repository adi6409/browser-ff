#define MOZ_UNIFIED_BUILD
#include "/worker/build/docshell/shistory/ChildSHistory.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/shistory/ChildSHistory.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/shistory/ChildSHistory.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/shistory/SessionHistoryEntry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/shistory/SessionHistoryEntry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/shistory/SessionHistoryEntry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/shistory/nsSHEntry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/shistory/nsSHEntry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/shistory/nsSHEntry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/shistory/nsSHEntryShared.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/shistory/nsSHEntryShared.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/shistory/nsSHEntryShared.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/docshell/shistory/nsSHistory.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/docshell/shistory/nsSHistory.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/docshell/shistory/nsSHistory.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif