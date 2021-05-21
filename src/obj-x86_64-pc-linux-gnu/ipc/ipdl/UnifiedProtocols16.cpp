#define MOZ_UNIFIED_BUILD
#include "PFilePickerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFilePickerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFilePickerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFilePickerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFilePickerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFilePickerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFileSystemParams.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFileSystemParams.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFileSystemParams.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFileSystemRequest.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFileSystemRequest.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFileSystemRequest.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFileSystemRequestChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFileSystemRequestChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFileSystemRequestChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFileSystemRequestParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFileSystemRequestParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFileSystemRequestParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFunctionBroker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFunctionBroker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFunctionBroker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFunctionBrokerChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFunctionBrokerChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFunctionBrokerChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PFunctionBrokerParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PFunctionBrokerParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PFunctionBrokerParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGIOChannel.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGIOChannel.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGIOChannel.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGIOChannelChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGIOChannelChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGIOChannelChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGIOChannelParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGIOChannelParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGIOChannelParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMP.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMP.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMP.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPContent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPContent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPContent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "PGMPContentChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "PGMPContentChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "PGMPContentChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif