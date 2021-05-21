#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/profile/gtest/TestProfileLock.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/profile/gtest/TestProfileLock.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/profile/gtest/TestProfileLock.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/profile/gtest/TestProfileLockRetry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/profile/gtest/TestProfileLockRetry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/profile/gtest/TestProfileLockRetry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif