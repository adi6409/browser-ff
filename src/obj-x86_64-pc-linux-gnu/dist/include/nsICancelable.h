/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICancelable.idl
 */

#ifndef __gen_nsICancelable_h__
#define __gen_nsICancelable_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICancelable */
#define NS_ICANCELABLE_IID_STR "d94ac0a0-bb18-46b8-844e-84159064b0bd"

#define NS_ICANCELABLE_IID \
  {0xd94ac0a0, 0xbb18, 0x46b8, \
    { 0x84, 0x4e, 0x84, 0x15, 0x90, 0x64, 0xb0, 0xbd }}

class NS_NO_VTABLE nsICancelable : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICANCELABLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICancelable;

  /* void cancel (in nsresult aReason); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(nsresult aReason) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICancelable, NS_ICANCELABLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICANCELABLE \
  NS_IMETHOD Cancel(nsresult aReason) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICANCELABLE \
  nsresult Cancel(nsresult aReason); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICANCELABLE(_to) \
  NS_IMETHOD Cancel(nsresult aReason) override { return _to Cancel(aReason); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICANCELABLE(_to) \
  NS_IMETHOD Cancel(nsresult aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(aReason); } 


#endif /* __gen_nsICancelable_h__ */
