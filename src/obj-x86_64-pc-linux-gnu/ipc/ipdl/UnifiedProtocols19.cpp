#define MOZ_UNIFIED_BUILD
#include "PHeapSnapshotTempFileHelper.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHeapSnapshotTempFileHelper.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHeapSnapshotTempFileHelper.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHeapSnapshotTempFileHelperChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHeapSnapshotTempFileHelperChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHeapSnapshotTempFileHelperChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHeapSnapshotTempFileHelperParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHeapSnapshotTempFileHelperParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHeapSnapshotTempFileHelperParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpBackgroundChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpBackgroundChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpBackgroundChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpBackgroundChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpBackgroundChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpBackgroundChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpBackgroundChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpBackgroundChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpBackgroundChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpConnectionMgr.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpConnectionMgr.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpConnectionMgr.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpConnectionMgrChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpConnectionMgrChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpConnectionMgrChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpConnectionMgrParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpConnectionMgrParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpConnectionMgrParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpTransaction.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpTransaction.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpTransaction.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpTransactionChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpTransactionChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpTransactionChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PHttpTransactionParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PHttpTransactionParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PHttpTransactionParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PIdleScheduler.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PIdleScheduler.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PIdleScheduler.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif