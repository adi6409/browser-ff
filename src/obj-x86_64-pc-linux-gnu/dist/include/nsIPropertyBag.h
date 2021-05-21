/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIPropertyBag.idl
 */

#ifndef __gen_nsIPropertyBag_h__
#define __gen_nsIPropertyBag_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIVariant; /* forward declaration */

class nsISimpleEnumerator; /* forward declaration */


/* starting interface:    nsIPropertyBag */
#define NS_IPROPERTYBAG_IID_STR "bfcd37b0-a49f-11d5-910d-0010a4e73d9a"

#define NS_IPROPERTYBAG_IID \
  {0xbfcd37b0, 0xa49f, 0x11d5, \
    { 0x91, 0x0d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIPropertyBag : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROPERTYBAG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPropertyBag;

  /* readonly attribute nsISimpleEnumerator enumerator; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnumerator(nsISimpleEnumerator **aEnumerator) = 0;

  /* nsIVariant getProperty (in AString name); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProperty(const nsAString& name, nsIVariant **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPropertyBag, NS_IPROPERTYBAG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROPERTYBAG \
  NS_IMETHOD GetEnumerator(nsISimpleEnumerator **aEnumerator) override; \
  NS_IMETHOD GetProperty(const nsAString& name, nsIVariant **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROPERTYBAG \
  nsresult GetEnumerator(nsISimpleEnumerator **aEnumerator); \
  nsresult GetProperty(const nsAString& name, nsIVariant **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROPERTYBAG(_to) \
  NS_IMETHOD GetEnumerator(nsISimpleEnumerator **aEnumerator) override { return _to GetEnumerator(aEnumerator); } \
  NS_IMETHOD GetProperty(const nsAString& name, nsIVariant **_retval) override { return _to GetProperty(name, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROPERTYBAG(_to) \
  NS_IMETHOD GetEnumerator(nsISimpleEnumerator **aEnumerator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnumerator(aEnumerator); } \
  NS_IMETHOD GetProperty(const nsAString& name, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProperty(name, _retval); } 


#endif /* __gen_nsIPropertyBag_h__ */
