/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIRefreshURI.idl
 */

#ifndef __gen_nsIRefreshURI_h__
#define __gen_nsIRefreshURI_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIRefreshURI */
#define NS_IREFRESHURI_IID_STR "a5e61a3c-51bd-45be-ac0c-e87b71860656"

#define NS_IREFRESHURI_IID \
  {0xa5e61a3c, 0x51bd, 0x45be, \
    { 0xac, 0x0c, 0xe8, 0x7b, 0x71, 0x86, 0x06, 0x56 }}

class NS_NO_VTABLE nsIRefreshURI : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREFRESHURI_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIRefreshURI;

  /* void refreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aRepeat, bool aMetaRefresh) = 0;

  /* void forceRefreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aMetaRefresh); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ForceRefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aMetaRefresh) = 0;

  /* void setupRefreshURI (in nsIChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetupRefreshURI(nsIChannel *aChannel) = 0;

  /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in unsigned long long aInnerWindowID, in ACString aHeader); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetupRefreshURIFromHeader(nsIURI *aBaseURI, nsIPrincipal *principal, uint64_t aInnerWindowID, const nsACString& aHeader) = 0;

  /* void cancelRefreshURITimers (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CancelRefreshURITimers(void) = 0;

  /* readonly attribute boolean refreshPending; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRefreshPending(bool *aRefreshPending) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIRefreshURI, NS_IREFRESHURI_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREFRESHURI \
  NS_IMETHOD RefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aRepeat, bool aMetaRefresh) override; \
  NS_IMETHOD ForceRefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aMetaRefresh) override; \
  NS_IMETHOD SetupRefreshURI(nsIChannel *aChannel) override; \
  NS_IMETHOD SetupRefreshURIFromHeader(nsIURI *aBaseURI, nsIPrincipal *principal, uint64_t aInnerWindowID, const nsACString& aHeader) override; \
  NS_IMETHOD CancelRefreshURITimers(void) override; \
  NS_IMETHOD GetRefreshPending(bool *aRefreshPending) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREFRESHURI \
  nsresult RefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aRepeat, bool aMetaRefresh); \
  nsresult ForceRefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aMetaRefresh); \
  nsresult SetupRefreshURI(nsIChannel *aChannel); \
  nsresult SetupRefreshURIFromHeader(nsIURI *aBaseURI, nsIPrincipal *principal, uint64_t aInnerWindowID, const nsACString& aHeader); \
  nsresult CancelRefreshURITimers(void); \
  nsresult GetRefreshPending(bool *aRefreshPending); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREFRESHURI(_to) \
  NS_IMETHOD RefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aRepeat, bool aMetaRefresh) override { return _to RefreshURI(aURI, aPrincipal, aMillis, aRepeat, aMetaRefresh); } \
  NS_IMETHOD ForceRefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aMetaRefresh) override { return _to ForceRefreshURI(aURI, aPrincipal, aMillis, aMetaRefresh); } \
  NS_IMETHOD SetupRefreshURI(nsIChannel *aChannel) override { return _to SetupRefreshURI(aChannel); } \
  NS_IMETHOD SetupRefreshURIFromHeader(nsIURI *aBaseURI, nsIPrincipal *principal, uint64_t aInnerWindowID, const nsACString& aHeader) override { return _to SetupRefreshURIFromHeader(aBaseURI, principal, aInnerWindowID, aHeader); } \
  NS_IMETHOD CancelRefreshURITimers(void) override { return _to CancelRefreshURITimers(); } \
  NS_IMETHOD GetRefreshPending(bool *aRefreshPending) override { return _to GetRefreshPending(aRefreshPending); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREFRESHURI(_to) \
  NS_IMETHOD RefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aRepeat, bool aMetaRefresh) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RefreshURI(aURI, aPrincipal, aMillis, aRepeat, aMetaRefresh); } \
  NS_IMETHOD ForceRefreshURI(nsIURI *aURI, nsIPrincipal *aPrincipal, int32_t aMillis, bool aMetaRefresh) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForceRefreshURI(aURI, aPrincipal, aMillis, aMetaRefresh); } \
  NS_IMETHOD SetupRefreshURI(nsIChannel *aChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetupRefreshURI(aChannel); } \
  NS_IMETHOD SetupRefreshURIFromHeader(nsIURI *aBaseURI, nsIPrincipal *principal, uint64_t aInnerWindowID, const nsACString& aHeader) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetupRefreshURIFromHeader(aBaseURI, principal, aInnerWindowID, aHeader); } \
  NS_IMETHOD CancelRefreshURITimers(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelRefreshURITimers(); } \
  NS_IMETHOD GetRefreshPending(bool *aRefreshPending) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRefreshPending(aRefreshPending); } 


#endif /* __gen_nsIRefreshURI_h__ */
