/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/mozIThirdPartyUtil.idl
 */

#ifndef __gen_mozIThirdPartyUtil_h__
#define __gen_mozIThirdPartyUtil_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class mozIDOMWindowProxy; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsILoadInfo; /* forward declaration */


#include "mozilla/EnumSet.h"
enum class ThirdPartyAnalysis {
  IsForeign,
  IsThirdPartyTrackingResource,
  IsThirdPartySocialTrackingResource,
  IsStorageAccessPermissionGranted,
};
using ThirdPartyAnalysisResult = mozilla::EnumSet<ThirdPartyAnalysis>;
typedef bool (*RequireThirdPartyCheck)(nsILoadInfo*);

/* starting interface:    mozIThirdPartyUtil */
#define MOZITHIRDPARTYUTIL_IID_STR "fd82700e-ffb4-4932-b7d6-08f0b5697dda"

#define MOZITHIRDPARTYUTIL_IID \
  {0xfd82700e, 0xffb4, 0x4932, \
    { 0xb7, 0xd6, 0x08, 0xf0, 0xb5, 0x69, 0x7d, 0xda }}

class NS_NO_VTABLE mozIThirdPartyUtil : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZITHIRDPARTYUTIL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIThirdPartyUtil;

  /* boolean isThirdPartyURI (in nsIURI aFirstURI, in nsIURI aSecondURI); */
  NS_IMETHOD IsThirdPartyURI(nsIURI *aFirstURI, nsIURI *aSecondURI, bool *_retval) = 0;

  /* boolean isThirdPartyWindow (in mozIDOMWindowProxy aWindow, [optional] in nsIURI aURI); */
  NS_IMETHOD IsThirdPartyWindow(mozIDOMWindowProxy *aWindow, nsIURI *aURI, bool *_retval) = 0;

  /* boolean isThirdPartyChannel (in nsIChannel aChannel, [optional] in nsIURI aURI); */
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *aChannel, nsIURI *aURI, bool *_retval) = 0;

  /* AUTF8String getBaseDomain (in nsIURI aHostURI); */
  NS_IMETHOD GetBaseDomain(nsIURI *aHostURI, nsACString& _retval) = 0;

  /* AUTF8String getBaseDomainFromSchemeHost (in AUTF8String aScheme, in AUTF8String aAsciiHost); */
  NS_IMETHOD GetBaseDomainFromSchemeHost(const nsACString& aScheme, const nsACString& aAsciiHost, nsACString& _retval) = 0;

  /* nsIURI getURIFromWindow (in mozIDOMWindowProxy aWindow); */
  NS_IMETHOD GetURIFromWindow(mozIDOMWindowProxy *aWindow, nsIURI **_retval) = 0;

  /* nsIPrincipal getPrincipalFromWindow (in mozIDOMWindowProxy aWindow); */
  NS_IMETHOD GetPrincipalFromWindow(mozIDOMWindowProxy *aWindow, nsIPrincipal **_retval) = 0;

  /* [noscript] mozIDOMWindowProxy getTopWindowForChannel (in nsIChannel aChannel, [optional] in nsIURI aURIBeingLoaded); */
  NS_IMETHOD GetTopWindowForChannel(nsIChannel *aChannel, nsIURI *aURIBeingLoaded, mozIDOMWindowProxy **_retval) = 0;

  /* [noscript,notxpcom] ThirdPartyAnalysisResult analyzeChannel (in nsIChannel aChannel, in boolean aNotify, [optional] in nsIURI aURI, [optional] in RequireThirdPartyCheck aRequireThirdPartyCheck, out uint32_t aRejectedReason); */
  NS_IMETHOD_(ThirdPartyAnalysisResult) AnalyzeChannel(nsIChannel *aChannel, bool aNotify, nsIURI *aURI, RequireThirdPartyCheck aRequireThirdPartyCheck, uint32_t *aRejectedReason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIThirdPartyUtil, MOZITHIRDPARTYUTIL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZITHIRDPARTYUTIL \
  NS_IMETHOD IsThirdPartyURI(nsIURI *aFirstURI, nsIURI *aSecondURI, bool *_retval) override; \
  NS_IMETHOD IsThirdPartyWindow(mozIDOMWindowProxy *aWindow, nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *aChannel, nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD GetBaseDomain(nsIURI *aHostURI, nsACString& _retval) override; \
  NS_IMETHOD GetBaseDomainFromSchemeHost(const nsACString& aScheme, const nsACString& aAsciiHost, nsACString& _retval) override; \
  NS_IMETHOD GetURIFromWindow(mozIDOMWindowProxy *aWindow, nsIURI **_retval) override; \
  NS_IMETHOD GetPrincipalFromWindow(mozIDOMWindowProxy *aWindow, nsIPrincipal **_retval) override; \
  NS_IMETHOD GetTopWindowForChannel(nsIChannel *aChannel, nsIURI *aURIBeingLoaded, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD_(ThirdPartyAnalysisResult) AnalyzeChannel(nsIChannel *aChannel, bool aNotify, nsIURI *aURI, RequireThirdPartyCheck aRequireThirdPartyCheck, uint32_t *aRejectedReason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZITHIRDPARTYUTIL \
  nsresult IsThirdPartyURI(nsIURI *aFirstURI, nsIURI *aSecondURI, bool *_retval); \
  nsresult IsThirdPartyWindow(mozIDOMWindowProxy *aWindow, nsIURI *aURI, bool *_retval); \
  nsresult IsThirdPartyChannel(nsIChannel *aChannel, nsIURI *aURI, bool *_retval); \
  nsresult GetBaseDomain(nsIURI *aHostURI, nsACString& _retval); \
  nsresult GetBaseDomainFromSchemeHost(const nsACString& aScheme, const nsACString& aAsciiHost, nsACString& _retval); \
  nsresult GetURIFromWindow(mozIDOMWindowProxy *aWindow, nsIURI **_retval); \
  nsresult GetPrincipalFromWindow(mozIDOMWindowProxy *aWindow, nsIPrincipal **_retval); \
  nsresult GetTopWindowForChannel(nsIChannel *aChannel, nsIURI *aURIBeingLoaded, mozIDOMWindowProxy **_retval); \
  nsresult_(ThirdPartyAnalysisResult) AnalyzeChannel(nsIChannel *aChannel, bool aNotify, nsIURI *aURI, RequireThirdPartyCheck aRequireThirdPartyCheck, uint32_t *aRejectedReason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZITHIRDPARTYUTIL(_to) \
  NS_IMETHOD IsThirdPartyURI(nsIURI *aFirstURI, nsIURI *aSecondURI, bool *_retval) override { return _to IsThirdPartyURI(aFirstURI, aSecondURI, _retval); } \
  NS_IMETHOD IsThirdPartyWindow(mozIDOMWindowProxy *aWindow, nsIURI *aURI, bool *_retval) override { return _to IsThirdPartyWindow(aWindow, aURI, _retval); } \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *aChannel, nsIURI *aURI, bool *_retval) override { return _to IsThirdPartyChannel(aChannel, aURI, _retval); } \
  NS_IMETHOD GetBaseDomain(nsIURI *aHostURI, nsACString& _retval) override { return _to GetBaseDomain(aHostURI, _retval); } \
  NS_IMETHOD GetBaseDomainFromSchemeHost(const nsACString& aScheme, const nsACString& aAsciiHost, nsACString& _retval) override { return _to GetBaseDomainFromSchemeHost(aScheme, aAsciiHost, _retval); } \
  NS_IMETHOD GetURIFromWindow(mozIDOMWindowProxy *aWindow, nsIURI **_retval) override { return _to GetURIFromWindow(aWindow, _retval); } \
  NS_IMETHOD GetPrincipalFromWindow(mozIDOMWindowProxy *aWindow, nsIPrincipal **_retval) override { return _to GetPrincipalFromWindow(aWindow, _retval); } \
  NS_IMETHOD GetTopWindowForChannel(nsIChannel *aChannel, nsIURI *aURIBeingLoaded, mozIDOMWindowProxy **_retval) override { return _to GetTopWindowForChannel(aChannel, aURIBeingLoaded, _retval); } \
  NS_IMETHOD_(ThirdPartyAnalysisResult) AnalyzeChannel(nsIChannel *aChannel, bool aNotify, nsIURI *aURI, RequireThirdPartyCheck aRequireThirdPartyCheck, uint32_t *aRejectedReason) override { return _to AnalyzeChannel(aChannel, aNotify, aURI, aRequireThirdPartyCheck, aRejectedReason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZITHIRDPARTYUTIL(_to) \
  NS_IMETHOD IsThirdPartyURI(nsIURI *aFirstURI, nsIURI *aSecondURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyURI(aFirstURI, aSecondURI, _retval); } \
  NS_IMETHOD IsThirdPartyWindow(mozIDOMWindowProxy *aWindow, nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyWindow(aWindow, aURI, _retval); } \
  NS_IMETHOD IsThirdPartyChannel(nsIChannel *aChannel, nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyChannel(aChannel, aURI, _retval); } \
  NS_IMETHOD GetBaseDomain(nsIURI *aHostURI, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseDomain(aHostURI, _retval); } \
  NS_IMETHOD GetBaseDomainFromSchemeHost(const nsACString& aScheme, const nsACString& aAsciiHost, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseDomainFromSchemeHost(aScheme, aAsciiHost, _retval); } \
  NS_IMETHOD GetURIFromWindow(mozIDOMWindowProxy *aWindow, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURIFromWindow(aWindow, _retval); } \
  NS_IMETHOD GetPrincipalFromWindow(mozIDOMWindowProxy *aWindow, nsIPrincipal **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipalFromWindow(aWindow, _retval); } \
  NS_IMETHOD GetTopWindowForChannel(nsIChannel *aChannel, nsIURI *aURIBeingLoaded, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTopWindowForChannel(aChannel, aURIBeingLoaded, _retval); } \
  NS_IMETHOD_(ThirdPartyAnalysisResult) AnalyzeChannel(nsIChannel *aChannel, bool aNotify, nsIURI *aURI, RequireThirdPartyCheck aRequireThirdPartyCheck, uint32_t *aRejectedReason) override; 

/**
 * The mozIThirdPartyUtil implementation is an XPCOM service registered
 * under the ContractID:
 */
#define THIRDPARTYUTIL_CONTRACTID "@mozilla.org/thirdpartyutil;1"

#endif /* __gen_mozIThirdPartyUtil_h__ */
