/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsISystemInfo.idl
 */

#ifndef __gen_nsISystemInfo_h__
#define __gen_nsISystemInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISystemInfo */
#define NS_ISYSTEMINFO_IID_STR "09a0502b-cedc-4cae-bf7c-35662dbd1249"

#define NS_ISYSTEMINFO_IID \
  {0x09a0502b, 0xcedc, 0x4cae, \
    { 0xbf, 0x7c, 0x35, 0x66, 0x2d, 0xbd, 0x12, 0x49 }}

class NS_NO_VTABLE nsISystemInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISYSTEMINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISystemInfo;

  /* [implicit_jscontext] readonly attribute Promise diskInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDiskInfo(JSContext* cx, ::mozilla::dom::Promise * * aDiskInfo) = 0;

  /* [implicit_jscontext] readonly attribute Promise countryCode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCountryCode(JSContext* cx, ::mozilla::dom::Promise * * aCountryCode) = 0;

  /* [implicit_jscontext] readonly attribute Promise osInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOsInfo(JSContext* cx, ::mozilla::dom::Promise * * aOsInfo) = 0;

  /* [implicit_jscontext] readonly attribute Promise processInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcessInfo(JSContext* cx, ::mozilla::dom::Promise * * aProcessInfo) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISystemInfo, NS_ISYSTEMINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISYSTEMINFO \
  NS_IMETHOD GetDiskInfo(JSContext* cx, ::mozilla::dom::Promise * * aDiskInfo) override; \
  NS_IMETHOD GetCountryCode(JSContext* cx, ::mozilla::dom::Promise * * aCountryCode) override; \
  NS_IMETHOD GetOsInfo(JSContext* cx, ::mozilla::dom::Promise * * aOsInfo) override; \
  NS_IMETHOD GetProcessInfo(JSContext* cx, ::mozilla::dom::Promise * * aProcessInfo) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISYSTEMINFO \
  nsresult GetDiskInfo(JSContext* cx, ::mozilla::dom::Promise * * aDiskInfo); \
  nsresult GetCountryCode(JSContext* cx, ::mozilla::dom::Promise * * aCountryCode); \
  nsresult GetOsInfo(JSContext* cx, ::mozilla::dom::Promise * * aOsInfo); \
  nsresult GetProcessInfo(JSContext* cx, ::mozilla::dom::Promise * * aProcessInfo); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISYSTEMINFO(_to) \
  NS_IMETHOD GetDiskInfo(JSContext* cx, ::mozilla::dom::Promise * * aDiskInfo) override { return _to GetDiskInfo(cx, aDiskInfo); } \
  NS_IMETHOD GetCountryCode(JSContext* cx, ::mozilla::dom::Promise * * aCountryCode) override { return _to GetCountryCode(cx, aCountryCode); } \
  NS_IMETHOD GetOsInfo(JSContext* cx, ::mozilla::dom::Promise * * aOsInfo) override { return _to GetOsInfo(cx, aOsInfo); } \
  NS_IMETHOD GetProcessInfo(JSContext* cx, ::mozilla::dom::Promise * * aProcessInfo) override { return _to GetProcessInfo(cx, aProcessInfo); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISYSTEMINFO(_to) \
  NS_IMETHOD GetDiskInfo(JSContext* cx, ::mozilla::dom::Promise * * aDiskInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDiskInfo(cx, aDiskInfo); } \
  NS_IMETHOD GetCountryCode(JSContext* cx, ::mozilla::dom::Promise * * aCountryCode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCountryCode(cx, aCountryCode); } \
  NS_IMETHOD GetOsInfo(JSContext* cx, ::mozilla::dom::Promise * * aOsInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOsInfo(cx, aOsInfo); } \
  NS_IMETHOD GetProcessInfo(JSContext* cx, ::mozilla::dom::Promise * * aProcessInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcessInfo(cx, aProcessInfo); } 


#endif /* __gen_nsISystemInfo_h__ */
