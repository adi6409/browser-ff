#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/chromium/src/base/at_exit.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/at_exit.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/at_exit.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/command_line.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/command_line.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/command_line.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/condition_variable_posix.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/condition_variable_posix.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/condition_variable_posix.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/file_path.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/file_path.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/file_path.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/histogram.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/histogram.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/histogram.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/lock_impl_posix.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/lock_impl_posix.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/lock_impl_posix.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/logging.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/logging.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/logging.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/message_loop.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/message_loop.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/message_loop.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/message_pump_default.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/message_pump_default.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/message_pump_default.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/message_pump_libevent.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/message_pump_libevent.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/message_pump_libevent.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/pickle.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/pickle.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/pickle.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/platform_thread_posix.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/platform_thread_posix.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/platform_thread_posix.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/process_util_posix.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/process_util_posix.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/process_util_posix.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/rand_util.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/rand_util.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/rand_util.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/revocable_store.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/revocable_store.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/revocable_store.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/base/shared_memory_posix.cc"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/base/shared_memory_posix.cc uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/base/shared_memory_posix.cc defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif