#define MOZ_UNIFIED_BUILD
#include "/worker/build/xpcom/threads/AbstractThread.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/AbstractThread.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/AbstractThread.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/BlockingResourceBase.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/BlockingResourceBase.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/BlockingResourceBase.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/CPUUsageWatcher.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/CPUUsageWatcher.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/CPUUsageWatcher.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/EventQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/EventQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/EventQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/IdlePeriodState.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/IdlePeriodState.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/IdlePeriodState.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/InputEventStatistics.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/InputEventStatistics.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/InputEventStatistics.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/InputTaskManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/InputTaskManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/InputTaskManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/LazyIdleThread.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/LazyIdleThread.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/LazyIdleThread.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/MainThreadIdlePeriod.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/MainThreadIdlePeriod.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/MainThreadIdlePeriod.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/PerformanceCounter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/PerformanceCounter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/PerformanceCounter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/RWLock.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/RWLock.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/RWLock.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/RecursiveMutex.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/RecursiveMutex.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/RecursiveMutex.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/SchedulerGroup.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/SchedulerGroup.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/SchedulerGroup.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/SharedThreadPool.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/SharedThreadPool.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/SharedThreadPool.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/SynchronizedEventQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/SynchronizedEventQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/SynchronizedEventQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/xpcom/threads/TaskController.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/xpcom/threads/TaskController.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/xpcom/threads/TaskController.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif