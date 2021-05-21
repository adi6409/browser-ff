/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/testing/marionette/components/nsIMarionette.idl
 */

#ifndef __gen_nsIMarionette_h__
#define __gen_nsIMarionette_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_MARIONETTE_CONTRACTID "@mozilla.org/remote/marionette;1"

/* starting interface:    nsIMarionette */
#define NS_IMARIONETTE_IID_STR "13fa7d76-f976-4711-a00c-29ac9c1881e1"

#define NS_IMARIONETTE_IID \
  {0x13fa7d76, 0xf976, 0x4711, \
    { 0xa0, 0x0c, 0x29, 0xac, 0x9c, 0x18, 0x81, 0xe1 }}

class NS_NO_VTABLE nsIMarionette : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMARIONETTE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMarionette;

  /* readonly attribute boolean running; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRunning(bool *aRunning) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMarionette, NS_IMARIONETTE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMARIONETTE \
  NS_IMETHOD GetRunning(bool *aRunning) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMARIONETTE \
  nsresult GetRunning(bool *aRunning); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMARIONETTE(_to) \
  NS_IMETHOD GetRunning(bool *aRunning) override { return _to GetRunning(aRunning); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMARIONETTE(_to) \
  NS_IMETHOD GetRunning(bool *aRunning) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRunning(aRunning); } 


#endif /* __gen_nsIMarionette_h__ */
