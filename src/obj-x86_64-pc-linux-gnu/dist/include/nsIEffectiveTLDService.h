/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIEffectiveTLDService.idl
 */

#ifndef __gen_nsIEffectiveTLDService_h__
#define __gen_nsIEffectiveTLDService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */


/* starting interface:    nsIEffectiveTLDService */
#define NS_IEFFECTIVETLDSERVICE_IID_STR "68067eb5-ad8d-43cb-a043-1cc85ebe06e7"

#define NS_IEFFECTIVETLDSERVICE_IID \
  {0x68067eb5, 0xad8d, 0x43cb, \
    { 0xa0, 0x43, 0x1c, 0xc8, 0x5e, 0xbe, 0x06, 0xe7 }}

class NS_NO_VTABLE nsIEffectiveTLDService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IEFFECTIVETLDSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEffectiveTLDService;

  /* ACString getPublicSuffix (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPublicSuffix(nsIURI *aURI, nsACString& _retval) = 0;

  /* ACString getKnownPublicSuffix (in nsIURI aURI); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKnownPublicSuffix(nsIURI *aURI, nsACString& _retval) = 0;

  /* ACString getBaseDomain (in nsIURI aURI, [optional] in uint32_t aAdditionalParts); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBaseDomain(nsIURI *aURI, uint32_t aAdditionalParts, nsACString& _retval) = 0;

  /* ACString getPublicSuffixFromHost (in AUTF8String aHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) = 0;

  /* ACString getKnownPublicSuffixFromHost (in AUTF8String aHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKnownPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) = 0;

  /* ACString getBaseDomainFromHost (in AUTF8String aHost, [optional] in uint32_t aAdditionalParts); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBaseDomainFromHost(const nsACString& aHost, uint32_t aAdditionalParts, nsACString& _retval) = 0;

  /* ACString getNextSubDomain (in AUTF8String aHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextSubDomain(const nsACString& aHost, nsACString& _retval) = 0;

  /* bool hasRootDomain (in AUTF8String aInput, in AUTF8String aHost); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasRootDomain(const nsACString& aInput, const nsACString& aHost, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEffectiveTLDService, NS_IEFFECTIVETLDSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIEFFECTIVETLDSERVICE \
  NS_IMETHOD GetPublicSuffix(nsIURI *aURI, nsACString& _retval) override; \
  NS_IMETHOD GetKnownPublicSuffix(nsIURI *aURI, nsACString& _retval) override; \
  NS_IMETHOD GetBaseDomain(nsIURI *aURI, uint32_t aAdditionalParts, nsACString& _retval) override; \
  NS_IMETHOD GetPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override; \
  NS_IMETHOD GetKnownPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override; \
  NS_IMETHOD GetBaseDomainFromHost(const nsACString& aHost, uint32_t aAdditionalParts, nsACString& _retval) override; \
  NS_IMETHOD GetNextSubDomain(const nsACString& aHost, nsACString& _retval) override; \
  NS_IMETHOD HasRootDomain(const nsACString& aInput, const nsACString& aHost, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIEFFECTIVETLDSERVICE \
  nsresult GetPublicSuffix(nsIURI *aURI, nsACString& _retval); \
  nsresult GetKnownPublicSuffix(nsIURI *aURI, nsACString& _retval); \
  nsresult GetBaseDomain(nsIURI *aURI, uint32_t aAdditionalParts, nsACString& _retval); \
  nsresult GetPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval); \
  nsresult GetKnownPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval); \
  nsresult GetBaseDomainFromHost(const nsACString& aHost, uint32_t aAdditionalParts, nsACString& _retval); \
  nsresult GetNextSubDomain(const nsACString& aHost, nsACString& _retval); \
  nsresult HasRootDomain(const nsACString& aInput, const nsACString& aHost, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIEFFECTIVETLDSERVICE(_to) \
  NS_IMETHOD GetPublicSuffix(nsIURI *aURI, nsACString& _retval) override { return _to GetPublicSuffix(aURI, _retval); } \
  NS_IMETHOD GetKnownPublicSuffix(nsIURI *aURI, nsACString& _retval) override { return _to GetKnownPublicSuffix(aURI, _retval); } \
  NS_IMETHOD GetBaseDomain(nsIURI *aURI, uint32_t aAdditionalParts, nsACString& _retval) override { return _to GetBaseDomain(aURI, aAdditionalParts, _retval); } \
  NS_IMETHOD GetPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override { return _to GetPublicSuffixFromHost(aHost, _retval); } \
  NS_IMETHOD GetKnownPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override { return _to GetKnownPublicSuffixFromHost(aHost, _retval); } \
  NS_IMETHOD GetBaseDomainFromHost(const nsACString& aHost, uint32_t aAdditionalParts, nsACString& _retval) override { return _to GetBaseDomainFromHost(aHost, aAdditionalParts, _retval); } \
  NS_IMETHOD GetNextSubDomain(const nsACString& aHost, nsACString& _retval) override { return _to GetNextSubDomain(aHost, _retval); } \
  NS_IMETHOD HasRootDomain(const nsACString& aInput, const nsACString& aHost, bool *_retval) override { return _to HasRootDomain(aInput, aHost, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIEFFECTIVETLDSERVICE(_to) \
  NS_IMETHOD GetPublicSuffix(nsIURI *aURI, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPublicSuffix(aURI, _retval); } \
  NS_IMETHOD GetKnownPublicSuffix(nsIURI *aURI, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKnownPublicSuffix(aURI, _retval); } \
  NS_IMETHOD GetBaseDomain(nsIURI *aURI, uint32_t aAdditionalParts, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseDomain(aURI, aAdditionalParts, _retval); } \
  NS_IMETHOD GetPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPublicSuffixFromHost(aHost, _retval); } \
  NS_IMETHOD GetKnownPublicSuffixFromHost(const nsACString& aHost, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKnownPublicSuffixFromHost(aHost, _retval); } \
  NS_IMETHOD GetBaseDomainFromHost(const nsACString& aHost, uint32_t aAdditionalParts, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseDomainFromHost(aHost, aAdditionalParts, _retval); } \
  NS_IMETHOD GetNextSubDomain(const nsACString& aHost, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextSubDomain(aHost, _retval); } \
  NS_IMETHOD HasRootDomain(const nsACString& aInput, const nsACString& aHost, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasRootDomain(aInput, aHost, _retval); } 


#endif /* __gen_nsIEffectiveTLDService_h__ */
