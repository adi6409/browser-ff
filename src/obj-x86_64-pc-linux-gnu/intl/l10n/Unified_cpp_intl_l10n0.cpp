#define MOZ_UNIFIED_BUILD
#include "/worker/build/intl/l10n/FluentBundle.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/l10n/FluentBundle.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/l10n/FluentBundle.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/l10n/FluentResource.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/l10n/FluentResource.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/l10n/FluentResource.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/l10n/L10nRegistry.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/l10n/L10nRegistry.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/l10n/L10nRegistry.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/intl/l10n/Localization.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/intl/l10n/Localization.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/intl/l10n/Localization.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif