#define MOZ_UNIFIED_BUILD
#include "/worker/build/extensions/spellcheck/src/mozEnglishWordUtils.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/src/mozEnglishWordUtils.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/src/mozEnglishWordUtils.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/src/mozInlineSpellChecker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/src/mozInlineSpellChecker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/src/mozInlineSpellChecker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/src/mozInlineSpellWordUtil.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/src/mozInlineSpellWordUtil.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/src/mozInlineSpellWordUtil.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/src/mozPersonalDictionary.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/src/mozPersonalDictionary.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/src/mozPersonalDictionary.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/src/mozSpellChecker.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/src/mozSpellChecker.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/src/mozSpellChecker.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif