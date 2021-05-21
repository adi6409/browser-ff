#define MOZ_UNIFIED_BUILD
#include "/worker/build/extensions/spellcheck/hunspell/glue/RLBoxHunspell.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/RLBoxHunspell.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/RLBoxHunspell.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineChild.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineChild.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineChild.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineParent.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineParent.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/RemoteSpellCheckEngineParent.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/hunspell/glue/hunspell_csutil.cxx"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/hunspell_csutil.cxx uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/hunspell_csutil.cxx defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspell.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspell.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspell.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspellRLBoxHost.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspellRLBoxHost.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/extensions/spellcheck/hunspell/glue/mozHunspellRLBoxHost.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif