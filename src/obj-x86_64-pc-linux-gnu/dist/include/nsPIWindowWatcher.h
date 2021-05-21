/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsPIWindowWatcher.idl
 */

#ifndef __gen_nsPIWindowWatcher_h__
#define __gen_nsPIWindowWatcher_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsDocShellLoadState;
namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

class mozIDOMWindowProxy; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

class nsIWebBrowserChrome; /* forward declaration */

class nsIDocShellTreeItem; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIRemoteTab; /* forward declaration */

class nsIOpenWindowInfo; /* forward declaration */


/* starting interface:    nsPIWindowWatcher */
#define NS_PIWINDOWWATCHER_IID_STR "d162f9c4-19d5-4723-931f-f1e51bfa9f68"

#define NS_PIWINDOWWATCHER_IID \
  {0xd162f9c4, 0x19d5, 0x4723, \
    { 0x93, 0x1f, 0xf1, 0xe5, 0x1b, 0xfa, 0x9f, 0x68 }}

class NS_NO_VTABLE nsPIWindowWatcher : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_PIWINDOWWATCHER_IID)

  /* void addWindow (in mozIDOMWindowProxy aWindow, in nsIWebBrowserChrome aChrome); */
  NS_IMETHOD AddWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome *aChrome) = 0;

  /* void removeWindow (in mozIDOMWindowProxy aWindow); */
  NS_IMETHOD RemoveWindow(mozIDOMWindowProxy *aWindow) = 0;

  enum PrintKind : uint8_t {
    PRINT_NONE = 0,
    PRINT_INTERNAL = 1,
    PRINT_WINDOW_DOT_PRINT = 2,
  };

  /* [noscript] BrowsingContext openWindow2 (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in boolean aCalledFromScript, in boolean aDialog, in boolean aNavigate, in nsISupports aArgs, in boolean aIsPopupSpam, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsPIWindowWatcher_PrintKind aPrintKind, in nsDocShellLoadStatePtr aLoadState); */
  NS_IMETHOD OpenWindow2(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, bool aCalledFromScript, bool aDialog, bool aNavigate, nsISupports *aArgs, bool aIsPopupSpam, bool aForceNoOpener, bool aForceNoReferrer, nsPIWindowWatcher::PrintKind aPrintKind, nsDocShellLoadState* aLoadState, mozilla::dom::BrowsingContext **_retval) = 0;

  /* nsIRemoteTab openWindowWithRemoteTab (in nsIRemoteTab aOpeningTab, in ACString aFeatures, in boolean aCalledFromJS, in float aOpenerFullZoom, in nsIOpenWindowInfo aOpenWindowInfo); */
  NS_IMETHOD OpenWindowWithRemoteTab(nsIRemoteTab *aOpeningTab, const nsACString& aFeatures, bool aCalledFromJS, float aOpenerFullZoom, nsIOpenWindowInfo *aOpenWindowInfo, nsIRemoteTab **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsPIWindowWatcher, NS_PIWINDOWWATCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSPIWINDOWWATCHER \
  NS_IMETHOD AddWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome *aChrome) override; \
  NS_IMETHOD RemoveWindow(mozIDOMWindowProxy *aWindow) override; \
  NS_IMETHOD OpenWindow2(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, bool aCalledFromScript, bool aDialog, bool aNavigate, nsISupports *aArgs, bool aIsPopupSpam, bool aForceNoOpener, bool aForceNoReferrer, nsPIWindowWatcher::PrintKind aPrintKind, nsDocShellLoadState* aLoadState, mozilla::dom::BrowsingContext **_retval) override; \
  NS_IMETHOD OpenWindowWithRemoteTab(nsIRemoteTab *aOpeningTab, const nsACString& aFeatures, bool aCalledFromJS, float aOpenerFullZoom, nsIOpenWindowInfo *aOpenWindowInfo, nsIRemoteTab **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSPIWINDOWWATCHER \
  nsresult AddWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome *aChrome); \
  nsresult RemoveWindow(mozIDOMWindowProxy *aWindow); \
  nsresult OpenWindow2(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, bool aCalledFromScript, bool aDialog, bool aNavigate, nsISupports *aArgs, bool aIsPopupSpam, bool aForceNoOpener, bool aForceNoReferrer, nsPIWindowWatcher::PrintKind aPrintKind, nsDocShellLoadState* aLoadState, mozilla::dom::BrowsingContext **_retval); \
  nsresult OpenWindowWithRemoteTab(nsIRemoteTab *aOpeningTab, const nsACString& aFeatures, bool aCalledFromJS, float aOpenerFullZoom, nsIOpenWindowInfo *aOpenWindowInfo, nsIRemoteTab **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSPIWINDOWWATCHER(_to) \
  NS_IMETHOD AddWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome *aChrome) override { return _to AddWindow(aWindow, aChrome); } \
  NS_IMETHOD RemoveWindow(mozIDOMWindowProxy *aWindow) override { return _to RemoveWindow(aWindow); } \
  NS_IMETHOD OpenWindow2(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, bool aCalledFromScript, bool aDialog, bool aNavigate, nsISupports *aArgs, bool aIsPopupSpam, bool aForceNoOpener, bool aForceNoReferrer, nsPIWindowWatcher::PrintKind aPrintKind, nsDocShellLoadState* aLoadState, mozilla::dom::BrowsingContext **_retval) override { return _to OpenWindow2(aParent, aUrl, aName, aFeatures, aCalledFromScript, aDialog, aNavigate, aArgs, aIsPopupSpam, aForceNoOpener, aForceNoReferrer, aPrintKind, aLoadState, _retval); } \
  NS_IMETHOD OpenWindowWithRemoteTab(nsIRemoteTab *aOpeningTab, const nsACString& aFeatures, bool aCalledFromJS, float aOpenerFullZoom, nsIOpenWindowInfo *aOpenWindowInfo, nsIRemoteTab **_retval) override { return _to OpenWindowWithRemoteTab(aOpeningTab, aFeatures, aCalledFromJS, aOpenerFullZoom, aOpenWindowInfo, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSPIWINDOWWATCHER(_to) \
  NS_IMETHOD AddWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome *aChrome) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddWindow(aWindow, aChrome); } \
  NS_IMETHOD RemoveWindow(mozIDOMWindowProxy *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWindow(aWindow); } \
  NS_IMETHOD OpenWindow2(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, bool aCalledFromScript, bool aDialog, bool aNavigate, nsISupports *aArgs, bool aIsPopupSpam, bool aForceNoOpener, bool aForceNoReferrer, nsPIWindowWatcher::PrintKind aPrintKind, nsDocShellLoadState* aLoadState, mozilla::dom::BrowsingContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenWindow2(aParent, aUrl, aName, aFeatures, aCalledFromScript, aDialog, aNavigate, aArgs, aIsPopupSpam, aForceNoOpener, aForceNoReferrer, aPrintKind, aLoadState, _retval); } \
  NS_IMETHOD OpenWindowWithRemoteTab(nsIRemoteTab *aOpeningTab, const nsACString& aFeatures, bool aCalledFromJS, float aOpenerFullZoom, nsIOpenWindowInfo *aOpenWindowInfo, nsIRemoteTab **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenWindowWithRemoteTab(aOpeningTab, aFeatures, aCalledFromJS, aOpenerFullZoom, aOpenWindowInfo, _retval); } 


#endif /* __gen_nsPIWindowWatcher_h__ */
