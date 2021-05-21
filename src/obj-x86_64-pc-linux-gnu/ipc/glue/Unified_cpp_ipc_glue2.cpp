#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/glue/SharedMemory.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/SharedMemory.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/SharedMemory.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/SharedMemory_posix.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/SharedMemory_posix.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/SharedMemory_posix.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/Shmem.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/Shmem.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/Shmem.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/StringUtil.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/StringUtil.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/StringUtil.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/TransportSecurityInfoUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/TransportSecurityInfoUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/TransportSecurityInfoUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/Transport_posix.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/Transport_posix.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/Transport_posix.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/URIUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/URIUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/URIUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif