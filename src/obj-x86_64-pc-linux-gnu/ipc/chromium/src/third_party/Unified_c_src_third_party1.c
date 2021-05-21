#define MOZ_UNIFIED_BUILD
#include "/worker/build/ipc/chromium/src/third_party/libevent/http.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/http.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/http.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/listener.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/listener.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/listener.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/log.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/log.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/log.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/poll.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/poll.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/poll.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/select.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/select.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/select.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/signal.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/signal.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/signal.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/ipc/chromium/src/third_party/libevent/strlcpy.c"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/ipc/chromium/src/third_party/libevent/strlcpy.c uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/ipc/chromium/src/third_party/libevent/strlcpy.c defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif