#define MOZ_UNIFIED_BUILD
#include "/worker/build/editor/txmgr/TransactionItem.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/txmgr/TransactionItem.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/txmgr/TransactionItem.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/editor/txmgr/TransactionManager.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/txmgr/TransactionManager.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/txmgr/TransactionManager.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/editor/txmgr/TransactionStack.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/txmgr/TransactionStack.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/txmgr/TransactionStack.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif