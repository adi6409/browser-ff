/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIWritablePropertyBag.idl
 */

#ifndef __gen_nsIWritablePropertyBag_h__
#define __gen_nsIWritablePropertyBag_h__


#ifndef __gen_nsIPropertyBag_h__
#include "nsIPropertyBag.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWritablePropertyBag */
#define NS_IWRITABLEPROPERTYBAG_IID_STR "96fc4671-eeb4-4823-9421-e50fb70ad353"

#define NS_IWRITABLEPROPERTYBAG_IID \
  {0x96fc4671, 0xeeb4, 0x4823, \
    { 0x94, 0x21, 0xe5, 0x0f, 0xb7, 0x0a, 0xd3, 0x53 }}

class NS_NO_VTABLE nsIWritablePropertyBag : public nsIPropertyBag {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWRITABLEPROPERTYBAG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWritablePropertyBag;

  /* void setProperty (in AString name, in nsIVariant value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetProperty(const nsAString& name, nsIVariant *value) = 0;

  /* void deleteProperty (in AString name); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DeleteProperty(const nsAString& name) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWritablePropertyBag, NS_IWRITABLEPROPERTYBAG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWRITABLEPROPERTYBAG \
  NS_IMETHOD SetProperty(const nsAString& name, nsIVariant *value) override; \
  NS_IMETHOD DeleteProperty(const nsAString& name) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWRITABLEPROPERTYBAG \
  nsresult SetProperty(const nsAString& name, nsIVariant *value); \
  nsresult DeleteProperty(const nsAString& name); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWRITABLEPROPERTYBAG(_to) \
  NS_IMETHOD SetProperty(const nsAString& name, nsIVariant *value) override { return _to SetProperty(name, value); } \
  NS_IMETHOD DeleteProperty(const nsAString& name) override { return _to DeleteProperty(name); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWRITABLEPROPERTYBAG(_to) \
  NS_IMETHOD SetProperty(const nsAString& name, nsIVariant *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetProperty(name, value); } \
  NS_IMETHOD DeleteProperty(const nsAString& name) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DeleteProperty(name); } 


#endif /* __gen_nsIWritablePropertyBag_h__ */
