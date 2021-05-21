/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPaper.idl
 */

#ifndef __gen_nsIPaper_h__
#define __gen_nsIPaper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPaper */
#define NS_IPAPER_IID_STR "a4dd9675-6311-45a9-a547-44e0127304a6"

#define NS_IPAPER_IID \
  {0xa4dd9675, 0x6311, 0x45a9, \
    { 0xa5, 0x47, 0x44, 0xe0, 0x12, 0x73, 0x04, 0xa6 }}

class NS_NO_VTABLE nsIPaper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPAPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPaper;

  /* readonly attribute AString id; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetId(nsAString& aId) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute double width; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWidth(double *aWidth) = 0;

  /* readonly attribute double height; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHeight(double *aHeight) = 0;

  /* [implicit_jscontext] readonly attribute Promise unwriteableMargin; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUnwriteableMargin(JSContext* cx, ::mozilla::dom::Promise * * aUnwriteableMargin) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPaper, NS_IPAPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPAPER \
  NS_IMETHOD GetId(nsAString& aId) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetWidth(double *aWidth) override; \
  NS_IMETHOD GetHeight(double *aHeight) override; \
  NS_IMETHOD GetUnwriteableMargin(JSContext* cx, ::mozilla::dom::Promise * * aUnwriteableMargin) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPAPER \
  nsresult GetId(nsAString& aId); \
  nsresult GetName(nsAString& aName); \
  nsresult GetWidth(double *aWidth); \
  nsresult GetHeight(double *aHeight); \
  nsresult GetUnwriteableMargin(JSContext* cx, ::mozilla::dom::Promise * * aUnwriteableMargin); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPAPER(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return _to GetId(aId); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetWidth(double *aWidth) override { return _to GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(double *aHeight) override { return _to GetHeight(aHeight); } \
  NS_IMETHOD GetUnwriteableMargin(JSContext* cx, ::mozilla::dom::Promise * * aUnwriteableMargin) override { return _to GetUnwriteableMargin(cx, aUnwriteableMargin); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPAPER(_to) \
  NS_IMETHOD GetId(nsAString& aId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetId(aId); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetWidth(double *aWidth) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWidth(aWidth); } \
  NS_IMETHOD GetHeight(double *aHeight) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHeight(aHeight); } \
  NS_IMETHOD GetUnwriteableMargin(JSContext* cx, ::mozilla::dom::Promise * * aUnwriteableMargin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUnwriteableMargin(cx, aUnwriteableMargin); } 


#endif /* __gen_nsIPaper_h__ */
