/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthInformation.idl
 */

#ifndef __gen_nsIAuthInformation_h__
#define __gen_nsIAuthInformation_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAuthInformation */
#define NS_IAUTHINFORMATION_IID_STR "0d73639c-2a92-4518-9f92-28f71fea5f20"

#define NS_IAUTHINFORMATION_IID \
  {0x0d73639c, 0x2a92, 0x4518, \
    { 0x9f, 0x92, 0x28, 0xf7, 0x1f, 0xea, 0x5f, 0x20 }}

class NS_NO_VTABLE nsIAuthInformation : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTHINFORMATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAuthInformation;

  enum {
    AUTH_HOST = 1U,
    AUTH_PROXY = 2U,
    NEED_DOMAIN = 4U,
    ONLY_PASSWORD = 8U,
    PREVIOUS_FAILED = 16U,
    CROSS_ORIGIN_SUB_RESOURCE = 32U
  };

  /* readonly attribute unsigned long flags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFlags(uint32_t *aFlags) = 0;

  /* readonly attribute AString realm; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRealm(nsAString& aRealm) = 0;

  /* readonly attribute AUTF8String authenticationScheme; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAuthenticationScheme(nsACString& aAuthenticationScheme) = 0;

  /* attribute AString username; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUsername(nsAString& aUsername) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetUsername(const nsAString& aUsername) = 0;

  /* attribute AString password; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPassword(nsAString& aPassword) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPassword(const nsAString& aPassword) = 0;

  /* attribute AString domain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDomain(nsAString& aDomain) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDomain(const nsAString& aDomain) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAuthInformation, NS_IAUTHINFORMATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTHINFORMATION \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override; \
  NS_IMETHOD GetRealm(nsAString& aRealm) override; \
  NS_IMETHOD GetAuthenticationScheme(nsACString& aAuthenticationScheme) override; \
  NS_IMETHOD GetUsername(nsAString& aUsername) override; \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override; \
  NS_IMETHOD GetPassword(nsAString& aPassword) override; \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override; \
  NS_IMETHOD GetDomain(nsAString& aDomain) override; \
  NS_IMETHOD SetDomain(const nsAString& aDomain) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTHINFORMATION \
  nsresult GetFlags(uint32_t *aFlags); \
  nsresult GetRealm(nsAString& aRealm); \
  nsresult GetAuthenticationScheme(nsACString& aAuthenticationScheme); \
  nsresult GetUsername(nsAString& aUsername); \
  nsresult SetUsername(const nsAString& aUsername); \
  nsresult GetPassword(nsAString& aPassword); \
  nsresult SetPassword(const nsAString& aPassword); \
  nsresult GetDomain(nsAString& aDomain); \
  nsresult SetDomain(const nsAString& aDomain); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTHINFORMATION(_to) \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return _to GetFlags(aFlags); } \
  NS_IMETHOD GetRealm(nsAString& aRealm) override { return _to GetRealm(aRealm); } \
  NS_IMETHOD GetAuthenticationScheme(nsACString& aAuthenticationScheme) override { return _to GetAuthenticationScheme(aAuthenticationScheme); } \
  NS_IMETHOD GetUsername(nsAString& aUsername) override { return _to GetUsername(aUsername); } \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override { return _to SetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsAString& aPassword) override { return _to GetPassword(aPassword); } \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override { return _to SetPassword(aPassword); } \
  NS_IMETHOD GetDomain(nsAString& aDomain) override { return _to GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(const nsAString& aDomain) override { return _to SetDomain(aDomain); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTHINFORMATION(_to) \
  NS_IMETHOD GetFlags(uint32_t *aFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFlags(aFlags); } \
  NS_IMETHOD GetRealm(nsAString& aRealm) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRealm(aRealm); } \
  NS_IMETHOD GetAuthenticationScheme(nsACString& aAuthenticationScheme) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAuthenticationScheme(aAuthenticationScheme); } \
  NS_IMETHOD GetUsername(nsAString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUsername(aUsername); } \
  NS_IMETHOD SetUsername(const nsAString& aUsername) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetUsername(aUsername); } \
  NS_IMETHOD GetPassword(nsAString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPassword(aPassword); } \
  NS_IMETHOD SetPassword(const nsAString& aPassword) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPassword(aPassword); } \
  NS_IMETHOD GetDomain(nsAString& aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDomain(aDomain); } \
  NS_IMETHOD SetDomain(const nsAString& aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDomain(aDomain); } 


#endif /* __gen_nsIAuthInformation_h__ */
