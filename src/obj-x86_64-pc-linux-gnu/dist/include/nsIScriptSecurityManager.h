/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIScriptSecurityManager.idl
 */

#ifndef __gen_nsIScriptSecurityManager_h__
#define __gen_nsIScriptSecurityManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIPrincipal_h__
#include "nsIPrincipal.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIClassInfo; /* forward declaration */

class nsIDocShell; /* forward declaration */

class nsIDomainPolicy; /* forward declaration */

class nsILoadContext; /* forward declaration */

#include "jspubtd.h"
namespace mozilla {
namespace dom {
class DomainPolicyClone;
}
}

/* starting interface:    nsIScriptSecurityManager */
#define NS_ISCRIPTSECURITYMANAGER_IID_STR "51daad87-3a0c-44cc-b620-7356801c9022"

#define NS_ISCRIPTSECURITYMANAGER_IID \
  {0x51daad87, 0x3a0c, 0x44cc, \
    { 0xb6, 0x20, 0x73, 0x56, 0x80, 0x1c, 0x90, 0x22 }}

class NS_NO_VTABLE nsIScriptSecurityManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISCRIPTSECURITYMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIScriptSecurityManager;

  /* [noscript] void canCreateWrapper (in JSContextPtr aJSContext, in nsIIDRef aIID, in nsISupports aObj, in nsIClassInfo aClassInfo); */
  NS_IMETHOD CanCreateWrapper(JSContext * aJSContext, const nsIID & aIID, nsISupports *aObj, nsIClassInfo *aClassInfo) = 0;

  /* [noscript] void canCreateInstance (in JSContextPtr aJSContext, in nsCIDRef aCID); */
  NS_IMETHOD CanCreateInstance(JSContext * aJSContext, const nsCID & aCID) = 0;

  /* [noscript] void canGetService (in JSContextPtr aJSContext, in nsCIDRef aCID); */
  NS_IMETHOD CanGetService(JSContext * aJSContext, const nsCID & aCID) = 0;

  /* [noscript] void checkLoadURIFromScript (in JSContextPtr cx, in nsIURI uri); */
  NS_IMETHOD CheckLoadURIFromScript(JSContext * cx, nsIURI *uri) = 0;

  enum {
    STANDARD = 0U,
    LOAD_IS_AUTOMATIC_DOCUMENT_REPLACEMENT = 1U,
    ALLOW_CHROME = 2U,
    DISALLOW_INHERIT_PRINCIPAL = 4U,
    DISALLOW_SCRIPT_OR_DATA = 4U,
    DISALLOW_SCRIPT = 8U,
    DONT_REPORT_ERRORS = 16U
  };

  /* [binaryname(CheckLoadURIWithPrincipal)] void checkLoadURIWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in nsIURI uri, in unsigned long flags, [optional] in unsigned long long innerWindowID); */
  NS_IMETHOD CheckLoadURIWithPrincipal(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID) = 0;

  /* [binaryname(CheckLoadURIWithPrincipalFromJS),implicit_jscontext] void checkLoadURIWithPrincipal (in nsIPrincipal aPrincipal, in nsIURI uri, [optional] in unsigned long flags, [optional] in unsigned long long innerWindowID); */
  NS_IMETHOD CheckLoadURIWithPrincipalFromJS(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID, JSContext* cx) = 0;

  /* [binaryname(CheckLoadURIStrWithPrincipal)] void checkLoadURIStrWithPrincipalXPCOM (in nsIPrincipal aPrincipal, in AUTF8String uri, in unsigned long flags); */
  NS_IMETHOD CheckLoadURIStrWithPrincipal(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags) = 0;

  /* [binaryname(CheckLoadURIStrWithPrincipalFromJS),implicit_jscontext] void checkLoadURIStrWithPrincipal (in nsIPrincipal aPrincipal, in AUTF8String uri, [optional] in unsigned long flags); */
  NS_IMETHOD CheckLoadURIStrWithPrincipalFromJS(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags, JSContext* cx) = 0;

  /* bool inFileURIAllowlist (in nsIURI aUri); */
  NS_IMETHOD InFileURIAllowlist(nsIURI *aUri, bool *_retval) = 0;

  /* nsIPrincipal getSystemPrincipal (); */
  NS_IMETHOD GetSystemPrincipal(nsIPrincipal **_retval) = 0;

  /* nsIPrincipal getLoadContextContentPrincipal (in nsIURI uri, in nsILoadContext loadContext); */
  NS_IMETHOD GetLoadContextContentPrincipal(nsIURI *uri, nsILoadContext *loadContext, nsIPrincipal **_retval) = 0;

  /* nsIPrincipal getDocShellContentPrincipal (in nsIURI uri, in nsIDocShell docShell); */
  NS_IMETHOD GetDocShellContentPrincipal(nsIURI *uri, nsIDocShell *docShell, nsIPrincipal **_retval) = 0;

  /* [implicit_jscontext] nsIPrincipal principalWithOA (in nsIPrincipal principal, in jsval originAttributes); */
  NS_IMETHOD PrincipalWithOA(nsIPrincipal *principal, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) = 0;

  /* [implicit_jscontext] nsIPrincipal createContentPrincipal (in nsIURI uri, in jsval originAttributes); */
  NS_IMETHOD CreateContentPrincipal(nsIURI *uri, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) = 0;

  /* nsIPrincipal createContentPrincipalFromOrigin (in ACString origin); */
  NS_IMETHOD CreateContentPrincipalFromOrigin(const nsACString& origin, nsIPrincipal **_retval) = 0;

  /* ACString principalToJSON (in nsIPrincipal principal); */
  NS_IMETHOD PrincipalToJSON(nsIPrincipal *principal, nsACString& _retval) = 0;

  /* nsIPrincipal JSONToPrincipal (in ACString json); */
  NS_IMETHOD JSONToPrincipal(const nsACString& json, nsIPrincipal **_retval) = 0;

  /* [implicit_jscontext] nsIPrincipal createNullPrincipal (in jsval originAttributes); */
  NS_IMETHOD CreateNullPrincipal(JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) = 0;

  /* void checkSameOriginURI (in nsIURI aSourceURI, in nsIURI aTargetURI, in boolean reportError, in boolean fromPrivateWindow); */
  NS_IMETHOD CheckSameOriginURI(nsIURI *aSourceURI, nsIURI *aTargetURI, bool reportError, bool fromPrivateWindow) = 0;

  /* nsIPrincipal getChannelResultPrincipal (in nsIChannel aChannel); */
  NS_IMETHOD GetChannelResultPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) = 0;

  /* nsIPrincipal getChannelResultStoragePrincipal (in nsIChannel aChannel); */
  NS_IMETHOD GetChannelResultStoragePrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) = 0;

  /* void getChannelResultPrincipals (in nsIChannel aChannel, out nsIPrincipal aPrincipal, out nsIPrincipal aPartitionedPrincipal); */
  NS_IMETHOD GetChannelResultPrincipals(nsIChannel *aChannel, nsIPrincipal **aPrincipal, nsIPrincipal **aPartitionedPrincipal) = 0;

  /* [noscript,nostdcall] nsIPrincipal getChannelResultPrincipalIfNotSandboxed (in nsIChannel aChannel); */
  virtual nsresult GetChannelResultPrincipalIfNotSandboxed(nsIChannel *aChannel, nsIPrincipal **_retval) = 0;

  /* nsIPrincipal getChannelURIPrincipal (in nsIChannel aChannel); */
  NS_IMETHOD GetChannelURIPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) = 0;

  enum {
    DEFAULT_USER_CONTEXT_ID = 0U,
    DEFAULT_PRIVATE_BROWSING_ID = 0U
  };

  /* nsIDomainPolicy activateDomainPolicy (); */
  NS_IMETHOD ActivateDomainPolicy(nsIDomainPolicy **_retval) = 0;

  /* readonly attribute boolean domainPolicyActive; */
  NS_IMETHOD GetDomainPolicyActive(bool *aDomainPolicyActive) = 0;

  /* [noscript] nsIDomainPolicy activateDomainPolicyInternal (); */
  NS_IMETHOD ActivateDomainPolicyInternal(nsIDomainPolicy **_retval) = 0;

  /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) = 0;

  /* bool policyAllowsScript (in nsIURI aDomain); */
  NS_IMETHOD PolicyAllowsScript(nsIURI *aDomain, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIScriptSecurityManager, NS_ISCRIPTSECURITYMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISCRIPTSECURITYMANAGER \
  NS_IMETHOD CanCreateWrapper(JSContext * aJSContext, const nsIID & aIID, nsISupports *aObj, nsIClassInfo *aClassInfo) override; \
  NS_IMETHOD CanCreateInstance(JSContext * aJSContext, const nsCID & aCID) override; \
  NS_IMETHOD CanGetService(JSContext * aJSContext, const nsCID & aCID) override; \
  NS_IMETHOD CheckLoadURIFromScript(JSContext * cx, nsIURI *uri) override; \
  NS_IMETHOD CheckLoadURIWithPrincipal(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID) override; \
  NS_IMETHOD CheckLoadURIWithPrincipalFromJS(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID, JSContext* cx) override; \
  NS_IMETHOD CheckLoadURIStrWithPrincipal(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags) override; \
  NS_IMETHOD CheckLoadURIStrWithPrincipalFromJS(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags, JSContext* cx) override; \
  NS_IMETHOD InFileURIAllowlist(nsIURI *aUri, bool *_retval) override; \
  NS_IMETHOD GetSystemPrincipal(nsIPrincipal **_retval) override; \
  NS_IMETHOD GetLoadContextContentPrincipal(nsIURI *uri, nsILoadContext *loadContext, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetDocShellContentPrincipal(nsIURI *uri, nsIDocShell *docShell, nsIPrincipal **_retval) override; \
  NS_IMETHOD PrincipalWithOA(nsIPrincipal *principal, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override; \
  NS_IMETHOD CreateContentPrincipal(nsIURI *uri, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override; \
  NS_IMETHOD CreateContentPrincipalFromOrigin(const nsACString& origin, nsIPrincipal **_retval) override; \
  NS_IMETHOD PrincipalToJSON(nsIPrincipal *principal, nsACString& _retval) override; \
  NS_IMETHOD JSONToPrincipal(const nsACString& json, nsIPrincipal **_retval) override; \
  NS_IMETHOD CreateNullPrincipal(JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override; \
  NS_IMETHOD CheckSameOriginURI(nsIURI *aSourceURI, nsIURI *aTargetURI, bool reportError, bool fromPrivateWindow) override; \
  NS_IMETHOD GetChannelResultPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetChannelResultStoragePrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetChannelResultPrincipals(nsIChannel *aChannel, nsIPrincipal **aPrincipal, nsIPrincipal **aPartitionedPrincipal) override; \
  virtual nsresult GetChannelResultPrincipalIfNotSandboxed(nsIChannel *aChannel, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetChannelURIPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override; \
  NS_IMETHOD ActivateDomainPolicy(nsIDomainPolicy **_retval) override; \
  NS_IMETHOD GetDomainPolicyActive(bool *aDomainPolicyActive) override; \
  NS_IMETHOD ActivateDomainPolicyInternal(nsIDomainPolicy **_retval) override; \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override; \
  NS_IMETHOD PolicyAllowsScript(nsIURI *aDomain, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISCRIPTSECURITYMANAGER \
  nsresult CanCreateWrapper(JSContext * aJSContext, const nsIID & aIID, nsISupports *aObj, nsIClassInfo *aClassInfo); \
  nsresult CanCreateInstance(JSContext * aJSContext, const nsCID & aCID); \
  nsresult CanGetService(JSContext * aJSContext, const nsCID & aCID); \
  nsresult CheckLoadURIFromScript(JSContext * cx, nsIURI *uri); \
  nsresult CheckLoadURIWithPrincipal(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID); \
  nsresult CheckLoadURIWithPrincipalFromJS(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID, JSContext* cx); \
  nsresult CheckLoadURIStrWithPrincipal(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags); \
  nsresult CheckLoadURIStrWithPrincipalFromJS(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags, JSContext* cx); \
  nsresult InFileURIAllowlist(nsIURI *aUri, bool *_retval); \
  nsresult GetSystemPrincipal(nsIPrincipal **_retval); \
  nsresult GetLoadContextContentPrincipal(nsIURI *uri, nsILoadContext *loadContext, nsIPrincipal **_retval); \
  nsresult GetDocShellContentPrincipal(nsIURI *uri, nsIDocShell *docShell, nsIPrincipal **_retval); \
  nsresult PrincipalWithOA(nsIPrincipal *principal, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval); \
  nsresult CreateContentPrincipal(nsIURI *uri, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval); \
  nsresult CreateContentPrincipalFromOrigin(const nsACString& origin, nsIPrincipal **_retval); \
  nsresult PrincipalToJSON(nsIPrincipal *principal, nsACString& _retval); \
  nsresult JSONToPrincipal(const nsACString& json, nsIPrincipal **_retval); \
  nsresult CreateNullPrincipal(JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval); \
  nsresult CheckSameOriginURI(nsIURI *aSourceURI, nsIURI *aTargetURI, bool reportError, bool fromPrivateWindow); \
  nsresult GetChannelResultPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval); \
  nsresult GetChannelResultStoragePrincipal(nsIChannel *aChannel, nsIPrincipal **_retval); \
  nsresult GetChannelResultPrincipals(nsIChannel *aChannel, nsIPrincipal **aPrincipal, nsIPrincipal **aPartitionedPrincipal); \
  nsresult GetChannelResultPrincipalIfNotSandboxed(nsIChannel *aChannel, nsIPrincipal **_retval); \
  nsresult GetChannelURIPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval); \
  nsresult ActivateDomainPolicy(nsIDomainPolicy **_retval); \
  nsresult GetDomainPolicyActive(bool *aDomainPolicyActive); \
  nsresult ActivateDomainPolicyInternal(nsIDomainPolicy **_retval); \
  nsresult_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone); \
  nsresult PolicyAllowsScript(nsIURI *aDomain, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISCRIPTSECURITYMANAGER(_to) \
  NS_IMETHOD CanCreateWrapper(JSContext * aJSContext, const nsIID & aIID, nsISupports *aObj, nsIClassInfo *aClassInfo) override { return _to CanCreateWrapper(aJSContext, aIID, aObj, aClassInfo); } \
  NS_IMETHOD CanCreateInstance(JSContext * aJSContext, const nsCID & aCID) override { return _to CanCreateInstance(aJSContext, aCID); } \
  NS_IMETHOD CanGetService(JSContext * aJSContext, const nsCID & aCID) override { return _to CanGetService(aJSContext, aCID); } \
  NS_IMETHOD CheckLoadURIFromScript(JSContext * cx, nsIURI *uri) override { return _to CheckLoadURIFromScript(cx, uri); } \
  NS_IMETHOD CheckLoadURIWithPrincipal(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID) override { return _to CheckLoadURIWithPrincipal(aPrincipal, uri, flags, innerWindowID); } \
  NS_IMETHOD CheckLoadURIWithPrincipalFromJS(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID, JSContext* cx) override { return _to CheckLoadURIWithPrincipalFromJS(aPrincipal, uri, flags, innerWindowID, cx); } \
  NS_IMETHOD CheckLoadURIStrWithPrincipal(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags) override { return _to CheckLoadURIStrWithPrincipal(aPrincipal, uri, flags); } \
  NS_IMETHOD CheckLoadURIStrWithPrincipalFromJS(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags, JSContext* cx) override { return _to CheckLoadURIStrWithPrincipalFromJS(aPrincipal, uri, flags, cx); } \
  NS_IMETHOD InFileURIAllowlist(nsIURI *aUri, bool *_retval) override { return _to InFileURIAllowlist(aUri, _retval); } \
  NS_IMETHOD GetSystemPrincipal(nsIPrincipal **_retval) override { return _to GetSystemPrincipal(_retval); } \
  NS_IMETHOD GetLoadContextContentPrincipal(nsIURI *uri, nsILoadContext *loadContext, nsIPrincipal **_retval) override { return _to GetLoadContextContentPrincipal(uri, loadContext, _retval); } \
  NS_IMETHOD GetDocShellContentPrincipal(nsIURI *uri, nsIDocShell *docShell, nsIPrincipal **_retval) override { return _to GetDocShellContentPrincipal(uri, docShell, _retval); } \
  NS_IMETHOD PrincipalWithOA(nsIPrincipal *principal, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return _to PrincipalWithOA(principal, originAttributes, cx, _retval); } \
  NS_IMETHOD CreateContentPrincipal(nsIURI *uri, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return _to CreateContentPrincipal(uri, originAttributes, cx, _retval); } \
  NS_IMETHOD CreateContentPrincipalFromOrigin(const nsACString& origin, nsIPrincipal **_retval) override { return _to CreateContentPrincipalFromOrigin(origin, _retval); } \
  NS_IMETHOD PrincipalToJSON(nsIPrincipal *principal, nsACString& _retval) override { return _to PrincipalToJSON(principal, _retval); } \
  NS_IMETHOD JSONToPrincipal(const nsACString& json, nsIPrincipal **_retval) override { return _to JSONToPrincipal(json, _retval); } \
  NS_IMETHOD CreateNullPrincipal(JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return _to CreateNullPrincipal(originAttributes, cx, _retval); } \
  NS_IMETHOD CheckSameOriginURI(nsIURI *aSourceURI, nsIURI *aTargetURI, bool reportError, bool fromPrivateWindow) override { return _to CheckSameOriginURI(aSourceURI, aTargetURI, reportError, fromPrivateWindow); } \
  NS_IMETHOD GetChannelResultPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return _to GetChannelResultPrincipal(aChannel, _retval); } \
  NS_IMETHOD GetChannelResultStoragePrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return _to GetChannelResultStoragePrincipal(aChannel, _retval); } \
  NS_IMETHOD GetChannelResultPrincipals(nsIChannel *aChannel, nsIPrincipal **aPrincipal, nsIPrincipal **aPartitionedPrincipal) override { return _to GetChannelResultPrincipals(aChannel, aPrincipal, aPartitionedPrincipal); } \
  virtual nsresult GetChannelResultPrincipalIfNotSandboxed(nsIChannel *aChannel, nsIPrincipal **_retval) override { return _to GetChannelResultPrincipalIfNotSandboxed(aChannel, _retval); } \
  NS_IMETHOD GetChannelURIPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return _to GetChannelURIPrincipal(aChannel, _retval); } \
  NS_IMETHOD ActivateDomainPolicy(nsIDomainPolicy **_retval) override { return _to ActivateDomainPolicy(_retval); } \
  NS_IMETHOD GetDomainPolicyActive(bool *aDomainPolicyActive) override { return _to GetDomainPolicyActive(aDomainPolicyActive); } \
  NS_IMETHOD ActivateDomainPolicyInternal(nsIDomainPolicy **_retval) override { return _to ActivateDomainPolicyInternal(_retval); } \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override { return _to CloneDomainPolicy(aClone); } \
  NS_IMETHOD PolicyAllowsScript(nsIURI *aDomain, bool *_retval) override { return _to PolicyAllowsScript(aDomain, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISCRIPTSECURITYMANAGER(_to) \
  NS_IMETHOD CanCreateWrapper(JSContext * aJSContext, const nsIID & aIID, nsISupports *aObj, nsIClassInfo *aClassInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanCreateWrapper(aJSContext, aIID, aObj, aClassInfo); } \
  NS_IMETHOD CanCreateInstance(JSContext * aJSContext, const nsCID & aCID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanCreateInstance(aJSContext, aCID); } \
  NS_IMETHOD CanGetService(JSContext * aJSContext, const nsCID & aCID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanGetService(aJSContext, aCID); } \
  NS_IMETHOD CheckLoadURIFromScript(JSContext * cx, nsIURI *uri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckLoadURIFromScript(cx, uri); } \
  NS_IMETHOD CheckLoadURIWithPrincipal(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckLoadURIWithPrincipal(aPrincipal, uri, flags, innerWindowID); } \
  NS_IMETHOD CheckLoadURIWithPrincipalFromJS(nsIPrincipal *aPrincipal, nsIURI *uri, uint32_t flags, uint64_t innerWindowID, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckLoadURIWithPrincipalFromJS(aPrincipal, uri, flags, innerWindowID, cx); } \
  NS_IMETHOD CheckLoadURIStrWithPrincipal(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckLoadURIStrWithPrincipal(aPrincipal, uri, flags); } \
  NS_IMETHOD CheckLoadURIStrWithPrincipalFromJS(nsIPrincipal *aPrincipal, const nsACString& uri, uint32_t flags, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckLoadURIStrWithPrincipalFromJS(aPrincipal, uri, flags, cx); } \
  NS_IMETHOD InFileURIAllowlist(nsIURI *aUri, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InFileURIAllowlist(aUri, _retval); } \
  NS_IMETHOD GetSystemPrincipal(nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemPrincipal(_retval); } \
  NS_IMETHOD GetLoadContextContentPrincipal(nsIURI *uri, nsILoadContext *loadContext, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLoadContextContentPrincipal(uri, loadContext, _retval); } \
  NS_IMETHOD GetDocShellContentPrincipal(nsIURI *uri, nsIDocShell *docShell, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDocShellContentPrincipal(uri, docShell, _retval); } \
  NS_IMETHOD PrincipalWithOA(nsIPrincipal *principal, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrincipalWithOA(principal, originAttributes, cx, _retval); } \
  NS_IMETHOD CreateContentPrincipal(nsIURI *uri, JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateContentPrincipal(uri, originAttributes, cx, _retval); } \
  NS_IMETHOD CreateContentPrincipalFromOrigin(const nsACString& origin, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateContentPrincipalFromOrigin(origin, _retval); } \
  NS_IMETHOD PrincipalToJSON(nsIPrincipal *principal, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrincipalToJSON(principal, _retval); } \
  NS_IMETHOD JSONToPrincipal(const nsACString& json, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->JSONToPrincipal(json, _retval); } \
  NS_IMETHOD CreateNullPrincipal(JS::HandleValue originAttributes, JSContext* cx, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateNullPrincipal(originAttributes, cx, _retval); } \
  NS_IMETHOD CheckSameOriginURI(nsIURI *aSourceURI, nsIURI *aTargetURI, bool reportError, bool fromPrivateWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckSameOriginURI(aSourceURI, aTargetURI, reportError, fromPrivateWindow); } \
  NS_IMETHOD GetChannelResultPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelResultPrincipal(aChannel, _retval); } \
  NS_IMETHOD GetChannelResultStoragePrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelResultStoragePrincipal(aChannel, _retval); } \
  NS_IMETHOD GetChannelResultPrincipals(nsIChannel *aChannel, nsIPrincipal **aPrincipal, nsIPrincipal **aPartitionedPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelResultPrincipals(aChannel, aPrincipal, aPartitionedPrincipal); } \
  virtual nsresult GetChannelResultPrincipalIfNotSandboxed(nsIChannel *aChannel, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelResultPrincipalIfNotSandboxed(aChannel, _retval); } \
  NS_IMETHOD GetChannelURIPrincipal(nsIChannel *aChannel, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChannelURIPrincipal(aChannel, _retval); } \
  NS_IMETHOD ActivateDomainPolicy(nsIDomainPolicy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ActivateDomainPolicy(_retval); } \
  NS_IMETHOD GetDomainPolicyActive(bool *aDomainPolicyActive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomainPolicyActive(aDomainPolicyActive); } \
  NS_IMETHOD ActivateDomainPolicyInternal(nsIDomainPolicy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ActivateDomainPolicyInternal(_retval); } \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override; \
  NS_IMETHOD PolicyAllowsScript(nsIURI *aDomain, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PolicyAllowsScript(aDomain, _retval); } 

#define NS_SCRIPTSECURITYMANAGER_CONTRACTID "@mozilla.org/scriptsecuritymanager;1"

#endif /* __gen_nsIScriptSecurityManager_h__ */
