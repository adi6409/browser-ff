/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileUnlocker.idl
 */

#ifndef __gen_nsIProfileUnlocker_h__
#define __gen_nsIProfileUnlocker_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIProfileUnlocker */
#define NS_IPROFILEUNLOCKER_IID_STR "08923af1-e7a3-4fae-ba02-128502193994"

#define NS_IPROFILEUNLOCKER_IID \
  {0x08923af1, 0xe7a3, 0x4fae, \
    { 0xba, 0x02, 0x12, 0x85, 0x02, 0x19, 0x39, 0x94 }}

class NS_NO_VTABLE nsIProfileUnlocker : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROFILEUNLOCKER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProfileUnlocker;

  enum {
    ATTEMPT_QUIT = 0U,
    FORCE_QUIT = 1U
  };

  /* void unlock (in unsigned long aSeverity); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Unlock(uint32_t aSeverity) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProfileUnlocker, NS_IPROFILEUNLOCKER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROFILEUNLOCKER \
  NS_IMETHOD Unlock(uint32_t aSeverity) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROFILEUNLOCKER \
  nsresult Unlock(uint32_t aSeverity); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROFILEUNLOCKER(_to) \
  NS_IMETHOD Unlock(uint32_t aSeverity) override { return _to Unlock(aSeverity); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROFILEUNLOCKER(_to) \
  NS_IMETHOD Unlock(uint32_t aSeverity) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unlock(aSeverity); } 


#endif /* __gen_nsIProfileUnlocker_h__ */
