/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThreadRetargetableRequest.idl
 */

#ifndef __gen_nsIThreadRetargetableRequest_h__
#define __gen_nsIThreadRetargetableRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIEventTarget; /* forward declaration */


/* starting interface:    nsIThreadRetargetableRequest */
#define NS_ITHREADRETARGETABLEREQUEST_IID_STR "27b84c48-5a73-4ba4-a8a4-8b5e649a145e"

#define NS_ITHREADRETARGETABLEREQUEST_IID \
  {0x27b84c48, 0x5a73, 0x4ba4, \
    { 0xa8, 0xa4, 0x8b, 0x5e, 0x64, 0x9a, 0x14, 0x5e }}

class NS_NO_VTABLE nsIThreadRetargetableRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITHREADRETARGETABLEREQUEST_IID)

  /* void retargetDeliveryTo (in nsIEventTarget aNewTarget); */
  NS_IMETHOD RetargetDeliveryTo(nsIEventTarget *aNewTarget) = 0;

  /* readonly attribute nsIEventTarget deliveryTarget; */
  NS_IMETHOD GetDeliveryTarget(nsIEventTarget **aDeliveryTarget) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIThreadRetargetableRequest, NS_ITHREADRETARGETABLEREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITHREADRETARGETABLEREQUEST \
  NS_IMETHOD RetargetDeliveryTo(nsIEventTarget *aNewTarget) override; \
  NS_IMETHOD GetDeliveryTarget(nsIEventTarget **aDeliveryTarget) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITHREADRETARGETABLEREQUEST \
  nsresult RetargetDeliveryTo(nsIEventTarget *aNewTarget); \
  nsresult GetDeliveryTarget(nsIEventTarget **aDeliveryTarget); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITHREADRETARGETABLEREQUEST(_to) \
  NS_IMETHOD RetargetDeliveryTo(nsIEventTarget *aNewTarget) override { return _to RetargetDeliveryTo(aNewTarget); } \
  NS_IMETHOD GetDeliveryTarget(nsIEventTarget **aDeliveryTarget) override { return _to GetDeliveryTarget(aDeliveryTarget); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITHREADRETARGETABLEREQUEST(_to) \
  NS_IMETHOD RetargetDeliveryTo(nsIEventTarget *aNewTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RetargetDeliveryTo(aNewTarget); } \
  NS_IMETHOD GetDeliveryTarget(nsIEventTarget **aDeliveryTarget) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDeliveryTarget(aDeliveryTarget); } 


#endif /* __gen_nsIThreadRetargetableRequest_h__ */
