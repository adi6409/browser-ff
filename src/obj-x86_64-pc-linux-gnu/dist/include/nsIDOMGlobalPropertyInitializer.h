/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIDOMGlobalPropertyInitializer.idl
 */

#ifndef __gen_nsIDOMGlobalPropertyInitializer_h__
#define __gen_nsIDOMGlobalPropertyInitializer_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */


/* starting interface:    nsIDOMGlobalPropertyInitializer */
#define NS_IDOMGLOBALPROPERTYINITIALIZER_IID_STR "5842e275-797f-4afb-b7e0-e29f0cb312ae"

#define NS_IDOMGLOBALPROPERTYINITIALIZER_IID \
  {0x5842e275, 0x797f, 0x4afb, \
    { 0xb7, 0xe0, 0xe2, 0x9f, 0x0c, 0xb3, 0x12, 0xae }}

class NS_NO_VTABLE nsIDOMGlobalPropertyInitializer : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMGLOBALPROPERTYINITIALIZER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMGlobalPropertyInitializer;

  /* jsval init (in mozIDOMWindow window); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(mozIDOMWindow *window, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMGlobalPropertyInitializer, NS_IDOMGLOBALPROPERTYINITIALIZER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMGLOBALPROPERTYINITIALIZER \
  NS_IMETHOD Init(mozIDOMWindow *window, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMGLOBALPROPERTYINITIALIZER \
  nsresult Init(mozIDOMWindow *window, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMGLOBALPROPERTYINITIALIZER(_to) \
  NS_IMETHOD Init(mozIDOMWindow *window, JS::MutableHandleValue _retval) override { return _to Init(window, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMGLOBALPROPERTYINITIALIZER(_to) \
  NS_IMETHOD Init(mozIDOMWindow *window, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(window, _retval); } 


#endif /* __gen_nsIDOMGlobalPropertyInitializer_h__ */
