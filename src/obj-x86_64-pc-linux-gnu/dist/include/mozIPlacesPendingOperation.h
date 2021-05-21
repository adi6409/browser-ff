/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIPlacesPendingOperation.idl
 */

#ifndef __gen_mozIPlacesPendingOperation_h__
#define __gen_mozIPlacesPendingOperation_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIPlacesPendingOperation */
#define MOZIPLACESPENDINGOPERATION_IID_STR "ebd31374-3808-40e4-9e73-303bf70467c3"

#define MOZIPLACESPENDINGOPERATION_IID \
  {0xebd31374, 0x3808, 0x40e4, \
    { 0x9e, 0x73, 0x30, 0x3b, 0xf7, 0x04, 0x67, 0xc3 }}

class NS_NO_VTABLE mozIPlacesPendingOperation : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIPLACESPENDINGOPERATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIPlacesPendingOperation;

  /* void cancel (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Cancel(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIPlacesPendingOperation, MOZIPLACESPENDINGOPERATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIPLACESPENDINGOPERATION \
  NS_IMETHOD Cancel(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIPLACESPENDINGOPERATION \
  nsresult Cancel(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIPLACESPENDINGOPERATION(_to) \
  NS_IMETHOD Cancel(void) override { return _to Cancel(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIPLACESPENDINGOPERATION(_to) \
  NS_IMETHOD Cancel(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Cancel(); } 


#endif /* __gen_mozIPlacesPendingOperation_h__ */
