#define MOZ_UNIFIED_BUILD
#include "/worker/build/netwerk/test/gtest/parse-ftp/TestParseFTPList.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/netwerk/test/gtest/parse-ftp/TestParseFTPList.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/netwerk/test/gtest/parse-ftp/TestParseFTPList.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif