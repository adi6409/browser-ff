#define MOZ_UNIFIED_BUILD
#include "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_reader.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_reader.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_reader.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_value.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_value.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_value.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_writer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_writer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/toolkit/components/jsoncpp/src/lib_json/json_writer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif