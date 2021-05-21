/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIWakeLock.idl
 */

#ifndef __gen_nsIWakeLock_h__
#define __gen_nsIWakeLock_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWakeLock */
#define NS_IWAKELOCK_IID_STR "e27e57ce-fa63-4035-b9ef-27c5dc0cc3ae"

#define NS_IWAKELOCK_IID \
  {0xe27e57ce, 0xfa63, 0x4035, \
    { 0xb9, 0xef, 0x27, 0xc5, 0xdc, 0x0c, 0xc3, 0xae }}

class NS_NO_VTABLE nsIWakeLock : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWAKELOCK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWakeLock;

  /* void unlock (); */
  NS_IMETHOD Unlock(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWakeLock, NS_IWAKELOCK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWAKELOCK \
  NS_IMETHOD Unlock(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWAKELOCK \
  nsresult Unlock(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWAKELOCK(_to) \
  NS_IMETHOD Unlock(void) override { return _to Unlock(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWAKELOCK(_to) \
  NS_IMETHOD Unlock(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Unlock(); } 


#endif /* __gen_nsIWakeLock_h__ */
