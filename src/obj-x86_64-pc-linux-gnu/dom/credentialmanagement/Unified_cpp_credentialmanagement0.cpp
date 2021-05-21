#define MOZ_UNIFIED_BUILD
#include "/worker/build/dom/credentialmanagement/Credential.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/credentialmanagement/Credential.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/credentialmanagement/Credential.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif
#include "/worker/build/dom/credentialmanagement/CredentialsContainer.cpp"
#ifdef PL_ARENA_CONST_ALIGN_MASK
#error "/worker/build/dom/credentialmanagement/CredentialsContainer.cpp uses PL_ARENA_CONST_ALIGN_MASK, so it cannot be built in unified mode."
#undef PL_ARENA_CONST_ALIGN_MASK
#endif
#ifdef INITGUID
#error "/worker/build/dom/credentialmanagement/CredentialsContainer.cpp defines INITGUID, so it cannot be built in unified mode."
#undef INITGUID
#endif