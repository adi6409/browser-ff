/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowser.idl
 */

#ifndef __gen_nsIBrowser_h__
#define __gen_nsIBrowser_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIContentSecurityPolicy; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsITransportSecurityInfo; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIWebProgress; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */

class nsIOpenWindowInfo; /* forward declaration */


/* starting interface:    nsIBrowser */
#define NS_IBROWSER_IID_STR "14e5a0cb-e223-4202-95e8-fe53275193ea"

#define NS_IBROWSER_IID \
  {0x14e5a0cb, 0xe223, 0x4202, \
    { 0x95, 0xe8, 0xfe, 0x53, 0x27, 0x51, 0x93, 0xea }}

class NS_NO_VTABLE nsIBrowser : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowser;

  /* void dropLinks (in Array<AString> links, in nsIPrincipal triggeringPrincipal); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DropLinks(const nsTArray<nsString >& links, nsIPrincipal *triggeringPrincipal) = 0;

  /* void swapBrowsers (in nsIBrowser aOtherBrowser); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SwapBrowsers(nsIBrowser *aOtherBrowser) = 0;

  /* void closeBrowser (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CloseBrowser(void) = 0;

  /* readonly attribute boolean isRemoteBrowser; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsRemoteBrowser(bool *aIsRemoteBrowser) = 0;

  /* readonly attribute nsIPrincipal contentPrincipal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentPrincipal(nsIPrincipal **aContentPrincipal) = 0;

  /* readonly attribute nsIPrincipal contentPartitionedPrincipal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentPartitionedPrincipal(nsIPrincipal **aContentPartitionedPrincipal) = 0;

  /* readonly attribute nsIContentSecurityPolicy csp; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) = 0;

  /* readonly attribute nsIReferrerInfo referrerInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;

  /* attribute boolean isNavigating; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsNavigating(bool *aIsNavigating) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetIsNavigating(bool aIsNavigating) = 0;

  /* attribute boolean mayEnableCharacterEncodingMenu; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMayEnableCharacterEncodingMenu(bool aMayEnableCharacterEncodingMenu) = 0;

  /* attribute boolean charsetAutodetected; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCharsetAutodetected(bool aCharsetAutodetected) = 0;

  /* void updateForStateChange (in AString aCharset, in nsIURI aDocumentURI, in AString aContentType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateForStateChange(const nsAString& aCharset, nsIURI *aDocumentURI, const nsAString& aContentType) = 0;

  /* void updateWebNavigationForLocationChange (in boolean aCanGoBack, in boolean aCanGoForward); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateWebNavigationForLocationChange(bool aCanGoBack, bool aCanGoForward) = 0;

  /* void updateForLocationChange (in nsIURI aLocation, in AString aCharset, in boolean aMayEnableCharacterEncodingMenu, in boolean aCharsetAutodetected, in nsIURI aDocumentURI, in AString aTitle, in nsIPrincipal aContentPrincipal, in nsIPrincipal aContentPartitionedPrincipal, in nsIContentSecurityPolicy aCSP, in nsIReferrerInfo aReferrerInfo, in boolean aIsSynthetic, in boolean aHasRequestContextID, in uint64_t aRequestContextID, in AString aContentType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateForLocationChange(nsIURI *aLocation, const nsAString& aCharset, bool aMayEnableCharacterEncodingMenu, bool aCharsetAutodetected, nsIURI *aDocumentURI, const nsAString& aTitle, nsIPrincipal *aContentPrincipal, nsIPrincipal *aContentPartitionedPrincipal, nsIContentSecurityPolicy *aCSP, nsIReferrerInfo *aReferrerInfo, bool aIsSynthetic, bool aHasRequestContextID, uint64_t aRequestContextID, const nsAString& aContentType) = 0;

  enum ProcessBehavior : uint8_t {
    PROCESS_BEHAVIOR_DISABLED = 0,
    PROCESS_BEHAVIOR_STANDARD = 1,
    PROCESS_BEHAVIOR_SUBFRAME_ONLY = 2,
  };

  /* readonly attribute nsIBrowser_ProcessBehavior processSwitchBehavior; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessSwitchBehavior(nsIBrowser::ProcessBehavior *aProcessSwitchBehavior) = 0;

  /* Promise prepareToChangeRemoteness (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PrepareToChangeRemoteness(::mozilla::dom::Promise * * _retval) = 0;

  /* void beforeChangeRemoteness (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD BeforeChangeRemoteness(void) = 0;

  /* bool finishChangeRemoteness (in uint64_t aPendingSwitchId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FinishChangeRemoteness(uint64_t aPendingSwitchId, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowser, NS_IBROWSER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSER \
  NS_IMETHOD DropLinks(const nsTArray<nsString >& links, nsIPrincipal *triggeringPrincipal) override; \
  NS_IMETHOD SwapBrowsers(nsIBrowser *aOtherBrowser) override; \
  NS_IMETHOD CloseBrowser(void) override; \
  NS_IMETHOD GetIsRemoteBrowser(bool *aIsRemoteBrowser) override; \
  NS_IMETHOD GetContentPrincipal(nsIPrincipal **aContentPrincipal) override; \
  NS_IMETHOD GetContentPartitionedPrincipal(nsIPrincipal **aContentPartitionedPrincipal) override; \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override; \
  NS_IMETHOD SetIsNavigating(bool aIsNavigating) override; \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override; \
  NS_IMETHOD SetMayEnableCharacterEncodingMenu(bool aMayEnableCharacterEncodingMenu) override; \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override; \
  NS_IMETHOD SetCharsetAutodetected(bool aCharsetAutodetected) override; \
  NS_IMETHOD UpdateForStateChange(const nsAString& aCharset, nsIURI *aDocumentURI, const nsAString& aContentType) override; \
  NS_IMETHOD UpdateWebNavigationForLocationChange(bool aCanGoBack, bool aCanGoForward) override; \
  NS_IMETHOD UpdateForLocationChange(nsIURI *aLocation, const nsAString& aCharset, bool aMayEnableCharacterEncodingMenu, bool aCharsetAutodetected, nsIURI *aDocumentURI, const nsAString& aTitle, nsIPrincipal *aContentPrincipal, nsIPrincipal *aContentPartitionedPrincipal, nsIContentSecurityPolicy *aCSP, nsIReferrerInfo *aReferrerInfo, bool aIsSynthetic, bool aHasRequestContextID, uint64_t aRequestContextID, const nsAString& aContentType) override; \
  NS_IMETHOD GetProcessSwitchBehavior(nsIBrowser::ProcessBehavior *aProcessSwitchBehavior) override; \
  NS_IMETHOD PrepareToChangeRemoteness(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD BeforeChangeRemoteness(void) override; \
  NS_IMETHOD FinishChangeRemoteness(uint64_t aPendingSwitchId, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSER \
  nsresult DropLinks(const nsTArray<nsString >& links, nsIPrincipal *triggeringPrincipal); \
  nsresult SwapBrowsers(nsIBrowser *aOtherBrowser); \
  nsresult CloseBrowser(void); \
  nsresult GetIsRemoteBrowser(bool *aIsRemoteBrowser); \
  nsresult GetContentPrincipal(nsIPrincipal **aContentPrincipal); \
  nsresult GetContentPartitionedPrincipal(nsIPrincipal **aContentPartitionedPrincipal); \
  nsresult GetCsp(nsIContentSecurityPolicy **aCsp); \
  nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  nsresult GetIsNavigating(bool *aIsNavigating); \
  nsresult SetIsNavigating(bool aIsNavigating); \
  nsresult GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu); \
  nsresult SetMayEnableCharacterEncodingMenu(bool aMayEnableCharacterEncodingMenu); \
  nsresult GetCharsetAutodetected(bool *aCharsetAutodetected); \
  nsresult SetCharsetAutodetected(bool aCharsetAutodetected); \
  nsresult UpdateForStateChange(const nsAString& aCharset, nsIURI *aDocumentURI, const nsAString& aContentType); \
  nsresult UpdateWebNavigationForLocationChange(bool aCanGoBack, bool aCanGoForward); \
  nsresult UpdateForLocationChange(nsIURI *aLocation, const nsAString& aCharset, bool aMayEnableCharacterEncodingMenu, bool aCharsetAutodetected, nsIURI *aDocumentURI, const nsAString& aTitle, nsIPrincipal *aContentPrincipal, nsIPrincipal *aContentPartitionedPrincipal, nsIContentSecurityPolicy *aCSP, nsIReferrerInfo *aReferrerInfo, bool aIsSynthetic, bool aHasRequestContextID, uint64_t aRequestContextID, const nsAString& aContentType); \
  nsresult GetProcessSwitchBehavior(nsIBrowser::ProcessBehavior *aProcessSwitchBehavior); \
  nsresult PrepareToChangeRemoteness(::mozilla::dom::Promise * * _retval); \
  nsresult BeforeChangeRemoteness(void); \
  nsresult FinishChangeRemoteness(uint64_t aPendingSwitchId, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSER(_to) \
  NS_IMETHOD DropLinks(const nsTArray<nsString >& links, nsIPrincipal *triggeringPrincipal) override { return _to DropLinks(links, triggeringPrincipal); } \
  NS_IMETHOD SwapBrowsers(nsIBrowser *aOtherBrowser) override { return _to SwapBrowsers(aOtherBrowser); } \
  NS_IMETHOD CloseBrowser(void) override { return _to CloseBrowser(); } \
  NS_IMETHOD GetIsRemoteBrowser(bool *aIsRemoteBrowser) override { return _to GetIsRemoteBrowser(aIsRemoteBrowser); } \
  NS_IMETHOD GetContentPrincipal(nsIPrincipal **aContentPrincipal) override { return _to GetContentPrincipal(aContentPrincipal); } \
  NS_IMETHOD GetContentPartitionedPrincipal(nsIPrincipal **aContentPartitionedPrincipal) override { return _to GetContentPartitionedPrincipal(aContentPartitionedPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return _to GetCsp(aCsp); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override { return _to GetIsNavigating(aIsNavigating); } \
  NS_IMETHOD SetIsNavigating(bool aIsNavigating) override { return _to SetIsNavigating(aIsNavigating); } \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override { return _to GetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  NS_IMETHOD SetMayEnableCharacterEncodingMenu(bool aMayEnableCharacterEncodingMenu) override { return _to SetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override { return _to GetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD SetCharsetAutodetected(bool aCharsetAutodetected) override { return _to SetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD UpdateForStateChange(const nsAString& aCharset, nsIURI *aDocumentURI, const nsAString& aContentType) override { return _to UpdateForStateChange(aCharset, aDocumentURI, aContentType); } \
  NS_IMETHOD UpdateWebNavigationForLocationChange(bool aCanGoBack, bool aCanGoForward) override { return _to UpdateWebNavigationForLocationChange(aCanGoBack, aCanGoForward); } \
  NS_IMETHOD UpdateForLocationChange(nsIURI *aLocation, const nsAString& aCharset, bool aMayEnableCharacterEncodingMenu, bool aCharsetAutodetected, nsIURI *aDocumentURI, const nsAString& aTitle, nsIPrincipal *aContentPrincipal, nsIPrincipal *aContentPartitionedPrincipal, nsIContentSecurityPolicy *aCSP, nsIReferrerInfo *aReferrerInfo, bool aIsSynthetic, bool aHasRequestContextID, uint64_t aRequestContextID, const nsAString& aContentType) override { return _to UpdateForLocationChange(aLocation, aCharset, aMayEnableCharacterEncodingMenu, aCharsetAutodetected, aDocumentURI, aTitle, aContentPrincipal, aContentPartitionedPrincipal, aCSP, aReferrerInfo, aIsSynthetic, aHasRequestContextID, aRequestContextID, aContentType); } \
  NS_IMETHOD GetProcessSwitchBehavior(nsIBrowser::ProcessBehavior *aProcessSwitchBehavior) override { return _to GetProcessSwitchBehavior(aProcessSwitchBehavior); } \
  NS_IMETHOD PrepareToChangeRemoteness(::mozilla::dom::Promise * * _retval) override { return _to PrepareToChangeRemoteness(_retval); } \
  NS_IMETHOD BeforeChangeRemoteness(void) override { return _to BeforeChangeRemoteness(); } \
  NS_IMETHOD FinishChangeRemoteness(uint64_t aPendingSwitchId, bool *_retval) override { return _to FinishChangeRemoteness(aPendingSwitchId, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSER(_to) \
  NS_IMETHOD DropLinks(const nsTArray<nsString >& links, nsIPrincipal *triggeringPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DropLinks(links, triggeringPrincipal); } \
  NS_IMETHOD SwapBrowsers(nsIBrowser *aOtherBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SwapBrowsers(aOtherBrowser); } \
  NS_IMETHOD CloseBrowser(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CloseBrowser(); } \
  NS_IMETHOD GetIsRemoteBrowser(bool *aIsRemoteBrowser) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsRemoteBrowser(aIsRemoteBrowser); } \
  NS_IMETHOD GetContentPrincipal(nsIPrincipal **aContentPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentPrincipal(aContentPrincipal); } \
  NS_IMETHOD GetContentPartitionedPrincipal(nsIPrincipal **aContentPartitionedPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentPartitionedPrincipal(aContentPartitionedPrincipal); } \
  NS_IMETHOD GetCsp(nsIContentSecurityPolicy **aCsp) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCsp(aCsp); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetIsNavigating(bool *aIsNavigating) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsNavigating(aIsNavigating); } \
  NS_IMETHOD SetIsNavigating(bool aIsNavigating) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetIsNavigating(aIsNavigating); } \
  NS_IMETHOD GetMayEnableCharacterEncodingMenu(bool *aMayEnableCharacterEncodingMenu) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  NS_IMETHOD SetMayEnableCharacterEncodingMenu(bool aMayEnableCharacterEncodingMenu) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMayEnableCharacterEncodingMenu(aMayEnableCharacterEncodingMenu); } \
  NS_IMETHOD GetCharsetAutodetected(bool *aCharsetAutodetected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD SetCharsetAutodetected(bool aCharsetAutodetected) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCharsetAutodetected(aCharsetAutodetected); } \
  NS_IMETHOD UpdateForStateChange(const nsAString& aCharset, nsIURI *aDocumentURI, const nsAString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateForStateChange(aCharset, aDocumentURI, aContentType); } \
  NS_IMETHOD UpdateWebNavigationForLocationChange(bool aCanGoBack, bool aCanGoForward) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateWebNavigationForLocationChange(aCanGoBack, aCanGoForward); } \
  NS_IMETHOD UpdateForLocationChange(nsIURI *aLocation, const nsAString& aCharset, bool aMayEnableCharacterEncodingMenu, bool aCharsetAutodetected, nsIURI *aDocumentURI, const nsAString& aTitle, nsIPrincipal *aContentPrincipal, nsIPrincipal *aContentPartitionedPrincipal, nsIContentSecurityPolicy *aCSP, nsIReferrerInfo *aReferrerInfo, bool aIsSynthetic, bool aHasRequestContextID, uint64_t aRequestContextID, const nsAString& aContentType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateForLocationChange(aLocation, aCharset, aMayEnableCharacterEncodingMenu, aCharsetAutodetected, aDocumentURI, aTitle, aContentPrincipal, aContentPartitionedPrincipal, aCSP, aReferrerInfo, aIsSynthetic, aHasRequestContextID, aRequestContextID, aContentType); } \
  NS_IMETHOD GetProcessSwitchBehavior(nsIBrowser::ProcessBehavior *aProcessSwitchBehavior) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessSwitchBehavior(aProcessSwitchBehavior); } \
  NS_IMETHOD PrepareToChangeRemoteness(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrepareToChangeRemoteness(_retval); } \
  NS_IMETHOD BeforeChangeRemoteness(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->BeforeChangeRemoteness(); } \
  NS_IMETHOD FinishChangeRemoteness(uint64_t aPendingSwitchId, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishChangeRemoteness(aPendingSwitchId, _retval); } 


#endif /* __gen_nsIBrowser_h__ */
