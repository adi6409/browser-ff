#define MOZ_UNIFIED_BUILD
#include "/worker/build/xpcom/threads/TaskQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/TaskQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/TaskQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/ThreadEventQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/ThreadEventQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/ThreadEventQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/ThreadEventTarget.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/ThreadEventTarget.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/ThreadEventTarget.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/ThreadLocalVariables.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/ThreadLocalVariables.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/ThreadLocalVariables.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/ThrottledEventQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/ThrottledEventQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/ThrottledEventQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/TimerThread.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/TimerThread.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/TimerThread.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsEnvironment.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsEnvironment.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsEnvironment.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsMemoryPressure.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsMemoryPressure.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsMemoryPressure.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsProcessCommon.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsProcessCommon.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsProcessCommon.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsProxyRelease.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsProxyRelease.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsProxyRelease.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsThread.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsThread.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsThread.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsThreadManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsThreadManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsThreadManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsThreadPool.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsThreadPool.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsThreadPool.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsThreadUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsThreadUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsThreadUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/nsTimerImpl.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/nsTimerImpl.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/nsTimerImpl.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif