/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPermission.idl
 */

#ifndef __gen_nsIPermission_h__
#define __gen_nsIPermission_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIPermission */
#define NS_IPERMISSION_IID_STR "bb409a51-2371-4fea-9dc9-b7286a458b8c"

#define NS_IPERMISSION_IID \
  {0xbb409a51, 0x2371, 0x4fea, \
    { 0x9d, 0xc9, 0xb7, 0x28, 0x6a, 0x45, 0x8b, 0x8c }}

class NS_NO_VTABLE nsIPermission : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPERMISSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPermission;

  /* readonly attribute nsIPrincipal principal; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) = 0;

  /* readonly attribute ACString type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(nsACString& aType) = 0;

  /* readonly attribute uint32_t capability; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCapability(uint32_t *aCapability) = 0;

  /* readonly attribute uint32_t expireType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpireType(uint32_t *aExpireType) = 0;

  /* readonly attribute int64_t expireTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetExpireTime(int64_t *aExpireTime) = 0;

  /* readonly attribute int64_t modificationTime; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetModificationTime(int64_t *aModificationTime) = 0;

  /* boolean matches (in nsIPrincipal principal, in boolean exactHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Matches(nsIPrincipal *principal, bool exactHost, bool *_retval) = 0;

  /* [noscript] boolean matchesPrincipalForPermission (in nsIPrincipal principal, in boolean exactHost); */
  NS_IMETHOD MatchesPrincipalForPermission(nsIPrincipal *principal, bool exactHost, bool *_retval) = 0;

  /* boolean matchesURI (in nsIURI uri, in boolean exactHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MatchesURI(nsIURI *uri, bool exactHost, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPermission, NS_IPERMISSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPERMISSION \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override; \
  NS_IMETHOD GetType(nsACString& aType) override; \
  NS_IMETHOD GetCapability(uint32_t *aCapability) override; \
  NS_IMETHOD GetExpireType(uint32_t *aExpireType) override; \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override; \
  NS_IMETHOD GetModificationTime(int64_t *aModificationTime) override; \
  NS_IMETHOD Matches(nsIPrincipal *principal, bool exactHost, bool *_retval) override; \
  NS_IMETHOD MatchesPrincipalForPermission(nsIPrincipal *principal, bool exactHost, bool *_retval) override; \
  NS_IMETHOD MatchesURI(nsIURI *uri, bool exactHost, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPERMISSION \
  nsresult GetPrincipal(nsIPrincipal **aPrincipal); \
  nsresult GetType(nsACString& aType); \
  nsresult GetCapability(uint32_t *aCapability); \
  nsresult GetExpireType(uint32_t *aExpireType); \
  nsresult GetExpireTime(int64_t *aExpireTime); \
  nsresult GetModificationTime(int64_t *aModificationTime); \
  nsresult Matches(nsIPrincipal *principal, bool exactHost, bool *_retval); \
  nsresult MatchesPrincipalForPermission(nsIPrincipal *principal, bool exactHost, bool *_retval); \
  nsresult MatchesURI(nsIURI *uri, bool exactHost, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPERMISSION(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return _to GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetType(nsACString& aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetCapability(uint32_t *aCapability) override { return _to GetCapability(aCapability); } \
  NS_IMETHOD GetExpireType(uint32_t *aExpireType) override { return _to GetExpireType(aExpireType); } \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override { return _to GetExpireTime(aExpireTime); } \
  NS_IMETHOD GetModificationTime(int64_t *aModificationTime) override { return _to GetModificationTime(aModificationTime); } \
  NS_IMETHOD Matches(nsIPrincipal *principal, bool exactHost, bool *_retval) override { return _to Matches(principal, exactHost, _retval); } \
  NS_IMETHOD MatchesPrincipalForPermission(nsIPrincipal *principal, bool exactHost, bool *_retval) override { return _to MatchesPrincipalForPermission(principal, exactHost, _retval); } \
  NS_IMETHOD MatchesURI(nsIURI *uri, bool exactHost, bool *_retval) override { return _to MatchesURI(uri, exactHost, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPERMISSION(_to) \
  NS_IMETHOD GetPrincipal(nsIPrincipal **aPrincipal) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrincipal(aPrincipal); } \
  NS_IMETHOD GetType(nsACString& aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetCapability(uint32_t *aCapability) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCapability(aCapability); } \
  NS_IMETHOD GetExpireType(uint32_t *aExpireType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpireType(aExpireType); } \
  NS_IMETHOD GetExpireTime(int64_t *aExpireTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExpireTime(aExpireTime); } \
  NS_IMETHOD GetModificationTime(int64_t *aModificationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModificationTime(aModificationTime); } \
  NS_IMETHOD Matches(nsIPrincipal *principal, bool exactHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Matches(principal, exactHost, _retval); } \
  NS_IMETHOD MatchesPrincipalForPermission(nsIPrincipal *principal, bool exactHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MatchesPrincipalForPermission(principal, exactHost, _retval); } \
  NS_IMETHOD MatchesURI(nsIURI *uri, bool exactHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MatchesURI(uri, exactHost, _retval); } 


#endif /* __gen_nsIPermission_h__ */
