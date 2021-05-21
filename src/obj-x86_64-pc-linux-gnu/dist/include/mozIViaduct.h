/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/viaduct/mozIViaduct.idl
 */

#ifndef __gen_mozIViaduct_h__
#define __gen_mozIViaduct_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIViaduct */
#define MOZIVIADUCT_IID_STR "f8f11a45-ff04-4d8b-80e0-9ce14824538e"

#define MOZIVIADUCT_IID \
  {0xf8f11a45, 0xff04, 0x4d8b, \
    { 0x80, 0xe0, 0x9c, 0xe1, 0x48, 0x24, 0x53, 0x8e }}

class NS_NO_VTABLE mozIViaduct : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIVIADUCT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIViaduct;

  /* void EnsureInitialized (); */
  NS_IMETHOD EnsureInitialized(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIViaduct, MOZIVIADUCT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIVIADUCT \
  NS_IMETHOD EnsureInitialized(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIVIADUCT \
  nsresult EnsureInitialized(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIVIADUCT(_to) \
  NS_IMETHOD EnsureInitialized(void) override { return _to EnsureInitialized(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIVIADUCT(_to) \
  NS_IMETHOD EnsureInitialized(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnsureInitialized(); } 


#endif /* __gen_mozIViaduct_h__ */
