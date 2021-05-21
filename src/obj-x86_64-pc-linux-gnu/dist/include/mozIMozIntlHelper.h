/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/mozintl/mozIMozIntlHelper.idl
 */

#ifndef __gen_mozIMozIntlHelper_h__
#define __gen_mozIMozIntlHelper_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIMozIntlHelper */
#define MOZIMOZINTLHELPER_IID_STR "189eaa7d-b29a-43a9-b1fb-7658990df940"

#define MOZIMOZINTLHELPER_IID \
  {0x189eaa7d, 0xb29a, 0x43a9, \
    { 0xb1, 0xfb, 0x76, 0x58, 0x99, 0x0d, 0xf9, 0x40 }}

class NS_NO_VTABLE mozIMozIntlHelper : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIMOZINTLHELPER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIMozIntlHelper;

  /* [implicit_jscontext] void addGetCalendarInfo (in jsval intlObject); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddGetCalendarInfo(JS::HandleValue intlObject, JSContext* cx) = 0;

  /* [implicit_jscontext] void addGetDisplayNames (in jsval intlObject); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddGetDisplayNames(JS::HandleValue intlObject, JSContext* cx) = 0;

  /* [implicit_jscontext] void addGetLocaleInfo (in jsval intlObject); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddGetLocaleInfo(JS::HandleValue intlObject, JSContext* cx) = 0;

  /* [implicit_jscontext] void addDateTimeFormatConstructor (in jsval intlObject); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddDateTimeFormatConstructor(JS::HandleValue intlObject, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIMozIntlHelper, MOZIMOZINTLHELPER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIMOZINTLHELPER \
  NS_IMETHOD AddGetCalendarInfo(JS::HandleValue intlObject, JSContext* cx) override; \
  NS_IMETHOD AddGetDisplayNames(JS::HandleValue intlObject, JSContext* cx) override; \
  NS_IMETHOD AddGetLocaleInfo(JS::HandleValue intlObject, JSContext* cx) override; \
  NS_IMETHOD AddDateTimeFormatConstructor(JS::HandleValue intlObject, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIMOZINTLHELPER \
  nsresult AddGetCalendarInfo(JS::HandleValue intlObject, JSContext* cx); \
  nsresult AddGetDisplayNames(JS::HandleValue intlObject, JSContext* cx); \
  nsresult AddGetLocaleInfo(JS::HandleValue intlObject, JSContext* cx); \
  nsresult AddDateTimeFormatConstructor(JS::HandleValue intlObject, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIMOZINTLHELPER(_to) \
  NS_IMETHOD AddGetCalendarInfo(JS::HandleValue intlObject, JSContext* cx) override { return _to AddGetCalendarInfo(intlObject, cx); } \
  NS_IMETHOD AddGetDisplayNames(JS::HandleValue intlObject, JSContext* cx) override { return _to AddGetDisplayNames(intlObject, cx); } \
  NS_IMETHOD AddGetLocaleInfo(JS::HandleValue intlObject, JSContext* cx) override { return _to AddGetLocaleInfo(intlObject, cx); } \
  NS_IMETHOD AddDateTimeFormatConstructor(JS::HandleValue intlObject, JSContext* cx) override { return _to AddDateTimeFormatConstructor(intlObject, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIMOZINTLHELPER(_to) \
  NS_IMETHOD AddGetCalendarInfo(JS::HandleValue intlObject, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddGetCalendarInfo(intlObject, cx); } \
  NS_IMETHOD AddGetDisplayNames(JS::HandleValue intlObject, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddGetDisplayNames(intlObject, cx); } \
  NS_IMETHOD AddGetLocaleInfo(JS::HandleValue intlObject, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddGetLocaleInfo(intlObject, cx); } \
  NS_IMETHOD AddDateTimeFormatConstructor(JS::HandleValue intlObject, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddDateTimeFormatConstructor(intlObject, cx); } 


#endif /* __gen_mozIMozIntlHelper_h__ */
