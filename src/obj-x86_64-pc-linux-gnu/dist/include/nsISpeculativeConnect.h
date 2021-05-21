/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISpeculativeConnect.idl
 */

#ifndef __gen_nsISpeculativeConnect_h__
#define __gen_nsISpeculativeConnect_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIInterfaceRequestor; /* forward declaration */


/* starting interface:    nsISpeculativeConnect */
#define NS_ISPECULATIVECONNECT_IID_STR "d74a17ac-5b8a-4824-a309-b1f04a3c4aed"

#define NS_ISPECULATIVECONNECT_IID \
  {0xd74a17ac, 0x5b8a, 0x4824, \
    { 0xa3, 0x09, 0xb1, 0xf0, 0x4a, 0x3c, 0x4a, 0xed }}

class NS_NO_VTABLE nsISpeculativeConnect : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPECULATIVECONNECT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISpeculativeConnect;

  /* void speculativeConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpeculativeConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) = 0;

  /* void speculativeAnonymousConnect (in nsIURI aURI, in nsIPrincipal aPrincipal, in nsIInterfaceRequestor aCallbacks); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpeculativeAnonymousConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeculativeConnect, NS_ISPECULATIVECONNECT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPECULATIVECONNECT \
  NS_IMETHOD SpeculativeConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override; \
  NS_IMETHOD SpeculativeAnonymousConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPECULATIVECONNECT \
  nsresult SpeculativeConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks); \
  nsresult SpeculativeAnonymousConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPECULATIVECONNECT(_to) \
  NS_IMETHOD SpeculativeConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override { return _to SpeculativeConnect(aURI, aPrincipal, aCallbacks); } \
  NS_IMETHOD SpeculativeAnonymousConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override { return _to SpeculativeAnonymousConnect(aURI, aPrincipal, aCallbacks); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPECULATIVECONNECT(_to) \
  NS_IMETHOD SpeculativeConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpeculativeConnect(aURI, aPrincipal, aCallbacks); } \
  NS_IMETHOD SpeculativeAnonymousConnect(nsIURI *aURI, nsIPrincipal *aPrincipal, nsIInterfaceRequestor *aCallbacks) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpeculativeAnonymousConnect(aURI, aPrincipal, aCallbacks); } 


/* starting interface:    nsISpeculativeConnectionOverrider */
#define NS_ISPECULATIVECONNECTIONOVERRIDER_IID_STR "1040ebe3-6ed1-45a6-8587-995e082518d7"

#define NS_ISPECULATIVECONNECTIONOVERRIDER_IID \
  {0x1040ebe3, 0x6ed1, 0x45a6, \
    { 0x85, 0x87, 0x99, 0x5e, 0x08, 0x25, 0x18, 0xd7 }}

class NS_NO_VTABLE nsISpeculativeConnectionOverrider : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISPECULATIVECONNECTIONOVERRIDER_IID)

  /* [infallible] readonly attribute unsigned long parallelSpeculativeConnectLimit; */
  NS_IMETHOD GetParallelSpeculativeConnectLimit(uint32_t *aParallelSpeculativeConnectLimit) = 0;
  inline uint32_t  GetParallelSpeculativeConnectLimit()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetParallelSpeculativeConnectLimit(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean ignoreIdle; */
  NS_IMETHOD GetIgnoreIdle(bool *aIgnoreIdle) = 0;
  inline bool  GetIgnoreIdle()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIgnoreIdle(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean isFromPredictor; */
  NS_IMETHOD GetIsFromPredictor(bool *aIsFromPredictor) = 0;
  inline bool  GetIsFromPredictor()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetIsFromPredictor(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute boolean allow1918; */
  NS_IMETHOD GetAllow1918(bool *aAllow1918) = 0;
  inline bool  GetAllow1918()
  {
    bool result;
    mozilla::DebugOnly<nsresult> rv = GetAllow1918(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISpeculativeConnectionOverrider, NS_ISPECULATIVECONNECTIONOVERRIDER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISPECULATIVECONNECTIONOVERRIDER \
  using nsISpeculativeConnectionOverrider::GetParallelSpeculativeConnectLimit; \
  NS_IMETHOD GetParallelSpeculativeConnectLimit(uint32_t *aParallelSpeculativeConnectLimit) override; \
  using nsISpeculativeConnectionOverrider::GetIgnoreIdle; \
  NS_IMETHOD GetIgnoreIdle(bool *aIgnoreIdle) override; \
  using nsISpeculativeConnectionOverrider::GetIsFromPredictor; \
  NS_IMETHOD GetIsFromPredictor(bool *aIsFromPredictor) override; \
  using nsISpeculativeConnectionOverrider::GetAllow1918; \
  NS_IMETHOD GetAllow1918(bool *aAllow1918) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISPECULATIVECONNECTIONOVERRIDER \
  using nsISpeculativeConnectionOverrider::GetParallelSpeculativeConnectLimit; \
  nsresult GetParallelSpeculativeConnectLimit(uint32_t *aParallelSpeculativeConnectLimit); \
  using nsISpeculativeConnectionOverrider::GetIgnoreIdle; \
  nsresult GetIgnoreIdle(bool *aIgnoreIdle); \
  using nsISpeculativeConnectionOverrider::GetIsFromPredictor; \
  nsresult GetIsFromPredictor(bool *aIsFromPredictor); \
  using nsISpeculativeConnectionOverrider::GetAllow1918; \
  nsresult GetAllow1918(bool *aAllow1918); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISPECULATIVECONNECTIONOVERRIDER(_to) \
  using nsISpeculativeConnectionOverrider::GetParallelSpeculativeConnectLimit; \
  NS_IMETHOD GetParallelSpeculativeConnectLimit(uint32_t *aParallelSpeculativeConnectLimit) override { return _to GetParallelSpeculativeConnectLimit(aParallelSpeculativeConnectLimit); } \
  using nsISpeculativeConnectionOverrider::GetIgnoreIdle; \
  NS_IMETHOD GetIgnoreIdle(bool *aIgnoreIdle) override { return _to GetIgnoreIdle(aIgnoreIdle); } \
  using nsISpeculativeConnectionOverrider::GetIsFromPredictor; \
  NS_IMETHOD GetIsFromPredictor(bool *aIsFromPredictor) override { return _to GetIsFromPredictor(aIsFromPredictor); } \
  using nsISpeculativeConnectionOverrider::GetAllow1918; \
  NS_IMETHOD GetAllow1918(bool *aAllow1918) override { return _to GetAllow1918(aAllow1918); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISPECULATIVECONNECTIONOVERRIDER(_to) \
  NS_IMETHOD GetParallelSpeculativeConnectLimit(uint32_t *aParallelSpeculativeConnectLimit) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParallelSpeculativeConnectLimit(aParallelSpeculativeConnectLimit); } \
  NS_IMETHOD GetIgnoreIdle(bool *aIgnoreIdle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIgnoreIdle(aIgnoreIdle); } \
  NS_IMETHOD GetIsFromPredictor(bool *aIsFromPredictor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsFromPredictor(aIsFromPredictor); } \
  NS_IMETHOD GetAllow1918(bool *aAllow1918) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllow1918(aAllow1918); } 


#endif /* __gen_nsISpeculativeConnect_h__ */
