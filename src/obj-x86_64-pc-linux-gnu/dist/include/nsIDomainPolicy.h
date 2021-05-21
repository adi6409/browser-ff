/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/caps/nsIDomainPolicy.idl
 */

#ifndef __gen_nsIDomainPolicy_h__
#define __gen_nsIDomainPolicy_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIDomainSet; /* forward declaration */

namespace mozilla {
namespace dom {
class DomainPolicyClone;
}
}

/* starting interface:    nsIDomainPolicy */
#define NS_IDOMAINPOLICY_IID_STR "82b24a20-6701-4d40-a0f9-f5dc7321b555"

#define NS_IDOMAINPOLICY_IID \
  {0x82b24a20, 0x6701, 0x4d40, \
    { 0xa0, 0xf9, 0xf5, 0xdc, 0x73, 0x21, 0xb5, 0x55 }}

class NS_NO_VTABLE nsIDomainPolicy : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMAINPOLICY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDomainPolicy;

  /* readonly attribute nsIDomainSet blocklist; */
  NS_IMETHOD GetBlocklist(nsIDomainSet **aBlocklist) = 0;

  /* readonly attribute nsIDomainSet superBlocklist; */
  NS_IMETHOD GetSuperBlocklist(nsIDomainSet **aSuperBlocklist) = 0;

  /* readonly attribute nsIDomainSet allowlist; */
  NS_IMETHOD GetAllowlist(nsIDomainSet **aAllowlist) = 0;

  /* readonly attribute nsIDomainSet superAllowlist; */
  NS_IMETHOD GetSuperAllowlist(nsIDomainSet **aSuperAllowlist) = 0;

  /* void deactivate (); */
  NS_IMETHOD Deactivate(void) = 0;

  /* [noscript,notxpcom] void cloneDomainPolicy (in DomainPolicyClonePtr aClone); */
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) = 0;

  /* [noscript,notxpcom] void applyClone (in DomainPolicyCloneConstPtr aClone); */
  NS_IMETHOD_(void) ApplyClone(const mozilla::dom::DomainPolicyClone * aClone) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDomainPolicy, NS_IDOMAINPOLICY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMAINPOLICY \
  NS_IMETHOD GetBlocklist(nsIDomainSet **aBlocklist) override; \
  NS_IMETHOD GetSuperBlocklist(nsIDomainSet **aSuperBlocklist) override; \
  NS_IMETHOD GetAllowlist(nsIDomainSet **aAllowlist) override; \
  NS_IMETHOD GetSuperAllowlist(nsIDomainSet **aSuperAllowlist) override; \
  NS_IMETHOD Deactivate(void) override; \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override; \
  NS_IMETHOD_(void) ApplyClone(const mozilla::dom::DomainPolicyClone * aClone) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMAINPOLICY \
  nsresult GetBlocklist(nsIDomainSet **aBlocklist); \
  nsresult GetSuperBlocklist(nsIDomainSet **aSuperBlocklist); \
  nsresult GetAllowlist(nsIDomainSet **aAllowlist); \
  nsresult GetSuperAllowlist(nsIDomainSet **aSuperAllowlist); \
  nsresult Deactivate(void); \
  nsresult_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone); \
  nsresult_(void) ApplyClone(const mozilla::dom::DomainPolicyClone * aClone); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMAINPOLICY(_to) \
  NS_IMETHOD GetBlocklist(nsIDomainSet **aBlocklist) override { return _to GetBlocklist(aBlocklist); } \
  NS_IMETHOD GetSuperBlocklist(nsIDomainSet **aSuperBlocklist) override { return _to GetSuperBlocklist(aSuperBlocklist); } \
  NS_IMETHOD GetAllowlist(nsIDomainSet **aAllowlist) override { return _to GetAllowlist(aAllowlist); } \
  NS_IMETHOD GetSuperAllowlist(nsIDomainSet **aSuperAllowlist) override { return _to GetSuperAllowlist(aSuperAllowlist); } \
  NS_IMETHOD Deactivate(void) override { return _to Deactivate(); } \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override { return _to CloneDomainPolicy(aClone); } \
  NS_IMETHOD_(void) ApplyClone(const mozilla::dom::DomainPolicyClone * aClone) override { return _to ApplyClone(aClone); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMAINPOLICY(_to) \
  NS_IMETHOD GetBlocklist(nsIDomainSet **aBlocklist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBlocklist(aBlocklist); } \
  NS_IMETHOD GetSuperBlocklist(nsIDomainSet **aSuperBlocklist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuperBlocklist(aSuperBlocklist); } \
  NS_IMETHOD GetAllowlist(nsIDomainSet **aAllowlist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowlist(aAllowlist); } \
  NS_IMETHOD GetSuperAllowlist(nsIDomainSet **aSuperAllowlist) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuperAllowlist(aSuperAllowlist); } \
  NS_IMETHOD Deactivate(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Deactivate(); } \
  NS_IMETHOD_(void) CloneDomainPolicy(mozilla::dom::DomainPolicyClone * aClone) override; \
  NS_IMETHOD_(void) ApplyClone(const mozilla::dom::DomainPolicyClone * aClone) override; 


/* starting interface:    nsIDomainSet */
#define NS_IDOMAINSET_IID_STR "665c981b-0a0f-4229-ac06-a826e02d4f69"

#define NS_IDOMAINSET_IID \
  {0x665c981b, 0x0a0f, 0x4229, \
    { 0xac, 0x06, 0xa8, 0x26, 0xe0, 0x2d, 0x4f, 0x69 }}

class NS_NO_VTABLE nsIDomainSet : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMAINSET_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDomainSet;

  /* void add (in nsIURI aDomain); */
  NS_IMETHOD Add(nsIURI *aDomain) = 0;

  /* void remove (in nsIURI aDomain); */
  NS_IMETHOD Remove(nsIURI *aDomain) = 0;

  /* void clear (); */
  NS_IMETHOD Clear(void) = 0;

  /* bool contains (in nsIURI aDomain); */
  NS_IMETHOD Contains(nsIURI *aDomain, bool *_retval) = 0;

  /* bool containsSuperDomain (in nsIURI aDomain); */
  NS_IMETHOD ContainsSuperDomain(nsIURI *aDomain, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDomainSet, NS_IDOMAINSET_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMAINSET \
  NS_IMETHOD Add(nsIURI *aDomain) override; \
  NS_IMETHOD Remove(nsIURI *aDomain) override; \
  NS_IMETHOD Clear(void) override; \
  NS_IMETHOD Contains(nsIURI *aDomain, bool *_retval) override; \
  NS_IMETHOD ContainsSuperDomain(nsIURI *aDomain, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMAINSET \
  nsresult Add(nsIURI *aDomain); \
  nsresult Remove(nsIURI *aDomain); \
  nsresult Clear(void); \
  nsresult Contains(nsIURI *aDomain, bool *_retval); \
  nsresult ContainsSuperDomain(nsIURI *aDomain, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMAINSET(_to) \
  NS_IMETHOD Add(nsIURI *aDomain) override { return _to Add(aDomain); } \
  NS_IMETHOD Remove(nsIURI *aDomain) override { return _to Remove(aDomain); } \
  NS_IMETHOD Clear(void) override { return _to Clear(); } \
  NS_IMETHOD Contains(nsIURI *aDomain, bool *_retval) override { return _to Contains(aDomain, _retval); } \
  NS_IMETHOD ContainsSuperDomain(nsIURI *aDomain, bool *_retval) override { return _to ContainsSuperDomain(aDomain, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMAINSET(_to) \
  NS_IMETHOD Add(nsIURI *aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Add(aDomain); } \
  NS_IMETHOD Remove(nsIURI *aDomain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Remove(aDomain); } \
  NS_IMETHOD Clear(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Clear(); } \
  NS_IMETHOD Contains(nsIURI *aDomain, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Contains(aDomain, _retval); } \
  NS_IMETHOD ContainsSuperDomain(nsIURI *aDomain, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContainsSuperDomain(aDomain, _retval); } 


#endif /* __gen_nsIDomainPolicy_h__ */
