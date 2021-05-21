#define MOZ_UNIFIED_BUILD
#include "PIdleSchedulerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PIdleSchedulerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PIdleSchedulerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PIdleSchedulerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PIdleSchedulerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PIdleSchedulerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PImageBridge.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PImageBridge.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PImageBridge.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PImageBridgeChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PImageBridgeChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PImageBridgeChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PImageBridgeParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PImageBridgeParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PImageBridgeParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInProcess.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInProcess.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInProcess.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInProcessChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInProcessChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInProcessChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInProcessParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInProcessParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInProcessParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInputChannelThrottleQueue.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInputChannelThrottleQueue.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInputChannelThrottleQueue.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInputChannelThrottleQueueChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInputChannelThrottleQueueChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInputChannelThrottleQueueChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PInputChannelThrottleQueueParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PInputChannelThrottleQueueParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PInputChannelThrottleQueueParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PLayerTransaction.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PLayerTransaction.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PLayerTransaction.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PLayerTransactionChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PLayerTransactionChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PLayerTransactionChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PLayerTransactionParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PLayerTransactionParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PLayerTransactionParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PLoginReputation.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PLoginReputation.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PLoginReputation.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PLoginReputationChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PLoginReputationChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PLoginReputationChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif