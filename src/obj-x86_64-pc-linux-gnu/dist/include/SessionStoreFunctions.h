/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/sessionstore/SessionStoreFunctions.idl
 */

#ifndef __gen_SessionStoreFunctions_h__
#define __gen_SessionStoreFunctions_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class BrowsingContext; /* webidl BrowsingContext */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsISessionStoreFunctions */
#define NS_ISESSIONSTOREFUNCTIONS_IID_STR "1a060fba-a19d-11e9-b7eb-580d0edd8e6f"

#define NS_ISESSIONSTOREFUNCTIONS_IID \
  {0x1a060fba, 0xa19d, 0x11e9, \
    { 0xb7, 0xeb, 0x58, 0x0d, 0x0e, 0xdd, 0x8e, 0x6f }}

class NS_NO_VTABLE nsISessionStoreFunctions : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISESSIONSTOREFUNCTIONS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISessionStoreFunctions;

  /* void UpdateSessionStore (in Element aBrowser, in BrowsingContext aBrowsingContext, in uint32_t aFlushId, in boolean aIsFinal, in uint32_t aEpoch, in jsval aData, in boolean aCollectSHistory); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD UpdateSessionStore(mozilla::dom::Element *aBrowser, mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t aFlushId, bool aIsFinal, uint32_t aEpoch, JS::HandleValue aData, bool aCollectSHistory) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISessionStoreFunctions, NS_ISESSIONSTOREFUNCTIONS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISESSIONSTOREFUNCTIONS \
  NS_IMETHOD UpdateSessionStore(mozilla::dom::Element *aBrowser, mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t aFlushId, bool aIsFinal, uint32_t aEpoch, JS::HandleValue aData, bool aCollectSHistory) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISESSIONSTOREFUNCTIONS \
  nsresult UpdateSessionStore(mozilla::dom::Element *aBrowser, mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t aFlushId, bool aIsFinal, uint32_t aEpoch, JS::HandleValue aData, bool aCollectSHistory); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISESSIONSTOREFUNCTIONS(_to) \
  NS_IMETHOD UpdateSessionStore(mozilla::dom::Element *aBrowser, mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t aFlushId, bool aIsFinal, uint32_t aEpoch, JS::HandleValue aData, bool aCollectSHistory) override { return _to UpdateSessionStore(aBrowser, aBrowsingContext, aFlushId, aIsFinal, aEpoch, aData, aCollectSHistory); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISESSIONSTOREFUNCTIONS(_to) \
  NS_IMETHOD UpdateSessionStore(mozilla::dom::Element *aBrowser, mozilla::dom::BrowsingContext *aBrowsingContext, uint32_t aFlushId, bool aIsFinal, uint32_t aEpoch, JS::HandleValue aData, bool aCollectSHistory) override { return !_to ? NS_ERROR_NULL_POINTER : _to->UpdateSessionStore(aBrowser, aBrowsingContext, aFlushId, aIsFinal, aEpoch, aData, aCollectSHistory); } 


#endif /* __gen_SessionStoreFunctions_h__ */
