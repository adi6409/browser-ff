/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIWindowWatcher.idl
 */

#ifndef __gen_nsIWindowWatcher_h__
#define __gen_nsIWindowWatcher_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */

class nsIObserver; /* forward declaration */

class nsIPrompt; /* forward declaration */

class nsIAuthPrompt; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */

class nsIWebBrowserChrome; /* forward declaration */

class nsIWindowCreator; /* forward declaration */


/* starting interface:    nsIWindowWatcher */
#define NS_IWINDOWWATCHER_IID_STR "641fe945-6902-4b3f-87c2-0daef32499b3"

#define NS_IWINDOWWATCHER_IID \
  {0x641fe945, 0x6902, 0x4b3f, \
    { 0x87, 0xc2, 0x0d, 0xae, 0xf3, 0x24, 0x99, 0xb3 }}

class NS_NO_VTABLE nsIWindowWatcher : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWWATCHER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowWatcher;

  /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in nsISupports aArguments); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OpenWindow(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, nsISupports *aArguments, mozIDOMWindowProxy **_retval) = 0;

  /* void registerNotification (in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RegisterNotification(nsIObserver *aObserver) = 0;

  /* void unregisterNotification (in nsIObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UnregisterNotification(nsIObserver *aObserver) = 0;

  /* nsISimpleEnumerator getWindowEnumerator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWindowEnumerator(nsISimpleEnumerator **_retval) = 0;

  /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNewPrompter(mozIDOMWindowProxy *aParent, nsIPrompt **_retval) = 0;

  /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNewAuthPrompter(mozIDOMWindowProxy *aParent, nsIAuthPrompt **_retval) = 0;

  /* void setWindowCreator (in nsIWindowCreator creator); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetWindowCreator(nsIWindowCreator *creator) = 0;

  /* boolean hasWindowCreator (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasWindowCreator(bool *_retval) = 0;

  /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetChromeForWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome **_retval) = 0;

  /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWindowByName(const nsAString& aTargetName, mozIDOMWindowProxy *aCurrentWindow, mozIDOMWindowProxy **_retval) = 0;

  /* readonly attribute mozIDOMWindowProxy activeWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowWatcher, NS_IWINDOWWATCHER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWWATCHER \
  NS_IMETHOD OpenWindow(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, nsISupports *aArguments, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD RegisterNotification(nsIObserver *aObserver) override; \
  NS_IMETHOD UnregisterNotification(nsIObserver *aObserver) override; \
  NS_IMETHOD GetWindowEnumerator(nsISimpleEnumerator **_retval) override; \
  NS_IMETHOD GetNewPrompter(mozIDOMWindowProxy *aParent, nsIPrompt **_retval) override; \
  NS_IMETHOD GetNewAuthPrompter(mozIDOMWindowProxy *aParent, nsIAuthPrompt **_retval) override; \
  NS_IMETHOD SetWindowCreator(nsIWindowCreator *creator) override; \
  NS_IMETHOD HasWindowCreator(bool *_retval) override; \
  NS_IMETHOD GetChromeForWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome **_retval) override; \
  NS_IMETHOD GetWindowByName(const nsAString& aTargetName, mozIDOMWindowProxy *aCurrentWindow, mozIDOMWindowProxy **_retval) override; \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWWATCHER \
  nsresult OpenWindow(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, nsISupports *aArguments, mozIDOMWindowProxy **_retval); \
  nsresult RegisterNotification(nsIObserver *aObserver); \
  nsresult UnregisterNotification(nsIObserver *aObserver); \
  nsresult GetWindowEnumerator(nsISimpleEnumerator **_retval); \
  nsresult GetNewPrompter(mozIDOMWindowProxy *aParent, nsIPrompt **_retval); \
  nsresult GetNewAuthPrompter(mozIDOMWindowProxy *aParent, nsIAuthPrompt **_retval); \
  nsresult SetWindowCreator(nsIWindowCreator *creator); \
  nsresult HasWindowCreator(bool *_retval); \
  nsresult GetChromeForWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome **_retval); \
  nsresult GetWindowByName(const nsAString& aTargetName, mozIDOMWindowProxy *aCurrentWindow, mozIDOMWindowProxy **_retval); \
  nsresult GetActiveWindow(mozIDOMWindowProxy **aActiveWindow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWWATCHER(_to) \
  NS_IMETHOD OpenWindow(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, nsISupports *aArguments, mozIDOMWindowProxy **_retval) override { return _to OpenWindow(aParent, aUrl, aName, aFeatures, aArguments, _retval); } \
  NS_IMETHOD RegisterNotification(nsIObserver *aObserver) override { return _to RegisterNotification(aObserver); } \
  NS_IMETHOD UnregisterNotification(nsIObserver *aObserver) override { return _to UnregisterNotification(aObserver); } \
  NS_IMETHOD GetWindowEnumerator(nsISimpleEnumerator **_retval) override { return _to GetWindowEnumerator(_retval); } \
  NS_IMETHOD GetNewPrompter(mozIDOMWindowProxy *aParent, nsIPrompt **_retval) override { return _to GetNewPrompter(aParent, _retval); } \
  NS_IMETHOD GetNewAuthPrompter(mozIDOMWindowProxy *aParent, nsIAuthPrompt **_retval) override { return _to GetNewAuthPrompter(aParent, _retval); } \
  NS_IMETHOD SetWindowCreator(nsIWindowCreator *creator) override { return _to SetWindowCreator(creator); } \
  NS_IMETHOD HasWindowCreator(bool *_retval) override { return _to HasWindowCreator(_retval); } \
  NS_IMETHOD GetChromeForWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome **_retval) override { return _to GetChromeForWindow(aWindow, _retval); } \
  NS_IMETHOD GetWindowByName(const nsAString& aTargetName, mozIDOMWindowProxy *aCurrentWindow, mozIDOMWindowProxy **_retval) override { return _to GetWindowByName(aTargetName, aCurrentWindow, _retval); } \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override { return _to GetActiveWindow(aActiveWindow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWWATCHER(_to) \
  NS_IMETHOD OpenWindow(mozIDOMWindowProxy *aParent, const nsACString& aUrl, const nsACString& aName, const nsACString& aFeatures, nsISupports *aArguments, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenWindow(aParent, aUrl, aName, aFeatures, aArguments, _retval); } \
  NS_IMETHOD RegisterNotification(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RegisterNotification(aObserver); } \
  NS_IMETHOD UnregisterNotification(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UnregisterNotification(aObserver); } \
  NS_IMETHOD GetWindowEnumerator(nsISimpleEnumerator **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowEnumerator(_retval); } \
  NS_IMETHOD GetNewPrompter(mozIDOMWindowProxy *aParent, nsIPrompt **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewPrompter(aParent, _retval); } \
  NS_IMETHOD GetNewAuthPrompter(mozIDOMWindowProxy *aParent, nsIAuthPrompt **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewAuthPrompter(aParent, _retval); } \
  NS_IMETHOD SetWindowCreator(nsIWindowCreator *creator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWindowCreator(creator); } \
  NS_IMETHOD HasWindowCreator(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasWindowCreator(_retval); } \
  NS_IMETHOD GetChromeForWindow(mozIDOMWindowProxy *aWindow, nsIWebBrowserChrome **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChromeForWindow(aWindow, _retval); } \
  NS_IMETHOD GetWindowByName(const nsAString& aTargetName, mozIDOMWindowProxy *aCurrentWindow, mozIDOMWindowProxy **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWindowByName(aTargetName, aCurrentWindow, _retval); } \
  NS_IMETHOD GetActiveWindow(mozIDOMWindowProxy **aActiveWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActiveWindow(aActiveWindow); } 

#define NS_WINDOWWATCHER_CONTRACTID "@mozilla.org/embedcomp/window-watcher;1"

#endif /* __gen_nsIWindowWatcher_h__ */
