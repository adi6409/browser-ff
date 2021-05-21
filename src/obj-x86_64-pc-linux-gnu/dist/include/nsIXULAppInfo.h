/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIXULAppInfo.idl
 */

#ifndef __gen_nsIXULAppInfo_h__
#define __gen_nsIXULAppInfo_h__


#ifndef __gen_nsIPlatformInfo_h__
#include "nsIPlatformInfo.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIXULAppInfo */
#define NS_IXULAPPINFO_IID_STR "ddea4f31-3c5e-4769-ac68-21ab4b3d7845"

#define NS_IXULAPPINFO_IID \
  {0xddea4f31, 0x3c5e, 0x4769, \
    { 0xac, 0x68, 0x21, 0xab, 0x4b, 0x3d, 0x78, 0x45 }}

class NS_NO_VTABLE nsIXULAppInfo : public nsIPlatformInfo {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IXULAPPINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIXULAppInfo;

  /* readonly attribute ACString vendor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVendor(nsACString& aVendor) = 0;

  /* readonly attribute ACString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsACString& aName) = 0;

  /* readonly attribute ACString ID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetID(nsACString& aID) = 0;

  /* readonly attribute ACString version; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVersion(nsACString& aVersion) = 0;

  /* readonly attribute ACString appBuildID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppBuildID(nsACString& aAppBuildID) = 0;

  /* readonly attribute ACString UAName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUAName(nsACString& aUAName) = 0;

  /* readonly attribute ACString sourceURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceURL(nsACString& aSourceURL) = 0;

  /* readonly attribute ACString updateURL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUpdateURL(nsACString& aUpdateURL) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIXULAppInfo, NS_IXULAPPINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIXULAPPINFO \
  NS_IMETHOD GetVendor(nsACString& aVendor) override; \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD GetID(nsACString& aID) override; \
  NS_IMETHOD GetVersion(nsACString& aVersion) override; \
  NS_IMETHOD GetAppBuildID(nsACString& aAppBuildID) override; \
  NS_IMETHOD GetUAName(nsACString& aUAName) override; \
  NS_IMETHOD GetSourceURL(nsACString& aSourceURL) override; \
  NS_IMETHOD GetUpdateURL(nsACString& aUpdateURL) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIXULAPPINFO \
  nsresult GetVendor(nsACString& aVendor); \
  nsresult GetName(nsACString& aName); \
  nsresult GetID(nsACString& aID); \
  nsresult GetVersion(nsACString& aVersion); \
  nsresult GetAppBuildID(nsACString& aAppBuildID); \
  nsresult GetUAName(nsACString& aUAName); \
  nsresult GetSourceURL(nsACString& aSourceURL); \
  nsresult GetUpdateURL(nsACString& aUpdateURL); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIXULAPPINFO(_to) \
  NS_IMETHOD GetVendor(nsACString& aVendor) override { return _to GetVendor(aVendor); } \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetID(nsACString& aID) override { return _to GetID(aID); } \
  NS_IMETHOD GetVersion(nsACString& aVersion) override { return _to GetVersion(aVersion); } \
  NS_IMETHOD GetAppBuildID(nsACString& aAppBuildID) override { return _to GetAppBuildID(aAppBuildID); } \
  NS_IMETHOD GetUAName(nsACString& aUAName) override { return _to GetUAName(aUAName); } \
  NS_IMETHOD GetSourceURL(nsACString& aSourceURL) override { return _to GetSourceURL(aSourceURL); } \
  NS_IMETHOD GetUpdateURL(nsACString& aUpdateURL) override { return _to GetUpdateURL(aUpdateURL); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIXULAPPINFO(_to) \
  NS_IMETHOD GetVendor(nsACString& aVendor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVendor(aVendor); } \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetID(nsACString& aID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetID(aID); } \
  NS_IMETHOD GetVersion(nsACString& aVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVersion(aVersion); } \
  NS_IMETHOD GetAppBuildID(nsACString& aAppBuildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppBuildID(aAppBuildID); } \
  NS_IMETHOD GetUAName(nsACString& aUAName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUAName(aUAName); } \
  NS_IMETHOD GetSourceURL(nsACString& aSourceURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceURL(aSourceURL); } \
  NS_IMETHOD GetUpdateURL(nsACString& aUpdateURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUpdateURL(aUpdateURL); } 


#endif /* __gen_nsIXULAppInfo_h__ */
