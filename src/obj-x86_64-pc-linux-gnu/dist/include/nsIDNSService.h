/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSService.idl
 */

#ifndef __gen_nsIDNSService_h__
#define __gen_nsIDNSService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIRequest_h__
#include "nsIRequest.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "mozilla/BasePrincipal.h"
class nsICancelable; /* forward declaration */

class nsIEventTarget; /* forward declaration */

class nsIDNSRecord; /* forward declaration */

class nsIDNSListener; /* forward declaration */

class nsIDNSResolverInfo; /* forward declaration */

#include "nsTArrayForwardDeclare.h"
namespace mozilla { namespace net {
    struct DNSCacheEntries;
} }

/* starting interface:    nsIDNSService */
#define NS_IDNSSERVICE_IID_STR "de5642c6-61fc-4fcf-9a47-03226b0d4e21"

#define NS_IDNSSERVICE_IID \
  {0xde5642c6, 0x61fc, 0x4fcf, \
    { 0x9a, 0x47, 0x03, 0x22, 0x6b, 0x0d, 0x4e, 0x21 }}

class nsIDNSService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSService;

  enum ResolveType : uint16_t {
    RESOLVE_TYPE_DEFAULT = 0,
    RESOLVE_TYPE_TXT = 16,
    RESOLVE_TYPE_HTTPSSVC = 65,
  };

  enum ResolverMode : uint32_t {
    MODE_NATIVEONLY = 0,
    MODE_RESERVED1 = 1,
    MODE_TRRFIRST = 2,
    MODE_TRRONLY = 3,
    MODE_RESERVED4 = 4,
    MODE_TRROFF = 5,
  };

  /* [implicit_jscontext,optional_argc] nsICancelable asyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, [optional] in jsval aOriginAttributes); */
  NS_IMETHOD AsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsICancelable **_retval) = 0;

  /* [notxpcom] nsresult asyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsIEventTarget aListenerTarget, in OriginAttributes aOriginAttributes, out nsICancelable aResult); */
  NS_IMETHOD AsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, const mozilla::OriginAttributes & aOriginAttributes, nsICancelable **aResult) = 0;

  /* nsIDNSResolverInfo newTRRResolverInfo (in AUTF8String aTrrURL); */
  NS_IMETHOD NewTRRResolverInfo(const nsACString& aTrrURL, nsIDNSResolverInfo **_retval) = 0;

  /* [implicit_jscontext,optional_argc] void cancelAsyncResolve (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, [optional] in jsval aOriginAttributes); */
  NS_IMETHOD CancelAsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) = 0;

  /* [notxpcom] nsresult cancelAsyncResolveNative (in AUTF8String aHostName, in nsIDNSService_ResolveType aType, in unsigned long aFlags, in nsIDNSResolverInfo aResolver, in nsIDNSListener aListener, in nsresult aReason, in OriginAttributes aOriginAttributes); */
  NS_IMETHOD CancelAsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, const mozilla::OriginAttributes & aOriginAttributes) = 0;

  /* [implicit_jscontext,optional_argc] nsIDNSRecord resolve (in AUTF8String aHostName, in unsigned long aFlags, [optional] in jsval aOriginAttributes); */
  NS_IMETHOD Resolve(const nsACString& aHostName, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsIDNSRecord **_retval) = 0;

  /* [notxpcom] nsresult resolveNative (in AUTF8String aHostName, in unsigned long aFlags, in OriginAttributes aOriginAttributes, out nsIDNSRecord aResult); */
  NS_IMETHOD ResolveNative(const nsACString& aHostName, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, nsIDNSRecord **aResult) = 0;

  /* [noscript] void getDNSCacheEntries (in EntriesArray args); */
  NS_IMETHOD GetDNSCacheEntries(nsTArray<mozilla::net::DNSCacheEntries> * args) = 0;

  /* void clearCache (in boolean aTrrToo); */
  NS_IMETHOD ClearCache(bool aTrrToo) = 0;

  /* void reloadParentalControlEnabled (); */
  NS_IMETHOD ReloadParentalControlEnabled(void) = 0;

  /* void setDetectedTrrURI (in AUTF8String aURI); */
  NS_IMETHOD SetDetectedTrrURI(const nsACString& aURI) = 0;

  /* [noscript] void ReportFailedSVCDomainName (in ACString aOwnerName, in ACString aSVCDomainName); */
  NS_IMETHOD ReportFailedSVCDomainName(const nsACString& aOwnerName, const nsACString& aSVCDomainName) = 0;

  /* [noscript] boolean IsSVCDomainNameFailed (in ACString aOwnerName, in ACString aSVCDomainName); */
  NS_IMETHOD IsSVCDomainNameFailed(const nsACString& aOwnerName, const nsACString& aSVCDomainName, bool *_retval) = 0;

  /* [noscript] void ResetExcludedSVCDomainName (in ACString aOwnerName); */
  NS_IMETHOD ResetExcludedSVCDomainName(const nsACString& aOwnerName) = 0;

  /* readonly attribute AUTF8String currentTrrURI; */
  NS_IMETHOD GetCurrentTrrURI(nsACString& aCurrentTrrURI) = 0;

  /* readonly attribute nsIDNSService_ResolverMode currentTrrMode; */
  NS_IMETHOD GetCurrentTrrMode(nsIDNSService::ResolverMode *aCurrentTrrMode) = 0;

  /* readonly attribute AUTF8String myHostName; */
  NS_IMETHOD GetMyHostName(nsACString& aMyHostName) = 0;

  /* readonly attribute boolean ODoHActivated; */
  NS_IMETHOD GetODoHActivated(bool *aODoHActivated) = 0;

  enum {
    RESOLVE_BYPASS_CACHE = 1U,
    RESOLVE_CANONICAL_NAME = 2U,
    RESOLVE_PRIORITY_MEDIUM = 4U,
    RESOLVE_PRIORITY_LOW = 8U,
    RESOLVE_SPECULATE = 16U,
    RESOLVE_DISABLE_IPV6 = 32U,
    RESOLVE_OFFLINE = 64U,
    RESOLVE_DISABLE_IPV4 = 128U,
    RESOLVE_ALLOW_NAME_COLLISION = 256U,
    RESOLVE_DISABLE_TRR = 512U,
    RESOLVE_REFRESH_CACHE = 1024U,
    RESOLVE_TRR_MODE_MASK = 6144U,
    RESOLVE_TRR_DISABLED_MODE = 2048U
  };

     static uint32_t GetFlagsFromTRRMode(nsIRequest::TRRMode aMode) {
        return static_cast<uint32_t>(aMode) << 11;
    }
    static nsIRequest::TRRMode GetTRRModeFromFlags(uint32_t aFlags) {
        return static_cast<nsIRequest::TRRMode>((aFlags & RESOLVE_TRR_MODE_MASK) >> 11);
    }
  enum {
    RESOLVE_IGNORE_SOCKS_DNS = 8192U,
    RESOLVE_IP_HINT = 16384U,
    RESOLVE_DISABLE_ODOH = 32768U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSService, NS_IDNSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSSERVICE \
  NS_IMETHOD AsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsICancelable **_retval) override; \
  NS_IMETHOD AsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, const mozilla::OriginAttributes & aOriginAttributes, nsICancelable **aResult) override; \
  NS_IMETHOD NewTRRResolverInfo(const nsACString& aTrrURL, nsIDNSResolverInfo **_retval) override; \
  NS_IMETHOD CancelAsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override; \
  NS_IMETHOD CancelAsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, const mozilla::OriginAttributes & aOriginAttributes) override; \
  NS_IMETHOD Resolve(const nsACString& aHostName, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsIDNSRecord **_retval) override; \
  NS_IMETHOD ResolveNative(const nsACString& aHostName, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, nsIDNSRecord **aResult) override; \
  NS_IMETHOD GetDNSCacheEntries(nsTArray<mozilla::net::DNSCacheEntries> * args) override; \
  NS_IMETHOD ClearCache(bool aTrrToo) override; \
  NS_IMETHOD ReloadParentalControlEnabled(void) override; \
  NS_IMETHOD SetDetectedTrrURI(const nsACString& aURI) override; \
  NS_IMETHOD ReportFailedSVCDomainName(const nsACString& aOwnerName, const nsACString& aSVCDomainName) override; \
  NS_IMETHOD IsSVCDomainNameFailed(const nsACString& aOwnerName, const nsACString& aSVCDomainName, bool *_retval) override; \
  NS_IMETHOD ResetExcludedSVCDomainName(const nsACString& aOwnerName) override; \
  NS_IMETHOD GetCurrentTrrURI(nsACString& aCurrentTrrURI) override; \
  NS_IMETHOD GetCurrentTrrMode(nsIDNSService::ResolverMode *aCurrentTrrMode) override; \
  NS_IMETHOD GetMyHostName(nsACString& aMyHostName) override; \
  NS_IMETHOD GetODoHActivated(bool *aODoHActivated) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSSERVICE \
  nsresult AsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsICancelable **_retval); \
  nsresult AsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, const mozilla::OriginAttributes & aOriginAttributes, nsICancelable **aResult); \
  nsresult NewTRRResolverInfo(const nsACString& aTrrURL, nsIDNSResolverInfo **_retval); \
  nsresult CancelAsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc); \
  nsresult CancelAsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, const mozilla::OriginAttributes & aOriginAttributes); \
  nsresult Resolve(const nsACString& aHostName, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsIDNSRecord **_retval); \
  nsresult ResolveNative(const nsACString& aHostName, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, nsIDNSRecord **aResult); \
  nsresult GetDNSCacheEntries(nsTArray<mozilla::net::DNSCacheEntries> * args); \
  nsresult ClearCache(bool aTrrToo); \
  nsresult ReloadParentalControlEnabled(void); \
  nsresult SetDetectedTrrURI(const nsACString& aURI); \
  nsresult ReportFailedSVCDomainName(const nsACString& aOwnerName, const nsACString& aSVCDomainName); \
  nsresult IsSVCDomainNameFailed(const nsACString& aOwnerName, const nsACString& aSVCDomainName, bool *_retval); \
  nsresult ResetExcludedSVCDomainName(const nsACString& aOwnerName); \
  nsresult GetCurrentTrrURI(nsACString& aCurrentTrrURI); \
  nsresult GetCurrentTrrMode(nsIDNSService::ResolverMode *aCurrentTrrMode); \
  nsresult GetMyHostName(nsACString& aMyHostName); \
  nsresult GetODoHActivated(bool *aODoHActivated); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSSERVICE(_to) \
  NS_IMETHOD AsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsICancelable **_retval) override { return _to AsyncResolve(aHostName, aType, aFlags, aResolver, aListener, aListenerTarget, aOriginAttributes, cx, _argc, _retval); } \
  NS_IMETHOD AsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, const mozilla::OriginAttributes & aOriginAttributes, nsICancelable **aResult) override { return _to AsyncResolveNative(aHostName, aType, aFlags, aResolver, aListener, aListenerTarget, aOriginAttributes, aResult); } \
  NS_IMETHOD NewTRRResolverInfo(const nsACString& aTrrURL, nsIDNSResolverInfo **_retval) override { return _to NewTRRResolverInfo(aTrrURL, _retval); } \
  NS_IMETHOD CancelAsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override { return _to CancelAsyncResolve(aHostName, aType, aFlags, aResolver, aListener, aReason, aOriginAttributes, cx, _argc); } \
  NS_IMETHOD CancelAsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, const mozilla::OriginAttributes & aOriginAttributes) override { return _to CancelAsyncResolveNative(aHostName, aType, aFlags, aResolver, aListener, aReason, aOriginAttributes); } \
  NS_IMETHOD Resolve(const nsACString& aHostName, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsIDNSRecord **_retval) override { return _to Resolve(aHostName, aFlags, aOriginAttributes, cx, _argc, _retval); } \
  NS_IMETHOD ResolveNative(const nsACString& aHostName, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, nsIDNSRecord **aResult) override { return _to ResolveNative(aHostName, aFlags, aOriginAttributes, aResult); } \
  NS_IMETHOD GetDNSCacheEntries(nsTArray<mozilla::net::DNSCacheEntries> * args) override { return _to GetDNSCacheEntries(args); } \
  NS_IMETHOD ClearCache(bool aTrrToo) override { return _to ClearCache(aTrrToo); } \
  NS_IMETHOD ReloadParentalControlEnabled(void) override { return _to ReloadParentalControlEnabled(); } \
  NS_IMETHOD SetDetectedTrrURI(const nsACString& aURI) override { return _to SetDetectedTrrURI(aURI); } \
  NS_IMETHOD ReportFailedSVCDomainName(const nsACString& aOwnerName, const nsACString& aSVCDomainName) override { return _to ReportFailedSVCDomainName(aOwnerName, aSVCDomainName); } \
  NS_IMETHOD IsSVCDomainNameFailed(const nsACString& aOwnerName, const nsACString& aSVCDomainName, bool *_retval) override { return _to IsSVCDomainNameFailed(aOwnerName, aSVCDomainName, _retval); } \
  NS_IMETHOD ResetExcludedSVCDomainName(const nsACString& aOwnerName) override { return _to ResetExcludedSVCDomainName(aOwnerName); } \
  NS_IMETHOD GetCurrentTrrURI(nsACString& aCurrentTrrURI) override { return _to GetCurrentTrrURI(aCurrentTrrURI); } \
  NS_IMETHOD GetCurrentTrrMode(nsIDNSService::ResolverMode *aCurrentTrrMode) override { return _to GetCurrentTrrMode(aCurrentTrrMode); } \
  NS_IMETHOD GetMyHostName(nsACString& aMyHostName) override { return _to GetMyHostName(aMyHostName); } \
  NS_IMETHOD GetODoHActivated(bool *aODoHActivated) override { return _to GetODoHActivated(aODoHActivated); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSSERVICE(_to) \
  NS_IMETHOD AsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsICancelable **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncResolve(aHostName, aType, aFlags, aResolver, aListener, aListenerTarget, aOriginAttributes, cx, _argc, _retval); } \
  NS_IMETHOD AsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsIEventTarget *aListenerTarget, const mozilla::OriginAttributes & aOriginAttributes, nsICancelable **aResult) override; \
  NS_IMETHOD NewTRRResolverInfo(const nsACString& aTrrURL, nsIDNSResolverInfo **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewTRRResolverInfo(aTrrURL, _retval); } \
  NS_IMETHOD CancelAsyncResolve(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CancelAsyncResolve(aHostName, aType, aFlags, aResolver, aListener, aReason, aOriginAttributes, cx, _argc); } \
  NS_IMETHOD CancelAsyncResolveNative(const nsACString& aHostName, nsIDNSService::ResolveType aType, uint32_t aFlags, nsIDNSResolverInfo *aResolver, nsIDNSListener *aListener, nsresult aReason, const mozilla::OriginAttributes & aOriginAttributes) override; \
  NS_IMETHOD Resolve(const nsACString& aHostName, uint32_t aFlags, JS::HandleValue aOriginAttributes, JSContext* cx, uint8_t _argc, nsIDNSRecord **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Resolve(aHostName, aFlags, aOriginAttributes, cx, _argc, _retval); } \
  NS_IMETHOD ResolveNative(const nsACString& aHostName, uint32_t aFlags, const mozilla::OriginAttributes & aOriginAttributes, nsIDNSRecord **aResult) override; \
  NS_IMETHOD GetDNSCacheEntries(nsTArray<mozilla::net::DNSCacheEntries> * args) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDNSCacheEntries(args); } \
  NS_IMETHOD ClearCache(bool aTrrToo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearCache(aTrrToo); } \
  NS_IMETHOD ReloadParentalControlEnabled(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReloadParentalControlEnabled(); } \
  NS_IMETHOD SetDetectedTrrURI(const nsACString& aURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDetectedTrrURI(aURI); } \
  NS_IMETHOD ReportFailedSVCDomainName(const nsACString& aOwnerName, const nsACString& aSVCDomainName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReportFailedSVCDomainName(aOwnerName, aSVCDomainName); } \
  NS_IMETHOD IsSVCDomainNameFailed(const nsACString& aOwnerName, const nsACString& aSVCDomainName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsSVCDomainNameFailed(aOwnerName, aSVCDomainName, _retval); } \
  NS_IMETHOD ResetExcludedSVCDomainName(const nsACString& aOwnerName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetExcludedSVCDomainName(aOwnerName); } \
  NS_IMETHOD GetCurrentTrrURI(nsACString& aCurrentTrrURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentTrrURI(aCurrentTrrURI); } \
  NS_IMETHOD GetCurrentTrrMode(nsIDNSService::ResolverMode *aCurrentTrrMode) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentTrrMode(aCurrentTrrMode); } \
  NS_IMETHOD GetMyHostName(nsACString& aMyHostName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMyHostName(aMyHostName); } \
  NS_IMETHOD GetODoHActivated(bool *aODoHActivated) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetODoHActivated(aODoHActivated); } \


/**
 * An observer notification for this topic is sent whenever the URI that the
 * TRR service is using has changed.
 */
#define NS_NETWORK_TRR_URI_CHANGED_TOPIC "network:trr-uri-changed"
/**
 * An observer notification for this topic is sent whenever the mode that the
 * TRR service is using has changed.
 */
#define NS_NETWORK_TRR_MODE_CHANGED_TOPIC "network:trr-mode-changed"

#endif /* __gen_nsIDNSService_h__ */
