/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleApplication.idl
 */

#ifndef __gen_nsIAccessibleApplication_h__
#define __gen_nsIAccessibleApplication_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleApplication */
#define NS_IACCESSIBLEAPPLICATION_IID_STR "79251626-387c-4531-89f3-680d31d6cf05"

#define NS_IACCESSIBLEAPPLICATION_IID \
  {0x79251626, 0x387c, 0x4531, \
    { 0x89, 0xf3, 0x68, 0x0d, 0x31, 0xd6, 0xcf, 0x05 }}

class NS_NO_VTABLE nsIAccessibleApplication : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEAPPLICATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleApplication;

  /* readonly attribute AString appName; */
  NS_IMETHOD GetAppName(nsAString& aAppName) = 0;

  /* readonly attribute AString appVersion; */
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) = 0;

  /* readonly attribute AString platformName; */
  NS_IMETHOD GetPlatformName(nsAString& aPlatformName) = 0;

  /* readonly attribute AString platformVersion; */
  NS_IMETHOD GetPlatformVersion(nsAString& aPlatformVersion) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleApplication, NS_IACCESSIBLEAPPLICATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEAPPLICATION \
  NS_IMETHOD GetAppName(nsAString& aAppName) override; \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override; \
  NS_IMETHOD GetPlatformName(nsAString& aPlatformName) override; \
  NS_IMETHOD GetPlatformVersion(nsAString& aPlatformVersion) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEAPPLICATION \
  nsresult GetAppName(nsAString& aAppName); \
  nsresult GetAppVersion(nsAString& aAppVersion); \
  nsresult GetPlatformName(nsAString& aPlatformName); \
  nsresult GetPlatformVersion(nsAString& aPlatformVersion); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEAPPLICATION(_to) \
  NS_IMETHOD GetAppName(nsAString& aAppName) override { return _to GetAppName(aAppName); } \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override { return _to GetAppVersion(aAppVersion); } \
  NS_IMETHOD GetPlatformName(nsAString& aPlatformName) override { return _to GetPlatformName(aPlatformName); } \
  NS_IMETHOD GetPlatformVersion(nsAString& aPlatformVersion) override { return _to GetPlatformVersion(aPlatformVersion); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEAPPLICATION(_to) \
  NS_IMETHOD GetAppName(nsAString& aAppName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppName(aAppName); } \
  NS_IMETHOD GetAppVersion(nsAString& aAppVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppVersion(aAppVersion); } \
  NS_IMETHOD GetPlatformName(nsAString& aPlatformName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatformName(aPlatformName); } \
  NS_IMETHOD GetPlatformVersion(nsAString& aPlatformVersion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPlatformVersion(aPlatformVersion); } 


#endif /* __gen_nsIAccessibleApplication_h__ */
