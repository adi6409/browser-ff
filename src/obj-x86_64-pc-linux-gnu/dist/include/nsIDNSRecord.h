/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSRecord.idl
 */

#ifndef __gen_nsIDNSRecord_h__
#define __gen_nsIDNSRecord_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace net {
union NetAddr;
}
}
#include "nsTArrayForwardDeclare.h"
class nsINetAddr; /* forward declaration */


/* starting interface:    nsIDNSRecord */
#define NS_IDNSRECORD_IID_STR "f92228ae-c417-4188-a604-0830a95e7eb9"

#define NS_IDNSRECORD_IID \
  {0xf92228ae, 0xc417, 0x4188, \
    { 0xa6, 0x04, 0x08, 0x30, 0xa9, 0x5e, 0x7e, 0xb9 }}

class NS_NO_VTABLE nsIDNSRecord : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSRecord;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSRecord, NS_IDNSRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSRECORD \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSRECORD \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSRECORD(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSRECORD(_to) \
  /* no methods! */


/* starting interface:    nsIDNSAddrRecord */
#define NS_IDNSADDRRECORD_IID_STR "cb260e20-943f-4309-953b-78c90d3a7638"

#define NS_IDNSADDRRECORD_IID \
  {0xcb260e20, 0x943f, 0x4309, \
    { 0x95, 0x3b, 0x78, 0xc9, 0x0d, 0x3a, 0x76, 0x38 }}

class NS_NO_VTABLE nsIDNSAddrRecord : public nsIDNSRecord {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSADDRRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSAddrRecord;

  /* readonly attribute ACString canonicalName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCanonicalName(nsACString& aCanonicalName) = 0;

  /* [noscript] NetAddr getNextAddr (in uint16_t aPort); */
  NS_IMETHOD GetNextAddr(uint16_t aPort, mozilla::net::NetAddr * _retval) = 0;

  /* [noscript] void getAddresses (out nsNetAddrTArrayRef aAddressArray); */
  NS_IMETHOD GetAddresses(nsTArray<mozilla::net::NetAddr> & aAddressArray) = 0;

  /* nsINetAddr getScriptableNextAddr (in uint16_t aPort); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetScriptableNextAddr(uint16_t aPort, nsINetAddr **_retval) = 0;

  /* ACString getNextAddrAsString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNextAddrAsString(nsACString& _retval) = 0;

  /* boolean hasMore (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HasMore(bool *_retval) = 0;

  /* void rewind (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Rewind(void) = 0;

  /* void reportUnusable (in uint16_t aPort); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReportUnusable(uint16_t aPort) = 0;

  /* bool IsTRR (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsTRR(bool *_retval) = 0;

  /* readonly attribute double trrFetchDuration; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTrrFetchDuration(double *aTrrFetchDuration) = 0;

  /* readonly attribute double trrFetchDurationNetworkOnly; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTrrFetchDurationNetworkOnly(double *aTrrFetchDurationNetworkOnly) = 0;

  /* readonly attribute unsigned long effectiveTRRMode; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEffectiveTRRMode(uint32_t *aEffectiveTRRMode) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSAddrRecord, NS_IDNSADDRRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSADDRRECORD \
  NS_IMETHOD GetCanonicalName(nsACString& aCanonicalName) override; \
  NS_IMETHOD GetNextAddr(uint16_t aPort, mozilla::net::NetAddr * _retval) override; \
  NS_IMETHOD GetAddresses(nsTArray<mozilla::net::NetAddr> & aAddressArray) override; \
  NS_IMETHOD GetScriptableNextAddr(uint16_t aPort, nsINetAddr **_retval) override; \
  NS_IMETHOD GetNextAddrAsString(nsACString& _retval) override; \
  NS_IMETHOD HasMore(bool *_retval) override; \
  NS_IMETHOD Rewind(void) override; \
  NS_IMETHOD ReportUnusable(uint16_t aPort) override; \
  NS_IMETHOD IsTRR(bool *_retval) override; \
  NS_IMETHOD GetTrrFetchDuration(double *aTrrFetchDuration) override; \
  NS_IMETHOD GetTrrFetchDurationNetworkOnly(double *aTrrFetchDurationNetworkOnly) override; \
  NS_IMETHOD GetEffectiveTRRMode(uint32_t *aEffectiveTRRMode) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSADDRRECORD \
  nsresult GetCanonicalName(nsACString& aCanonicalName); \
  nsresult GetNextAddr(uint16_t aPort, mozilla::net::NetAddr * _retval); \
  nsresult GetAddresses(nsTArray<mozilla::net::NetAddr> & aAddressArray); \
  nsresult GetScriptableNextAddr(uint16_t aPort, nsINetAddr **_retval); \
  nsresult GetNextAddrAsString(nsACString& _retval); \
  nsresult HasMore(bool *_retval); \
  nsresult Rewind(void); \
  nsresult ReportUnusable(uint16_t aPort); \
  nsresult IsTRR(bool *_retval); \
  nsresult GetTrrFetchDuration(double *aTrrFetchDuration); \
  nsresult GetTrrFetchDurationNetworkOnly(double *aTrrFetchDurationNetworkOnly); \
  nsresult GetEffectiveTRRMode(uint32_t *aEffectiveTRRMode); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSADDRRECORD(_to) \
  NS_IMETHOD GetCanonicalName(nsACString& aCanonicalName) override { return _to GetCanonicalName(aCanonicalName); } \
  NS_IMETHOD GetNextAddr(uint16_t aPort, mozilla::net::NetAddr * _retval) override { return _to GetNextAddr(aPort, _retval); } \
  NS_IMETHOD GetAddresses(nsTArray<mozilla::net::NetAddr> & aAddressArray) override { return _to GetAddresses(aAddressArray); } \
  NS_IMETHOD GetScriptableNextAddr(uint16_t aPort, nsINetAddr **_retval) override { return _to GetScriptableNextAddr(aPort, _retval); } \
  NS_IMETHOD GetNextAddrAsString(nsACString& _retval) override { return _to GetNextAddrAsString(_retval); } \
  NS_IMETHOD HasMore(bool *_retval) override { return _to HasMore(_retval); } \
  NS_IMETHOD Rewind(void) override { return _to Rewind(); } \
  NS_IMETHOD ReportUnusable(uint16_t aPort) override { return _to ReportUnusable(aPort); } \
  NS_IMETHOD IsTRR(bool *_retval) override { return _to IsTRR(_retval); } \
  NS_IMETHOD GetTrrFetchDuration(double *aTrrFetchDuration) override { return _to GetTrrFetchDuration(aTrrFetchDuration); } \
  NS_IMETHOD GetTrrFetchDurationNetworkOnly(double *aTrrFetchDurationNetworkOnly) override { return _to GetTrrFetchDurationNetworkOnly(aTrrFetchDurationNetworkOnly); } \
  NS_IMETHOD GetEffectiveTRRMode(uint32_t *aEffectiveTRRMode) override { return _to GetEffectiveTRRMode(aEffectiveTRRMode); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSADDRRECORD(_to) \
  NS_IMETHOD GetCanonicalName(nsACString& aCanonicalName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCanonicalName(aCanonicalName); } \
  NS_IMETHOD GetNextAddr(uint16_t aPort, mozilla::net::NetAddr * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextAddr(aPort, _retval); } \
  NS_IMETHOD GetAddresses(nsTArray<mozilla::net::NetAddr> & aAddressArray) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAddresses(aAddressArray); } \
  NS_IMETHOD GetScriptableNextAddr(uint16_t aPort, nsINetAddr **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScriptableNextAddr(aPort, _retval); } \
  NS_IMETHOD GetNextAddrAsString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextAddrAsString(_retval); } \
  NS_IMETHOD HasMore(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasMore(_retval); } \
  NS_IMETHOD Rewind(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Rewind(); } \
  NS_IMETHOD ReportUnusable(uint16_t aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReportUnusable(aPort); } \
  NS_IMETHOD IsTRR(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsTRR(_retval); } \
  NS_IMETHOD GetTrrFetchDuration(double *aTrrFetchDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTrrFetchDuration(aTrrFetchDuration); } \
  NS_IMETHOD GetTrrFetchDurationNetworkOnly(double *aTrrFetchDurationNetworkOnly) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTrrFetchDurationNetworkOnly(aTrrFetchDurationNetworkOnly); } \
  NS_IMETHOD GetEffectiveTRRMode(uint32_t *aEffectiveTRRMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEffectiveTRRMode(aEffectiveTRRMode); } 


#endif /* __gen_nsIDNSRecord_h__ */
