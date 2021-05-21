/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIPurgeTrackerService.idl
 */

#ifndef __gen_nsIPurgeTrackerService_h__
#define __gen_nsIPurgeTrackerService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPurgeTrackerService */
#define NS_IPURGETRACKERSERVICE_IID_STR "cd68d61e-9a44-402d-9671-838ac0872176"

#define NS_IPURGETRACKERSERVICE_IID \
  {0xcd68d61e, 0x9a44, 0x402d, \
    { 0x96, 0x71, 0x83, 0x8a, 0xc0, 0x87, 0x21, 0x76 }}

class NS_NO_VTABLE nsIPurgeTrackerService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPURGETRACKERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPurgeTrackerService;

  /* Promise purgeTrackingCookieJars (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PurgeTrackingCookieJars(::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPurgeTrackerService, NS_IPURGETRACKERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPURGETRACKERSERVICE \
  NS_IMETHOD PurgeTrackingCookieJars(::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPURGETRACKERSERVICE \
  nsresult PurgeTrackingCookieJars(::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPURGETRACKERSERVICE(_to) \
  NS_IMETHOD PurgeTrackingCookieJars(::mozilla::dom::Promise * * _retval) override { return _to PurgeTrackingCookieJars(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPURGETRACKERSERVICE(_to) \
  NS_IMETHOD PurgeTrackingCookieJars(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PurgeTrackingCookieJars(_retval); } 


#endif /* __gen_nsIPurgeTrackerService_h__ */
