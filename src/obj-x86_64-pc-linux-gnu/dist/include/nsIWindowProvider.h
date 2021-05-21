/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowcreator/nsIWindowProvider.idl
 */

#ifndef __gen_nsIWindowProvider_h__
#define __gen_nsIWindowProvider_h__


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

class nsIURI; /* forward declaration */

class nsIOpenWindowInfo; /* forward declaration */


/* starting interface:    nsIWindowProvider */
#define NS_IWINDOWPROVIDER_IID_STR "e97a3830-15ef-499b-8372-c22d128091c1"

#define NS_IWINDOWPROVIDER_IID \
  {0xe97a3830, 0x15ef, 0x499b, \
    { 0x83, 0x72, 0xc2, 0x2d, 0x12, 0x80, 0x91, 0xc1 }}

class NS_NO_VTABLE nsIWindowProvider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWINDOWPROVIDER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWindowProvider;

  /* [noscript] BrowsingContext provideWindow (in nsIOpenWindowInfo aOpenWindowInfo, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aWidthSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsDocShellLoadStatePtr aLoadState, out boolean aWindowIsNew); */
  NS_IMETHOD ProvideWindow(nsIOpenWindowInfo *aOpenWindowInfo, uint32_t aChromeFlags, bool aCalledFromJS, bool aWidthSpecified, nsIURI *aURI, const nsAString& aName, const nsACString& aFeatures, bool aForceNoOpener, bool aForceNoReferrer, nsDocShellLoadState* aLoadState, bool *aWindowIsNew, mozilla::dom::BrowsingContext **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWindowProvider, NS_IWINDOWPROVIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWINDOWPROVIDER \
  NS_IMETHOD ProvideWindow(nsIOpenWindowInfo *aOpenWindowInfo, uint32_t aChromeFlags, bool aCalledFromJS, bool aWidthSpecified, nsIURI *aURI, const nsAString& aName, const nsACString& aFeatures, bool aForceNoOpener, bool aForceNoReferrer, nsDocShellLoadState* aLoadState, bool *aWindowIsNew, mozilla::dom::BrowsingContext **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWINDOWPROVIDER \
  nsresult ProvideWindow(nsIOpenWindowInfo *aOpenWindowInfo, uint32_t aChromeFlags, bool aCalledFromJS, bool aWidthSpecified, nsIURI *aURI, const nsAString& aName, const nsACString& aFeatures, bool aForceNoOpener, bool aForceNoReferrer, nsDocShellLoadState* aLoadState, bool *aWindowIsNew, mozilla::dom::BrowsingContext **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWINDOWPROVIDER(_to) \
  NS_IMETHOD ProvideWindow(nsIOpenWindowInfo *aOpenWindowInfo, uint32_t aChromeFlags, bool aCalledFromJS, bool aWidthSpecified, nsIURI *aURI, const nsAString& aName, const nsACString& aFeatures, bool aForceNoOpener, bool aForceNoReferrer, nsDocShellLoadState* aLoadState, bool *aWindowIsNew, mozilla::dom::BrowsingContext **_retval) override { return _to ProvideWindow(aOpenWindowInfo, aChromeFlags, aCalledFromJS, aWidthSpecified, aURI, aName, aFeatures, aForceNoOpener, aForceNoReferrer, aLoadState, aWindowIsNew, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWINDOWPROVIDER(_to) \
  NS_IMETHOD ProvideWindow(nsIOpenWindowInfo *aOpenWindowInfo, uint32_t aChromeFlags, bool aCalledFromJS, bool aWidthSpecified, nsIURI *aURI, const nsAString& aName, const nsACString& aFeatures, bool aForceNoOpener, bool aForceNoReferrer, nsDocShellLoadState* aLoadState, bool *aWindowIsNew, mozilla::dom::BrowsingContext **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProvideWindow(aOpenWindowInfo, aChromeFlags, aCalledFromJS, aWidthSpecified, aURI, aName, aFeatures, aForceNoOpener, aForceNoReferrer, aLoadState, aWindowIsNew, _retval); } 


#endif /* __gen_nsIWindowProvider_h__ */
