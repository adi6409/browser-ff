/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICacheInfoChannel.idl
 */

#ifndef __gen_nsICacheInfoChannel_h__
#define __gen_nsICacheInfoChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAsyncOutputStream; /* forward declaration */

class nsIInputStream; /* forward declaration */

#include "nsTArray.h"
namespace mozilla {
namespace net {
class PreferredAlternativeDataTypeParams;
}
} // namespace mozilla

/* starting interface:    nsIInputStreamReceiver */
#define NS_IINPUTSTREAMRECEIVER_IID_STR "1fb8ccf2-5fa5-45ec-bc57-8c8022a5d0d3"

#define NS_IINPUTSTREAMRECEIVER_IID \
  {0x1fb8ccf2, 0x5fa5, 0x45ec, \
    { 0xbc, 0x57, 0x8c, 0x80, 0x22, 0xa5, 0xd0, 0xd3 }}

class NS_NO_VTABLE nsIInputStreamReceiver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMRECEIVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamReceiver;

  /* void onInputStreamReady (in nsIInputStream aStream); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnInputStreamReady(nsIInputStream *aStream) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamReceiver, NS_IINPUTSTREAMRECEIVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMRECEIVER \
  NS_IMETHOD OnInputStreamReady(nsIInputStream *aStream) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMRECEIVER \
  nsresult OnInputStreamReady(nsIInputStream *aStream); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMRECEIVER(_to) \
  NS_IMETHOD OnInputStreamReady(nsIInputStream *aStream) override { return _to OnInputStreamReady(aStream); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMRECEIVER(_to) \
  NS_IMETHOD OnInputStreamReady(nsIInputStream *aStream) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnInputStreamReady(aStream); } 


/* starting interface:    nsICacheInfoChannel */
#define NS_ICACHEINFOCHANNEL_IID_STR "72c34415-c6eb-48af-851f-772fa9ee5972"

#define NS_ICACHEINFOCHANNEL_IID \
  {0x72c34415, 0xc6eb, 0x48af, \
    { 0x85, 0x1f, 0x77, 0x2f, 0xa9, 0xee, 0x59, 0x72 }}

class NS_NO_VTABLE nsICacheInfoChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHEINFOCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheInfoChannel;

  /* readonly attribute int32_t cacheTokenFetchCount; */
  NS_IMETHOD GetCacheTokenFetchCount(int32_t *aCacheTokenFetchCount) = 0;

  /* readonly attribute uint32_t cacheTokenExpirationTime; */
  NS_IMETHOD GetCacheTokenExpirationTime(uint32_t *aCacheTokenExpirationTime) = 0;

  /* boolean isFromCache (); */
  NS_IMETHOD IsFromCache(bool *_retval) = 0;

  /* boolean isRacing (); */
  NS_IMETHOD IsRacing(bool *_retval) = 0;

  /* uint64_t getCacheEntryId (); */
  NS_IMETHOD GetCacheEntryId(uint64_t *_retval) = 0;

  /* attribute unsigned long cacheKey; */
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) = 0;
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) = 0;

  /* attribute boolean allowStaleCacheContent; */
  NS_IMETHOD GetAllowStaleCacheContent(bool *aAllowStaleCacheContent) = 0;
  NS_IMETHOD SetAllowStaleCacheContent(bool aAllowStaleCacheContent) = 0;

  /* attribute boolean preferCacheLoadOverBypass; */
  NS_IMETHOD GetPreferCacheLoadOverBypass(bool *aPreferCacheLoadOverBypass) = 0;
  NS_IMETHOD SetPreferCacheLoadOverBypass(bool aPreferCacheLoadOverBypass) = 0;

  /* void preferAlternativeDataType (in ACString type, in ACString contentType, in boolean deliverAltData); */
  NS_IMETHOD PreferAlternativeDataType(const nsACString& type, const nsACString& contentType, bool deliverAltData) = 0;

  /* [noscript,nostdcall,notxpcom] ConstPreferredAlternativeDataTypeArray preferredAlternativeDataTypes (); */
  virtual const nsTArray<mozilla::net::PreferredAlternativeDataTypeParams> & PreferredAlternativeDataTypes(void) = 0;

  /* readonly attribute ACString alternativeDataType; */
  NS_IMETHOD GetAlternativeDataType(nsACString& aAlternativeDataType) = 0;

  /* void getAltDataInputStream (in ACString type, in nsIInputStreamReceiver aReceiver); */
  NS_IMETHOD GetAltDataInputStream(const nsACString& type, nsIInputStreamReceiver *aReceiver) = 0;

  /* void getOriginalInputStream (in nsIInputStreamReceiver aReceiver); */
  NS_IMETHOD GetOriginalInputStream(nsIInputStreamReceiver *aReceiver) = 0;

  /* nsIAsyncOutputStream openAlternativeOutputStream (in ACString type, in long long predictedSize); */
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheInfoChannel, NS_ICACHEINFOCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHEINFOCHANNEL \
  NS_IMETHOD GetCacheTokenFetchCount(int32_t *aCacheTokenFetchCount) override; \
  NS_IMETHOD GetCacheTokenExpirationTime(uint32_t *aCacheTokenExpirationTime) override; \
  NS_IMETHOD IsFromCache(bool *_retval) override; \
  NS_IMETHOD IsRacing(bool *_retval) override; \
  NS_IMETHOD GetCacheEntryId(uint64_t *_retval) override; \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override; \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override; \
  NS_IMETHOD GetAllowStaleCacheContent(bool *aAllowStaleCacheContent) override; \
  NS_IMETHOD SetAllowStaleCacheContent(bool aAllowStaleCacheContent) override; \
  NS_IMETHOD GetPreferCacheLoadOverBypass(bool *aPreferCacheLoadOverBypass) override; \
  NS_IMETHOD SetPreferCacheLoadOverBypass(bool aPreferCacheLoadOverBypass) override; \
  NS_IMETHOD PreferAlternativeDataType(const nsACString& type, const nsACString& contentType, bool deliverAltData) override; \
  virtual const nsTArray<mozilla::net::PreferredAlternativeDataTypeParams> & PreferredAlternativeDataTypes(void) override; \
  NS_IMETHOD GetAlternativeDataType(nsACString& aAlternativeDataType) override; \
  NS_IMETHOD GetAltDataInputStream(const nsACString& type, nsIInputStreamReceiver *aReceiver) override; \
  NS_IMETHOD GetOriginalInputStream(nsIInputStreamReceiver *aReceiver) override; \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHEINFOCHANNEL \
  nsresult GetCacheTokenFetchCount(int32_t *aCacheTokenFetchCount); \
  nsresult GetCacheTokenExpirationTime(uint32_t *aCacheTokenExpirationTime); \
  nsresult IsFromCache(bool *_retval); \
  nsresult IsRacing(bool *_retval); \
  nsresult GetCacheEntryId(uint64_t *_retval); \
  nsresult GetCacheKey(uint32_t *aCacheKey); \
  nsresult SetCacheKey(uint32_t aCacheKey); \
  nsresult GetAllowStaleCacheContent(bool *aAllowStaleCacheContent); \
  nsresult SetAllowStaleCacheContent(bool aAllowStaleCacheContent); \
  nsresult GetPreferCacheLoadOverBypass(bool *aPreferCacheLoadOverBypass); \
  nsresult SetPreferCacheLoadOverBypass(bool aPreferCacheLoadOverBypass); \
  nsresult PreferAlternativeDataType(const nsACString& type, const nsACString& contentType, bool deliverAltData); \
  const nsTArray<mozilla::net::PreferredAlternativeDataTypeParams> & PreferredAlternativeDataTypes(void); \
  nsresult GetAlternativeDataType(nsACString& aAlternativeDataType); \
  nsresult GetAltDataInputStream(const nsACString& type, nsIInputStreamReceiver *aReceiver); \
  nsresult GetOriginalInputStream(nsIInputStreamReceiver *aReceiver); \
  nsresult OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHEINFOCHANNEL(_to) \
  NS_IMETHOD GetCacheTokenFetchCount(int32_t *aCacheTokenFetchCount) override { return _to GetCacheTokenFetchCount(aCacheTokenFetchCount); } \
  NS_IMETHOD GetCacheTokenExpirationTime(uint32_t *aCacheTokenExpirationTime) override { return _to GetCacheTokenExpirationTime(aCacheTokenExpirationTime); } \
  NS_IMETHOD IsFromCache(bool *_retval) override { return _to IsFromCache(_retval); } \
  NS_IMETHOD IsRacing(bool *_retval) override { return _to IsRacing(_retval); } \
  NS_IMETHOD GetCacheEntryId(uint64_t *_retval) override { return _to GetCacheEntryId(_retval); } \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return _to GetCacheKey(aCacheKey); } \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override { return _to SetCacheKey(aCacheKey); } \
  NS_IMETHOD GetAllowStaleCacheContent(bool *aAllowStaleCacheContent) override { return _to GetAllowStaleCacheContent(aAllowStaleCacheContent); } \
  NS_IMETHOD SetAllowStaleCacheContent(bool aAllowStaleCacheContent) override { return _to SetAllowStaleCacheContent(aAllowStaleCacheContent); } \
  NS_IMETHOD GetPreferCacheLoadOverBypass(bool *aPreferCacheLoadOverBypass) override { return _to GetPreferCacheLoadOverBypass(aPreferCacheLoadOverBypass); } \
  NS_IMETHOD SetPreferCacheLoadOverBypass(bool aPreferCacheLoadOverBypass) override { return _to SetPreferCacheLoadOverBypass(aPreferCacheLoadOverBypass); } \
  NS_IMETHOD PreferAlternativeDataType(const nsACString& type, const nsACString& contentType, bool deliverAltData) override { return _to PreferAlternativeDataType(type, contentType, deliverAltData); } \
  virtual const nsTArray<mozilla::net::PreferredAlternativeDataTypeParams> & PreferredAlternativeDataTypes(void) override { return _to PreferredAlternativeDataTypes(); } \
  NS_IMETHOD GetAlternativeDataType(nsACString& aAlternativeDataType) override { return _to GetAlternativeDataType(aAlternativeDataType); } \
  NS_IMETHOD GetAltDataInputStream(const nsACString& type, nsIInputStreamReceiver *aReceiver) override { return _to GetAltDataInputStream(type, aReceiver); } \
  NS_IMETHOD GetOriginalInputStream(nsIInputStreamReceiver *aReceiver) override { return _to GetOriginalInputStream(aReceiver); } \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override { return _to OpenAlternativeOutputStream(type, predictedSize, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHEINFOCHANNEL(_to) \
  NS_IMETHOD GetCacheTokenFetchCount(int32_t *aCacheTokenFetchCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheTokenFetchCount(aCacheTokenFetchCount); } \
  NS_IMETHOD GetCacheTokenExpirationTime(uint32_t *aCacheTokenExpirationTime) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheTokenExpirationTime(aCacheTokenExpirationTime); } \
  NS_IMETHOD IsFromCache(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsFromCache(_retval); } \
  NS_IMETHOD IsRacing(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRacing(_retval); } \
  NS_IMETHOD GetCacheEntryId(uint64_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheEntryId(_retval); } \
  NS_IMETHOD GetCacheKey(uint32_t *aCacheKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCacheKey(aCacheKey); } \
  NS_IMETHOD SetCacheKey(uint32_t aCacheKey) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCacheKey(aCacheKey); } \
  NS_IMETHOD GetAllowStaleCacheContent(bool *aAllowStaleCacheContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAllowStaleCacheContent(aAllowStaleCacheContent); } \
  NS_IMETHOD SetAllowStaleCacheContent(bool aAllowStaleCacheContent) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAllowStaleCacheContent(aAllowStaleCacheContent); } \
  NS_IMETHOD GetPreferCacheLoadOverBypass(bool *aPreferCacheLoadOverBypass) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPreferCacheLoadOverBypass(aPreferCacheLoadOverBypass); } \
  NS_IMETHOD SetPreferCacheLoadOverBypass(bool aPreferCacheLoadOverBypass) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPreferCacheLoadOverBypass(aPreferCacheLoadOverBypass); } \
  NS_IMETHOD PreferAlternativeDataType(const nsACString& type, const nsACString& contentType, bool deliverAltData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PreferAlternativeDataType(type, contentType, deliverAltData); } \
  virtual const nsTArray<mozilla::net::PreferredAlternativeDataTypeParams> & PreferredAlternativeDataTypes(void) override; \
  NS_IMETHOD GetAlternativeDataType(nsACString& aAlternativeDataType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlternativeDataType(aAlternativeDataType); } \
  NS_IMETHOD GetAltDataInputStream(const nsACString& type, nsIInputStreamReceiver *aReceiver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAltDataInputStream(type, aReceiver); } \
  NS_IMETHOD GetOriginalInputStream(nsIInputStreamReceiver *aReceiver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalInputStream(aReceiver); } \
  NS_IMETHOD OpenAlternativeOutputStream(const nsACString& type, int64_t predictedSize, nsIAsyncOutputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OpenAlternativeOutputStream(type, predictedSize, _retval); } 


#endif /* __gen_nsICacheInfoChannel_h__ */
