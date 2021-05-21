/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSByTypeRecord.idl
 */

#ifndef __gen_nsIDNSByTypeRecord_h__
#define __gen_nsIDNSByTypeRecord_h__


#ifndef __gen_nsIDNSRecord_h__
#include "nsIDNSRecord.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

#include "mozilla/Maybe.h"
#include "mozilla/Tuple.h"
#include "nsTArrayForwardDeclare.h"
#include "nsStringFwd.h"
namespace mozilla {
template <typename... Ts> class Variant;
struct Nothing;
namespace net {
  struct SVCB;
  using TypeRecordResultType =
    Variant<Nothing, CopyableTArray<nsCString>, CopyableTArray<SVCB>>;
}
}

/* starting interface:    nsIDNSByTypeRecord */
#define NS_IDNSBYTYPERECORD_IID_STR "5d13241b-9d46-448a-90d8-77c418491026"

#define NS_IDNSBYTYPERECORD_IID \
  {0x5d13241b, 0x9d46, 0x448a, \
    { 0x90, 0xd8, 0x77, 0xc4, 0x18, 0x49, 0x10, 0x26 }}

class NS_NO_VTABLE nsIDNSByTypeRecord : public nsIDNSRecord {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSBYTYPERECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSByTypeRecord;

  /* readonly attribute unsigned long type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint32_t *aType) = 0;

  /* [noscript] readonly attribute TypeResult results; */
  NS_IMETHOD GetResults(mozilla::net::TypeRecordResultType * aResults) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSByTypeRecord, NS_IDNSBYTYPERECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSBYTYPERECORD \
  NS_IMETHOD GetType(uint32_t *aType) override; \
  NS_IMETHOD GetResults(mozilla::net::TypeRecordResultType * aResults) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSBYTYPERECORD \
  nsresult GetType(uint32_t *aType); \
  nsresult GetResults(mozilla::net::TypeRecordResultType * aResults); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSBYTYPERECORD(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return _to GetType(aType); } \
  NS_IMETHOD GetResults(mozilla::net::TypeRecordResultType * aResults) override { return _to GetResults(aResults); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSBYTYPERECORD(_to) \
  NS_IMETHOD GetType(uint32_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } \
  NS_IMETHOD GetResults(mozilla::net::TypeRecordResultType * aResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResults(aResults); } 


/* starting interface:    nsIDNSTXTRecord */
#define NS_IDNSTXTRECORD_IID_STR "2a71750d-cb21-45f1-9e1c-666d18dd7645"

#define NS_IDNSTXTRECORD_IID \
  {0x2a71750d, 0xcb21, 0x45f1, \
    { 0x9e, 0x1c, 0x66, 0x6d, 0x18, 0xdd, 0x76, 0x45 }}

class NS_NO_VTABLE nsIDNSTXTRecord : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSTXTRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSTXTRecord;

  /* CStringArrayRef getRecords (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRecords(CopyableTArray<nsCString> & _retval) = 0;

  /* ACString getRecordsAsOneString (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRecordsAsOneString(nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSTXTRecord, NS_IDNSTXTRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSTXTRECORD \
  NS_IMETHOD GetRecords(CopyableTArray<nsCString> & _retval) override; \
  NS_IMETHOD GetRecordsAsOneString(nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSTXTRECORD \
  nsresult GetRecords(CopyableTArray<nsCString> & _retval); \
  nsresult GetRecordsAsOneString(nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSTXTRECORD(_to) \
  NS_IMETHOD GetRecords(CopyableTArray<nsCString> & _retval) override { return _to GetRecords(_retval); } \
  NS_IMETHOD GetRecordsAsOneString(nsACString& _retval) override { return _to GetRecordsAsOneString(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSTXTRECORD(_to) \
  NS_IMETHOD GetRecords(CopyableTArray<nsCString> & _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecords(_retval); } \
  NS_IMETHOD GetRecordsAsOneString(nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecordsAsOneString(_retval); } 


/* starting interface:    nsISVCParam */
#define NS_ISVCPARAM_IID_STR "2979ceaa-9c7e-49de-84b8-ea81c16aebf1"

#define NS_ISVCPARAM_IID \
  {0x2979ceaa, 0x9c7e, 0x49de, \
    { 0x84, 0xb8, 0xea, 0x81, 0xc1, 0x6a, 0xeb, 0xf1 }}

class NS_NO_VTABLE nsISVCParam : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParam;

  /* readonly attribute uint16_t type; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetType(uint16_t *aType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParam, NS_ISVCPARAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAM \
  NS_IMETHOD GetType(uint16_t *aType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAM \
  nsresult GetType(uint16_t *aType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAM(_to) \
  NS_IMETHOD GetType(uint16_t *aType) override { return _to GetType(aType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAM(_to) \
  NS_IMETHOD GetType(uint16_t *aType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetType(aType); } 


/* starting interface:    nsISVCParamAlpn */
#define NS_ISVCPARAMALPN_IID_STR "0dc58309-4d67-4fc4-a4e3-38dbde9d9f4c"

#define NS_ISVCPARAMALPN_IID \
  {0x0dc58309, 0x4d67, 0x4fc4, \
    { 0xa4, 0xe3, 0x38, 0xdb, 0xde, 0x9d, 0x9f, 0x4c }}

class NS_NO_VTABLE nsISVCParamAlpn : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMALPN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamAlpn;

  /* readonly attribute Array<ACString> alpn; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlpn(nsTArray<nsCString >& aAlpn) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamAlpn, NS_ISVCPARAMALPN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMALPN \
  NS_IMETHOD GetAlpn(nsTArray<nsCString >& aAlpn) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMALPN \
  nsresult GetAlpn(nsTArray<nsCString >& aAlpn); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMALPN(_to) \
  NS_IMETHOD GetAlpn(nsTArray<nsCString >& aAlpn) override { return _to GetAlpn(aAlpn); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMALPN(_to) \
  NS_IMETHOD GetAlpn(nsTArray<nsCString >& aAlpn) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlpn(aAlpn); } 


/* starting interface:    nsISVCParamNoDefaultAlpn */
#define NS_ISVCPARAMNODEFAULTALPN_IID_STR "b3ed89c3-2ae6-4c92-8176-b76bc2437fcb"

#define NS_ISVCPARAMNODEFAULTALPN_IID \
  {0xb3ed89c3, 0x2ae6, 0x4c92, \
    { 0x81, 0x76, 0xb7, 0x6b, 0xc2, 0x43, 0x7f, 0xcb }}

class NS_NO_VTABLE nsISVCParamNoDefaultAlpn : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMNODEFAULTALPN_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamNoDefaultAlpn;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamNoDefaultAlpn, NS_ISVCPARAMNODEFAULTALPN_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMNODEFAULTALPN \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMNODEFAULTALPN \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMNODEFAULTALPN(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMNODEFAULTALPN(_to) \
  /* no methods! */


/* starting interface:    nsISVCParamPort */
#define NS_ISVCPARAMPORT_IID_STR "a37c7bcb-bfcd-4ab4-b826-cc583859ba73"

#define NS_ISVCPARAMPORT_IID \
  {0xa37c7bcb, 0xbfcd, 0x4ab4, \
    { 0xb8, 0x26, 0xcc, 0x58, 0x38, 0x59, 0xba, 0x73 }}

class NS_NO_VTABLE nsISVCParamPort : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMPORT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamPort;

  /* readonly attribute uint16_t port; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPort(uint16_t *aPort) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamPort, NS_ISVCPARAMPORT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMPORT \
  NS_IMETHOD GetPort(uint16_t *aPort) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMPORT \
  nsresult GetPort(uint16_t *aPort); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMPORT(_to) \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return _to GetPort(aPort); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMPORT(_to) \
  NS_IMETHOD GetPort(uint16_t *aPort) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPort(aPort); } 


/* starting interface:    nsISVCParamIPv4Hint */
#define NS_ISVCPARAMIPV4HINT_IID_STR "d3163d2f-0bbe-47d4-bcac-db3fb1433b39"

#define NS_ISVCPARAMIPV4HINT_IID \
  {0xd3163d2f, 0x0bbe, 0x47d4, \
    { 0xbc, 0xac, 0xdb, 0x3f, 0xb1, 0x43, 0x3b, 0x39 }}

class NS_NO_VTABLE nsISVCParamIPv4Hint : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMIPV4HINT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamIPv4Hint;

  /* readonly attribute Array<nsINetAddr> ipv4Hint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIpv4Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv4Hint) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamIPv4Hint, NS_ISVCPARAMIPV4HINT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMIPV4HINT \
  NS_IMETHOD GetIpv4Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv4Hint) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMIPV4HINT \
  nsresult GetIpv4Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv4Hint); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMIPV4HINT(_to) \
  NS_IMETHOD GetIpv4Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv4Hint) override { return _to GetIpv4Hint(aIpv4Hint); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMIPV4HINT(_to) \
  NS_IMETHOD GetIpv4Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv4Hint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIpv4Hint(aIpv4Hint); } 


/* starting interface:    nsISVCParamEchConfig */
#define NS_ISVCPARAMECHCONFIG_IID_STR "1f31e41d-b6d8-4796-b12a-82ef8d2b0e43"

#define NS_ISVCPARAMECHCONFIG_IID \
  {0x1f31e41d, 0xb6d8, 0x4796, \
    { 0xb1, 0x2a, 0x82, 0xef, 0x8d, 0x2b, 0x0e, 0x43 }}

class NS_NO_VTABLE nsISVCParamEchConfig : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMECHCONFIG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamEchConfig;

  /* readonly attribute ACString echconfig; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEchconfig(nsACString& aEchconfig) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamEchConfig, NS_ISVCPARAMECHCONFIG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMECHCONFIG \
  NS_IMETHOD GetEchconfig(nsACString& aEchconfig) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMECHCONFIG \
  nsresult GetEchconfig(nsACString& aEchconfig); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMECHCONFIG(_to) \
  NS_IMETHOD GetEchconfig(nsACString& aEchconfig) override { return _to GetEchconfig(aEchconfig); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMECHCONFIG(_to) \
  NS_IMETHOD GetEchconfig(nsACString& aEchconfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEchconfig(aEchconfig); } 


/* starting interface:    nsISVCParamIPv6Hint */
#define NS_ISVCPARAMIPV6HINT_IID_STR "5100bce4-9d3b-42e1-a3c9-0f386bbc9dad"

#define NS_ISVCPARAMIPV6HINT_IID \
  {0x5100bce4, 0x9d3b, 0x42e1, \
    { 0xa3, 0xc9, 0x0f, 0x38, 0x6b, 0xbc, 0x9d, 0xad }}

class NS_NO_VTABLE nsISVCParamIPv6Hint : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMIPV6HINT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamIPv6Hint;

  /* readonly attribute Array<nsINetAddr> ipv6Hint; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIpv6Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv6Hint) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamIPv6Hint, NS_ISVCPARAMIPV6HINT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMIPV6HINT \
  NS_IMETHOD GetIpv6Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv6Hint) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMIPV6HINT \
  nsresult GetIpv6Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv6Hint); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMIPV6HINT(_to) \
  NS_IMETHOD GetIpv6Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv6Hint) override { return _to GetIpv6Hint(aIpv6Hint); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMIPV6HINT(_to) \
  NS_IMETHOD GetIpv6Hint(nsTArray<RefPtr<nsINetAddr>>& aIpv6Hint) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIpv6Hint(aIpv6Hint); } 


/* starting interface:    nsISVCParamODoHConfig */
#define NS_ISVCPARAMODOHCONFIG_IID_STR "bdcef040-452e-11eb-b378-0242ac130002"

#define NS_ISVCPARAMODOHCONFIG_IID \
  {0xbdcef040, 0x452e, 0x11eb, \
    { 0xb3, 0x78, 0x02, 0x42, 0xac, 0x13, 0x00, 0x02 }}

class NS_NO_VTABLE nsISVCParamODoHConfig : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCPARAMODOHCONFIG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCParamODoHConfig;

  /* readonly attribute ACString ODoHConfig; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCParamODoHConfig, NS_ISVCPARAMODOHCONFIG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCPARAMODOHCONFIG \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCPARAMODOHCONFIG \
  nsresult GetODoHConfig(nsACString& aODoHConfig); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCPARAMODOHCONFIG(_to) \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override { return _to GetODoHConfig(aODoHConfig); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCPARAMODOHCONFIG(_to) \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetODoHConfig(aODoHConfig); } 


/* starting interface:    nsISVCBRecord */
#define NS_ISVCBRECORD_IID_STR "a4da5645-2160-4439-bd11-540a2d26c989"

#define NS_ISVCBRECORD_IID \
  {0xa4da5645, 0x2160, 0x4439, \
    { 0xbd, 0x11, 0x54, 0x0a, 0x2d, 0x26, 0xc9, 0x89 }}

class NS_NO_VTABLE nsISVCBRecord : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISVCBRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISVCBRecord;

  /* readonly attribute uint16_t priority; */
  NS_IMETHOD GetPriority(uint16_t *aPriority) = 0;

  /* readonly attribute ACString name; */
  NS_IMETHOD GetName(nsACString& aName) = 0;

  /* [noscript,nostdcall,notxpcom] readonly attribute MaybePort port; */
  virtual mozilla::Maybe<uint16_t> GetPort() = 0;

  /* [noscript,nostdcall,notxpcom] readonly attribute MaybeAlpnTuple alpn; */
  virtual mozilla::Maybe<mozilla::Tuple<nsCString, bool>> GetAlpn() = 0;

  /* readonly attribute ACString echConfig; */
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) = 0;

  /* readonly attribute ACString ODoHConfig; */
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) = 0;

  /* readonly attribute bool hasIPHintAddress; */
  NS_IMETHOD GetHasIPHintAddress(bool *aHasIPHintAddress) = 0;

  /* readonly attribute Array<nsISVCParam> values; */
  NS_IMETHOD GetValues(nsTArray<RefPtr<nsISVCParam>>& aValues) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISVCBRecord, NS_ISVCBRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISVCBRECORD \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override; \
  NS_IMETHOD GetName(nsACString& aName) override; \
  virtual mozilla::Maybe<uint16_t> GetPort() override; \
  virtual mozilla::Maybe<mozilla::Tuple<nsCString, bool>> GetAlpn() override; \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override; \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override; \
  NS_IMETHOD GetHasIPHintAddress(bool *aHasIPHintAddress) override; \
  NS_IMETHOD GetValues(nsTArray<RefPtr<nsISVCParam>>& aValues) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISVCBRECORD \
  nsresult GetPriority(uint16_t *aPriority); \
  nsresult GetName(nsACString& aName); \
  mozilla::Maybe<uint16_t> GetPort(); \
  mozilla::Maybe<mozilla::Tuple<nsCString, bool>> GetAlpn(); \
  nsresult GetEchConfig(nsACString& aEchConfig); \
  nsresult GetODoHConfig(nsACString& aODoHConfig); \
  nsresult GetHasIPHintAddress(bool *aHasIPHintAddress); \
  nsresult GetValues(nsTArray<RefPtr<nsISVCParam>>& aValues); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISVCBRECORD(_to) \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override { return _to GetPriority(aPriority); } \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  virtual mozilla::Maybe<uint16_t> GetPort() override { return _to GetPort(); } \
  virtual mozilla::Maybe<mozilla::Tuple<nsCString, bool>> GetAlpn() override { return _to GetAlpn(); } \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override { return _to GetEchConfig(aEchConfig); } \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override { return _to GetODoHConfig(aODoHConfig); } \
  NS_IMETHOD GetHasIPHintAddress(bool *aHasIPHintAddress) override { return _to GetHasIPHintAddress(aHasIPHintAddress); } \
  NS_IMETHOD GetValues(nsTArray<RefPtr<nsISVCParam>>& aValues) override { return _to GetValues(aValues); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISVCBRECORD(_to) \
  NS_IMETHOD GetPriority(uint16_t *aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPriority(aPriority); } \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  virtual mozilla::Maybe<uint16_t> GetPort() override; \
  virtual mozilla::Maybe<mozilla::Tuple<nsCString, bool>> GetAlpn() override; \
  NS_IMETHOD GetEchConfig(nsACString& aEchConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEchConfig(aEchConfig); } \
  NS_IMETHOD GetODoHConfig(nsACString& aODoHConfig) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetODoHConfig(aODoHConfig); } \
  NS_IMETHOD GetHasIPHintAddress(bool *aHasIPHintAddress) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasIPHintAddress(aHasIPHintAddress); } \
  NS_IMETHOD GetValues(nsTArray<RefPtr<nsISVCParam>>& aValues) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValues(aValues); } 


/* starting interface:    nsIDNSHTTPSSVCRecord */
#define NS_IDNSHTTPSSVCRECORD_IID_STR "5b649e95-e0d3-422b-99a6-79d70a041387"

#define NS_IDNSHTTPSSVCRECORD_IID \
  {0x5b649e95, 0xe0d3, 0x422b, \
    { 0x99, 0xa6, 0x79, 0xd7, 0x0a, 0x04, 0x13, 0x87 }}

class NS_NO_VTABLE nsIDNSHTTPSSVCRecord : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSHTTPSSVCRECORD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSHTTPSSVCRecord;

  /* readonly attribute Array<nsISVCBRecord> records; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRecords(nsTArray<RefPtr<nsISVCBRecord>>& aRecords) = 0;

  /* nsISVCBRecord GetServiceModeRecord (in boolean aNoHttp2, in boolean aNoHttp3); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetServiceModeRecord(bool aNoHttp2, bool aNoHttp3, nsISVCBRecord **_retval) = 0;

  /* readonly attribute boolean hasIPAddresses; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHasIPAddresses(bool *aHasIPAddresses) = 0;

  /* readonly attribute boolean allRecordsExcluded; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllRecordsExcluded(bool *aAllRecordsExcluded) = 0;

  /* Array<nsISVCBRecord> GetAllRecordsWithEchConfig (in boolean aNoHttp2, in boolean aNoHttp3, out boolean aAllRecordsHaveEchConfig, out boolean aAllRecordsInH3ExcludedList); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAllRecordsWithEchConfig(bool aNoHttp2, bool aNoHttp3, bool *aAllRecordsHaveEchConfig, bool *aAllRecordsInH3ExcludedList, nsTArray<RefPtr<nsISVCBRecord>>& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSHTTPSSVCRecord, NS_IDNSHTTPSSVCRECORD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSHTTPSSVCRECORD \
  NS_IMETHOD GetRecords(nsTArray<RefPtr<nsISVCBRecord>>& aRecords) override; \
  NS_IMETHOD GetServiceModeRecord(bool aNoHttp2, bool aNoHttp3, nsISVCBRecord **_retval) override; \
  NS_IMETHOD GetHasIPAddresses(bool *aHasIPAddresses) override; \
  NS_IMETHOD GetAllRecordsExcluded(bool *aAllRecordsExcluded) override; \
  NS_IMETHOD GetAllRecordsWithEchConfig(bool aNoHttp2, bool aNoHttp3, bool *aAllRecordsHaveEchConfig, bool *aAllRecordsInH3ExcludedList, nsTArray<RefPtr<nsISVCBRecord>>& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSHTTPSSVCRECORD \
  nsresult GetRecords(nsTArray<RefPtr<nsISVCBRecord>>& aRecords); \
  nsresult GetServiceModeRecord(bool aNoHttp2, bool aNoHttp3, nsISVCBRecord **_retval); \
  nsresult GetHasIPAddresses(bool *aHasIPAddresses); \
  nsresult GetAllRecordsExcluded(bool *aAllRecordsExcluded); \
  nsresult GetAllRecordsWithEchConfig(bool aNoHttp2, bool aNoHttp3, bool *aAllRecordsHaveEchConfig, bool *aAllRecordsInH3ExcludedList, nsTArray<RefPtr<nsISVCBRecord>>& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSHTTPSSVCRECORD(_to) \
  NS_IMETHOD GetRecords(nsTArray<RefPtr<nsISVCBRecord>>& aRecords) override { return _to GetRecords(aRecords); } \
  NS_IMETHOD GetServiceModeRecord(bool aNoHttp2, bool aNoHttp3, nsISVCBRecord **_retval) override { return _to GetServiceModeRecord(aNoHttp2, aNoHttp3, _retval); } \
  NS_IMETHOD GetHasIPAddresses(bool *aHasIPAddresses) override { return _to GetHasIPAddresses(aHasIPAddresses); } \
  NS_IMETHOD GetAllRecordsExcluded(bool *aAllRecordsExcluded) override { return _to GetAllRecordsExcluded(aAllRecordsExcluded); } \
  NS_IMETHOD GetAllRecordsWithEchConfig(bool aNoHttp2, bool aNoHttp3, bool *aAllRecordsHaveEchConfig, bool *aAllRecordsInH3ExcludedList, nsTArray<RefPtr<nsISVCBRecord>>& _retval) override { return _to GetAllRecordsWithEchConfig(aNoHttp2, aNoHttp3, aAllRecordsHaveEchConfig, aAllRecordsInH3ExcludedList, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSHTTPSSVCRECORD(_to) \
  NS_IMETHOD GetRecords(nsTArray<RefPtr<nsISVCBRecord>>& aRecords) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRecords(aRecords); } \
  NS_IMETHOD GetServiceModeRecord(bool aNoHttp2, bool aNoHttp3, nsISVCBRecord **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetServiceModeRecord(aNoHttp2, aNoHttp3, _retval); } \
  NS_IMETHOD GetHasIPAddresses(bool *aHasIPAddresses) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHasIPAddresses(aHasIPAddresses); } \
  NS_IMETHOD GetAllRecordsExcluded(bool *aAllRecordsExcluded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllRecordsExcluded(aAllRecordsExcluded); } \
  NS_IMETHOD GetAllRecordsWithEchConfig(bool aNoHttp2, bool aNoHttp3, bool *aAllRecordsHaveEchConfig, bool *aAllRecordsInH3ExcludedList, nsTArray<RefPtr<nsISVCBRecord>>& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllRecordsWithEchConfig(aNoHttp2, aNoHttp3, aAllRecordsHaveEchConfig, aAllRecordsInH3ExcludedList, _retval); } 


#endif /* __gen_nsIDNSByTypeRecord_h__ */
