/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIProperty.idl
 */

#ifndef __gen_nsIProperty_h__
#define __gen_nsIProperty_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIVariant; /* forward declaration */


/* starting interface:    nsIProperty */
#define NS_IPROPERTY_IID_STR "6dcf9030-a49f-11d5-910d-0010a4e73d9a"

#define NS_IPROPERTY_IID \
  {0x6dcf9030, 0xa49f, 0x11d5, \
    { 0x91, 0x0d, 0x00, 0x10, 0xa4, 0xe7, 0x3d, 0x9a }}

class NS_NO_VTABLE nsIProperty : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPROPERTY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIProperty;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute nsIVariant value; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetValue(nsIVariant **aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIProperty, NS_IPROPERTY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPROPERTY \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetValue(nsIVariant **aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPROPERTY \
  nsresult GetName(nsAString& aName); \
  nsresult GetValue(nsIVariant **aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPROPERTY(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return _to GetValue(aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPROPERTY(_to) \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetValue(nsIVariant **aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValue(aValue); } 


#endif /* __gen_nsIProperty_h__ */
