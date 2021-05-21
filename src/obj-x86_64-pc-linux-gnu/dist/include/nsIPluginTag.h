/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginTag.idl
 */

#ifndef __gen_nsIPluginTag_h__
#define __gen_nsIPluginTag_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIPluginTag */
#define NS_IPLUGINTAG_IID_STR "5daa99d5-265a-4397-b429-c943803e2619"

#define NS_IPLUGINTAG_IID \
  {0x5daa99d5, 0x265a, 0x4397, \
    { 0xb4, 0x29, 0xc9, 0x43, 0x80, 0x3e, 0x26, 0x19 }}

class NS_NO_VTABLE nsIPluginTag : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPLUGINTAG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPluginTag;

  enum {
    STATE_DISABLED = 0U,
    STATE_CLICKTOPLAY = 1U,
    STATE_ENABLED = 2U
  };

  /* readonly attribute AUTF8String description; */
  NS_IMETHOD GetDescription(nsACString& aDescription) = 0;

  /* readonly attribute AUTF8String filename; */
  NS_IMETHOD GetFilename(nsACString& aFilename) = 0;

  /* readonly attribute AUTF8String fullpath; */
  NS_IMETHOD GetFullpath(nsACString& aFullpath) = 0;

  /* readonly attribute AUTF8String version; */
  NS_IMETHOD GetVersion(nsACString& aVersion) = 0;

  /* readonly attribute AUTF8String name; */
  NS_IMETHOD GetName(nsACString& aName) = 0;

  /* readonly attribute AUTF8String niceName; */
  NS_IMETHOD GetNiceName(nsACString& aNiceName) = 0;

  /* [infallible] readonly attribute boolean blocklisted; */
  NS_IMETHOD GetBlocklisted(bool *aBlocklisted) = 0;
  inline bool  GetBlocklisted()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBlocklisted(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isEnabledStateLocked; */
  NS_IMETHOD GetIsEnabledStateLocked(bool *aIsEnabledStateLocked) = 0;
  inline bool  GetIsEnabledStateLocked()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsEnabledStateLocked(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean active; */
  NS_IMETHOD GetActive(bool *aActive) = 0;
  inline bool  GetActive()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetActive(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long blocklistState; */
  NS_IMETHOD GetBlocklistState(uint32_t *aBlocklistState) = 0;
  inline uint32_t  GetBlocklistState()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetBlocklistState(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean disabled; */
  NS_IMETHOD GetDisabled(bool *aDisabled) = 0;
  inline bool  GetDisabled()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetDisabled(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean clicktoplay; */
  NS_IMETHOD GetClicktoplay(bool *aClicktoplay) = 0;
  inline bool  GetClicktoplay()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetClicktoplay(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean loaded; */
  NS_IMETHOD GetLoaded(bool *aLoaded) = 0;
  inline bool  GetLoaded()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoaded(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute unsigned long enabledState; */
  NS_IMETHOD GetEnabledState(uint32_t *aEnabledState) = 0;
  NS_IMETHOD SetEnabledState(uint32_t aEnabledState) = 0;

  /* readonly attribute PRTime lastModifiedTime; */
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) = 0;

  /* readonly attribute boolean isFlashPlugin; */
  NS_IMETHOD GetIsFlashPlugin(bool *aIsFlashPlugin) = 0;

  /* Array<AUTF8String> getMimeTypes (); */
  NS_IMETHOD GetMimeTypes(nsTArray<nsCString >& _retval) = 0;

  /* Array<AUTF8String> getMimeDescriptions (); */
  NS_IMETHOD GetMimeDescriptions(nsTArray<nsCString >& _retval) = 0;

  /* Array<AUTF8String> getExtensions (); */
  NS_IMETHOD GetExtensions(nsTArray<nsCString >& _retval) = 0;

  /* readonly attribute unsigned long id; */
  NS_IMETHOD GetId(uint32_t *aId) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPluginTag, NS_IPLUGINTAG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPLUGINTAG \
  NS_IMETHOD GetDescription(nsACString& aDescription) override; \
  NS_IMETHOD GetFilename(nsACString& aFilename) override; \
  NS_IMETHOD GetFullpath(nsACString& aFullpath) override; \
  NS_IMETHOD GetVersion(nsACString& aVersion) override; \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD GetNiceName(nsACString& aNiceName) override; \
  using nsIPluginTag::GetBlocklisted; \
  NS_IMETHOD GetBlocklisted(bool *aBlocklisted) override; \
  using nsIPluginTag::GetIsEnabledStateLocked; \
  NS_IMETHOD GetIsEnabledStateLocked(bool *aIsEnabledStateLocked) override; \
  using nsIPluginTag::GetActive; \
  NS_IMETHOD GetActive(bool *aActive) override; \
  using nsIPluginTag::GetBlocklistState; \
  NS_IMETHOD GetBlocklistState(uint32_t *aBlocklistState) override; \
  using nsIPluginTag::GetDisabled; \
  NS_IMETHOD GetDisabled(bool *aDisabled) override; \
  using nsIPluginTag::GetClicktoplay; \
  NS_IMETHOD GetClicktoplay(bool *aClicktoplay) override; \
  using nsIPluginTag::GetLoaded; \
  NS_IMETHOD GetLoaded(bool *aLoaded) override; \
  NS_IMETHOD GetEnabledState(uint32_t *aEnabledState) override; \
  NS_IMETHOD SetEnabledState(uint32_t aEnabledState) override; \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override; \
  NS_IMETHOD GetIsFlashPlugin(bool *aIsFlashPlugin) override; \
  NS_IMETHOD GetMimeTypes(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetMimeDescriptions(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetExtensions(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetId(uint32_t *aId) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPLUGINTAG \
  nsresult GetDescription(nsACString& aDescription); \
  nsresult GetFilename(nsACString& aFilename); \
  nsresult GetFullpath(nsACString& aFullpath); \
  nsresult GetVersion(nsACString& aVersion); \
  nsresult GetName(nsACString& aName); \
  nsresult GetNiceName(nsACString& aNiceName); \
  using nsIPluginTag::GetBlocklisted; \
  nsresult GetBlocklisted(bool *aBlocklisted); \
  using nsIPluginTag::GetIsEnabledStateLocked; \
  nsresult GetIsEnabledStateLocked(bool *aIsEnabledStateLocked); \
  using nsIPluginTag::GetActive; \
  nsresult GetActive(bool *aActive); \
  using nsIPluginTag::GetBlocklistState; \
  nsresult GetBlocklistState(uint32_t *aBlocklistState); \
  using nsIPluginTag::GetDisabled; \
  nsresult GetDisabled(bool *aDisabled); \
  using nsIPluginTag::GetClicktoplay; \
  nsresult GetClicktoplay(bool *aClicktoplay); \
  using nsIPluginTag::GetLoaded; \
  nsresult GetLoaded(bool *aLoaded); \
  nsresult GetEnabledState(uint32_t *aEnabledState); \
  nsresult SetEnabledState(uint32_t aEnabledState); \
  nsresult GetLastModifiedTime(PRTime *aLastModifiedTime); \
  nsresult GetIsFlashPlugin(bool *aIsFlashPlugin); \
  nsresult GetMimeTypes(nsTArray<nsCString >& _retval); \
  nsresult GetMimeDescriptions(nsTArray<nsCString >& _retval); \
  nsresult GetExtensions(nsTArray<nsCString >& _retval); \
  nsresult GetId(uint32_t *aId); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPLUGINTAG(_to) \
  NS_IMETHOD GetDescription(nsACString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD GetFilename(nsACString& aFilename) override { return _to GetFilename(aFilename); } \
  NS_IMETHOD GetFullpath(nsACString& aFullpath) override { return _to GetFullpath(aFullpath); } \
  NS_IMETHOD GetVersion(nsACString& aVersion) override { return _to GetVersion(aVersion); } \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetNiceName(nsACString& aNiceName) override { return _to GetNiceName(aNiceName); } \
  using nsIPluginTag::GetBlocklisted; \
  NS_IMETHOD GetBlocklisted(bool *aBlocklisted) override { return _to GetBlocklisted(aBlocklisted); } \
  using nsIPluginTag::GetIsEnabledStateLocked; \
  NS_IMETHOD GetIsEnabledStateLocked(bool *aIsEnabledStateLocked) override { return _to GetIsEnabledStateLocked(aIsEnabledStateLocked); } \
  using nsIPluginTag::GetActive; \
  NS_IMETHOD GetActive(bool *aActive) override { return _to GetActive(aActive); } \
  using nsIPluginTag::GetBlocklistState; \
  NS_IMETHOD GetBlocklistState(uint32_t *aBlocklistState) override { return _to GetBlocklistState(aBlocklistState); } \
  using nsIPluginTag::GetDisabled; \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return _to GetDisabled(aDisabled); } \
  using nsIPluginTag::GetClicktoplay; \
  NS_IMETHOD GetClicktoplay(bool *aClicktoplay) override { return _to GetClicktoplay(aClicktoplay); } \
  using nsIPluginTag::GetLoaded; \
  NS_IMETHOD GetLoaded(bool *aLoaded) override { return _to GetLoaded(aLoaded); } \
  NS_IMETHOD GetEnabledState(uint32_t *aEnabledState) override { return _to GetEnabledState(aEnabledState); } \
  NS_IMETHOD SetEnabledState(uint32_t aEnabledState) override { return _to SetEnabledState(aEnabledState); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return _to GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetIsFlashPlugin(bool *aIsFlashPlugin) override { return _to GetIsFlashPlugin(aIsFlashPlugin); } \
  NS_IMETHOD GetMimeTypes(nsTArray<nsCString >& _retval) override { return _to GetMimeTypes(_retval); } \
  NS_IMETHOD GetMimeDescriptions(nsTArray<nsCString >& _retval) override { return _to GetMimeDescriptions(_retval); } \
  NS_IMETHOD GetExtensions(nsTArray<nsCString >& _retval) override { return _to GetExtensions(_retval); } \
  NS_IMETHOD GetId(uint32_t *aId) override { return _to GetId(aId); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPLUGINTAG(_to) \
  NS_IMETHOD GetDescription(nsACString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD GetFilename(nsACString& aFilename) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilename(aFilename); } \
  NS_IMETHOD GetFullpath(nsACString& aFullpath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFullpath(aFullpath); } \
  NS_IMETHOD GetVersion(nsACString& aVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVersion(aVersion); } \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetNiceName(nsACString& aNiceName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNiceName(aNiceName); } \
  NS_IMETHOD GetBlocklisted(bool *aBlocklisted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlocklisted(aBlocklisted); } \
  NS_IMETHOD GetIsEnabledStateLocked(bool *aIsEnabledStateLocked) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsEnabledStateLocked(aIsEnabledStateLocked); } \
  NS_IMETHOD GetActive(bool *aActive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActive(aActive); } \
  NS_IMETHOD GetBlocklistState(uint32_t *aBlocklistState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlocklistState(aBlocklistState); } \
  NS_IMETHOD GetDisabled(bool *aDisabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisabled(aDisabled); } \
  NS_IMETHOD GetClicktoplay(bool *aClicktoplay) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClicktoplay(aClicktoplay); } \
  NS_IMETHOD GetLoaded(bool *aLoaded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoaded(aLoaded); } \
  NS_IMETHOD GetEnabledState(uint32_t *aEnabledState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnabledState(aEnabledState); } \
  NS_IMETHOD SetEnabledState(uint32_t aEnabledState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEnabledState(aEnabledState); } \
  NS_IMETHOD GetLastModifiedTime(PRTime *aLastModifiedTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastModifiedTime(aLastModifiedTime); } \
  NS_IMETHOD GetIsFlashPlugin(bool *aIsFlashPlugin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFlashPlugin(aIsFlashPlugin); } \
  NS_IMETHOD GetMimeTypes(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMimeTypes(_retval); } \
  NS_IMETHOD GetMimeDescriptions(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMimeDescriptions(_retval); } \
  NS_IMETHOD GetExtensions(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensions(_retval); } \
  NS_IMETHOD GetId(uint32_t *aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } 


/* starting interface:    nsIFakePluginTag */
#define NS_IFAKEPLUGINTAG_IID_STR "6d22c968-226d-4156-b230-da6ad6bbf6e8"

#define NS_IFAKEPLUGINTAG_IID \
  {0x6d22c968, 0x226d, 0x4156, \
    { 0xb2, 0x30, 0xda, 0x6a, 0xd6, 0xbb, 0xf6, 0xe8 }}

class NS_NO_VTABLE nsIFakePluginTag : public nsIPluginTag {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFAKEPLUGINTAG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFakePluginTag;

  /* readonly attribute nsIURI handlerURI; */
  NS_IMETHOD GetHandlerURI(nsIURI **aHandlerURI) = 0;

  /* readonly attribute AString sandboxScript; */
  NS_IMETHOD GetSandboxScript(nsAString& aSandboxScript) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFakePluginTag, NS_IFAKEPLUGINTAG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFAKEPLUGINTAG \
  NS_IMETHOD GetHandlerURI(nsIURI **aHandlerURI) override; \
  NS_IMETHOD GetSandboxScript(nsAString& aSandboxScript) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFAKEPLUGINTAG \
  nsresult GetHandlerURI(nsIURI **aHandlerURI); \
  nsresult GetSandboxScript(nsAString& aSandboxScript); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFAKEPLUGINTAG(_to) \
  NS_IMETHOD GetHandlerURI(nsIURI **aHandlerURI) override { return _to GetHandlerURI(aHandlerURI); } \
  NS_IMETHOD GetSandboxScript(nsAString& aSandboxScript) override { return _to GetSandboxScript(aSandboxScript); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFAKEPLUGINTAG(_to) \
  NS_IMETHOD GetHandlerURI(nsIURI **aHandlerURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHandlerURI(aHandlerURI); } \
  NS_IMETHOD GetSandboxScript(nsAString& aSandboxScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSandboxScript(aSandboxScript); } 


#endif /* __gen_nsIPluginTag_h__ */
