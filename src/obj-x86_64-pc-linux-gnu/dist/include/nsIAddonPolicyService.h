/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIAddonPolicyService.idl
 */

#ifndef __gen_nsIAddonPolicyService_h__
#define __gen_nsIAddonPolicyService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIURI_h__
#include "nsIURI.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAddonPolicyService */
#define NS_IADDONPOLICYSERVICE_IID_STR "8a034ef9-9d14-4c5d-8319-06c1ab574baa"

#define NS_IADDONPOLICYSERVICE_IID \
  {0x8a034ef9, 0x9d14, 0x4c5d, \
    { 0x83, 0x19, 0x06, 0xc1, 0xab, 0x57, 0x4b, 0xaa }}

class NS_NO_VTABLE nsIAddonPolicyService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IADDONPOLICYSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAddonPolicyService;

  /* readonly attribute AString defaultCSP; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultCSP(nsAString& aDefaultCSP) = 0;

  /* AString getBaseCSP (in AString aAddonId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBaseCSP(const nsAString& aAddonId, nsAString& _retval) = 0;

  /* AString getExtensionPageCSP (in AString aAddonId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExtensionPageCSP(const nsAString& aAddonId, nsAString& _retval) = 0;

  /* ACString getGeneratedBackgroundPageUrl (in ACString aAddonId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetGeneratedBackgroundPageUrl(const nsACString& aAddonId, nsACString& _retval) = 0;

  /* boolean addonHasPermission (in AString aAddonId, in AString aPerm); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddonHasPermission(const nsAString& aAddonId, const nsAString& aPerm, bool *_retval) = 0;

  /* boolean addonMayLoadURI (in AString aAddonId, in nsIURI aURI, [optional] in boolean aExplicit); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddonMayLoadURI(const nsAString& aAddonId, nsIURI *aURI, bool aExplicit, bool *_retval) = 0;

  /* AString getExtensionName (in AString aAddonId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExtensionName(const nsAString& aAddonId, nsAString& _retval) = 0;

  /* boolean extensionURILoadableByAnyone (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExtensionURILoadableByAnyone(nsIURI *aURI, bool *_retval) = 0;

  /* AString extensionURIToAddonId (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExtensionURIToAddonId(nsIURI *aURI, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAddonPolicyService, NS_IADDONPOLICYSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIADDONPOLICYSERVICE \
  NS_IMETHOD GetDefaultCSP(nsAString& aDefaultCSP) override; \
  NS_IMETHOD GetBaseCSP(const nsAString& aAddonId, nsAString& _retval) override; \
  NS_IMETHOD GetExtensionPageCSP(const nsAString& aAddonId, nsAString& _retval) override; \
  NS_IMETHOD GetGeneratedBackgroundPageUrl(const nsACString& aAddonId, nsACString& _retval) override; \
  NS_IMETHOD AddonHasPermission(const nsAString& aAddonId, const nsAString& aPerm, bool *_retval) override; \
  NS_IMETHOD AddonMayLoadURI(const nsAString& aAddonId, nsIURI *aURI, bool aExplicit, bool *_retval) override; \
  NS_IMETHOD GetExtensionName(const nsAString& aAddonId, nsAString& _retval) override; \
  NS_IMETHOD ExtensionURILoadableByAnyone(nsIURI *aURI, bool *_retval) override; \
  NS_IMETHOD ExtensionURIToAddonId(nsIURI *aURI, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIADDONPOLICYSERVICE \
  nsresult GetDefaultCSP(nsAString& aDefaultCSP); \
  nsresult GetBaseCSP(const nsAString& aAddonId, nsAString& _retval); \
  nsresult GetExtensionPageCSP(const nsAString& aAddonId, nsAString& _retval); \
  nsresult GetGeneratedBackgroundPageUrl(const nsACString& aAddonId, nsACString& _retval); \
  nsresult AddonHasPermission(const nsAString& aAddonId, const nsAString& aPerm, bool *_retval); \
  nsresult AddonMayLoadURI(const nsAString& aAddonId, nsIURI *aURI, bool aExplicit, bool *_retval); \
  nsresult GetExtensionName(const nsAString& aAddonId, nsAString& _retval); \
  nsresult ExtensionURILoadableByAnyone(nsIURI *aURI, bool *_retval); \
  nsresult ExtensionURIToAddonId(nsIURI *aURI, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIADDONPOLICYSERVICE(_to) \
  NS_IMETHOD GetDefaultCSP(nsAString& aDefaultCSP) override { return _to GetDefaultCSP(aDefaultCSP); } \
  NS_IMETHOD GetBaseCSP(const nsAString& aAddonId, nsAString& _retval) override { return _to GetBaseCSP(aAddonId, _retval); } \
  NS_IMETHOD GetExtensionPageCSP(const nsAString& aAddonId, nsAString& _retval) override { return _to GetExtensionPageCSP(aAddonId, _retval); } \
  NS_IMETHOD GetGeneratedBackgroundPageUrl(const nsACString& aAddonId, nsACString& _retval) override { return _to GetGeneratedBackgroundPageUrl(aAddonId, _retval); } \
  NS_IMETHOD AddonHasPermission(const nsAString& aAddonId, const nsAString& aPerm, bool *_retval) override { return _to AddonHasPermission(aAddonId, aPerm, _retval); } \
  NS_IMETHOD AddonMayLoadURI(const nsAString& aAddonId, nsIURI *aURI, bool aExplicit, bool *_retval) override { return _to AddonMayLoadURI(aAddonId, aURI, aExplicit, _retval); } \
  NS_IMETHOD GetExtensionName(const nsAString& aAddonId, nsAString& _retval) override { return _to GetExtensionName(aAddonId, _retval); } \
  NS_IMETHOD ExtensionURILoadableByAnyone(nsIURI *aURI, bool *_retval) override { return _to ExtensionURILoadableByAnyone(aURI, _retval); } \
  NS_IMETHOD ExtensionURIToAddonId(nsIURI *aURI, nsAString& _retval) override { return _to ExtensionURIToAddonId(aURI, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIADDONPOLICYSERVICE(_to) \
  NS_IMETHOD GetDefaultCSP(nsAString& aDefaultCSP) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultCSP(aDefaultCSP); } \
  NS_IMETHOD GetBaseCSP(const nsAString& aAddonId, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseCSP(aAddonId, _retval); } \
  NS_IMETHOD GetExtensionPageCSP(const nsAString& aAddonId, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensionPageCSP(aAddonId, _retval); } \
  NS_IMETHOD GetGeneratedBackgroundPageUrl(const nsACString& aAddonId, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetGeneratedBackgroundPageUrl(aAddonId, _retval); } \
  NS_IMETHOD AddonHasPermission(const nsAString& aAddonId, const nsAString& aPerm, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddonHasPermission(aAddonId, aPerm, _retval); } \
  NS_IMETHOD AddonMayLoadURI(const nsAString& aAddonId, nsIURI *aURI, bool aExplicit, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddonMayLoadURI(aAddonId, aURI, aExplicit, _retval); } \
  NS_IMETHOD GetExtensionName(const nsAString& aAddonId, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExtensionName(aAddonId, _retval); } \
  NS_IMETHOD ExtensionURILoadableByAnyone(nsIURI *aURI, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExtensionURILoadableByAnyone(aURI, _retval); } \
  NS_IMETHOD ExtensionURIToAddonId(nsIURI *aURI, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExtensionURIToAddonId(aURI, _retval); } 


/* starting interface:    nsIAddonContentPolicy */
#define NS_IADDONCONTENTPOLICY_IID_STR "7a4fe60b-9131-45f5-83f3-dc63b5d71a5d"

#define NS_IADDONCONTENTPOLICY_IID \
  {0x7a4fe60b, 0x9131, 0x45f5, \
    { 0x83, 0xf3, 0xdc, 0x63, 0xb5, 0xd7, 0x1a, 0x5d }}

class NS_NO_VTABLE nsIAddonContentPolicy : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IADDONCONTENTPOLICY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAddonContentPolicy;

  enum {
    CSP_ALLOW_ANY = 65535U,
    CSP_ALLOW_LOCALHOST = 1U,
    CSP_ALLOW_EVAL = 2U,
    CSP_ALLOW_REMOTE = 4U
  };

  /* AString validateAddonCSP (in AString aPolicyString, in unsigned long aPermittedPolicy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ValidateAddonCSP(const nsAString& aPolicyString, uint32_t aPermittedPolicy, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAddonContentPolicy, NS_IADDONCONTENTPOLICY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIADDONCONTENTPOLICY \
  NS_IMETHOD ValidateAddonCSP(const nsAString& aPolicyString, uint32_t aPermittedPolicy, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIADDONCONTENTPOLICY \
  nsresult ValidateAddonCSP(const nsAString& aPolicyString, uint32_t aPermittedPolicy, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIADDONCONTENTPOLICY(_to) \
  NS_IMETHOD ValidateAddonCSP(const nsAString& aPolicyString, uint32_t aPermittedPolicy, nsAString& _retval) override { return _to ValidateAddonCSP(aPolicyString, aPermittedPolicy, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIADDONCONTENTPOLICY(_to) \
  NS_IMETHOD ValidateAddonCSP(const nsAString& aPolicyString, uint32_t aPermittedPolicy, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ValidateAddonCSP(aPolicyString, aPermittedPolicy, _retval); } 


#endif /* __gen_nsIAddonPolicyService_h__ */
