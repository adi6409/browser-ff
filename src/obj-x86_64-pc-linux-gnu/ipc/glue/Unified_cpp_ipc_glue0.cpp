#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/glue/BackgroundImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/BackgroundImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/BackgroundImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/BackgroundUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/BackgroundUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/BackgroundUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/BrowserProcessSubThread.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/BrowserProcessSubThread.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/BrowserProcessSubThread.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/CrashReporterClient.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/CrashReporterClient.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/CrashReporterClient.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/CrashReporterHost.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/CrashReporterHost.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/CrashReporterHost.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/CrossProcessMutex_posix.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/CrossProcessMutex_posix.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/CrossProcessMutex_posix.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/CrossProcessSemaphore_posix.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/CrossProcessSemaphore_posix.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/CrossProcessSemaphore_posix.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/FileDescriptor.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/FileDescriptor.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/FileDescriptor.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/FileDescriptorShuffle.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/FileDescriptorShuffle.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/FileDescriptorShuffle.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/FileDescriptorUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/FileDescriptorUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/FileDescriptorUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ForkServer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ForkServer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ForkServer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/ForkServiceChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/ForkServiceChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/ForkServiceChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/GeckoChildProcessHost.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/GeckoChildProcessHost.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/GeckoChildProcessHost.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IPCMessageUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCMessageUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCMessageUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IPCStreamChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCStreamChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCStreamChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/glue/IPCStreamDestination.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/glue/IPCStreamDestination.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/glue/IPCStreamDestination.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif