/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIPrincipal.idl
 */

#ifndef __gen_nsIPrincipal_h__
#define __gen_nsIPrincipal_h__


#ifndef __gen_nsIContentSecurityPolicy_h__
#include "nsIContentSecurityPolicy.h"
#endif

#ifndef __gen_nsISerializable_h__
#include "nsISerializable.h"
#endif

#ifndef __gen_nsIAboutModule_h__
#include "nsIAboutModule.h"
#endif

#ifndef __gen_nsIReferrerInfo_h__
#include "nsIReferrerInfo.h"
#endif

#ifndef __gen_mozIDOMWindow_h__
#include "mozIDOMWindow.h"
#endif

#include "js/Value.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

struct JSPrincipals;
#include "nsCOMPtr.h"
#include "nsTArray.h"
#include "nsString.h"
#include "mozilla/DebugOnly.h"
namespace mozilla {
class OriginAttributes;
}
/**
 * Some methods have a fast path for the case when we're comparing a principal
 * to itself. The situation may happen for example with about:blank documents.
 */
#define DECL_FAST_INLINE_HELPER(method_)                       \
  inline bool method_(nsIPrincipal* aOther)                    \
  {                                                            \
    mozilla::DebugOnly<bool> val = false;                      \
    MOZ_ASSERT_IF(this == aOther,                              \
                  NS_SUCCEEDED(method_(aOther, &val)) && val); \
                                                               \
    bool retVal = false;                                       \
    return                                                     \
      this == aOther ||                                        \
      (NS_SUCCEEDED(method_(aOther, &retVal)) && retVal);      \
  }
class nsIURI; /* forward declaration */


/* starting interface:    nsIPrincipal */
#define NS_IPRINCIPAL_IID_STR "f75f502d-79fd-48be-a079-e5a7b8f80c8b"

#define NS_IPRINCIPAL_IID \
  {0xf75f502d, 0x79fd, 0x48be, \
    { 0xa0, 0x79, 0xe5, 0xa7, 0xb8, 0xf8, 0x0c, 0x8b }}

class nsIPrincipal : public nsISerializable {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRINCIPAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrincipal;

  /* boolean equals (in nsIPrincipal other); */
  NS_IMETHOD Equals(nsIPrincipal *other, bool *_retval) = 0;

  /* boolean equalsForPermission (in nsIPrincipal other, in bool aExactHost); */
  NS_IMETHOD EqualsForPermission(nsIPrincipal *other, bool aExactHost, bool *_retval) = 0;

  /* boolean equalsConsideringDomain (in nsIPrincipal other); */
  NS_IMETHOD EqualsConsideringDomain(nsIPrincipal *other, bool *_retval) = 0;

       DECL_FAST_INLINE_HELPER(Equals)
      DECL_FAST_INLINE_HELPER(EqualsConsideringDomain)
      /* boolean equalsURI (in nsIURI aOtherURI); */
  NS_IMETHOD EqualsURI(nsIURI *aOtherURI, bool *_retval) = 0;

  /* [nostdcall,notxpcom] readonly attribute unsigned long hashValue; */
  virtual uint32_t GetHashValue() = 0;

  /* [infallible] readonly attribute nsIURI URI; */
  NS_IMETHOD GetURI(nsIURI **aURI) = 0;
   inline already_AddRefed<nsIURI> GetURI()
  {
    nsIURI* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIURI>(result);
  }

  /* [noscript] attribute nsIURI domain; */
  NS_IMETHOD GetDomain(nsIURI **aDomain) = 0;
  NS_IMETHOD SetDomain(nsIURI *aDomain) = 0;

  /* boolean subsumes (in nsIPrincipal other); */
  NS_IMETHOD Subsumes(nsIPrincipal *other, bool *_retval) = 0;

  /* boolean subsumesConsideringDomain (in nsIPrincipal other); */
  NS_IMETHOD SubsumesConsideringDomain(nsIPrincipal *other, bool *_retval) = 0;

  /* boolean subsumesConsideringDomainIgnoringFPD (in nsIPrincipal other); */
  NS_IMETHOD SubsumesConsideringDomainIgnoringFPD(nsIPrincipal *other, bool *_retval) = 0;

       DECL_FAST_INLINE_HELPER(Subsumes)
      DECL_FAST_INLINE_HELPER(SubsumesConsideringDomain)
      DECL_FAST_INLINE_HELPER(SubsumesConsideringDomainIgnoringFPD)
#undef DECL_FAST_INLINE_HELPER
      /* void checkMayLoad (in nsIURI uri, in boolean allowIfInheritsPrincipal); */
  NS_IMETHOD CheckMayLoad(nsIURI *uri, bool allowIfInheritsPrincipal) = 0;

  /* void checkMayLoadWithReporting (in nsIURI uri, in boolean allowIfInheritsPrincipal, in unsigned long long innerWindowID); */
  NS_IMETHOD CheckMayLoadWithReporting(nsIURI *uri, bool allowIfInheritsPrincipal, uint64_t innerWindowID) = 0;

  /* boolean isThirdPartyURI (in nsIURI uri); */
  NS_IMETHOD IsThirdPartyURI(nsIURI *uri, bool *_retval) = 0;

  /* boolean isThirdPartyPrincipal (in nsIPrincipal principal); */
  NS_IMETHOD IsThirdPartyPrincipal(nsIPrincipal *principal, bool *_retval) = 0;

  /* boolean isThirdPartyChannel (in nsIChannel channel); */
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *channel, bool *_retval) = 0;

  /* [implicit_jscontext] readonly attribute jsval originAttributes; */
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;

  /* [binaryname(OriginAttributesRef),noscript,nostdcall,notxpcom] const_OriginAttributes OriginAttributesRef (); */
  virtual const mozilla::OriginAttributes & OriginAttributesRef(void) = 0;

  /* readonly attribute ACString origin; */
  NS_IMETHOD GetOrigin(nsACString& aOrigin) = 0;

  /* [noscript] readonly attribute ACString asciiOrigin; */
  NS_IMETHOD GetAsciiOrigin(nsACString& aAsciiOrigin) = 0;

  /* readonly attribute ACString hostPort; */
  NS_IMETHOD GetHostPort(nsACString& aHostPort) = 0;

  /* readonly attribute ACString asciiHost; */
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) = 0;

  /* readonly attribute ACString host; */
  NS_IMETHOD GetHost(nsACString& aHost) = 0;

  /* readonly attribute ACString prepath; */
  NS_IMETHOD GetPrepath(nsACString& aPrepath) = 0;

  /* readonly attribute ACString filePath; */
  NS_IMETHOD GetFilePath(nsACString& aFilePath) = 0;

  /* readonly attribute ACString asciiSpec; */
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) = 0;

  /* readonly attribute ACString spec; */
  NS_IMETHOD GetSpec(nsACString& aSpec) = 0;

  /* readonly attribute ACString exposablePrePath; */
  NS_IMETHOD GetExposablePrePath(nsACString& aExposablePrePath) = 0;

  /* readonly attribute ACString exposableSpec; */
  NS_IMETHOD GetExposableSpec(nsACString& aExposableSpec) = 0;

  /* readonly attribute ACString scheme; */
  NS_IMETHOD GetScheme(nsACString& aScheme) = 0;

  /* boolean schemeIs (in string scheme); */
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) = 0;

       inline bool SchemeIs(const char* aScheme) {
        bool ret;
        SchemeIs(aScheme, &ret);
        return ret;
      }
      /* bool isURIInPrefList (in string pref); */
  NS_IMETHOD IsURIInPrefList(const char * pref, bool *_retval) = 0;

  /* bool isSameOrigin (in nsIURI otherURI, in bool aIsPrivateWin); */
  NS_IMETHOD IsSameOrigin(nsIURI *otherURI, bool aIsPrivateWin, bool *_retval) = 0;

  /* bool allowsRelaxStrictFileOriginPolicy (in nsIURI aURI); */
  NS_IMETHOD AllowsRelaxStrictFileOriginPolicy(nsIURI *aURI, bool *_retval) = 0;

  /* [noscript] ACString getPrefLightCacheKey (in nsIURI aURI, in bool aWithCredentials, in const_OriginAttributes aOriginAttributes); */
  NS_IMETHOD GetPrefLightCacheKey(nsIURI *aURI, bool aWithCredentials, const mozilla::OriginAttributes & aOriginAttributes, nsACString& _retval) = 0;

  /* bool hasFirstpartyStorageAccess (in mozIDOMWindow aWindow, out uint32_t rejectedReason); */
  NS_IMETHOD HasFirstpartyStorageAccess(mozIDOMWindow *aWindow, uint32_t *rejectedReason, bool *_retval) = 0;

  /* readonly attribute ACString localStorageQuotaKey; */
  NS_IMETHOD GetLocalStorageQuotaKey(nsACString& aLocalStorageQuotaKey) = 0;

  /* [infallible] readonly attribute boolean isOriginPotentiallyTrustworthy; */
  NS_IMETHOD GetIsOriginPotentiallyTrustworthy(bool *aIsOriginPotentiallyTrustworthy) = 0;
  inline bool  GetIsOriginPotentiallyTrustworthy()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsOriginPotentiallyTrustworthy(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* uint32_t getAboutModuleFlags (); */
  NS_IMETHOD GetAboutModuleFlags(uint32_t *_retval) = 0;

  /* readonly attribute ACString storageOriginKey; */
  NS_IMETHOD GetStorageOriginKey(nsACString& aStorageOriginKey) = 0;

  /* nsIReferrerInfo createReferrerInfo (in ReferrerPolicy aReferrerPolicy); */
  NS_IMETHOD CreateReferrerInfo(mozilla::dom::ReferrerPolicy aReferrerPolicy, nsIReferrerInfo **_retval) = 0;

  /* readonly attribute ACString originNoSuffix; */
  NS_IMETHOD GetOriginNoSuffix(nsACString& aOriginNoSuffix) = 0;

  /* readonly attribute AUTF8String originSuffix; */
  NS_IMETHOD GetOriginSuffix(nsACString& aOriginSuffix) = 0;

  /* readonly attribute ACString siteOrigin; */
  NS_IMETHOD GetSiteOrigin(nsACString& aSiteOrigin) = 0;

  /* readonly attribute ACString siteOriginNoSuffix; */
  NS_IMETHOD GetSiteOriginNoSuffix(nsACString& aSiteOriginNoSuffix) = 0;

  /* readonly attribute ACString baseDomain; */
  NS_IMETHOD GetBaseDomain(nsACString& aBaseDomain) = 0;

  /* readonly attribute AString addonId; */
  NS_IMETHOD GetAddonId(nsAString& aAddonId) = 0;

  /* readonly attribute nsISupports addonPolicy; */
  NS_IMETHOD GetAddonPolicy(nsISupports **aAddonPolicy) = 0;

  /* [infallible] readonly attribute unsigned long userContextId; */
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) = 0;
  inline uint32_t  GetUserContextId()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetUserContextId(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long privateBrowsingId; */
  NS_IMETHOD GetPrivateBrowsingId(uint32_t *aPrivateBrowsingId) = 0;
  inline uint32_t  GetPrivateBrowsingId()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetPrivateBrowsingId(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isInIsolatedMozBrowserElement; */
  NS_IMETHOD GetIsInIsolatedMozBrowserElement(bool *aIsInIsolatedMozBrowserElement) = 0;
  inline bool  GetIsInIsolatedMozBrowserElement()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsInIsolatedMozBrowserElement(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isNullPrincipal; */
  NS_IMETHOD GetIsNullPrincipal(bool *aIsNullPrincipal) = 0;
  inline bool  GetIsNullPrincipal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsNullPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isContentPrincipal; */
  NS_IMETHOD GetIsContentPrincipal(bool *aIsContentPrincipal) = 0;
  inline bool  GetIsContentPrincipal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsContentPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isExpandedPrincipal; */
  NS_IMETHOD GetIsExpandedPrincipal(bool *aIsExpandedPrincipal) = 0;
  inline bool  GetIsExpandedPrincipal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsExpandedPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute boolean isSystemPrincipal; */
  NS_IMETHOD GetIsSystemPrincipal(bool *aIsSystemPrincipal) = 0;

       inline bool IsSystemPrincipal() const;
      /* [infallible] readonly attribute boolean isAddonOrExpandedAddonPrincipal; */
  NS_IMETHOD GetIsAddonOrExpandedAddonPrincipal(bool *aIsAddonOrExpandedAddonPrincipal) = 0;
  inline bool  GetIsAddonOrExpandedAddonPrincipal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsAddonOrExpandedAddonPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

     // MOZ_DBG support
    friend std::ostream& operator<<(std::ostream& aOut, const nsIPrincipal& aPrincipal) {
      nsIPrincipal* principal = const_cast<nsIPrincipal*>(&aPrincipal);
      nsAutoCString origin;
      mozilla::DebugOnly<nsresult> rv = principal->GetOrigin(origin);
      MOZ_ASSERT(NS_SUCCEEDED(rv));
      return aOut << "nsIPrincipal { " << origin << " }";
    }
      /* [infallible] readonly attribute boolean isOnion; */
  NS_IMETHOD GetIsOnion(bool *aIsOnion) = 0;
  inline bool  GetIsOnion()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsOnion(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* readonly attribute boolean isScriptAllowedByPolicy; */
  NS_IMETHOD GetIsScriptAllowedByPolicy(bool *aIsScriptAllowedByPolicy) = 0;

  /* boolean isL10nAllowed (in nsIURI aDocumentURI); */
  NS_IMETHOD IsL10nAllowed(nsIURI *aDocumentURI, bool *_retval) = 0;

  /* [infallible] readonly attribute nsIPrincipal nextSubDomainPrincipal; */
  NS_IMETHOD GetNextSubDomainPrincipal(nsIPrincipal **aNextSubDomainPrincipal) = 0;
   inline already_AddRefed<nsIPrincipal> GetNextSubDomainPrincipal()
  {
    nsIPrincipal* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetNextSubDomainPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<nsIPrincipal>(result);
  }

  /* [infallible] readonly attribute boolean isIpAddress; */
  NS_IMETHOD GetIsIpAddress(bool *aIsIpAddress) = 0;
  inline bool  GetIsIpAddress()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsIpAddress(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isLocalIpAddress; */
  NS_IMETHOD GetIsLocalIpAddress(bool *aIsLocalIpAddress) = 0;
  inline bool  GetIsLocalIpAddress()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsLocalIpAddress(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrincipal, NS_IPRINCIPAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRINCIPAL \
  NS_IMETHOD Equals(nsIPrincipal *other, bool *_retval) override; \
  NS_IMETHOD EqualsForPermission(nsIPrincipal *other, bool aExactHost, bool *_retval) override; \
  NS_IMETHOD EqualsConsideringDomain(nsIPrincipal *other, bool *_retval) override; \
  NS_IMETHOD EqualsURI(nsIURI *aOtherURI, bool *_retval) override; \
  virtual uint32_t GetHashValue() override; \
  using nsIPrincipal::GetURI; \
  NS_IMETHOD GetURI(nsIURI **aURI) override; \
  NS_IMETHOD GetDomain(nsIURI **aDomain) override; \
  NS_IMETHOD SetDomain(nsIURI *aDomain) override; \
  NS_IMETHOD Subsumes(nsIPrincipal *other, bool *_retval) override; \
  NS_IMETHOD SubsumesConsideringDomain(nsIPrincipal *other, bool *_retval) override; \
  NS_IMETHOD SubsumesConsideringDomainIgnoringFPD(nsIPrincipal *other, bool *_retval) override; \
  NS_IMETHOD CheckMayLoad(nsIURI *uri, bool allowIfInheritsPrincipal) override; \
  NS_IMETHOD CheckMayLoadWithReporting(nsIURI *uri, bool allowIfInheritsPrincipal, uint64_t innerWindowID) override; \
  NS_IMETHOD IsThirdPartyURI(nsIURI *uri, bool *_retval) override; \
  NS_IMETHOD IsThirdPartyPrincipal(nsIPrincipal *principal, bool *_retval) override; \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *channel, bool *_retval) override; \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  virtual const mozilla::OriginAttributes & OriginAttributesRef(void) override; \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override; \
  NS_IMETHOD GetAsciiOrigin(nsACString& aAsciiOrigin) override; \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override; \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override; \
  NS_IMETHOD GetHost(nsACString& aHost) override; \
  NS_IMETHOD GetPrepath(nsACString& aPrepath) override; \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override; \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override; \
  NS_IMETHOD GetSpec(nsACString& aSpec) override; \
  NS_IMETHOD GetExposablePrePath(nsACString& aExposablePrePath) override; \
  NS_IMETHOD GetExposableSpec(nsACString& aExposableSpec) override; \
  NS_IMETHOD GetScheme(nsACString& aScheme) override; \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override; \
  NS_IMETHOD IsURIInPrefList(const char * pref, bool *_retval) override; \
  NS_IMETHOD IsSameOrigin(nsIURI *otherURI, bool aIsPrivateWin, bool *_retval) override; \
  NS_IMETHOD AllowsRelaxStrictFileOriginPolicy(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD GetPrefLightCacheKey(nsIURI *aURI, bool aWithCredentials, const mozilla::OriginAttributes & aOriginAttributes, nsACString& _retval) override; \
  NS_IMETHOD HasFirstpartyStorageAccess(mozIDOMWindow *aWindow, uint32_t *rejectedReason, bool *_retval) override; \
  NS_IMETHOD GetLocalStorageQuotaKey(nsACString& aLocalStorageQuotaKey) override; \
  using nsIPrincipal::GetIsOriginPotentiallyTrustworthy; \
  NS_IMETHOD GetIsOriginPotentiallyTrustworthy(bool *aIsOriginPotentiallyTrustworthy) override; \
  NS_IMETHOD GetAboutModuleFlags(uint32_t *_retval) override; \
  NS_IMETHOD GetStorageOriginKey(nsACString& aStorageOriginKey) override; \
  NS_IMETHOD CreateReferrerInfo(mozilla::dom::ReferrerPolicy aReferrerPolicy, nsIReferrerInfo **_retval) override; \
  NS_IMETHOD GetOriginNoSuffix(nsACString& aOriginNoSuffix) override; \
  NS_IMETHOD GetOriginSuffix(nsACString& aOriginSuffix) override; \
  NS_IMETHOD GetSiteOrigin(nsACString& aSiteOrigin) override; \
  NS_IMETHOD GetSiteOriginNoSuffix(nsACString& aSiteOriginNoSuffix) override; \
  NS_IMETHOD GetBaseDomain(nsACString& aBaseDomain) override; \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override; \
  NS_IMETHOD GetAddonPolicy(nsISupports **aAddonPolicy) override; \
  using nsIPrincipal::GetUserContextId; \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override; \
  using nsIPrincipal::GetPrivateBrowsingId; \
  NS_IMETHOD GetPrivateBrowsingId(uint32_t *aPrivateBrowsingId) override; \
  using nsIPrincipal::GetIsInIsolatedMozBrowserElement; \
  NS_IMETHOD GetIsInIsolatedMozBrowserElement(bool *aIsInIsolatedMozBrowserElement) override; \
  using nsIPrincipal::GetIsNullPrincipal; \
  NS_IMETHOD GetIsNullPrincipal(bool *aIsNullPrincipal) override; \
  using nsIPrincipal::GetIsContentPrincipal; \
  NS_IMETHOD GetIsContentPrincipal(bool *aIsContentPrincipal) override; \
  using nsIPrincipal::GetIsExpandedPrincipal; \
  NS_IMETHOD GetIsExpandedPrincipal(bool *aIsExpandedPrincipal) override; \
  NS_IMETHOD GetIsSystemPrincipal(bool *aIsSystemPrincipal) override; \
  using nsIPrincipal::GetIsAddonOrExpandedAddonPrincipal; \
  NS_IMETHOD GetIsAddonOrExpandedAddonPrincipal(bool *aIsAddonOrExpandedAddonPrincipal) override; \
  using nsIPrincipal::GetIsOnion; \
  NS_IMETHOD GetIsOnion(bool *aIsOnion) override; \
  NS_IMETHOD GetIsScriptAllowedByPolicy(bool *aIsScriptAllowedByPolicy) override; \
  NS_IMETHOD IsL10nAllowed(nsIURI *aDocumentURI, bool *_retval) override; \
  using nsIPrincipal::GetNextSubDomainPrincipal; \
  NS_IMETHOD GetNextSubDomainPrincipal(nsIPrincipal **aNextSubDomainPrincipal) override; \
  using nsIPrincipal::GetIsIpAddress; \
  NS_IMETHOD GetIsIpAddress(bool *aIsIpAddress) override; \
  using nsIPrincipal::GetIsLocalIpAddress; \
  NS_IMETHOD GetIsLocalIpAddress(bool *aIsLocalIpAddress) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRINCIPAL \
  nsresult Equals(nsIPrincipal *other, bool *_retval); \
  nsresult EqualsForPermission(nsIPrincipal *other, bool aExactHost, bool *_retval); \
  nsresult EqualsConsideringDomain(nsIPrincipal *other, bool *_retval); \
  nsresult EqualsURI(nsIURI *aOtherURI, bool *_retval); \
  uint32_t GetHashValue(); \
  using nsIPrincipal::GetURI; \
  nsresult GetURI(nsIURI **aURI); \
  nsresult GetDomain(nsIURI **aDomain); \
  nsresult SetDomain(nsIURI *aDomain); \
  nsresult Subsumes(nsIPrincipal *other, bool *_retval); \
  nsresult SubsumesConsideringDomain(nsIPrincipal *other, bool *_retval); \
  nsresult SubsumesConsideringDomainIgnoringFPD(nsIPrincipal *other, bool *_retval); \
  nsresult CheckMayLoad(nsIURI *uri, bool allowIfInheritsPrincipal); \
  nsresult CheckMayLoadWithReporting(nsIURI *uri, bool allowIfInheritsPrincipal, uint64_t innerWindowID); \
  nsresult IsThirdPartyURI(nsIURI *uri, bool *_retval); \
  nsresult IsThirdPartyPrincipal(nsIPrincipal *principal, bool *_retval); \
  nsresult IsThirdPartyChannel(nsIChannel *channel, bool *_retval); \
  nsresult GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  const mozilla::OriginAttributes & OriginAttributesRef(void); \
  nsresult GetOrigin(nsACString& aOrigin); \
  nsresult GetAsciiOrigin(nsACString& aAsciiOrigin); \
  nsresult GetHostPort(nsACString& aHostPort); \
  nsresult GetAsciiHost(nsACString& aAsciiHost); \
  nsresult GetHost(nsACString& aHost); \
  nsresult GetPrepath(nsACString& aPrepath); \
  nsresult GetFilePath(nsACString& aFilePath); \
  nsresult GetAsciiSpec(nsACString& aAsciiSpec); \
  nsresult GetSpec(nsACString& aSpec); \
  nsresult GetExposablePrePath(nsACString& aExposablePrePath); \
  nsresult GetExposableSpec(nsACString& aExposableSpec); \
  nsresult GetScheme(nsACString& aScheme); \
  nsresult SchemeIs(const char * scheme, bool *_retval); \
  nsresult IsURIInPrefList(const char * pref, bool *_retval); \
  nsresult IsSameOrigin(nsIURI *otherURI, bool aIsPrivateWin, bool *_retval); \
  nsresult AllowsRelaxStrictFileOriginPolicy(nsIURI *aURI, bool *_retval); \
  nsresult GetPrefLightCacheKey(nsIURI *aURI, bool aWithCredentials, const mozilla::OriginAttributes & aOriginAttributes, nsACString& _retval); \
  nsresult HasFirstpartyStorageAccess(mozIDOMWindow *aWindow, uint32_t *rejectedReason, bool *_retval); \
  nsresult GetLocalStorageQuotaKey(nsACString& aLocalStorageQuotaKey); \
  using nsIPrincipal::GetIsOriginPotentiallyTrustworthy; \
  nsresult GetIsOriginPotentiallyTrustworthy(bool *aIsOriginPotentiallyTrustworthy); \
  nsresult GetAboutModuleFlags(uint32_t *_retval); \
  nsresult GetStorageOriginKey(nsACString& aStorageOriginKey); \
  nsresult CreateReferrerInfo(mozilla::dom::ReferrerPolicy aReferrerPolicy, nsIReferrerInfo **_retval); \
  nsresult GetOriginNoSuffix(nsACString& aOriginNoSuffix); \
  nsresult GetOriginSuffix(nsACString& aOriginSuffix); \
  nsresult GetSiteOrigin(nsACString& aSiteOrigin); \
  nsresult GetSiteOriginNoSuffix(nsACString& aSiteOriginNoSuffix); \
  nsresult GetBaseDomain(nsACString& aBaseDomain); \
  nsresult GetAddonId(nsAString& aAddonId); \
  nsresult GetAddonPolicy(nsISupports **aAddonPolicy); \
  using nsIPrincipal::GetUserContextId; \
  nsresult GetUserContextId(uint32_t *aUserContextId); \
  using nsIPrincipal::GetPrivateBrowsingId; \
  nsresult GetPrivateBrowsingId(uint32_t *aPrivateBrowsingId); \
  using nsIPrincipal::GetIsInIsolatedMozBrowserElement; \
  nsresult GetIsInIsolatedMozBrowserElement(bool *aIsInIsolatedMozBrowserElement); \
  using nsIPrincipal::GetIsNullPrincipal; \
  nsresult GetIsNullPrincipal(bool *aIsNullPrincipal); \
  using nsIPrincipal::GetIsContentPrincipal; \
  nsresult GetIsContentPrincipal(bool *aIsContentPrincipal); \
  using nsIPrincipal::GetIsExpandedPrincipal; \
  nsresult GetIsExpandedPrincipal(bool *aIsExpandedPrincipal); \
  nsresult GetIsSystemPrincipal(bool *aIsSystemPrincipal); \
  using nsIPrincipal::GetIsAddonOrExpandedAddonPrincipal; \
  nsresult GetIsAddonOrExpandedAddonPrincipal(bool *aIsAddonOrExpandedAddonPrincipal); \
  using nsIPrincipal::GetIsOnion; \
  nsresult GetIsOnion(bool *aIsOnion); \
  nsresult GetIsScriptAllowedByPolicy(bool *aIsScriptAllowedByPolicy); \
  nsresult IsL10nAllowed(nsIURI *aDocumentURI, bool *_retval); \
  using nsIPrincipal::GetNextSubDomainPrincipal; \
  nsresult GetNextSubDomainPrincipal(nsIPrincipal **aNextSubDomainPrincipal); \
  using nsIPrincipal::GetIsIpAddress; \
  nsresult GetIsIpAddress(bool *aIsIpAddress); \
  using nsIPrincipal::GetIsLocalIpAddress; \
  nsresult GetIsLocalIpAddress(bool *aIsLocalIpAddress); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRINCIPAL(_to) \
  NS_IMETHOD Equals(nsIPrincipal *other, bool *_retval) override { return _to Equals(other, _retval); } \
  NS_IMETHOD EqualsForPermission(nsIPrincipal *other, bool aExactHost, bool *_retval) override { return _to EqualsForPermission(other, aExactHost, _retval); } \
  NS_IMETHOD EqualsConsideringDomain(nsIPrincipal *other, bool *_retval) override { return _to EqualsConsideringDomain(other, _retval); } \
  NS_IMETHOD EqualsURI(nsIURI *aOtherURI, bool *_retval) override { return _to EqualsURI(aOtherURI, _retval); } \
  virtual uint32_t GetHashValue() override { return _to GetHashValue(); } \
  using nsIPrincipal::GetURI; \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return _to GetURI(aURI); } \
  NS_IMETHOD GetDomain(nsIURI **aDomain) override { return _to GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(nsIURI *aDomain) override { return _to SetDomain(aDomain); } \
  NS_IMETHOD Subsumes(nsIPrincipal *other, bool *_retval) override { return _to Subsumes(other, _retval); } \
  NS_IMETHOD SubsumesConsideringDomain(nsIPrincipal *other, bool *_retval) override { return _to SubsumesConsideringDomain(other, _retval); } \
  NS_IMETHOD SubsumesConsideringDomainIgnoringFPD(nsIPrincipal *other, bool *_retval) override { return _to SubsumesConsideringDomainIgnoringFPD(other, _retval); } \
  NS_IMETHOD CheckMayLoad(nsIURI *uri, bool allowIfInheritsPrincipal) override { return _to CheckMayLoad(uri, allowIfInheritsPrincipal); } \
  NS_IMETHOD CheckMayLoadWithReporting(nsIURI *uri, bool allowIfInheritsPrincipal, uint64_t innerWindowID) override { return _to CheckMayLoadWithReporting(uri, allowIfInheritsPrincipal, innerWindowID); } \
  NS_IMETHOD IsThirdPartyURI(nsIURI *uri, bool *_retval) override { return _to IsThirdPartyURI(uri, _retval); } \
  NS_IMETHOD IsThirdPartyPrincipal(nsIPrincipal *principal, bool *_retval) override { return _to IsThirdPartyPrincipal(principal, _retval); } \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *channel, bool *_retval) override { return _to IsThirdPartyChannel(channel, _retval); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes & OriginAttributesRef(void) override { return _to OriginAttributesRef(); } \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override { return _to GetOrigin(aOrigin); } \
  NS_IMETHOD GetAsciiOrigin(nsACString& aAsciiOrigin) override { return _to GetAsciiOrigin(aAsciiOrigin); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return _to GetHostPort(aHostPort); } \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return _to GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return _to GetHost(aHost); } \
  NS_IMETHOD GetPrepath(nsACString& aPrepath) override { return _to GetPrepath(aPrepath); } \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override { return _to GetFilePath(aFilePath); } \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override { return _to GetAsciiSpec(aAsciiSpec); } \
  NS_IMETHOD GetSpec(nsACString& aSpec) override { return _to GetSpec(aSpec); } \
  NS_IMETHOD GetExposablePrePath(nsACString& aExposablePrePath) override { return _to GetExposablePrePath(aExposablePrePath); } \
  NS_IMETHOD GetExposableSpec(nsACString& aExposableSpec) override { return _to GetExposableSpec(aExposableSpec); } \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return _to GetScheme(aScheme); } \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override { return _to SchemeIs(scheme, _retval); } \
  NS_IMETHOD IsURIInPrefList(const char * pref, bool *_retval) override { return _to IsURIInPrefList(pref, _retval); } \
  NS_IMETHOD IsSameOrigin(nsIURI *otherURI, bool aIsPrivateWin, bool *_retval) override { return _to IsSameOrigin(otherURI, aIsPrivateWin, _retval); } \
  NS_IMETHOD AllowsRelaxStrictFileOriginPolicy(nsIURI *aURI, bool *_retval) override { return _to AllowsRelaxStrictFileOriginPolicy(aURI, _retval); } \
  NS_IMETHOD GetPrefLightCacheKey(nsIURI *aURI, bool aWithCredentials, const mozilla::OriginAttributes & aOriginAttributes, nsACString& _retval) override { return _to GetPrefLightCacheKey(aURI, aWithCredentials, aOriginAttributes, _retval); } \
  NS_IMETHOD HasFirstpartyStorageAccess(mozIDOMWindow *aWindow, uint32_t *rejectedReason, bool *_retval) override { return _to HasFirstpartyStorageAccess(aWindow, rejectedReason, _retval); } \
  NS_IMETHOD GetLocalStorageQuotaKey(nsACString& aLocalStorageQuotaKey) override { return _to GetLocalStorageQuotaKey(aLocalStorageQuotaKey); } \
  using nsIPrincipal::GetIsOriginPotentiallyTrustworthy; \
  NS_IMETHOD GetIsOriginPotentiallyTrustworthy(bool *aIsOriginPotentiallyTrustworthy) override { return _to GetIsOriginPotentiallyTrustworthy(aIsOriginPotentiallyTrustworthy); } \
  NS_IMETHOD GetAboutModuleFlags(uint32_t *_retval) override { return _to GetAboutModuleFlags(_retval); } \
  NS_IMETHOD GetStorageOriginKey(nsACString& aStorageOriginKey) override { return _to GetStorageOriginKey(aStorageOriginKey); } \
  NS_IMETHOD CreateReferrerInfo(mozilla::dom::ReferrerPolicy aReferrerPolicy, nsIReferrerInfo **_retval) override { return _to CreateReferrerInfo(aReferrerPolicy, _retval); } \
  NS_IMETHOD GetOriginNoSuffix(nsACString& aOriginNoSuffix) override { return _to GetOriginNoSuffix(aOriginNoSuffix); } \
  NS_IMETHOD GetOriginSuffix(nsACString& aOriginSuffix) override { return _to GetOriginSuffix(aOriginSuffix); } \
  NS_IMETHOD GetSiteOrigin(nsACString& aSiteOrigin) override { return _to GetSiteOrigin(aSiteOrigin); } \
  NS_IMETHOD GetSiteOriginNoSuffix(nsACString& aSiteOriginNoSuffix) override { return _to GetSiteOriginNoSuffix(aSiteOriginNoSuffix); } \
  NS_IMETHOD GetBaseDomain(nsACString& aBaseDomain) override { return _to GetBaseDomain(aBaseDomain); } \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override { return _to GetAddonId(aAddonId); } \
  NS_IMETHOD GetAddonPolicy(nsISupports **aAddonPolicy) override { return _to GetAddonPolicy(aAddonPolicy); } \
  using nsIPrincipal::GetUserContextId; \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override { return _to GetUserContextId(aUserContextId); } \
  using nsIPrincipal::GetPrivateBrowsingId; \
  NS_IMETHOD GetPrivateBrowsingId(uint32_t *aPrivateBrowsingId) override { return _to GetPrivateBrowsingId(aPrivateBrowsingId); } \
  using nsIPrincipal::GetIsInIsolatedMozBrowserElement; \
  NS_IMETHOD GetIsInIsolatedMozBrowserElement(bool *aIsInIsolatedMozBrowserElement) override { return _to GetIsInIsolatedMozBrowserElement(aIsInIsolatedMozBrowserElement); } \
  using nsIPrincipal::GetIsNullPrincipal; \
  NS_IMETHOD GetIsNullPrincipal(bool *aIsNullPrincipal) override { return _to GetIsNullPrincipal(aIsNullPrincipal); } \
  using nsIPrincipal::GetIsContentPrincipal; \
  NS_IMETHOD GetIsContentPrincipal(bool *aIsContentPrincipal) override { return _to GetIsContentPrincipal(aIsContentPrincipal); } \
  using nsIPrincipal::GetIsExpandedPrincipal; \
  NS_IMETHOD GetIsExpandedPrincipal(bool *aIsExpandedPrincipal) override { return _to GetIsExpandedPrincipal(aIsExpandedPrincipal); } \
  NS_IMETHOD GetIsSystemPrincipal(bool *aIsSystemPrincipal) override { return _to GetIsSystemPrincipal(aIsSystemPrincipal); } \
  using nsIPrincipal::GetIsAddonOrExpandedAddonPrincipal; \
  NS_IMETHOD GetIsAddonOrExpandedAddonPrincipal(bool *aIsAddonOrExpandedAddonPrincipal) override { return _to GetIsAddonOrExpandedAddonPrincipal(aIsAddonOrExpandedAddonPrincipal); } \
  using nsIPrincipal::GetIsOnion; \
  NS_IMETHOD GetIsOnion(bool *aIsOnion) override { return _to GetIsOnion(aIsOnion); } \
  NS_IMETHOD GetIsScriptAllowedByPolicy(bool *aIsScriptAllowedByPolicy) override { return _to GetIsScriptAllowedByPolicy(aIsScriptAllowedByPolicy); } \
  NS_IMETHOD IsL10nAllowed(nsIURI *aDocumentURI, bool *_retval) override { return _to IsL10nAllowed(aDocumentURI, _retval); } \
  using nsIPrincipal::GetNextSubDomainPrincipal; \
  NS_IMETHOD GetNextSubDomainPrincipal(nsIPrincipal **aNextSubDomainPrincipal) override { return _to GetNextSubDomainPrincipal(aNextSubDomainPrincipal); } \
  using nsIPrincipal::GetIsIpAddress; \
  NS_IMETHOD GetIsIpAddress(bool *aIsIpAddress) override { return _to GetIsIpAddress(aIsIpAddress); } \
  using nsIPrincipal::GetIsLocalIpAddress; \
  NS_IMETHOD GetIsLocalIpAddress(bool *aIsLocalIpAddress) override { return _to GetIsLocalIpAddress(aIsLocalIpAddress); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRINCIPAL(_to) \
  NS_IMETHOD Equals(nsIPrincipal *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Equals(other, _retval); } \
  NS_IMETHOD EqualsForPermission(nsIPrincipal *other, bool aExactHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EqualsForPermission(other, aExactHost, _retval); } \
  NS_IMETHOD EqualsConsideringDomain(nsIPrincipal *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EqualsConsideringDomain(other, _retval); } \
  NS_IMETHOD EqualsURI(nsIURI *aOtherURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EqualsURI(aOtherURI, _retval); } \
  virtual uint32_t GetHashValue() override; \
  NS_IMETHOD GetURI(nsIURI **aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURI(aURI); } \
  NS_IMETHOD GetDomain(nsIURI **aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(nsIURI *aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDomain(aDomain); } \
  NS_IMETHOD Subsumes(nsIPrincipal *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Subsumes(other, _retval); } \
  NS_IMETHOD SubsumesConsideringDomain(nsIPrincipal *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SubsumesConsideringDomain(other, _retval); } \
  NS_IMETHOD SubsumesConsideringDomainIgnoringFPD(nsIPrincipal *other, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SubsumesConsideringDomainIgnoringFPD(other, _retval); } \
  NS_IMETHOD CheckMayLoad(nsIURI *uri, bool allowIfInheritsPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckMayLoad(uri, allowIfInheritsPrincipal); } \
  NS_IMETHOD CheckMayLoadWithReporting(nsIURI *uri, bool allowIfInheritsPrincipal, uint64_t innerWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckMayLoadWithReporting(uri, allowIfInheritsPrincipal, innerWindowID); } \
  NS_IMETHOD IsThirdPartyURI(nsIURI *uri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyURI(uri, _retval); } \
  NS_IMETHOD IsThirdPartyPrincipal(nsIPrincipal *principal, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyPrincipal(principal, _retval); } \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *channel, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyChannel(channel, _retval); } \
  NS_IMETHOD GetOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(cx, aOriginAttributes); } \
  virtual const mozilla::OriginAttributes & OriginAttributesRef(void) override; \
  NS_IMETHOD GetOrigin(nsACString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOrigin(aOrigin); } \
  NS_IMETHOD GetAsciiOrigin(nsACString& aAsciiOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiOrigin(aAsciiOrigin); } \
  NS_IMETHOD GetHostPort(nsACString& aHostPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHostPort(aHostPort); } \
  NS_IMETHOD GetAsciiHost(nsACString& aAsciiHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiHost(aAsciiHost); } \
  NS_IMETHOD GetHost(nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHost(aHost); } \
  NS_IMETHOD GetPrepath(nsACString& aPrepath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrepath(aPrepath); } \
  NS_IMETHOD GetFilePath(nsACString& aFilePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFilePath(aFilePath); } \
  NS_IMETHOD GetAsciiSpec(nsACString& aAsciiSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAsciiSpec(aAsciiSpec); } \
  NS_IMETHOD GetSpec(nsACString& aSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSpec(aSpec); } \
  NS_IMETHOD GetExposablePrePath(nsACString& aExposablePrePath) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExposablePrePath(aExposablePrePath); } \
  NS_IMETHOD GetExposableSpec(nsACString& aExposableSpec) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExposableSpec(aExposableSpec); } \
  NS_IMETHOD GetScheme(nsACString& aScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScheme(aScheme); } \
  NS_IMETHOD SchemeIs(const char * scheme, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SchemeIs(scheme, _retval); } \
  NS_IMETHOD IsURIInPrefList(const char * pref, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsURIInPrefList(pref, _retval); } \
  NS_IMETHOD IsSameOrigin(nsIURI *otherURI, bool aIsPrivateWin, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSameOrigin(otherURI, aIsPrivateWin, _retval); } \
  NS_IMETHOD AllowsRelaxStrictFileOriginPolicy(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AllowsRelaxStrictFileOriginPolicy(aURI, _retval); } \
  NS_IMETHOD GetPrefLightCacheKey(nsIURI *aURI, bool aWithCredentials, const mozilla::OriginAttributes & aOriginAttributes, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrefLightCacheKey(aURI, aWithCredentials, aOriginAttributes, _retval); } \
  NS_IMETHOD HasFirstpartyStorageAccess(mozIDOMWindow *aWindow, uint32_t *rejectedReason, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasFirstpartyStorageAccess(aWindow, rejectedReason, _retval); } \
  NS_IMETHOD GetLocalStorageQuotaKey(nsACString& aLocalStorageQuotaKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocalStorageQuotaKey(aLocalStorageQuotaKey); } \
  NS_IMETHOD GetIsOriginPotentiallyTrustworthy(bool *aIsOriginPotentiallyTrustworthy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOriginPotentiallyTrustworthy(aIsOriginPotentiallyTrustworthy); } \
  NS_IMETHOD GetAboutModuleFlags(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAboutModuleFlags(_retval); } \
  NS_IMETHOD GetStorageOriginKey(nsACString& aStorageOriginKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStorageOriginKey(aStorageOriginKey); } \
  NS_IMETHOD CreateReferrerInfo(mozilla::dom::ReferrerPolicy aReferrerPolicy, nsIReferrerInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateReferrerInfo(aReferrerPolicy, _retval); } \
  NS_IMETHOD GetOriginNoSuffix(nsACString& aOriginNoSuffix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginNoSuffix(aOriginNoSuffix); } \
  NS_IMETHOD GetOriginSuffix(nsACString& aOriginSuffix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginSuffix(aOriginSuffix); } \
  NS_IMETHOD GetSiteOrigin(nsACString& aSiteOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSiteOrigin(aSiteOrigin); } \
  NS_IMETHOD GetSiteOriginNoSuffix(nsACString& aSiteOriginNoSuffix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSiteOriginNoSuffix(aSiteOriginNoSuffix); } \
  NS_IMETHOD GetBaseDomain(nsACString& aBaseDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseDomain(aBaseDomain); } \
  NS_IMETHOD GetAddonId(nsAString& aAddonId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddonId(aAddonId); } \
  NS_IMETHOD GetAddonPolicy(nsISupports **aAddonPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddonPolicy(aAddonPolicy); } \
  NS_IMETHOD GetUserContextId(uint32_t *aUserContextId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUserContextId(aUserContextId); } \
  NS_IMETHOD GetPrivateBrowsingId(uint32_t *aPrivateBrowsingId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrivateBrowsingId(aPrivateBrowsingId); } \
  NS_IMETHOD GetIsInIsolatedMozBrowserElement(bool *aIsInIsolatedMozBrowserElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInIsolatedMozBrowserElement(aIsInIsolatedMozBrowserElement); } \
  NS_IMETHOD GetIsNullPrincipal(bool *aIsNullPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNullPrincipal(aIsNullPrincipal); } \
  NS_IMETHOD GetIsContentPrincipal(bool *aIsContentPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsContentPrincipal(aIsContentPrincipal); } \
  NS_IMETHOD GetIsExpandedPrincipal(bool *aIsExpandedPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsExpandedPrincipal(aIsExpandedPrincipal); } \
  NS_IMETHOD GetIsSystemPrincipal(bool *aIsSystemPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsSystemPrincipal(aIsSystemPrincipal); } \
  NS_IMETHOD GetIsAddonOrExpandedAddonPrincipal(bool *aIsAddonOrExpandedAddonPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAddonOrExpandedAddonPrincipal(aIsAddonOrExpandedAddonPrincipal); } \
  NS_IMETHOD GetIsOnion(bool *aIsOnion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsOnion(aIsOnion); } \
  NS_IMETHOD GetIsScriptAllowedByPolicy(bool *aIsScriptAllowedByPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsScriptAllowedByPolicy(aIsScriptAllowedByPolicy); } \
  NS_IMETHOD IsL10nAllowed(nsIURI *aDocumentURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsL10nAllowed(aDocumentURI, _retval); } \
  NS_IMETHOD GetNextSubDomainPrincipal(nsIPrincipal **aNextSubDomainPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextSubDomainPrincipal(aNextSubDomainPrincipal); } \
  NS_IMETHOD GetIsIpAddress(bool *aIsIpAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsIpAddress(aIsIpAddress); } \
  NS_IMETHOD GetIsLocalIpAddress(bool *aIsLocalIpAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLocalIpAddress(aIsLocalIpAddress); } 


/* starting interface:    nsIExpandedPrincipal */
#define NS_IEXPANDEDPRINCIPAL_IID_STR "f3e177df-6a5e-489f-80a7-2dd1481471d8"

#define NS_IEXPANDEDPRINCIPAL_IID \
  {0xf3e177df, 0x6a5e, 0x489f, \
    { 0x80, 0xa7, 0x2d, 0xd1, 0x48, 0x14, 0x71, 0xd8 }}

class nsIExpandedPrincipal : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEXPANDEDPRINCIPAL_IID)

  /* [noscript,nostdcall,notxpcom] PrincipalArray AllowList (); */
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AllowList(void) = 0;

  /* readonly attribute nsIContentSecurityPolicy csp; */
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) = 0;

   inline already_AddRefed<nsIContentSecurityPolicy> GetCsp()
  {
    nsCOMPtr<nsIContentSecurityPolicy> result;
    mozilla::DebugOnly<nsresult> rv = GetCsp(getter_AddRefs(result));
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result.forget();
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIExpandedPrincipal, NS_IEXPANDEDPRINCIPAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEXPANDEDPRINCIPAL \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AllowList(void) override; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEXPANDEDPRINCIPAL \
  const nsTArray<nsCOMPtr<nsIPrincipal>> & AllowList(void); \
  nsresult GetCsp(nsIContentSecurityPolicy **aCsp); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEXPANDEDPRINCIPAL(_to) \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AllowList(void) override { return _to AllowList(); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return _to GetCsp(aCsp); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEXPANDEDPRINCIPAL(_to) \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AllowList(void) override; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCsp(aCsp); } \


#endif /* __gen_nsIPrincipal_h__ */
