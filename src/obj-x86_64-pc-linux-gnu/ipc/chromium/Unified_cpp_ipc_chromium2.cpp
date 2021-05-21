#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/chromium/src/chrome/common/process_watcher_posix_sigchld.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/chrome/common/process_watcher_posix_sigchld.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/chrome/common/process_watcher_posix_sigchld.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif