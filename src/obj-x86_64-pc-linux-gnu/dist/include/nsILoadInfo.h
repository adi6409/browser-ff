/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadInfo.idl
 */

#ifndef __gen_nsILoadInfo_h__
#define __gen_nsILoadInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIContentPolicy_h__
#include "nsIContentPolicy.h"
#endif

#ifndef __gen_nsIScriptSecurityManager_h__
#include "nsIScriptSecurityManager.h"
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

class nsIContentSecurityPolicy; /* forward declaration */

class nsICookieJarSettings; /* forward declaration */

class nsICSPEventListener; /* forward declaration */

class nsINode; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIRedirectHistoryEntry; /* forward declaration */

class nsIURI; /* forward declaration */

namespace mozilla {
namespace dom {
class Document; /* webidl Document */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

#include "nsTArray.h"
#include "mozilla/LoadTainting.h"
#include "mozilla/OriginAttributes.h"
#include "mozilla/UniquePtr.h"
#include "nsStringFwd.h"
namespace mozilla {
namespace dom {
class ClientInfo;
class ClientSource;
class PerformanceStorage;
class ServiceWorkerDescriptor;
} // namespace dom
} // namespace mozilla
typedef uint32_t  nsSecurityFlags;


/* starting interface:    nsILoadInfo */
#define NS_ILOADINFO_IID_STR "ddc65bf9-2f60-41ab-b22a-4f1ae9efcd36"

#define NS_ILOADINFO_IID \
  {0xddc65bf9, 0x2f60, 0x41ab, \
    { 0xb2, 0x2a, 0x4f, 0x1a, 0xe9, 0xef, 0xcd, 0x36 }}

class nsILoadInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadInfo;

  enum {
    SEC_ONLY_FOR_EXPLICIT_CONTENTSEC_CHECK = 0U,
    SEC_REQUIRE_SAME_ORIGIN_INHERITS_SEC_CONTEXT = 1U,
    SEC_REQUIRE_SAME_ORIGIN_DATA_IS_BLOCKED = 2U,
    SEC_ALLOW_CROSS_ORIGIN_INHERITS_SEC_CONTEXT = 4U,
    SEC_ALLOW_CROSS_ORIGIN_SEC_CONTEXT_IS_NULL = 8U,
    SEC_REQUIRE_CORS_INHERITS_SEC_CONTEXT = 16U,
    SEC_COOKIES_DEFAULT = 0U,
    SEC_COOKIES_INCLUDE = 32U,
    SEC_COOKIES_SAME_ORIGIN = 64U,
    SEC_COOKIES_OMIT = 96U,
    SEC_FORCE_INHERIT_PRINCIPAL = 128U,
    SEC_ABOUT_BLANK_INHERITS = 512U,
    SEC_ALLOW_CHROME = 1024U,
    SEC_DISALLOW_SCRIPT = 2048U,
    SEC_DONT_FOLLOW_REDIRECTS = 4096U,
    SEC_LOAD_ERROR_PAGE = 8192U,
    SEC_FORCE_INHERIT_PRINCIPAL_OVERRULE_OWNER = 16384U
  };

  /* readonly attribute nsIPrincipal loadingPrincipal; */
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) = 0;

  /* [noscript,nostdcall,notxpcom] nsIPrincipal virtualGetLoadingPrincipal (); */
  virtual nsIPrincipal * VirtualGetLoadingPrincipal(void) = 0;

   nsIPrincipal* GetLoadingPrincipal() {
    return VirtualGetLoadingPrincipal();
  }
  /* readonly attribute nsIPrincipal triggeringPrincipal; */
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) = 0;

  /* [binaryname(TriggeringPrincipal),noscript,nostdcall,notxpcom] nsIPrincipal binaryTriggeringPrincipal (); */
  virtual nsIPrincipal * TriggeringPrincipal(void) = 0;

  /* attribute nsIPrincipal principalToInherit; */
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) = 0;
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) = 0;

  /* [binaryname(PrincipalToInherit),noscript,nostdcall,notxpcom] nsIPrincipal binaryPrincipalToInherit (); */
  virtual nsIPrincipal * PrincipalToInherit(void) = 0;

  /* [noscript,nostdcall,notxpcom] nsIPrincipal FindPrincipalToInherit (in nsIChannel aChannel); */
  virtual nsIPrincipal * FindPrincipalToInherit(nsIChannel *aChannel) = 0;

  /* readonly attribute Document loadingDocument; */
  NS_IMETHOD GetLoadingDocument(mozilla::dom::Document **aLoadingDocument) = 0;

  /* [binaryname(LoadingNode),noscript,nostdcall,notxpcom] nsINode binaryLoadingNode (); */
  virtual nsINode * LoadingNode(void) = 0;

  /* [binaryname(ContextForTopLevelLoad),noscript,nostdcall,notxpcom] LoadContextRef binaryContextForTopLevelLoad (); */
  virtual already_AddRefed<nsISupports> ContextForTopLevelLoad(void) = 0;

  /* [binaryname(LoadingContextXPCOM)] readonly attribute nsISupports loadingContext; */
  NS_IMETHOD GetLoadingContextXPCOM(nsISupports **aLoadingContext) = 0;

  /* [binaryname(GetLoadingContext),noscript,nostdcall,notxpcom] LoadContextRef binaryGetLoadingContext (); */
  virtual already_AddRefed<nsISupports> GetLoadingContext(void) = 0;

  /* readonly attribute nsSecurityFlags securityFlags; */
  NS_IMETHOD GetSecurityFlags(nsSecurityFlags *aSecurityFlags) = 0;

   inline nsSecurityFlags GetSecurityFlags()
  {
    nsSecurityFlags result;
    mozilla::DebugOnly<nsresult> rv = GetSecurityFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  /* [infallible] readonly attribute unsigned long sandboxFlags; */
  NS_IMETHOD GetSandboxFlags(uint32_t *aSandboxFlags) = 0;
  inline uint32_t  GetSandboxFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetSandboxFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute unsigned long triggeringSandboxFlags; */
  NS_IMETHOD GetTriggeringSandboxFlags(uint32_t *aTriggeringSandboxFlags) = 0;
  inline uint32_t  GetTriggeringSandboxFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetTriggeringSandboxFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetTriggeringSandboxFlags(uint32_t aTriggeringSandboxFlags) = 0;

  /* [infallible] readonly attribute unsigned long securityMode; */
  NS_IMETHOD GetSecurityMode(uint32_t *aSecurityMode) = 0;
  inline uint32_t  GetSecurityMode()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetSecurityMode(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean skipContentSniffing; */
  NS_IMETHOD GetSkipContentSniffing(bool *aSkipContentSniffing) = 0;
  inline bool  GetSkipContentSniffing()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSkipContentSniffing(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetSkipContentSniffing(bool aSkipContentSniffing) = 0;

  enum {
    HTTPS_ONLY_UNINITIALIZED = 1U,
    HTTPS_ONLY_UPGRADED_LISTENER_NOT_REGISTERED = 2U,
    HTTPS_ONLY_UPGRADED_LISTENER_REGISTERED = 4U,
    HTTPS_ONLY_EXEMPT = 8U,
    HTTPS_ONLY_TOP_LEVEL_LOAD_IN_PROGRESS = 16U,
    HTTPS_ONLY_DO_NOT_LOG_TO_CONSOLE = 32U
  };

  /* [infallible] attribute unsigned long httpsOnlyStatus; */
  NS_IMETHOD GetHttpsOnlyStatus(uint32_t *aHttpsOnlyStatus) = 0;
  inline uint32_t  GetHttpsOnlyStatus()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetHttpsOnlyStatus(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetHttpsOnlyStatus(uint32_t aHttpsOnlyStatus) = 0;

  /* [infallible] attribute boolean hasValidUserGestureActivation; */
  NS_IMETHOD GetHasValidUserGestureActivation(bool *aHasValidUserGestureActivation) = 0;
  inline bool  GetHasValidUserGestureActivation()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetHasValidUserGestureActivation(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetHasValidUserGestureActivation(bool aHasValidUserGestureActivation) = 0;

  /* [infallible] attribute boolean allowDeprecatedSystemRequests; */
  NS_IMETHOD GetAllowDeprecatedSystemRequests(bool *aAllowDeprecatedSystemRequests) = 0;
  inline bool  GetAllowDeprecatedSystemRequests()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowDeprecatedSystemRequests(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowDeprecatedSystemRequests(bool aAllowDeprecatedSystemRequests) = 0;

  /* [infallible] attribute boolean parserCreatedScript; */
  NS_IMETHOD GetParserCreatedScript(bool *aParserCreatedScript) = 0;
  inline bool  GetParserCreatedScript()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetParserCreatedScript(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetParserCreatedScript(bool aParserCreatedScript) = 0;

  /* [infallible] attribute boolean isInDevToolsContext; */
  NS_IMETHOD GetIsInDevToolsContext(bool *aIsInDevToolsContext) = 0;
  inline bool  GetIsInDevToolsContext()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsInDevToolsContext(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsInDevToolsContext(bool aIsInDevToolsContext) = 0;

  /* [infallible] attribute boolean isInThirdPartyContext; */
  NS_IMETHOD GetIsInThirdPartyContext(bool *aIsInThirdPartyContext) = 0;
  inline bool  GetIsInThirdPartyContext()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsInThirdPartyContext(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsInThirdPartyContext(bool aIsInThirdPartyContext) = 0;

  /* [infallible] attribute boolean isThirdPartyContextToTopWindow; */
  NS_IMETHOD GetIsThirdPartyContextToTopWindow(bool *aIsThirdPartyContextToTopWindow) = 0;
  inline bool  GetIsThirdPartyContextToTopWindow()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsThirdPartyContextToTopWindow(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsThirdPartyContextToTopWindow(bool aIsThirdPartyContextToTopWindow) = 0;

  /* [infallible] readonly attribute unsigned long cookiePolicy; */
  NS_IMETHOD GetCookiePolicy(uint32_t *aCookiePolicy) = 0;
  inline uint32_t  GetCookiePolicy()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetCookiePolicy(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute nsICookieJarSettings cookieJarSettings; */
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) = 0;
  NS_IMETHOD SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) = 0;

  /* [infallible] attribute boolean hasStoragePermission; */
  NS_IMETHOD GetHasStoragePermission(bool *aHasStoragePermission) = 0;
  inline bool  GetHasStoragePermission()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetHasStoragePermission(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetHasStoragePermission(bool aHasStoragePermission) = 0;

  /* [infallible] readonly attribute boolean forceInheritPrincipal; */
  NS_IMETHOD GetForceInheritPrincipal(bool *aForceInheritPrincipal) = 0;
  inline bool  GetForceInheritPrincipal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForceInheritPrincipal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean forceInheritPrincipalOverruleOwner; */
  NS_IMETHOD GetForceInheritPrincipalOverruleOwner(bool *aForceInheritPrincipalOverruleOwner) = 0;
  inline bool  GetForceInheritPrincipalOverruleOwner()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForceInheritPrincipalOverruleOwner(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean loadingSandboxed; */
  NS_IMETHOD GetLoadingSandboxed(bool *aLoadingSandboxed) = 0;
  inline bool  GetLoadingSandboxed()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoadingSandboxed(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean aboutBlankInherits; */
  NS_IMETHOD GetAboutBlankInherits(bool *aAboutBlankInherits) = 0;
  inline bool  GetAboutBlankInherits()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAboutBlankInherits(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean allowChrome; */
  NS_IMETHOD GetAllowChrome(bool *aAllowChrome) = 0;
  inline bool  GetAllowChrome()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowChrome(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean disallowScript; */
  NS_IMETHOD GetDisallowScript(bool *aDisallowScript) = 0;
  inline bool  GetDisallowScript()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetDisallowScript(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

   uint32_t CheckLoadURIFlags() {
    uint32_t flags = nsIScriptSecurityManager::STANDARD;
    if (GetAllowChrome()) {
      flags |= nsIScriptSecurityManager::ALLOW_CHROME;
    }
    if (GetDisallowScript()) {
      flags |= nsIScriptSecurityManager::DISALLOW_SCRIPT;
    }
    return flags;
  }
  /* [infallible] readonly attribute boolean dontFollowRedirects; */
  NS_IMETHOD GetDontFollowRedirects(bool *aDontFollowRedirects) = 0;
  inline bool  GetDontFollowRedirects()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetDontFollowRedirects(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean loadErrorPage; */
  NS_IMETHOD GetLoadErrorPage(bool *aLoadErrorPage) = 0;
  inline bool  GetLoadErrorPage()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoadErrorPage(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean isFormSubmission; */
  NS_IMETHOD GetIsFormSubmission(bool *aIsFormSubmission) = 0;
  inline bool  GetIsFormSubmission()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsFormSubmission(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetIsFormSubmission(bool aIsFormSubmission) = 0;

  /* readonly attribute nsContentPolicyType externalContentPolicyType; */
  NS_IMETHOD GetExternalContentPolicyType(nsContentPolicyType *aExternalContentPolicyType) = 0;

  /* [infallible] attribute boolean sendCSPViolationEvents; */
  NS_IMETHOD GetSendCSPViolationEvents(bool *aSendCSPViolationEvents) = 0;
  inline bool  GetSendCSPViolationEvents()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSendCSPViolationEvents(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetSendCSPViolationEvents(bool aSendCSPViolationEvents) = 0;

   inline ExtContentPolicyType GetExternalContentPolicyType()
  {
    nsContentPolicyType result;
    mozilla::DebugOnly<nsresult> rv = GetExternalContentPolicyType(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return static_cast<ExtContentPolicyType>(result);
  }
  /* [binaryname(InternalContentPolicyType),noscript,nostdcall,notxpcom] nsContentPolicyType binaryInternalContentPolicyType (); */
  virtual nsContentPolicyType InternalContentPolicyType(void) = 0;

  /* readonly attribute nsContentPolicyType internalContentPolicyType; */
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) = 0;

  /* [infallible] readonly attribute boolean blockAllMixedContent; */
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) = 0;
  inline bool  GetBlockAllMixedContent()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBlockAllMixedContent(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean upgradeInsecureRequests; */
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) = 0;
  inline bool  GetUpgradeInsecureRequests()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetUpgradeInsecureRequests(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean browserUpgradeInsecureRequests; */
  NS_IMETHOD GetBrowserUpgradeInsecureRequests(bool *aBrowserUpgradeInsecureRequests) = 0;
  inline bool  GetBrowserUpgradeInsecureRequests()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBrowserUpgradeInsecureRequests(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean browserDidUpgradeInsecureRequests; */
  NS_IMETHOD GetBrowserDidUpgradeInsecureRequests(bool *aBrowserDidUpgradeInsecureRequests) = 0;
  inline bool  GetBrowserDidUpgradeInsecureRequests()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBrowserDidUpgradeInsecureRequests(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetBrowserDidUpgradeInsecureRequests(bool aBrowserDidUpgradeInsecureRequests) = 0;

  /* [infallible] readonly attribute boolean browserWouldUpgradeInsecureRequests; */
  NS_IMETHOD GetBrowserWouldUpgradeInsecureRequests(bool *aBrowserWouldUpgradeInsecureRequests) = 0;
  inline bool  GetBrowserWouldUpgradeInsecureRequests()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBrowserWouldUpgradeInsecureRequests(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] attribute boolean forceAllowDataURI; */
  NS_IMETHOD GetForceAllowDataURI(bool *aForceAllowDataURI) = 0;
  inline bool  GetForceAllowDataURI()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForceAllowDataURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetForceAllowDataURI(bool aForceAllowDataURI) = 0;

  /* [infallible] attribute boolean allowInsecureRedirectToDataURI; */
  NS_IMETHOD GetAllowInsecureRedirectToDataURI(bool *aAllowInsecureRedirectToDataURI) = 0;
  inline bool  GetAllowInsecureRedirectToDataURI()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowInsecureRedirectToDataURI(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowInsecureRedirectToDataURI(bool aAllowInsecureRedirectToDataURI) = 0;

  /* [infallible] attribute boolean bypassCORSChecks; */
  NS_IMETHOD GetBypassCORSChecks(bool *aBypassCORSChecks) = 0;
  inline bool  GetBypassCORSChecks()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetBypassCORSChecks(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetBypassCORSChecks(bool aBypassCORSChecks) = 0;

  /* [infallible] attribute boolean skipContentPolicyCheckForWebRequest; */
  NS_IMETHOD GetSkipContentPolicyCheckForWebRequest(bool *aSkipContentPolicyCheckForWebRequest) = 0;
  inline bool  GetSkipContentPolicyCheckForWebRequest()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetSkipContentPolicyCheckForWebRequest(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetSkipContentPolicyCheckForWebRequest(bool aSkipContentPolicyCheckForWebRequest) = 0;

  /* [infallible] attribute boolean originalFrameSrcLoad; */
  NS_IMETHOD GetOriginalFrameSrcLoad(bool *aOriginalFrameSrcLoad) = 0;
  inline bool  GetOriginalFrameSrcLoad()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetOriginalFrameSrcLoad(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetOriginalFrameSrcLoad(bool aOriginalFrameSrcLoad) = 0;

  /* [infallible] readonly attribute boolean forceInheritPrincipalDropped; */
  NS_IMETHOD GetForceInheritPrincipalDropped(bool *aForceInheritPrincipalDropped) = 0;
  inline bool  GetForceInheritPrincipalDropped()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForceInheritPrincipalDropped(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long long innerWindowID; */
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) = 0;
  inline uint64_t  GetInnerWindowID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetInnerWindowID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long long browsingContextID; */
  NS_IMETHOD GetBrowsingContextID(uint64_t *aBrowsingContextID) = 0;
  inline uint64_t  GetBrowsingContextID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetBrowsingContextID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute BrowsingContext browsingContext; */
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) = 0;
   inline already_AddRefed<mozilla::dom::BrowsingContext> GetBrowsingContext()
  {
    mozilla::dom::BrowsingContext* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetBrowsingContext(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::BrowsingContext>(result);
  }

  /* [infallible] readonly attribute unsigned long long frameBrowsingContextID; */
  NS_IMETHOD GetFrameBrowsingContextID(uint64_t *aFrameBrowsingContextID) = 0;
  inline uint64_t  GetFrameBrowsingContextID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetFrameBrowsingContextID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute BrowsingContext frameBrowsingContext; */
  NS_IMETHOD GetFrameBrowsingContext(mozilla::dom::BrowsingContext **aFrameBrowsingContext) = 0;
   inline already_AddRefed<mozilla::dom::BrowsingContext> GetFrameBrowsingContext()
  {
    mozilla::dom::BrowsingContext* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetFrameBrowsingContext(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::BrowsingContext>(result);
  }

  /* [infallible] readonly attribute unsigned long long targetBrowsingContextID; */
  NS_IMETHOD GetTargetBrowsingContextID(uint64_t *aTargetBrowsingContextID) = 0;
  inline uint64_t  GetTargetBrowsingContextID()
  {
    uint64_t result;
    mozilla::DebugOnly<nsresult> rv = GetTargetBrowsingContextID(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute BrowsingContext targetBrowsingContext; */
  NS_IMETHOD GetTargetBrowsingContext(mozilla::dom::BrowsingContext **aTargetBrowsingContext) = 0;
   inline already_AddRefed<mozilla::dom::BrowsingContext> GetTargetBrowsingContext()
  {
    mozilla::dom::BrowsingContext* result = nullptr;
    mozilla::DebugOnly<nsresult> rv = GetTargetBrowsingContext(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return already_AddRefed<mozilla::dom::BrowsingContext>(result);
  }

  /* void resetPrincipalToInheritToNullPrincipal (); */
  NS_IMETHOD ResetPrincipalToInheritToNullPrincipal(void) = 0;

  /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] attribute jsval originAttributes; */
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) = 0;
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) = 0;

  /* [binaryname(GetOriginAttributes),noscript,nostdcall] OriginAttributes binaryGetOriginAttributes (); */
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) = 0;

  /* [binaryname(SetOriginAttributes),noscript,nostdcall] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) = 0;

   inline mozilla::OriginAttributes GetOriginAttributes()
  {
    mozilla::OriginAttributes result;
    mozilla::DebugOnly<nsresult> rv = GetOriginAttributes(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  /* [infallible] attribute boolean initialSecurityCheckDone; */
  NS_IMETHOD GetInitialSecurityCheckDone(bool *aInitialSecurityCheckDone) = 0;
  inline bool  GetInitialSecurityCheckDone()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetInitialSecurityCheckDone(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetInitialSecurityCheckDone(bool aInitialSecurityCheckDone) = 0;

  /* [infallible] attribute boolean loadTriggeredFromExternal; */
  NS_IMETHOD GetLoadTriggeredFromExternal(bool *aLoadTriggeredFromExternal) = 0;
  inline bool  GetLoadTriggeredFromExternal()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetLoadTriggeredFromExternal(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLoadTriggeredFromExternal(bool aLoadTriggeredFromExternal) = 0;

  /* [infallible,noscript] readonly attribute boolean serviceWorkerTaintingSynthesized; */
  NS_IMETHOD GetServiceWorkerTaintingSynthesized(bool *aServiceWorkerTaintingSynthesized) = 0;
  inline bool  GetServiceWorkerTaintingSynthesized()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetServiceWorkerTaintingSynthesized(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* void appendRedirectHistoryEntry (in nsIRedirectHistoryEntry entry, in boolean isInternalRedirect); */
  NS_IMETHOD AppendRedirectHistoryEntry(nsIRedirectHistoryEntry *entry, bool isInternalRedirect) = 0;

  /* [implicit_jscontext] readonly attribute jsval redirectChainIncludingInternalRedirects; */
  NS_IMETHOD GetRedirectChainIncludingInternalRedirects(JSContext* cx, JS::MutableHandleValue aRedirectChainIncludingInternalRedirects) = 0;

  /* [binaryname(RedirectChainIncludingInternalRedirects),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChainIncludingInternalRedirects (); */
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChainIncludingInternalRedirects(void) = 0;

  /* [implicit_jscontext] readonly attribute jsval redirectChain; */
  NS_IMETHOD GetRedirectChain(JSContext* cx, JS::MutableHandleValue aRedirectChain) = 0;

  /* [binaryname(RedirectChain),noscript,nostdcall,notxpcom] nsIRedirectHistoryEntryArray binaryRedirectChain (); */
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChain(void) = 0;

  /* [noscript,nostdcall,notxpcom] PrincipalArrayRef AncestorPrincipals (); */
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AncestorPrincipals(void) = 0;

  /* [noscript,nostdcall,notxpcom] Uint64ArrayRef AncestorBrowsingContextIDs (); */
  virtual const nsTArray<uint64_t> & AncestorBrowsingContextIDs(void) = 0;

  /* [noscript,nostdcall,notxpcom] void setCorsPreflightInfo (in CStringArrayRef unsafeHeaders, in boolean forcePreflight); */
  virtual void SetCorsPreflightInfo(const nsTArray<nsCString> & unsafeHeaders, bool forcePreflight) = 0;

  /* [binaryname(CorsUnsafeHeaders),noscript,nostdcall,notxpcom] CStringArrayRef corsUnsafeHeaders (); */
  virtual const nsTArray<nsCString> & CorsUnsafeHeaders(void) = 0;

  /* [infallible] readonly attribute boolean forcePreflight; */
  NS_IMETHOD GetForcePreflight(bool *aForcePreflight) = 0;
  inline bool  GetForcePreflight()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetForcePreflight(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isPreflight; */
  NS_IMETHOD GetIsPreflight(bool *aIsPreflight) = 0;
  inline bool  GetIsPreflight()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsPreflight(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum {
    TAINTING_BASIC = 0U,
    TAINTING_CORS = 1U,
    TAINTING_OPAQUE = 2U
  };

  /* readonly attribute unsigned long tainting; */
  NS_IMETHOD GetTainting(uint32_t *aTainting) = 0;

  /* void maybeIncreaseTainting (in unsigned long aTainting); */
  NS_IMETHOD MaybeIncreaseTainting(uint32_t aTainting) = 0;

   static_assert(TAINTING_BASIC == static_cast<uint32_t>(mozilla::LoadTainting::Basic),
                "basic tainting enums should match");
  static_assert(TAINTING_CORS == static_cast<uint32_t>(mozilla::LoadTainting::CORS),
                "cors tainting enums should match");
  static_assert(TAINTING_OPAQUE == static_cast<uint32_t>(mozilla::LoadTainting::Opaque),
                "opaque tainting enums should match");
  mozilla::LoadTainting GetTainting()
  {
    uint32_t tainting = TAINTING_BASIC;
    MOZ_ALWAYS_SUCCEEDS(GetTainting(&tainting));
    return static_cast<mozilla::LoadTainting>(tainting);
  }
  void MaybeIncreaseTainting(mozilla::LoadTainting aTainting)
  {
    uint32_t tainting = static_cast<uint32_t>(aTainting);
    MOZ_ALWAYS_SUCCEEDS(MaybeIncreaseTainting(tainting));
  }
  /* [infallible] readonly attribute boolean isTopLevelLoad; */
  NS_IMETHOD GetIsTopLevelLoad(bool *aIsTopLevelLoad) = 0;
  inline bool  GetIsTopLevelLoad()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsTopLevelLoad(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* attribute nsIURI resultPrincipalURI; */
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) = 0;
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) = 0;

  /* [nostdcall,notxpcom] readonly attribute nsIPrincipal sandboxedLoadingPrincipal; */
  virtual nsIPrincipal * GetSandboxedLoadingPrincipal() = 0;

  /* [nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelPrincipal; */
  virtual nsIPrincipal * GetTopLevelPrincipal() = 0;

  /* [nostdcall,notxpcom] readonly attribute nsIPrincipal topLevelStorageAreaPrincipal; */
  virtual nsIPrincipal * GetTopLevelStorageAreaPrincipal() = 0;

  /* [noscript,nostdcall,notxpcom] void SetClientInfo (in const_ClientInfoRef aClientInfo); */
  virtual void SetClientInfo(const mozilla::dom::ClientInfo & aClientInfo) = 0;

  /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetClientInfo (); */
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetClientInfo(void) = 0;

  /* [noscript,nostdcall,notxpcom] void GiveReservedClientSource (in UniqueClientSourceMove aClientSource); */
  virtual void GiveReservedClientSource(mozilla::UniquePtr<mozilla::dom::ClientSource>&& aClientSource) = 0;

  /* [noscript,nostdcall,notxpcom] UniqueClientSource TakeReservedClientSource (); */
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeReservedClientSource(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SetReservedClientInfo (in const_ClientInfoRef aClientInfo); */
  virtual void SetReservedClientInfo(const mozilla::dom::ClientInfo & aClientInfo) = 0;

  /* [noscript,nostdcall,notxpcom] void OverrideReservedClientInfoInParent (in const_ClientInfoRef aClientInfo); */
  virtual void OverrideReservedClientInfoInParent(const mozilla::dom::ClientInfo & aClientInfo) = 0;

  /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetReservedClientInfo (); */
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetReservedClientInfo(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SetInitialClientInfo (in const_ClientInfoRef aClientInfo); */
  virtual void SetInitialClientInfo(const mozilla::dom::ClientInfo & aClientInfo) = 0;

  /* [noscript,nostdcall,notxpcom] const_MaybeClientInfoRef GetInitialClientInfo (); */
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetInitialClientInfo(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SetController (in const_ServiceWorkerDescriptorRef aServiceWorker); */
  virtual void SetController(const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) = 0;

  /* [noscript,nostdcall,notxpcom] void ClearController (); */
  virtual void ClearController(void) = 0;

  /* [noscript,nostdcall,notxpcom] const_MaybeServiceWorkerDescriptorRef GetController (); */
  virtual const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & GetController(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SetPerformanceStorage (in PerformanceStoragePtr aPerformanceStorage); */
  virtual void SetPerformanceStorage(mozilla::dom::PerformanceStorage * aPerformanceStorage) = 0;

  /* [noscript,nostdcall,notxpcom] PerformanceStoragePtr GetPerformanceStorage (); */
  virtual mozilla::dom::PerformanceStorage * GetPerformanceStorage(void) = 0;

  /* [nostdcall,notxpcom] CSPRef GetCsp (); */
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCsp(void) = 0;

  /* [nostdcall,notxpcom] CSPRef GetPreloadCsp (); */
  virtual already_AddRefed<nsIContentSecurityPolicy> GetPreloadCsp(void) = 0;

  /* [nostdcall,notxpcom] CSPRef GetCspToInherit (); */
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCspToInherit(void) = 0;

  /* [noscript,nostdcall,notxpcom] void SynthesizeServiceWorkerTainting (in LoadTainting aTainting); */
  virtual void SynthesizeServiceWorkerTainting(mozilla::LoadTainting aTainting) = 0;

  /* [infallible] attribute boolean documentHasUserInteracted; */
  NS_IMETHOD GetDocumentHasUserInteracted(bool *aDocumentHasUserInteracted) = 0;
  inline bool  GetDocumentHasUserInteracted()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetDocumentHasUserInteracted(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetDocumentHasUserInteracted(bool aDocumentHasUserInteracted) = 0;

  /* [infallible] attribute boolean allowListFutureDocumentsCreatedFromThisRedirectChain; */
  NS_IMETHOD GetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool *aAllowListFutureDocumentsCreatedFromThisRedirectChain) = 0;
  inline bool  GetAllowListFutureDocumentsCreatedFromThisRedirectChain()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllowListFutureDocumentsCreatedFromThisRedirectChain(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool aAllowListFutureDocumentsCreatedFromThisRedirectChain) = 0;

  /* attribute AString cspNonce; */
  NS_IMETHOD GetCspNonce(nsAString& aCspNonce) = 0;
  NS_IMETHOD SetCspNonce(const nsAString& aCspNonce) = 0;

  enum {
    BLOCKING_REASON_NONE = 0U,
    BLOCKING_REASON_CORSDISABLED = 1001U,
    BLOCKING_REASON_CORSDIDNOTSUCCEED = 1002U,
    BLOCKING_REASON_CORSREQUESTNOTHTTP = 1003U,
    BLOCKING_REASON_CORSMULTIPLEALLOWORIGINNOTALLOWED = 1004U,
    BLOCKING_REASON_CORSMISSINGALLOWORIGIN = 1005U,
    BLOCKING_REASON_CORSNOTSUPPORTINGCREDENTIALS = 1006U,
    BLOCKING_REASON_CORSALLOWORIGINNOTMATCHINGORIGIN = 1007U,
    BLOCKING_REASON_CORSMISSINGALLOWCREDENTIALS = 1008U,
    BLOCKING_REASON_CORSORIGINHEADERNOTADDED = 1009U,
    BLOCKING_REASON_CORSEXTERNALREDIRECTNOTALLOWED = 1010U,
    BLOCKING_REASON_CORSPREFLIGHTDIDNOTSUCCEED = 1011U,
    BLOCKING_REASON_CORSINVALIDALLOWMETHOD = 1012U,
    BLOCKING_REASON_CORSMETHODNOTFOUND = 1013U,
    BLOCKING_REASON_CORSINVALIDALLOWHEADER = 1014U,
    BLOCKING_REASON_CORSMISSINGALLOWHEADERFROMPREFLIGHT = 1015U,
    BLOCKING_REASON_CLASSIFY_MALWARE_URI = 2001U,
    BLOCKING_REASON_CLASSIFY_PHISHING_URI = 2002U,
    BLOCKING_REASON_CLASSIFY_UNWANTED_URI = 2003U,
    BLOCKING_REASON_CLASSIFY_TRACKING_URI = 2004U,
    BLOCKING_REASON_CLASSIFY_BLOCKED_URI = 2005U,
    BLOCKING_REASON_CLASSIFY_HARMFUL_URI = 2006U,
    BLOCKING_REASON_CLASSIFY_CRYPTOMINING_URI = 2007U,
    BLOCKING_REASON_CLASSIFY_FINGERPRINTING_URI = 2008U,
    BLOCKING_REASON_CLASSIFY_SOCIALTRACKING_URI = 2009U,
    BLOCKING_REASON_MIXED_BLOCKED = 3001U,
    BLOCKING_REASON_CONTENT_POLICY_GENERAL = 4000U,
    BLOCKING_REASON_CONTENT_POLICY_NO_DATA_PROTOCOL = 4001U,
    BLOCKING_REASON_CONTENT_POLICY_WEBEXT = 4002U,
    BLOCKING_REASON_CONTENT_POLICY_CONTENT_BLOCKED = 4003U,
    BLOCKING_REASON_CONTENT_POLICY_DATA_DOCUMENT = 4004U,
    BLOCKING_REASON_CONTENT_POLICY_WEB_BROWSER = 4005U,
    BLOCKING_REASON_CONTENT_POLICY_PRELOAD = 4006U,
    BLOCKING_REASON_NOT_SAME_ORIGIN = 5000U,
    BLOCKING_REASON_EXTENSION_WEBREQUEST = 6000U
  };

  /* [infallible] attribute unsigned long requestBlockingReason; */
  NS_IMETHOD GetRequestBlockingReason(uint32_t *aRequestBlockingReason) = 0;
  inline uint32_t  GetRequestBlockingReason()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetRequestBlockingReason(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetRequestBlockingReason(uint32_t aRequestBlockingReason) = 0;

  /* attribute nsICSPEventListener cspEventListener; */
  NS_IMETHOD GetCspEventListener(nsICSPEventListener **aCspEventListener) = 0;
  NS_IMETHOD SetCspEventListener(nsICSPEventListener *aCspEventListener) = 0;

  /* [infallible] readonly attribute boolean isFromProcessingFrameAttributes; */
  NS_IMETHOD GetIsFromProcessingFrameAttributes(bool *aIsFromProcessingFrameAttributes) = 0;
  inline bool  GetIsFromProcessingFrameAttributes()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsFromProcessingFrameAttributes(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum CrossOriginOpenerPolicy : uint8_t {
    OPENER_POLICY_UNSAFE_NONE = 0,
    OPENER_POLICY_SAME_ORIGIN = 1,
    OPENER_POLICY_SAME_ORIGIN_ALLOW_POPUPS = 2,
    OPENER_POLICY_EMBEDDER_POLICY_REQUIRE_CORP_FLAG = 16,
    OPENER_POLICY_SAME_ORIGIN_EMBEDDER_POLICY_REQUIRE_CORP = 17,
  };

  enum CrossOriginEmbedderPolicy : uint8_t {
    EMBEDDER_POLICY_NULL = 0,
    EMBEDDER_POLICY_REQUIRE_CORP = 1,
  };

  /* [infallible] attribute nsILoadInfo_CrossOriginEmbedderPolicy loadingEmbedderPolicy; */
  NS_IMETHOD GetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *aLoadingEmbedderPolicy) = 0;
  inline nsILoadInfo::CrossOriginEmbedderPolicy  GetLoadingEmbedderPolicy()
  {
    nsILoadInfo::CrossOriginEmbedderPolicy result;
    mozilla::DebugOnly<nsresult> rv = GetLoadingEmbedderPolicy(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }
  NS_IMETHOD SetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy aLoadingEmbedderPolicy) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadInfo, NS_ILOADINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADINFO \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override; \
  virtual nsIPrincipal * VirtualGetLoadingPrincipal(void) override; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override; \
  virtual nsIPrincipal * TriggeringPrincipal(void) override; \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override; \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override; \
  virtual nsIPrincipal * PrincipalToInherit(void) override; \
  virtual nsIPrincipal * FindPrincipalToInherit(nsIChannel *aChannel) override; \
  NS_IMETHOD GetLoadingDocument(mozilla::dom::Document **aLoadingDocument) override; \
  virtual nsINode * LoadingNode(void) override; \
  virtual already_AddRefed<nsISupports> ContextForTopLevelLoad(void) override; \
  NS_IMETHOD GetLoadingContextXPCOM(nsISupports **aLoadingContext) override; \
  virtual already_AddRefed<nsISupports> GetLoadingContext(void) override; \
  NS_IMETHOD GetSecurityFlags(nsSecurityFlags *aSecurityFlags) override; \
  using nsILoadInfo::GetSandboxFlags; \
  NS_IMETHOD GetSandboxFlags(uint32_t *aSandboxFlags) override; \
  using nsILoadInfo::GetTriggeringSandboxFlags; \
  NS_IMETHOD GetTriggeringSandboxFlags(uint32_t *aTriggeringSandboxFlags) override; \
  NS_IMETHOD SetTriggeringSandboxFlags(uint32_t aTriggeringSandboxFlags) override; \
  using nsILoadInfo::GetSecurityMode; \
  NS_IMETHOD GetSecurityMode(uint32_t *aSecurityMode) override; \
  using nsILoadInfo::GetSkipContentSniffing; \
  NS_IMETHOD GetSkipContentSniffing(bool *aSkipContentSniffing) override; \
  NS_IMETHOD SetSkipContentSniffing(bool aSkipContentSniffing) override; \
  using nsILoadInfo::GetHttpsOnlyStatus; \
  NS_IMETHOD GetHttpsOnlyStatus(uint32_t *aHttpsOnlyStatus) override; \
  NS_IMETHOD SetHttpsOnlyStatus(uint32_t aHttpsOnlyStatus) override; \
  using nsILoadInfo::GetHasValidUserGestureActivation; \
  NS_IMETHOD GetHasValidUserGestureActivation(bool *aHasValidUserGestureActivation) override; \
  NS_IMETHOD SetHasValidUserGestureActivation(bool aHasValidUserGestureActivation) override; \
  using nsILoadInfo::GetAllowDeprecatedSystemRequests; \
  NS_IMETHOD GetAllowDeprecatedSystemRequests(bool *aAllowDeprecatedSystemRequests) override; \
  NS_IMETHOD SetAllowDeprecatedSystemRequests(bool aAllowDeprecatedSystemRequests) override; \
  using nsILoadInfo::GetParserCreatedScript; \
  NS_IMETHOD GetParserCreatedScript(bool *aParserCreatedScript) override; \
  NS_IMETHOD SetParserCreatedScript(bool aParserCreatedScript) override; \
  using nsILoadInfo::GetIsInDevToolsContext; \
  NS_IMETHOD GetIsInDevToolsContext(bool *aIsInDevToolsContext) override; \
  NS_IMETHOD SetIsInDevToolsContext(bool aIsInDevToolsContext) override; \
  using nsILoadInfo::GetIsInThirdPartyContext; \
  NS_IMETHOD GetIsInThirdPartyContext(bool *aIsInThirdPartyContext) override; \
  NS_IMETHOD SetIsInThirdPartyContext(bool aIsInThirdPartyContext) override; \
  using nsILoadInfo::GetIsThirdPartyContextToTopWindow; \
  NS_IMETHOD GetIsThirdPartyContextToTopWindow(bool *aIsThirdPartyContextToTopWindow) override; \
  NS_IMETHOD SetIsThirdPartyContextToTopWindow(bool aIsThirdPartyContextToTopWindow) override; \
  using nsILoadInfo::GetCookiePolicy; \
  NS_IMETHOD GetCookiePolicy(uint32_t *aCookiePolicy) override; \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override; \
  NS_IMETHOD SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override; \
  using nsILoadInfo::GetHasStoragePermission; \
  NS_IMETHOD GetHasStoragePermission(bool *aHasStoragePermission) override; \
  NS_IMETHOD SetHasStoragePermission(bool aHasStoragePermission) override; \
  using nsILoadInfo::GetForceInheritPrincipal; \
  NS_IMETHOD GetForceInheritPrincipal(bool *aForceInheritPrincipal) override; \
  using nsILoadInfo::GetForceInheritPrincipalOverruleOwner; \
  NS_IMETHOD GetForceInheritPrincipalOverruleOwner(bool *aForceInheritPrincipalOverruleOwner) override; \
  using nsILoadInfo::GetLoadingSandboxed; \
  NS_IMETHOD GetLoadingSandboxed(bool *aLoadingSandboxed) override; \
  using nsILoadInfo::GetAboutBlankInherits; \
  NS_IMETHOD GetAboutBlankInherits(bool *aAboutBlankInherits) override; \
  using nsILoadInfo::GetAllowChrome; \
  NS_IMETHOD GetAllowChrome(bool *aAllowChrome) override; \
  using nsILoadInfo::GetDisallowScript; \
  NS_IMETHOD GetDisallowScript(bool *aDisallowScript) override; \
  using nsILoadInfo::GetDontFollowRedirects; \
  NS_IMETHOD GetDontFollowRedirects(bool *aDontFollowRedirects) override; \
  using nsILoadInfo::GetLoadErrorPage; \
  NS_IMETHOD GetLoadErrorPage(bool *aLoadErrorPage) override; \
  using nsILoadInfo::GetIsFormSubmission; \
  NS_IMETHOD GetIsFormSubmission(bool *aIsFormSubmission) override; \
  NS_IMETHOD SetIsFormSubmission(bool aIsFormSubmission) override; \
  NS_IMETHOD GetExternalContentPolicyType(nsContentPolicyType *aExternalContentPolicyType) override; \
  using nsILoadInfo::GetSendCSPViolationEvents; \
  NS_IMETHOD GetSendCSPViolationEvents(bool *aSendCSPViolationEvents) override; \
  NS_IMETHOD SetSendCSPViolationEvents(bool aSendCSPViolationEvents) override; \
  virtual nsContentPolicyType InternalContentPolicyType(void) override; \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override; \
  using nsILoadInfo::GetBlockAllMixedContent; \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override; \
  using nsILoadInfo::GetUpgradeInsecureRequests; \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override; \
  using nsILoadInfo::GetBrowserUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserUpgradeInsecureRequests(bool *aBrowserUpgradeInsecureRequests) override; \
  using nsILoadInfo::GetBrowserDidUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserDidUpgradeInsecureRequests(bool *aBrowserDidUpgradeInsecureRequests) override; \
  NS_IMETHOD SetBrowserDidUpgradeInsecureRequests(bool aBrowserDidUpgradeInsecureRequests) override; \
  using nsILoadInfo::GetBrowserWouldUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserWouldUpgradeInsecureRequests(bool *aBrowserWouldUpgradeInsecureRequests) override; \
  using nsILoadInfo::GetForceAllowDataURI; \
  NS_IMETHOD GetForceAllowDataURI(bool *aForceAllowDataURI) override; \
  NS_IMETHOD SetForceAllowDataURI(bool aForceAllowDataURI) override; \
  using nsILoadInfo::GetAllowInsecureRedirectToDataURI; \
  NS_IMETHOD GetAllowInsecureRedirectToDataURI(bool *aAllowInsecureRedirectToDataURI) override; \
  NS_IMETHOD SetAllowInsecureRedirectToDataURI(bool aAllowInsecureRedirectToDataURI) override; \
  using nsILoadInfo::GetBypassCORSChecks; \
  NS_IMETHOD GetBypassCORSChecks(bool *aBypassCORSChecks) override; \
  NS_IMETHOD SetBypassCORSChecks(bool aBypassCORSChecks) override; \
  using nsILoadInfo::GetSkipContentPolicyCheckForWebRequest; \
  NS_IMETHOD GetSkipContentPolicyCheckForWebRequest(bool *aSkipContentPolicyCheckForWebRequest) override; \
  NS_IMETHOD SetSkipContentPolicyCheckForWebRequest(bool aSkipContentPolicyCheckForWebRequest) override; \
  using nsILoadInfo::GetOriginalFrameSrcLoad; \
  NS_IMETHOD GetOriginalFrameSrcLoad(bool *aOriginalFrameSrcLoad) override; \
  NS_IMETHOD SetOriginalFrameSrcLoad(bool aOriginalFrameSrcLoad) override; \
  using nsILoadInfo::GetForceInheritPrincipalDropped; \
  NS_IMETHOD GetForceInheritPrincipalDropped(bool *aForceInheritPrincipalDropped) override; \
  using nsILoadInfo::GetInnerWindowID; \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override; \
  using nsILoadInfo::GetBrowsingContextID; \
  NS_IMETHOD GetBrowsingContextID(uint64_t *aBrowsingContextID) override; \
  using nsILoadInfo::GetBrowsingContext; \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override; \
  using nsILoadInfo::GetFrameBrowsingContextID; \
  NS_IMETHOD GetFrameBrowsingContextID(uint64_t *aFrameBrowsingContextID) override; \
  using nsILoadInfo::GetFrameBrowsingContext; \
  NS_IMETHOD GetFrameBrowsingContext(mozilla::dom::BrowsingContext **aFrameBrowsingContext) override; \
  using nsILoadInfo::GetTargetBrowsingContextID; \
  NS_IMETHOD GetTargetBrowsingContextID(uint64_t *aTargetBrowsingContextID) override; \
  using nsILoadInfo::GetTargetBrowsingContext; \
  NS_IMETHOD GetTargetBrowsingContext(mozilla::dom::BrowsingContext **aTargetBrowsingContext) override; \
  NS_IMETHOD ResetPrincipalToInheritToNullPrincipal(void) override; \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override; \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override; \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override; \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override; \
  using nsILoadInfo::GetInitialSecurityCheckDone; \
  NS_IMETHOD GetInitialSecurityCheckDone(bool *aInitialSecurityCheckDone) override; \
  NS_IMETHOD SetInitialSecurityCheckDone(bool aInitialSecurityCheckDone) override; \
  using nsILoadInfo::GetLoadTriggeredFromExternal; \
  NS_IMETHOD GetLoadTriggeredFromExternal(bool *aLoadTriggeredFromExternal) override; \
  NS_IMETHOD SetLoadTriggeredFromExternal(bool aLoadTriggeredFromExternal) override; \
  using nsILoadInfo::GetServiceWorkerTaintingSynthesized; \
  NS_IMETHOD GetServiceWorkerTaintingSynthesized(bool *aServiceWorkerTaintingSynthesized) override; \
  NS_IMETHOD AppendRedirectHistoryEntry(nsIRedirectHistoryEntry *entry, bool isInternalRedirect) override; \
  NS_IMETHOD GetRedirectChainIncludingInternalRedirects(JSContext* cx, JS::MutableHandleValue aRedirectChainIncludingInternalRedirects) override; \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChainIncludingInternalRedirects(void) override; \
  NS_IMETHOD GetRedirectChain(JSContext* cx, JS::MutableHandleValue aRedirectChain) override; \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChain(void) override; \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AncestorPrincipals(void) override; \
  virtual const nsTArray<uint64_t> & AncestorBrowsingContextIDs(void) override; \
  virtual void SetCorsPreflightInfo(const nsTArray<nsCString> & unsafeHeaders, bool forcePreflight) override; \
  virtual const nsTArray<nsCString> & CorsUnsafeHeaders(void) override; \
  using nsILoadInfo::GetForcePreflight; \
  NS_IMETHOD GetForcePreflight(bool *aForcePreflight) override; \
  using nsILoadInfo::GetIsPreflight; \
  NS_IMETHOD GetIsPreflight(bool *aIsPreflight) override; \
  NS_IMETHOD GetTainting(uint32_t *aTainting) override; \
  NS_IMETHOD MaybeIncreaseTainting(uint32_t aTainting) override; \
  using nsILoadInfo::GetIsTopLevelLoad; \
  NS_IMETHOD GetIsTopLevelLoad(bool *aIsTopLevelLoad) override; \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override; \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override; \
  virtual nsIPrincipal * GetSandboxedLoadingPrincipal() override; \
  virtual nsIPrincipal * GetTopLevelPrincipal() override; \
  virtual nsIPrincipal * GetTopLevelStorageAreaPrincipal() override; \
  virtual void SetClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetClientInfo(void) override; \
  virtual void GiveReservedClientSource(mozilla::UniquePtr<mozilla::dom::ClientSource>&& aClientSource) override; \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeReservedClientSource(void) override; \
  virtual void SetReservedClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual void OverrideReservedClientInfoInParent(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetReservedClientInfo(void) override; \
  virtual void SetInitialClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetInitialClientInfo(void) override; \
  virtual void SetController(const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override; \
  virtual void ClearController(void) override; \
  virtual const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & GetController(void) override; \
  virtual void SetPerformanceStorage(mozilla::dom::PerformanceStorage * aPerformanceStorage) override; \
  virtual mozilla::dom::PerformanceStorage * GetPerformanceStorage(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCsp(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetPreloadCsp(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCspToInherit(void) override; \
  virtual void SynthesizeServiceWorkerTainting(mozilla::LoadTainting aTainting) override; \
  using nsILoadInfo::GetDocumentHasUserInteracted; \
  NS_IMETHOD GetDocumentHasUserInteracted(bool *aDocumentHasUserInteracted) override; \
  NS_IMETHOD SetDocumentHasUserInteracted(bool aDocumentHasUserInteracted) override; \
  using nsILoadInfo::GetAllowListFutureDocumentsCreatedFromThisRedirectChain; \
  NS_IMETHOD GetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool *aAllowListFutureDocumentsCreatedFromThisRedirectChain) override; \
  NS_IMETHOD SetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool aAllowListFutureDocumentsCreatedFromThisRedirectChain) override; \
  NS_IMETHOD GetCspNonce(nsAString& aCspNonce) override; \
  NS_IMETHOD SetCspNonce(const nsAString& aCspNonce) override; \
  using nsILoadInfo::GetRequestBlockingReason; \
  NS_IMETHOD GetRequestBlockingReason(uint32_t *aRequestBlockingReason) override; \
  NS_IMETHOD SetRequestBlockingReason(uint32_t aRequestBlockingReason) override; \
  NS_IMETHOD GetCspEventListener(nsICSPEventListener **aCspEventListener) override; \
  NS_IMETHOD SetCspEventListener(nsICSPEventListener *aCspEventListener) override; \
  using nsILoadInfo::GetIsFromProcessingFrameAttributes; \
  NS_IMETHOD GetIsFromProcessingFrameAttributes(bool *aIsFromProcessingFrameAttributes) override; \
  using nsILoadInfo::GetLoadingEmbedderPolicy; \
  NS_IMETHOD GetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *aLoadingEmbedderPolicy) override; \
  NS_IMETHOD SetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy aLoadingEmbedderPolicy) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADINFO \
  nsresult GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal); \
  nsIPrincipal * VirtualGetLoadingPrincipal(void); \
  nsresult GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal); \
  nsIPrincipal * TriggeringPrincipal(void); \
  nsresult GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit); \
  nsresult SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit); \
  nsIPrincipal * PrincipalToInherit(void); \
  nsIPrincipal * FindPrincipalToInherit(nsIChannel *aChannel); \
  nsresult GetLoadingDocument(mozilla::dom::Document **aLoadingDocument); \
  nsINode * LoadingNode(void); \
  already_AddRefed<nsISupports> ContextForTopLevelLoad(void); \
  nsresult GetLoadingContextXPCOM(nsISupports **aLoadingContext); \
  already_AddRefed<nsISupports> GetLoadingContext(void); \
  nsresult GetSecurityFlags(nsSecurityFlags *aSecurityFlags); \
  using nsILoadInfo::GetSandboxFlags; \
  nsresult GetSandboxFlags(uint32_t *aSandboxFlags); \
  using nsILoadInfo::GetTriggeringSandboxFlags; \
  nsresult GetTriggeringSandboxFlags(uint32_t *aTriggeringSandboxFlags); \
  nsresult SetTriggeringSandboxFlags(uint32_t aTriggeringSandboxFlags); \
  using nsILoadInfo::GetSecurityMode; \
  nsresult GetSecurityMode(uint32_t *aSecurityMode); \
  using nsILoadInfo::GetSkipContentSniffing; \
  nsresult GetSkipContentSniffing(bool *aSkipContentSniffing); \
  nsresult SetSkipContentSniffing(bool aSkipContentSniffing); \
  using nsILoadInfo::GetHttpsOnlyStatus; \
  nsresult GetHttpsOnlyStatus(uint32_t *aHttpsOnlyStatus); \
  nsresult SetHttpsOnlyStatus(uint32_t aHttpsOnlyStatus); \
  using nsILoadInfo::GetHasValidUserGestureActivation; \
  nsresult GetHasValidUserGestureActivation(bool *aHasValidUserGestureActivation); \
  nsresult SetHasValidUserGestureActivation(bool aHasValidUserGestureActivation); \
  using nsILoadInfo::GetAllowDeprecatedSystemRequests; \
  nsresult GetAllowDeprecatedSystemRequests(bool *aAllowDeprecatedSystemRequests); \
  nsresult SetAllowDeprecatedSystemRequests(bool aAllowDeprecatedSystemRequests); \
  using nsILoadInfo::GetParserCreatedScript; \
  nsresult GetParserCreatedScript(bool *aParserCreatedScript); \
  nsresult SetParserCreatedScript(bool aParserCreatedScript); \
  using nsILoadInfo::GetIsInDevToolsContext; \
  nsresult GetIsInDevToolsContext(bool *aIsInDevToolsContext); \
  nsresult SetIsInDevToolsContext(bool aIsInDevToolsContext); \
  using nsILoadInfo::GetIsInThirdPartyContext; \
  nsresult GetIsInThirdPartyContext(bool *aIsInThirdPartyContext); \
  nsresult SetIsInThirdPartyContext(bool aIsInThirdPartyContext); \
  using nsILoadInfo::GetIsThirdPartyContextToTopWindow; \
  nsresult GetIsThirdPartyContextToTopWindow(bool *aIsThirdPartyContextToTopWindow); \
  nsresult SetIsThirdPartyContextToTopWindow(bool aIsThirdPartyContextToTopWindow); \
  using nsILoadInfo::GetCookiePolicy; \
  nsresult GetCookiePolicy(uint32_t *aCookiePolicy); \
  nsresult GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings); \
  nsresult SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings); \
  using nsILoadInfo::GetHasStoragePermission; \
  nsresult GetHasStoragePermission(bool *aHasStoragePermission); \
  nsresult SetHasStoragePermission(bool aHasStoragePermission); \
  using nsILoadInfo::GetForceInheritPrincipal; \
  nsresult GetForceInheritPrincipal(bool *aForceInheritPrincipal); \
  using nsILoadInfo::GetForceInheritPrincipalOverruleOwner; \
  nsresult GetForceInheritPrincipalOverruleOwner(bool *aForceInheritPrincipalOverruleOwner); \
  using nsILoadInfo::GetLoadingSandboxed; \
  nsresult GetLoadingSandboxed(bool *aLoadingSandboxed); \
  using nsILoadInfo::GetAboutBlankInherits; \
  nsresult GetAboutBlankInherits(bool *aAboutBlankInherits); \
  using nsILoadInfo::GetAllowChrome; \
  nsresult GetAllowChrome(bool *aAllowChrome); \
  using nsILoadInfo::GetDisallowScript; \
  nsresult GetDisallowScript(bool *aDisallowScript); \
  using nsILoadInfo::GetDontFollowRedirects; \
  nsresult GetDontFollowRedirects(bool *aDontFollowRedirects); \
  using nsILoadInfo::GetLoadErrorPage; \
  nsresult GetLoadErrorPage(bool *aLoadErrorPage); \
  using nsILoadInfo::GetIsFormSubmission; \
  nsresult GetIsFormSubmission(bool *aIsFormSubmission); \
  nsresult SetIsFormSubmission(bool aIsFormSubmission); \
  nsresult GetExternalContentPolicyType(nsContentPolicyType *aExternalContentPolicyType); \
  using nsILoadInfo::GetSendCSPViolationEvents; \
  nsresult GetSendCSPViolationEvents(bool *aSendCSPViolationEvents); \
  nsresult SetSendCSPViolationEvents(bool aSendCSPViolationEvents); \
  nsContentPolicyType InternalContentPolicyType(void); \
  nsresult GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType); \
  using nsILoadInfo::GetBlockAllMixedContent; \
  nsresult GetBlockAllMixedContent(bool *aBlockAllMixedContent); \
  using nsILoadInfo::GetUpgradeInsecureRequests; \
  nsresult GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests); \
  using nsILoadInfo::GetBrowserUpgradeInsecureRequests; \
  nsresult GetBrowserUpgradeInsecureRequests(bool *aBrowserUpgradeInsecureRequests); \
  using nsILoadInfo::GetBrowserDidUpgradeInsecureRequests; \
  nsresult GetBrowserDidUpgradeInsecureRequests(bool *aBrowserDidUpgradeInsecureRequests); \
  nsresult SetBrowserDidUpgradeInsecureRequests(bool aBrowserDidUpgradeInsecureRequests); \
  using nsILoadInfo::GetBrowserWouldUpgradeInsecureRequests; \
  nsresult GetBrowserWouldUpgradeInsecureRequests(bool *aBrowserWouldUpgradeInsecureRequests); \
  using nsILoadInfo::GetForceAllowDataURI; \
  nsresult GetForceAllowDataURI(bool *aForceAllowDataURI); \
  nsresult SetForceAllowDataURI(bool aForceAllowDataURI); \
  using nsILoadInfo::GetAllowInsecureRedirectToDataURI; \
  nsresult GetAllowInsecureRedirectToDataURI(bool *aAllowInsecureRedirectToDataURI); \
  nsresult SetAllowInsecureRedirectToDataURI(bool aAllowInsecureRedirectToDataURI); \
  using nsILoadInfo::GetBypassCORSChecks; \
  nsresult GetBypassCORSChecks(bool *aBypassCORSChecks); \
  nsresult SetBypassCORSChecks(bool aBypassCORSChecks); \
  using nsILoadInfo::GetSkipContentPolicyCheckForWebRequest; \
  nsresult GetSkipContentPolicyCheckForWebRequest(bool *aSkipContentPolicyCheckForWebRequest); \
  nsresult SetSkipContentPolicyCheckForWebRequest(bool aSkipContentPolicyCheckForWebRequest); \
  using nsILoadInfo::GetOriginalFrameSrcLoad; \
  nsresult GetOriginalFrameSrcLoad(bool *aOriginalFrameSrcLoad); \
  nsresult SetOriginalFrameSrcLoad(bool aOriginalFrameSrcLoad); \
  using nsILoadInfo::GetForceInheritPrincipalDropped; \
  nsresult GetForceInheritPrincipalDropped(bool *aForceInheritPrincipalDropped); \
  using nsILoadInfo::GetInnerWindowID; \
  nsresult GetInnerWindowID(uint64_t *aInnerWindowID); \
  using nsILoadInfo::GetBrowsingContextID; \
  nsresult GetBrowsingContextID(uint64_t *aBrowsingContextID); \
  using nsILoadInfo::GetBrowsingContext; \
  nsresult GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext); \
  using nsILoadInfo::GetFrameBrowsingContextID; \
  nsresult GetFrameBrowsingContextID(uint64_t *aFrameBrowsingContextID); \
  using nsILoadInfo::GetFrameBrowsingContext; \
  nsresult GetFrameBrowsingContext(mozilla::dom::BrowsingContext **aFrameBrowsingContext); \
  using nsILoadInfo::GetTargetBrowsingContextID; \
  nsresult GetTargetBrowsingContextID(uint64_t *aTargetBrowsingContextID); \
  using nsILoadInfo::GetTargetBrowsingContext; \
  nsresult GetTargetBrowsingContext(mozilla::dom::BrowsingContext **aTargetBrowsingContext); \
  nsresult ResetPrincipalToInheritToNullPrincipal(void); \
  nsresult GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes); \
  nsresult SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes); \
  nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval); \
  nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs); \
  using nsILoadInfo::GetInitialSecurityCheckDone; \
  nsresult GetInitialSecurityCheckDone(bool *aInitialSecurityCheckDone); \
  nsresult SetInitialSecurityCheckDone(bool aInitialSecurityCheckDone); \
  using nsILoadInfo::GetLoadTriggeredFromExternal; \
  nsresult GetLoadTriggeredFromExternal(bool *aLoadTriggeredFromExternal); \
  nsresult SetLoadTriggeredFromExternal(bool aLoadTriggeredFromExternal); \
  using nsILoadInfo::GetServiceWorkerTaintingSynthesized; \
  nsresult GetServiceWorkerTaintingSynthesized(bool *aServiceWorkerTaintingSynthesized); \
  nsresult AppendRedirectHistoryEntry(nsIRedirectHistoryEntry *entry, bool isInternalRedirect); \
  nsresult GetRedirectChainIncludingInternalRedirects(JSContext* cx, JS::MutableHandleValue aRedirectChainIncludingInternalRedirects); \
  const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChainIncludingInternalRedirects(void); \
  nsresult GetRedirectChain(JSContext* cx, JS::MutableHandleValue aRedirectChain); \
  const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChain(void); \
  const nsTArray<nsCOMPtr<nsIPrincipal>> & AncestorPrincipals(void); \
  const nsTArray<uint64_t> & AncestorBrowsingContextIDs(void); \
  void SetCorsPreflightInfo(const nsTArray<nsCString> & unsafeHeaders, bool forcePreflight); \
  const nsTArray<nsCString> & CorsUnsafeHeaders(void); \
  using nsILoadInfo::GetForcePreflight; \
  nsresult GetForcePreflight(bool *aForcePreflight); \
  using nsILoadInfo::GetIsPreflight; \
  nsresult GetIsPreflight(bool *aIsPreflight); \
  nsresult GetTainting(uint32_t *aTainting); \
  nsresult MaybeIncreaseTainting(uint32_t aTainting); \
  using nsILoadInfo::GetIsTopLevelLoad; \
  nsresult GetIsTopLevelLoad(bool *aIsTopLevelLoad); \
  nsresult GetResultPrincipalURI(nsIURI **aResultPrincipalURI); \
  nsresult SetResultPrincipalURI(nsIURI *aResultPrincipalURI); \
  nsIPrincipal * GetSandboxedLoadingPrincipal(); \
  nsIPrincipal * GetTopLevelPrincipal(); \
  nsIPrincipal * GetTopLevelStorageAreaPrincipal(); \
  void SetClientInfo(const mozilla::dom::ClientInfo & aClientInfo); \
  const mozilla::Maybe<mozilla::dom::ClientInfo> & GetClientInfo(void); \
  void GiveReservedClientSource(mozilla::UniquePtr<mozilla::dom::ClientSource>&& aClientSource); \
  mozilla::UniquePtr<mozilla::dom::ClientSource> TakeReservedClientSource(void); \
  void SetReservedClientInfo(const mozilla::dom::ClientInfo & aClientInfo); \
  void OverrideReservedClientInfoInParent(const mozilla::dom::ClientInfo & aClientInfo); \
  const mozilla::Maybe<mozilla::dom::ClientInfo> & GetReservedClientInfo(void); \
  void SetInitialClientInfo(const mozilla::dom::ClientInfo & aClientInfo); \
  const mozilla::Maybe<mozilla::dom::ClientInfo> & GetInitialClientInfo(void); \
  void SetController(const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker); \
  void ClearController(void); \
  const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & GetController(void); \
  void SetPerformanceStorage(mozilla::dom::PerformanceStorage * aPerformanceStorage); \
  mozilla::dom::PerformanceStorage * GetPerformanceStorage(void); \
  already_AddRefed<nsIContentSecurityPolicy> GetCsp(void); \
  already_AddRefed<nsIContentSecurityPolicy> GetPreloadCsp(void); \
  already_AddRefed<nsIContentSecurityPolicy> GetCspToInherit(void); \
  void SynthesizeServiceWorkerTainting(mozilla::LoadTainting aTainting); \
  using nsILoadInfo::GetDocumentHasUserInteracted; \
  nsresult GetDocumentHasUserInteracted(bool *aDocumentHasUserInteracted); \
  nsresult SetDocumentHasUserInteracted(bool aDocumentHasUserInteracted); \
  using nsILoadInfo::GetAllowListFutureDocumentsCreatedFromThisRedirectChain; \
  nsresult GetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool *aAllowListFutureDocumentsCreatedFromThisRedirectChain); \
  nsresult SetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool aAllowListFutureDocumentsCreatedFromThisRedirectChain); \
  nsresult GetCspNonce(nsAString& aCspNonce); \
  nsresult SetCspNonce(const nsAString& aCspNonce); \
  using nsILoadInfo::GetRequestBlockingReason; \
  nsresult GetRequestBlockingReason(uint32_t *aRequestBlockingReason); \
  nsresult SetRequestBlockingReason(uint32_t aRequestBlockingReason); \
  nsresult GetCspEventListener(nsICSPEventListener **aCspEventListener); \
  nsresult SetCspEventListener(nsICSPEventListener *aCspEventListener); \
  using nsILoadInfo::GetIsFromProcessingFrameAttributes; \
  nsresult GetIsFromProcessingFrameAttributes(bool *aIsFromProcessingFrameAttributes); \
  using nsILoadInfo::GetLoadingEmbedderPolicy; \
  nsresult GetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *aLoadingEmbedderPolicy); \
  nsresult SetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy aLoadingEmbedderPolicy); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADINFO(_to) \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override { return _to GetLoadingPrincipal(aLoadingPrincipal); } \
  virtual nsIPrincipal * VirtualGetLoadingPrincipal(void) override { return _to VirtualGetLoadingPrincipal(); } \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return _to GetTriggeringPrincipal(aTriggeringPrincipal); } \
  virtual nsIPrincipal * TriggeringPrincipal(void) override { return _to TriggeringPrincipal(); } \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override { return _to GetPrincipalToInherit(aPrincipalToInherit); } \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override { return _to SetPrincipalToInherit(aPrincipalToInherit); } \
  virtual nsIPrincipal * PrincipalToInherit(void) override { return _to PrincipalToInherit(); } \
  virtual nsIPrincipal * FindPrincipalToInherit(nsIChannel *aChannel) override { return _to FindPrincipalToInherit(aChannel); } \
  NS_IMETHOD GetLoadingDocument(mozilla::dom::Document **aLoadingDocument) override { return _to GetLoadingDocument(aLoadingDocument); } \
  virtual nsINode * LoadingNode(void) override { return _to LoadingNode(); } \
  virtual already_AddRefed<nsISupports> ContextForTopLevelLoad(void) override { return _to ContextForTopLevelLoad(); } \
  NS_IMETHOD GetLoadingContextXPCOM(nsISupports **aLoadingContext) override { return _to GetLoadingContextXPCOM(aLoadingContext); } \
  virtual already_AddRefed<nsISupports> GetLoadingContext(void) override { return _to GetLoadingContext(); } \
  NS_IMETHOD GetSecurityFlags(nsSecurityFlags *aSecurityFlags) override { return _to GetSecurityFlags(aSecurityFlags); } \
  using nsILoadInfo::GetSandboxFlags; \
  NS_IMETHOD GetSandboxFlags(uint32_t *aSandboxFlags) override { return _to GetSandboxFlags(aSandboxFlags); } \
  using nsILoadInfo::GetTriggeringSandboxFlags; \
  NS_IMETHOD GetTriggeringSandboxFlags(uint32_t *aTriggeringSandboxFlags) override { return _to GetTriggeringSandboxFlags(aTriggeringSandboxFlags); } \
  NS_IMETHOD SetTriggeringSandboxFlags(uint32_t aTriggeringSandboxFlags) override { return _to SetTriggeringSandboxFlags(aTriggeringSandboxFlags); } \
  using nsILoadInfo::GetSecurityMode; \
  NS_IMETHOD GetSecurityMode(uint32_t *aSecurityMode) override { return _to GetSecurityMode(aSecurityMode); } \
  using nsILoadInfo::GetSkipContentSniffing; \
  NS_IMETHOD GetSkipContentSniffing(bool *aSkipContentSniffing) override { return _to GetSkipContentSniffing(aSkipContentSniffing); } \
  NS_IMETHOD SetSkipContentSniffing(bool aSkipContentSniffing) override { return _to SetSkipContentSniffing(aSkipContentSniffing); } \
  using nsILoadInfo::GetHttpsOnlyStatus; \
  NS_IMETHOD GetHttpsOnlyStatus(uint32_t *aHttpsOnlyStatus) override { return _to GetHttpsOnlyStatus(aHttpsOnlyStatus); } \
  NS_IMETHOD SetHttpsOnlyStatus(uint32_t aHttpsOnlyStatus) override { return _to SetHttpsOnlyStatus(aHttpsOnlyStatus); } \
  using nsILoadInfo::GetHasValidUserGestureActivation; \
  NS_IMETHOD GetHasValidUserGestureActivation(bool *aHasValidUserGestureActivation) override { return _to GetHasValidUserGestureActivation(aHasValidUserGestureActivation); } \
  NS_IMETHOD SetHasValidUserGestureActivation(bool aHasValidUserGestureActivation) override { return _to SetHasValidUserGestureActivation(aHasValidUserGestureActivation); } \
  using nsILoadInfo::GetAllowDeprecatedSystemRequests; \
  NS_IMETHOD GetAllowDeprecatedSystemRequests(bool *aAllowDeprecatedSystemRequests) override { return _to GetAllowDeprecatedSystemRequests(aAllowDeprecatedSystemRequests); } \
  NS_IMETHOD SetAllowDeprecatedSystemRequests(bool aAllowDeprecatedSystemRequests) override { return _to SetAllowDeprecatedSystemRequests(aAllowDeprecatedSystemRequests); } \
  using nsILoadInfo::GetParserCreatedScript; \
  NS_IMETHOD GetParserCreatedScript(bool *aParserCreatedScript) override { return _to GetParserCreatedScript(aParserCreatedScript); } \
  NS_IMETHOD SetParserCreatedScript(bool aParserCreatedScript) override { return _to SetParserCreatedScript(aParserCreatedScript); } \
  using nsILoadInfo::GetIsInDevToolsContext; \
  NS_IMETHOD GetIsInDevToolsContext(bool *aIsInDevToolsContext) override { return _to GetIsInDevToolsContext(aIsInDevToolsContext); } \
  NS_IMETHOD SetIsInDevToolsContext(bool aIsInDevToolsContext) override { return _to SetIsInDevToolsContext(aIsInDevToolsContext); } \
  using nsILoadInfo::GetIsInThirdPartyContext; \
  NS_IMETHOD GetIsInThirdPartyContext(bool *aIsInThirdPartyContext) override { return _to GetIsInThirdPartyContext(aIsInThirdPartyContext); } \
  NS_IMETHOD SetIsInThirdPartyContext(bool aIsInThirdPartyContext) override { return _to SetIsInThirdPartyContext(aIsInThirdPartyContext); } \
  using nsILoadInfo::GetIsThirdPartyContextToTopWindow; \
  NS_IMETHOD GetIsThirdPartyContextToTopWindow(bool *aIsThirdPartyContextToTopWindow) override { return _to GetIsThirdPartyContextToTopWindow(aIsThirdPartyContextToTopWindow); } \
  NS_IMETHOD SetIsThirdPartyContextToTopWindow(bool aIsThirdPartyContextToTopWindow) override { return _to SetIsThirdPartyContextToTopWindow(aIsThirdPartyContextToTopWindow); } \
  using nsILoadInfo::GetCookiePolicy; \
  NS_IMETHOD GetCookiePolicy(uint32_t *aCookiePolicy) override { return _to GetCookiePolicy(aCookiePolicy); } \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override { return _to GetCookieJarSettings(aCookieJarSettings); } \
  NS_IMETHOD SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override { return _to SetCookieJarSettings(aCookieJarSettings); } \
  using nsILoadInfo::GetHasStoragePermission; \
  NS_IMETHOD GetHasStoragePermission(bool *aHasStoragePermission) override { return _to GetHasStoragePermission(aHasStoragePermission); } \
  NS_IMETHOD SetHasStoragePermission(bool aHasStoragePermission) override { return _to SetHasStoragePermission(aHasStoragePermission); } \
  using nsILoadInfo::GetForceInheritPrincipal; \
  NS_IMETHOD GetForceInheritPrincipal(bool *aForceInheritPrincipal) override { return _to GetForceInheritPrincipal(aForceInheritPrincipal); } \
  using nsILoadInfo::GetForceInheritPrincipalOverruleOwner; \
  NS_IMETHOD GetForceInheritPrincipalOverruleOwner(bool *aForceInheritPrincipalOverruleOwner) override { return _to GetForceInheritPrincipalOverruleOwner(aForceInheritPrincipalOverruleOwner); } \
  using nsILoadInfo::GetLoadingSandboxed; \
  NS_IMETHOD GetLoadingSandboxed(bool *aLoadingSandboxed) override { return _to GetLoadingSandboxed(aLoadingSandboxed); } \
  using nsILoadInfo::GetAboutBlankInherits; \
  NS_IMETHOD GetAboutBlankInherits(bool *aAboutBlankInherits) override { return _to GetAboutBlankInherits(aAboutBlankInherits); } \
  using nsILoadInfo::GetAllowChrome; \
  NS_IMETHOD GetAllowChrome(bool *aAllowChrome) override { return _to GetAllowChrome(aAllowChrome); } \
  using nsILoadInfo::GetDisallowScript; \
  NS_IMETHOD GetDisallowScript(bool *aDisallowScript) override { return _to GetDisallowScript(aDisallowScript); } \
  using nsILoadInfo::GetDontFollowRedirects; \
  NS_IMETHOD GetDontFollowRedirects(bool *aDontFollowRedirects) override { return _to GetDontFollowRedirects(aDontFollowRedirects); } \
  using nsILoadInfo::GetLoadErrorPage; \
  NS_IMETHOD GetLoadErrorPage(bool *aLoadErrorPage) override { return _to GetLoadErrorPage(aLoadErrorPage); } \
  using nsILoadInfo::GetIsFormSubmission; \
  NS_IMETHOD GetIsFormSubmission(bool *aIsFormSubmission) override { return _to GetIsFormSubmission(aIsFormSubmission); } \
  NS_IMETHOD SetIsFormSubmission(bool aIsFormSubmission) override { return _to SetIsFormSubmission(aIsFormSubmission); } \
  NS_IMETHOD GetExternalContentPolicyType(nsContentPolicyType *aExternalContentPolicyType) override { return _to GetExternalContentPolicyType(aExternalContentPolicyType); } \
  using nsILoadInfo::GetSendCSPViolationEvents; \
  NS_IMETHOD GetSendCSPViolationEvents(bool *aSendCSPViolationEvents) override { return _to GetSendCSPViolationEvents(aSendCSPViolationEvents); } \
  NS_IMETHOD SetSendCSPViolationEvents(bool aSendCSPViolationEvents) override { return _to SetSendCSPViolationEvents(aSendCSPViolationEvents); } \
  virtual nsContentPolicyType InternalContentPolicyType(void) override { return _to InternalContentPolicyType(); } \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override { return _to GetInternalContentPolicyType(aInternalContentPolicyType); } \
  using nsILoadInfo::GetBlockAllMixedContent; \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override { return _to GetBlockAllMixedContent(aBlockAllMixedContent); } \
  using nsILoadInfo::GetUpgradeInsecureRequests; \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override { return _to GetUpgradeInsecureRequests(aUpgradeInsecureRequests); } \
  using nsILoadInfo::GetBrowserUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserUpgradeInsecureRequests(bool *aBrowserUpgradeInsecureRequests) override { return _to GetBrowserUpgradeInsecureRequests(aBrowserUpgradeInsecureRequests); } \
  using nsILoadInfo::GetBrowserDidUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserDidUpgradeInsecureRequests(bool *aBrowserDidUpgradeInsecureRequests) override { return _to GetBrowserDidUpgradeInsecureRequests(aBrowserDidUpgradeInsecureRequests); } \
  NS_IMETHOD SetBrowserDidUpgradeInsecureRequests(bool aBrowserDidUpgradeInsecureRequests) override { return _to SetBrowserDidUpgradeInsecureRequests(aBrowserDidUpgradeInsecureRequests); } \
  using nsILoadInfo::GetBrowserWouldUpgradeInsecureRequests; \
  NS_IMETHOD GetBrowserWouldUpgradeInsecureRequests(bool *aBrowserWouldUpgradeInsecureRequests) override { return _to GetBrowserWouldUpgradeInsecureRequests(aBrowserWouldUpgradeInsecureRequests); } \
  using nsILoadInfo::GetForceAllowDataURI; \
  NS_IMETHOD GetForceAllowDataURI(bool *aForceAllowDataURI) override { return _to GetForceAllowDataURI(aForceAllowDataURI); } \
  NS_IMETHOD SetForceAllowDataURI(bool aForceAllowDataURI) override { return _to SetForceAllowDataURI(aForceAllowDataURI); } \
  using nsILoadInfo::GetAllowInsecureRedirectToDataURI; \
  NS_IMETHOD GetAllowInsecureRedirectToDataURI(bool *aAllowInsecureRedirectToDataURI) override { return _to GetAllowInsecureRedirectToDataURI(aAllowInsecureRedirectToDataURI); } \
  NS_IMETHOD SetAllowInsecureRedirectToDataURI(bool aAllowInsecureRedirectToDataURI) override { return _to SetAllowInsecureRedirectToDataURI(aAllowInsecureRedirectToDataURI); } \
  using nsILoadInfo::GetBypassCORSChecks; \
  NS_IMETHOD GetBypassCORSChecks(bool *aBypassCORSChecks) override { return _to GetBypassCORSChecks(aBypassCORSChecks); } \
  NS_IMETHOD SetBypassCORSChecks(bool aBypassCORSChecks) override { return _to SetBypassCORSChecks(aBypassCORSChecks); } \
  using nsILoadInfo::GetSkipContentPolicyCheckForWebRequest; \
  NS_IMETHOD GetSkipContentPolicyCheckForWebRequest(bool *aSkipContentPolicyCheckForWebRequest) override { return _to GetSkipContentPolicyCheckForWebRequest(aSkipContentPolicyCheckForWebRequest); } \
  NS_IMETHOD SetSkipContentPolicyCheckForWebRequest(bool aSkipContentPolicyCheckForWebRequest) override { return _to SetSkipContentPolicyCheckForWebRequest(aSkipContentPolicyCheckForWebRequest); } \
  using nsILoadInfo::GetOriginalFrameSrcLoad; \
  NS_IMETHOD GetOriginalFrameSrcLoad(bool *aOriginalFrameSrcLoad) override { return _to GetOriginalFrameSrcLoad(aOriginalFrameSrcLoad); } \
  NS_IMETHOD SetOriginalFrameSrcLoad(bool aOriginalFrameSrcLoad) override { return _to SetOriginalFrameSrcLoad(aOriginalFrameSrcLoad); } \
  using nsILoadInfo::GetForceInheritPrincipalDropped; \
  NS_IMETHOD GetForceInheritPrincipalDropped(bool *aForceInheritPrincipalDropped) override { return _to GetForceInheritPrincipalDropped(aForceInheritPrincipalDropped); } \
  using nsILoadInfo::GetInnerWindowID; \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override { return _to GetInnerWindowID(aInnerWindowID); } \
  using nsILoadInfo::GetBrowsingContextID; \
  NS_IMETHOD GetBrowsingContextID(uint64_t *aBrowsingContextID) override { return _to GetBrowsingContextID(aBrowsingContextID); } \
  using nsILoadInfo::GetBrowsingContext; \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override { return _to GetBrowsingContext(aBrowsingContext); } \
  using nsILoadInfo::GetFrameBrowsingContextID; \
  NS_IMETHOD GetFrameBrowsingContextID(uint64_t *aFrameBrowsingContextID) override { return _to GetFrameBrowsingContextID(aFrameBrowsingContextID); } \
  using nsILoadInfo::GetFrameBrowsingContext; \
  NS_IMETHOD GetFrameBrowsingContext(mozilla::dom::BrowsingContext **aFrameBrowsingContext) override { return _to GetFrameBrowsingContext(aFrameBrowsingContext); } \
  using nsILoadInfo::GetTargetBrowsingContextID; \
  NS_IMETHOD GetTargetBrowsingContextID(uint64_t *aTargetBrowsingContextID) override { return _to GetTargetBrowsingContextID(aTargetBrowsingContextID); } \
  using nsILoadInfo::GetTargetBrowsingContext; \
  NS_IMETHOD GetTargetBrowsingContext(mozilla::dom::BrowsingContext **aTargetBrowsingContext) override { return _to GetTargetBrowsingContext(aTargetBrowsingContext); } \
  NS_IMETHOD ResetPrincipalToInheritToNullPrincipal(void) override { return _to ResetPrincipalToInheritToNullPrincipal(); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return _to GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override { return _to SetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override { return _to GetOriginAttributes(_retval); } \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override { return _to SetOriginAttributes(aOriginAttrs); } \
  using nsILoadInfo::GetInitialSecurityCheckDone; \
  NS_IMETHOD GetInitialSecurityCheckDone(bool *aInitialSecurityCheckDone) override { return _to GetInitialSecurityCheckDone(aInitialSecurityCheckDone); } \
  NS_IMETHOD SetInitialSecurityCheckDone(bool aInitialSecurityCheckDone) override { return _to SetInitialSecurityCheckDone(aInitialSecurityCheckDone); } \
  using nsILoadInfo::GetLoadTriggeredFromExternal; \
  NS_IMETHOD GetLoadTriggeredFromExternal(bool *aLoadTriggeredFromExternal) override { return _to GetLoadTriggeredFromExternal(aLoadTriggeredFromExternal); } \
  NS_IMETHOD SetLoadTriggeredFromExternal(bool aLoadTriggeredFromExternal) override { return _to SetLoadTriggeredFromExternal(aLoadTriggeredFromExternal); } \
  using nsILoadInfo::GetServiceWorkerTaintingSynthesized; \
  NS_IMETHOD GetServiceWorkerTaintingSynthesized(bool *aServiceWorkerTaintingSynthesized) override { return _to GetServiceWorkerTaintingSynthesized(aServiceWorkerTaintingSynthesized); } \
  NS_IMETHOD AppendRedirectHistoryEntry(nsIRedirectHistoryEntry *entry, bool isInternalRedirect) override { return _to AppendRedirectHistoryEntry(entry, isInternalRedirect); } \
  NS_IMETHOD GetRedirectChainIncludingInternalRedirects(JSContext* cx, JS::MutableHandleValue aRedirectChainIncludingInternalRedirects) override { return _to GetRedirectChainIncludingInternalRedirects(cx, aRedirectChainIncludingInternalRedirects); } \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChainIncludingInternalRedirects(void) override { return _to RedirectChainIncludingInternalRedirects(); } \
  NS_IMETHOD GetRedirectChain(JSContext* cx, JS::MutableHandleValue aRedirectChain) override { return _to GetRedirectChain(cx, aRedirectChain); } \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChain(void) override { return _to RedirectChain(); } \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AncestorPrincipals(void) override { return _to AncestorPrincipals(); } \
  virtual const nsTArray<uint64_t> & AncestorBrowsingContextIDs(void) override { return _to AncestorBrowsingContextIDs(); } \
  virtual void SetCorsPreflightInfo(const nsTArray<nsCString> & unsafeHeaders, bool forcePreflight) override { return _to SetCorsPreflightInfo(unsafeHeaders, forcePreflight); } \
  virtual const nsTArray<nsCString> & CorsUnsafeHeaders(void) override { return _to CorsUnsafeHeaders(); } \
  using nsILoadInfo::GetForcePreflight; \
  NS_IMETHOD GetForcePreflight(bool *aForcePreflight) override { return _to GetForcePreflight(aForcePreflight); } \
  using nsILoadInfo::GetIsPreflight; \
  NS_IMETHOD GetIsPreflight(bool *aIsPreflight) override { return _to GetIsPreflight(aIsPreflight); } \
  NS_IMETHOD GetTainting(uint32_t *aTainting) override { return _to GetTainting(aTainting); } \
  NS_IMETHOD MaybeIncreaseTainting(uint32_t aTainting) override { return _to MaybeIncreaseTainting(aTainting); } \
  using nsILoadInfo::GetIsTopLevelLoad; \
  NS_IMETHOD GetIsTopLevelLoad(bool *aIsTopLevelLoad) override { return _to GetIsTopLevelLoad(aIsTopLevelLoad); } \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override { return _to GetResultPrincipalURI(aResultPrincipalURI); } \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override { return _to SetResultPrincipalURI(aResultPrincipalURI); } \
  virtual nsIPrincipal * GetSandboxedLoadingPrincipal() override { return _to GetSandboxedLoadingPrincipal(); } \
  virtual nsIPrincipal * GetTopLevelPrincipal() override { return _to GetTopLevelPrincipal(); } \
  virtual nsIPrincipal * GetTopLevelStorageAreaPrincipal() override { return _to GetTopLevelStorageAreaPrincipal(); } \
  virtual void SetClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override { return _to SetClientInfo(aClientInfo); } \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetClientInfo(void) override { return _to GetClientInfo(); } \
  virtual void GiveReservedClientSource(mozilla::UniquePtr<mozilla::dom::ClientSource>&& aClientSource) override { return _to GiveReservedClientSource(aClientSource); } \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeReservedClientSource(void) override { return _to TakeReservedClientSource(); } \
  virtual void SetReservedClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override { return _to SetReservedClientInfo(aClientInfo); } \
  virtual void OverrideReservedClientInfoInParent(const mozilla::dom::ClientInfo & aClientInfo) override { return _to OverrideReservedClientInfoInParent(aClientInfo); } \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetReservedClientInfo(void) override { return _to GetReservedClientInfo(); } \
  virtual void SetInitialClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override { return _to SetInitialClientInfo(aClientInfo); } \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetInitialClientInfo(void) override { return _to GetInitialClientInfo(); } \
  virtual void SetController(const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override { return _to SetController(aServiceWorker); } \
  virtual void ClearController(void) override { return _to ClearController(); } \
  virtual const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & GetController(void) override { return _to GetController(); } \
  virtual void SetPerformanceStorage(mozilla::dom::PerformanceStorage * aPerformanceStorage) override { return _to SetPerformanceStorage(aPerformanceStorage); } \
  virtual mozilla::dom::PerformanceStorage * GetPerformanceStorage(void) override { return _to GetPerformanceStorage(); } \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCsp(void) override { return _to GetCsp(); } \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetPreloadCsp(void) override { return _to GetPreloadCsp(); } \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCspToInherit(void) override { return _to GetCspToInherit(); } \
  virtual void SynthesizeServiceWorkerTainting(mozilla::LoadTainting aTainting) override { return _to SynthesizeServiceWorkerTainting(aTainting); } \
  using nsILoadInfo::GetDocumentHasUserInteracted; \
  NS_IMETHOD GetDocumentHasUserInteracted(bool *aDocumentHasUserInteracted) override { return _to GetDocumentHasUserInteracted(aDocumentHasUserInteracted); } \
  NS_IMETHOD SetDocumentHasUserInteracted(bool aDocumentHasUserInteracted) override { return _to SetDocumentHasUserInteracted(aDocumentHasUserInteracted); } \
  using nsILoadInfo::GetAllowListFutureDocumentsCreatedFromThisRedirectChain; \
  NS_IMETHOD GetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool *aAllowListFutureDocumentsCreatedFromThisRedirectChain) override { return _to GetAllowListFutureDocumentsCreatedFromThisRedirectChain(aAllowListFutureDocumentsCreatedFromThisRedirectChain); } \
  NS_IMETHOD SetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool aAllowListFutureDocumentsCreatedFromThisRedirectChain) override { return _to SetAllowListFutureDocumentsCreatedFromThisRedirectChain(aAllowListFutureDocumentsCreatedFromThisRedirectChain); } \
  NS_IMETHOD GetCspNonce(nsAString& aCspNonce) override { return _to GetCspNonce(aCspNonce); } \
  NS_IMETHOD SetCspNonce(const nsAString& aCspNonce) override { return _to SetCspNonce(aCspNonce); } \
  using nsILoadInfo::GetRequestBlockingReason; \
  NS_IMETHOD GetRequestBlockingReason(uint32_t *aRequestBlockingReason) override { return _to GetRequestBlockingReason(aRequestBlockingReason); } \
  NS_IMETHOD SetRequestBlockingReason(uint32_t aRequestBlockingReason) override { return _to SetRequestBlockingReason(aRequestBlockingReason); } \
  NS_IMETHOD GetCspEventListener(nsICSPEventListener **aCspEventListener) override { return _to GetCspEventListener(aCspEventListener); } \
  NS_IMETHOD SetCspEventListener(nsICSPEventListener *aCspEventListener) override { return _to SetCspEventListener(aCspEventListener); } \
  using nsILoadInfo::GetIsFromProcessingFrameAttributes; \
  NS_IMETHOD GetIsFromProcessingFrameAttributes(bool *aIsFromProcessingFrameAttributes) override { return _to GetIsFromProcessingFrameAttributes(aIsFromProcessingFrameAttributes); } \
  using nsILoadInfo::GetLoadingEmbedderPolicy; \
  NS_IMETHOD GetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *aLoadingEmbedderPolicy) override { return _to GetLoadingEmbedderPolicy(aLoadingEmbedderPolicy); } \
  NS_IMETHOD SetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy aLoadingEmbedderPolicy) override { return _to SetLoadingEmbedderPolicy(aLoadingEmbedderPolicy); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADINFO(_to) \
  NS_IMETHOD GetLoadingPrincipal(nsIPrincipal **aLoadingPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingPrincipal(aLoadingPrincipal); } \
  virtual nsIPrincipal * VirtualGetLoadingPrincipal(void) override; \
  NS_IMETHOD GetTriggeringPrincipal(nsIPrincipal **aTriggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringPrincipal(aTriggeringPrincipal); } \
  virtual nsIPrincipal * TriggeringPrincipal(void) override; \
  NS_IMETHOD GetPrincipalToInherit(nsIPrincipal **aPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipalToInherit(aPrincipalToInherit); } \
  NS_IMETHOD SetPrincipalToInherit(nsIPrincipal *aPrincipalToInherit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrincipalToInherit(aPrincipalToInherit); } \
  virtual nsIPrincipal * PrincipalToInherit(void) override; \
  virtual nsIPrincipal * FindPrincipalToInherit(nsIChannel *aChannel) override; \
  NS_IMETHOD GetLoadingDocument(mozilla::dom::Document **aLoadingDocument) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingDocument(aLoadingDocument); } \
  virtual nsINode * LoadingNode(void) override; \
  virtual already_AddRefed<nsISupports> ContextForTopLevelLoad(void) override; \
  NS_IMETHOD GetLoadingContextXPCOM(nsISupports **aLoadingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingContextXPCOM(aLoadingContext); } \
  virtual already_AddRefed<nsISupports> GetLoadingContext(void) override; \
  NS_IMETHOD GetSecurityFlags(nsSecurityFlags *aSecurityFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityFlags(aSecurityFlags); } \
  NS_IMETHOD GetSandboxFlags(uint32_t *aSandboxFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSandboxFlags(aSandboxFlags); } \
  NS_IMETHOD GetTriggeringSandboxFlags(uint32_t *aTriggeringSandboxFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTriggeringSandboxFlags(aTriggeringSandboxFlags); } \
  NS_IMETHOD SetTriggeringSandboxFlags(uint32_t aTriggeringSandboxFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTriggeringSandboxFlags(aTriggeringSandboxFlags); } \
  NS_IMETHOD GetSecurityMode(uint32_t *aSecurityMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSecurityMode(aSecurityMode); } \
  NS_IMETHOD GetSkipContentSniffing(bool *aSkipContentSniffing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSkipContentSniffing(aSkipContentSniffing); } \
  NS_IMETHOD SetSkipContentSniffing(bool aSkipContentSniffing) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSkipContentSniffing(aSkipContentSniffing); } \
  NS_IMETHOD GetHttpsOnlyStatus(uint32_t *aHttpsOnlyStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHttpsOnlyStatus(aHttpsOnlyStatus); } \
  NS_IMETHOD SetHttpsOnlyStatus(uint32_t aHttpsOnlyStatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHttpsOnlyStatus(aHttpsOnlyStatus); } \
  NS_IMETHOD GetHasValidUserGestureActivation(bool *aHasValidUserGestureActivation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasValidUserGestureActivation(aHasValidUserGestureActivation); } \
  NS_IMETHOD SetHasValidUserGestureActivation(bool aHasValidUserGestureActivation) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHasValidUserGestureActivation(aHasValidUserGestureActivation); } \
  NS_IMETHOD GetAllowDeprecatedSystemRequests(bool *aAllowDeprecatedSystemRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowDeprecatedSystemRequests(aAllowDeprecatedSystemRequests); } \
  NS_IMETHOD SetAllowDeprecatedSystemRequests(bool aAllowDeprecatedSystemRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowDeprecatedSystemRequests(aAllowDeprecatedSystemRequests); } \
  NS_IMETHOD GetParserCreatedScript(bool *aParserCreatedScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParserCreatedScript(aParserCreatedScript); } \
  NS_IMETHOD SetParserCreatedScript(bool aParserCreatedScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParserCreatedScript(aParserCreatedScript); } \
  NS_IMETHOD GetIsInDevToolsContext(bool *aIsInDevToolsContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInDevToolsContext(aIsInDevToolsContext); } \
  NS_IMETHOD SetIsInDevToolsContext(bool aIsInDevToolsContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsInDevToolsContext(aIsInDevToolsContext); } \
  NS_IMETHOD GetIsInThirdPartyContext(bool *aIsInThirdPartyContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInThirdPartyContext(aIsInThirdPartyContext); } \
  NS_IMETHOD SetIsInThirdPartyContext(bool aIsInThirdPartyContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsInThirdPartyContext(aIsInThirdPartyContext); } \
  NS_IMETHOD GetIsThirdPartyContextToTopWindow(bool *aIsThirdPartyContextToTopWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsThirdPartyContextToTopWindow(aIsThirdPartyContextToTopWindow); } \
  NS_IMETHOD SetIsThirdPartyContextToTopWindow(bool aIsThirdPartyContextToTopWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsThirdPartyContextToTopWindow(aIsThirdPartyContextToTopWindow); } \
  NS_IMETHOD GetCookiePolicy(uint32_t *aCookiePolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookiePolicy(aCookiePolicy); } \
  NS_IMETHOD GetCookieJarSettings(nsICookieJarSettings **aCookieJarSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCookieJarSettings(aCookieJarSettings); } \
  NS_IMETHOD SetCookieJarSettings(nsICookieJarSettings *aCookieJarSettings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCookieJarSettings(aCookieJarSettings); } \
  NS_IMETHOD GetHasStoragePermission(bool *aHasStoragePermission) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasStoragePermission(aHasStoragePermission); } \
  NS_IMETHOD SetHasStoragePermission(bool aHasStoragePermission) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHasStoragePermission(aHasStoragePermission); } \
  NS_IMETHOD GetForceInheritPrincipal(bool *aForceInheritPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceInheritPrincipal(aForceInheritPrincipal); } \
  NS_IMETHOD GetForceInheritPrincipalOverruleOwner(bool *aForceInheritPrincipalOverruleOwner) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceInheritPrincipalOverruleOwner(aForceInheritPrincipalOverruleOwner); } \
  NS_IMETHOD GetLoadingSandboxed(bool *aLoadingSandboxed) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingSandboxed(aLoadingSandboxed); } \
  NS_IMETHOD GetAboutBlankInherits(bool *aAboutBlankInherits) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAboutBlankInherits(aAboutBlankInherits); } \
  NS_IMETHOD GetAllowChrome(bool *aAllowChrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowChrome(aAllowChrome); } \
  NS_IMETHOD GetDisallowScript(bool *aDisallowScript) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisallowScript(aDisallowScript); } \
  NS_IMETHOD GetDontFollowRedirects(bool *aDontFollowRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDontFollowRedirects(aDontFollowRedirects); } \
  NS_IMETHOD GetLoadErrorPage(bool *aLoadErrorPage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadErrorPage(aLoadErrorPage); } \
  NS_IMETHOD GetIsFormSubmission(bool *aIsFormSubmission) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFormSubmission(aIsFormSubmission); } \
  NS_IMETHOD SetIsFormSubmission(bool aIsFormSubmission) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsFormSubmission(aIsFormSubmission); } \
  NS_IMETHOD GetExternalContentPolicyType(nsContentPolicyType *aExternalContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExternalContentPolicyType(aExternalContentPolicyType); } \
  NS_IMETHOD GetSendCSPViolationEvents(bool *aSendCSPViolationEvents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendCSPViolationEvents(aSendCSPViolationEvents); } \
  NS_IMETHOD SetSendCSPViolationEvents(bool aSendCSPViolationEvents) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSendCSPViolationEvents(aSendCSPViolationEvents); } \
  virtual nsContentPolicyType InternalContentPolicyType(void) override; \
  NS_IMETHOD GetInternalContentPolicyType(nsContentPolicyType *aInternalContentPolicyType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInternalContentPolicyType(aInternalContentPolicyType); } \
  NS_IMETHOD GetBlockAllMixedContent(bool *aBlockAllMixedContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlockAllMixedContent(aBlockAllMixedContent); } \
  NS_IMETHOD GetUpgradeInsecureRequests(bool *aUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpgradeInsecureRequests(aUpgradeInsecureRequests); } \
  NS_IMETHOD GetBrowserUpgradeInsecureRequests(bool *aBrowserUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserUpgradeInsecureRequests(aBrowserUpgradeInsecureRequests); } \
  NS_IMETHOD GetBrowserDidUpgradeInsecureRequests(bool *aBrowserDidUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserDidUpgradeInsecureRequests(aBrowserDidUpgradeInsecureRequests); } \
  NS_IMETHOD SetBrowserDidUpgradeInsecureRequests(bool aBrowserDidUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBrowserDidUpgradeInsecureRequests(aBrowserDidUpgradeInsecureRequests); } \
  NS_IMETHOD GetBrowserWouldUpgradeInsecureRequests(bool *aBrowserWouldUpgradeInsecureRequests) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowserWouldUpgradeInsecureRequests(aBrowserWouldUpgradeInsecureRequests); } \
  NS_IMETHOD GetForceAllowDataURI(bool *aForceAllowDataURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceAllowDataURI(aForceAllowDataURI); } \
  NS_IMETHOD SetForceAllowDataURI(bool aForceAllowDataURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetForceAllowDataURI(aForceAllowDataURI); } \
  NS_IMETHOD GetAllowInsecureRedirectToDataURI(bool *aAllowInsecureRedirectToDataURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowInsecureRedirectToDataURI(aAllowInsecureRedirectToDataURI); } \
  NS_IMETHOD SetAllowInsecureRedirectToDataURI(bool aAllowInsecureRedirectToDataURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowInsecureRedirectToDataURI(aAllowInsecureRedirectToDataURI); } \
  NS_IMETHOD GetBypassCORSChecks(bool *aBypassCORSChecks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBypassCORSChecks(aBypassCORSChecks); } \
  NS_IMETHOD SetBypassCORSChecks(bool aBypassCORSChecks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetBypassCORSChecks(aBypassCORSChecks); } \
  NS_IMETHOD GetSkipContentPolicyCheckForWebRequest(bool *aSkipContentPolicyCheckForWebRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSkipContentPolicyCheckForWebRequest(aSkipContentPolicyCheckForWebRequest); } \
  NS_IMETHOD SetSkipContentPolicyCheckForWebRequest(bool aSkipContentPolicyCheckForWebRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSkipContentPolicyCheckForWebRequest(aSkipContentPolicyCheckForWebRequest); } \
  NS_IMETHOD GetOriginalFrameSrcLoad(bool *aOriginalFrameSrcLoad) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalFrameSrcLoad(aOriginalFrameSrcLoad); } \
  NS_IMETHOD SetOriginalFrameSrcLoad(bool aOriginalFrameSrcLoad) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginalFrameSrcLoad(aOriginalFrameSrcLoad); } \
  NS_IMETHOD GetForceInheritPrincipalDropped(bool *aForceInheritPrincipalDropped) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForceInheritPrincipalDropped(aForceInheritPrincipalDropped); } \
  NS_IMETHOD GetInnerWindowID(uint64_t *aInnerWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnerWindowID(aInnerWindowID); } \
  NS_IMETHOD GetBrowsingContextID(uint64_t *aBrowsingContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowsingContextID(aBrowsingContextID); } \
  NS_IMETHOD GetBrowsingContext(mozilla::dom::BrowsingContext **aBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBrowsingContext(aBrowsingContext); } \
  NS_IMETHOD GetFrameBrowsingContextID(uint64_t *aFrameBrowsingContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFrameBrowsingContextID(aFrameBrowsingContextID); } \
  NS_IMETHOD GetFrameBrowsingContext(mozilla::dom::BrowsingContext **aFrameBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFrameBrowsingContext(aFrameBrowsingContext); } \
  NS_IMETHOD GetTargetBrowsingContextID(uint64_t *aTargetBrowsingContextID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetBrowsingContextID(aTargetBrowsingContextID); } \
  NS_IMETHOD GetTargetBrowsingContext(mozilla::dom::BrowsingContext **aTargetBrowsingContext) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetBrowsingContext(aTargetBrowsingContext); } \
  NS_IMETHOD ResetPrincipalToInheritToNullPrincipal(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetPrincipalToInheritToNullPrincipal(); } \
  NS_IMETHOD GetScriptableOriginAttributes(JSContext* cx, JS::MutableHandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableOriginAttributes(cx, aOriginAttributes); } \
  NS_IMETHOD SetScriptableOriginAttributes(JSContext* cx, JS::HandleValue aOriginAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetScriptableOriginAttributes(cx, aOriginAttributes); } \
  virtual nsresult GetOriginAttributes(mozilla::OriginAttributes * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginAttributes(_retval); } \
  virtual nsresult SetOriginAttributes(const mozilla::OriginAttributes & aOriginAttrs) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetOriginAttributes(aOriginAttrs); } \
  NS_IMETHOD GetInitialSecurityCheckDone(bool *aInitialSecurityCheckDone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInitialSecurityCheckDone(aInitialSecurityCheckDone); } \
  NS_IMETHOD SetInitialSecurityCheckDone(bool aInitialSecurityCheckDone) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetInitialSecurityCheckDone(aInitialSecurityCheckDone); } \
  NS_IMETHOD GetLoadTriggeredFromExternal(bool *aLoadTriggeredFromExternal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadTriggeredFromExternal(aLoadTriggeredFromExternal); } \
  NS_IMETHOD SetLoadTriggeredFromExternal(bool aLoadTriggeredFromExternal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadTriggeredFromExternal(aLoadTriggeredFromExternal); } \
  NS_IMETHOD GetServiceWorkerTaintingSynthesized(bool *aServiceWorkerTaintingSynthesized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceWorkerTaintingSynthesized(aServiceWorkerTaintingSynthesized); } \
  NS_IMETHOD AppendRedirectHistoryEntry(nsIRedirectHistoryEntry *entry, bool isInternalRedirect) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendRedirectHistoryEntry(entry, isInternalRedirect); } \
  NS_IMETHOD GetRedirectChainIncludingInternalRedirects(JSContext* cx, JS::MutableHandleValue aRedirectChainIncludingInternalRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectChainIncludingInternalRedirects(cx, aRedirectChainIncludingInternalRedirects); } \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChainIncludingInternalRedirects(void) override; \
  NS_IMETHOD GetRedirectChain(JSContext* cx, JS::MutableHandleValue aRedirectChain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirectChain(cx, aRedirectChain); } \
  virtual const nsTArray<nsCOMPtr<nsIRedirectHistoryEntry>> & RedirectChain(void) override; \
  virtual const nsTArray<nsCOMPtr<nsIPrincipal>> & AncestorPrincipals(void) override; \
  virtual const nsTArray<uint64_t> & AncestorBrowsingContextIDs(void) override; \
  virtual void SetCorsPreflightInfo(const nsTArray<nsCString> & unsafeHeaders, bool forcePreflight) override; \
  virtual const nsTArray<nsCString> & CorsUnsafeHeaders(void) override; \
  NS_IMETHOD GetForcePreflight(bool *aForcePreflight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetForcePreflight(aForcePreflight); } \
  NS_IMETHOD GetIsPreflight(bool *aIsPreflight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsPreflight(aIsPreflight); } \
  NS_IMETHOD GetTainting(uint32_t *aTainting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTainting(aTainting); } \
  NS_IMETHOD MaybeIncreaseTainting(uint32_t aTainting) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeIncreaseTainting(aTainting); } \
  NS_IMETHOD GetIsTopLevelLoad(bool *aIsTopLevelLoad) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsTopLevelLoad(aIsTopLevelLoad); } \
  NS_IMETHOD GetResultPrincipalURI(nsIURI **aResultPrincipalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultPrincipalURI(aResultPrincipalURI); } \
  NS_IMETHOD SetResultPrincipalURI(nsIURI *aResultPrincipalURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetResultPrincipalURI(aResultPrincipalURI); } \
  virtual nsIPrincipal * GetSandboxedLoadingPrincipal() override; \
  virtual nsIPrincipal * GetTopLevelPrincipal() override; \
  virtual nsIPrincipal * GetTopLevelStorageAreaPrincipal() override; \
  virtual void SetClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetClientInfo(void) override; \
  virtual void GiveReservedClientSource(mozilla::UniquePtr<mozilla::dom::ClientSource>&& aClientSource) override; \
  virtual mozilla::UniquePtr<mozilla::dom::ClientSource> TakeReservedClientSource(void) override; \
  virtual void SetReservedClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual void OverrideReservedClientInfoInParent(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetReservedClientInfo(void) override; \
  virtual void SetInitialClientInfo(const mozilla::dom::ClientInfo & aClientInfo) override; \
  virtual const mozilla::Maybe<mozilla::dom::ClientInfo> & GetInitialClientInfo(void) override; \
  virtual void SetController(const mozilla::dom::ServiceWorkerDescriptor & aServiceWorker) override; \
  virtual void ClearController(void) override; \
  virtual const mozilla::Maybe<mozilla::dom::ServiceWorkerDescriptor> & GetController(void) override; \
  virtual void SetPerformanceStorage(mozilla::dom::PerformanceStorage * aPerformanceStorage) override; \
  virtual mozilla::dom::PerformanceStorage * GetPerformanceStorage(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCsp(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetPreloadCsp(void) override; \
  virtual already_AddRefed<nsIContentSecurityPolicy> GetCspToInherit(void) override; \
  virtual void SynthesizeServiceWorkerTainting(mozilla::LoadTainting aTainting) override; \
  NS_IMETHOD GetDocumentHasUserInteracted(bool *aDocumentHasUserInteracted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocumentHasUserInteracted(aDocumentHasUserInteracted); } \
  NS_IMETHOD SetDocumentHasUserInteracted(bool aDocumentHasUserInteracted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocumentHasUserInteracted(aDocumentHasUserInteracted); } \
  NS_IMETHOD GetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool *aAllowListFutureDocumentsCreatedFromThisRedirectChain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowListFutureDocumentsCreatedFromThisRedirectChain(aAllowListFutureDocumentsCreatedFromThisRedirectChain); } \
  NS_IMETHOD SetAllowListFutureDocumentsCreatedFromThisRedirectChain(bool aAllowListFutureDocumentsCreatedFromThisRedirectChain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowListFutureDocumentsCreatedFromThisRedirectChain(aAllowListFutureDocumentsCreatedFromThisRedirectChain); } \
  NS_IMETHOD GetCspNonce(nsAString& aCspNonce) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCspNonce(aCspNonce); } \
  NS_IMETHOD SetCspNonce(const nsAString& aCspNonce) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCspNonce(aCspNonce); } \
  NS_IMETHOD GetRequestBlockingReason(uint32_t *aRequestBlockingReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestBlockingReason(aRequestBlockingReason); } \
  NS_IMETHOD SetRequestBlockingReason(uint32_t aRequestBlockingReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestBlockingReason(aRequestBlockingReason); } \
  NS_IMETHOD GetCspEventListener(nsICSPEventListener **aCspEventListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCspEventListener(aCspEventListener); } \
  NS_IMETHOD SetCspEventListener(nsICSPEventListener *aCspEventListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCspEventListener(aCspEventListener); } \
  NS_IMETHOD GetIsFromProcessingFrameAttributes(bool *aIsFromProcessingFrameAttributes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFromProcessingFrameAttributes(aIsFromProcessingFrameAttributes); } \
  NS_IMETHOD GetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy *aLoadingEmbedderPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadingEmbedderPolicy(aLoadingEmbedderPolicy); } \
  NS_IMETHOD SetLoadingEmbedderPolicy(nsILoadInfo::CrossOriginEmbedderPolicy aLoadingEmbedderPolicy) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLoadingEmbedderPolicy(aLoadingEmbedderPolicy); } 


#endif /* __gen_nsILoadInfo_h__ */
