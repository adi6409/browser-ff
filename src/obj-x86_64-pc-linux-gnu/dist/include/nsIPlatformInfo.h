/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIPlatformInfo.idl
 */

#ifndef __gen_nsIPlatformInfo_h__
#define __gen_nsIPlatformInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPlatformInfo */
#define NS_IPLATFORMINFO_IID_STR "ab6650cf-0806-4aea-b8f2-40fdae74f1cc"

#define NS_IPLATFORMINFO_IID \
  {0xab6650cf, 0x0806, 0x4aea, \
    { 0xb8, 0xf2, 0x40, 0xfd, 0xae, 0x74, 0xf1, 0xcc }}

class NS_NO_VTABLE nsIPlatformInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPLATFORMINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPlatformInfo;

  /* readonly attribute ACString platformVersion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPlatformVersion(nsACString& aPlatformVersion) = 0;

  /* readonly attribute ACString platformBuildID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPlatformBuildID(nsACString& aPlatformBuildID) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPlatformInfo, NS_IPLATFORMINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPLATFORMINFO \
  NS_IMETHOD GetPlatformVersion(nsACString& aPlatformVersion) override; \
  NS_IMETHOD GetPlatformBuildID(nsACString& aPlatformBuildID) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPLATFORMINFO \
  nsresult GetPlatformVersion(nsACString& aPlatformVersion); \
  nsresult GetPlatformBuildID(nsACString& aPlatformBuildID); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPLATFORMINFO(_to) \
  NS_IMETHOD GetPlatformVersion(nsACString& aPlatformVersion) override { return _to GetPlatformVersion(aPlatformVersion); } \
  NS_IMETHOD GetPlatformBuildID(nsACString& aPlatformBuildID) override { return _to GetPlatformBuildID(aPlatformBuildID); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPLATFORMINFO(_to) \
  NS_IMETHOD GetPlatformVersion(nsACString& aPlatformVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatformVersion(aPlatformVersion); } \
  NS_IMETHOD GetPlatformBuildID(nsACString& aPlatformBuildID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatformBuildID(aPlatformBuildID); } 


#endif /* __gen_nsIPlatformInfo_h__ */
