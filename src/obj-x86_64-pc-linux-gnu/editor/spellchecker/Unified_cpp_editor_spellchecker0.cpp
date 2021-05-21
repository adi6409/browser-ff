#define MOZ_UNIFIED_BUILD
#include "/worker/build/editor/spellchecker/EditorSpellCheck.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/spellchecker/EditorSpellCheck.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/spellchecker/EditorSpellCheck.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/editor/spellchecker/FilteredContentIterator.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/spellchecker/FilteredContentIterator.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/spellchecker/FilteredContentIterator.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/editor/spellchecker/TextServicesDocument.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/spellchecker/TextServicesDocument.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/spellchecker/TextServicesDocument.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/editor/spellchecker/nsComposeTxtSrvFilter.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/editor/spellchecker/nsComposeTxtSrvFilter.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/editor/spellchecker/nsComposeTxtSrvFilter.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif