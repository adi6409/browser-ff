/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIInterruptible.idl
 */

#ifndef __gen_mozIInterruptible_h__
#define __gen_mozIInterruptible_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIInterruptible */
#define MOZIINTERRUPTIBLE_IID_STR "1c06bfd3-76b1-46fa-a64a-db682d478374"

#define MOZIINTERRUPTIBLE_IID \
  {0x1c06bfd3, 0x76b1, 0x46fa, \
    { 0xa6, 0x4a, 0xdb, 0x68, 0x2d, 0x47, 0x83, 0x74 }}

class NS_NO_VTABLE mozIInterruptible : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIINTERRUPTIBLE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIInterruptible;

  /* void interrupt (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Interrupt(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIInterruptible, MOZIINTERRUPTIBLE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIINTERRUPTIBLE \
  NS_IMETHOD Interrupt(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIINTERRUPTIBLE \
  nsresult Interrupt(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIINTERRUPTIBLE(_to) \
  NS_IMETHOD Interrupt(void) override { return _to Interrupt(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIINTERRUPTIBLE(_to) \
  NS_IMETHOD Interrupt(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Interrupt(); } 


#endif /* __gen_mozIInterruptible_h__ */
