#define MOZ_UNIFIED_BUILD
#include "/worker/build/accessible/ipc/DocAccessibleChildBase.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/accessible/ipc/DocAccessibleChildBase.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/accessible/ipc/DocAccessibleChildBase.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/accessible/ipc/DocAccessibleParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/accessible/ipc/DocAccessibleParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/accessible/ipc/DocAccessibleParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/accessible/ipc/RemoteAccessibleBase.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/accessible/ipc/RemoteAccessibleBase.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/accessible/ipc/RemoteAccessibleBase.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif