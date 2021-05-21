/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierUtils.idl
 */

#ifndef __gen_nsIUrlClassifierUtils_h__
#define __gen_nsIUrlClassifierUtils_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIChannel; /* forward declaration */


/* starting interface:    nsIUrlClassifierParseFindFullHashCallback */
#define NS_IURLCLASSIFIERPARSEFINDFULLHASHCALLBACK_IID_STR "fbb9684a-a0aa-11e6-88b0-08606e456b8a"

#define NS_IURLCLASSIFIERPARSEFINDFULLHASHCALLBACK_IID \
  {0xfbb9684a, 0xa0aa, 0x11e6, \
    { 0x88, 0xb0, 0x08, 0x60, 0x6e, 0x45, 0x6b, 0x8a }}

class NS_NO_VTABLE nsIUrlClassifierParseFindFullHashCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERPARSEFINDFULLHASHCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierParseFindFullHashCallback;

  /* void onCompleteHashFound (in ACString aCompleteHash, in ACString aTableNames, in unsigned long aPerHashCacheDuration); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnCompleteHashFound(const nsACString& aCompleteHash, const nsACString& aTableNames, uint32_t aPerHashCacheDuration) = 0;

  /* void onResponseParsed (in unsigned long aMinWaitDuration, in unsigned long aNegCacheDuration); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnResponseParsed(uint32_t aMinWaitDuration, uint32_t aNegCacheDuration) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierParseFindFullHashCallback, NS_IURLCLASSIFIERPARSEFINDFULLHASHCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERPARSEFINDFULLHASHCALLBACK \
  NS_IMETHOD OnCompleteHashFound(const nsACString& aCompleteHash, const nsACString& aTableNames, uint32_t aPerHashCacheDuration) override; \
  NS_IMETHOD OnResponseParsed(uint32_t aMinWaitDuration, uint32_t aNegCacheDuration) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERPARSEFINDFULLHASHCALLBACK \
  nsresult OnCompleteHashFound(const nsACString& aCompleteHash, const nsACString& aTableNames, uint32_t aPerHashCacheDuration); \
  nsresult OnResponseParsed(uint32_t aMinWaitDuration, uint32_t aNegCacheDuration); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERPARSEFINDFULLHASHCALLBACK(_to) \
  NS_IMETHOD OnCompleteHashFound(const nsACString& aCompleteHash, const nsACString& aTableNames, uint32_t aPerHashCacheDuration) override { return _to OnCompleteHashFound(aCompleteHash, aTableNames, aPerHashCacheDuration); } \
  NS_IMETHOD OnResponseParsed(uint32_t aMinWaitDuration, uint32_t aNegCacheDuration) override { return _to OnResponseParsed(aMinWaitDuration, aNegCacheDuration); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERPARSEFINDFULLHASHCALLBACK(_to) \
  NS_IMETHOD OnCompleteHashFound(const nsACString& aCompleteHash, const nsACString& aTableNames, uint32_t aPerHashCacheDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnCompleteHashFound(aCompleteHash, aTableNames, aPerHashCacheDuration); } \
  NS_IMETHOD OnResponseParsed(uint32_t aMinWaitDuration, uint32_t aNegCacheDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnResponseParsed(aMinWaitDuration, aNegCacheDuration); } 


/* starting interface:    nsIUrlClassifierUtils */
#define NS_IURLCLASSIFIERUTILS_IID_STR "e4f0e59c-b922-48b0-a7b6-1735c1f96fed"

#define NS_IURLCLASSIFIERUTILS_IID \
  {0xe4f0e59c, 0xb922, 0x48b0, \
    { 0xa7, 0xb6, 0x17, 0x35, 0xc1, 0xf9, 0x6f, 0xed }}

class NS_NO_VTABLE nsIUrlClassifierUtils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERUTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierUtils;

  /* ACString getKeyForURI (in nsIURI uri); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetKeyForURI(nsIURI *uri, nsACString& _retval) = 0;

  /* ACString getProvider (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProvider(const nsACString& tableName, nsACString& _retval) = 0;

  /* ACString getTelemetryProvider (in ACString tableName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTelemetryProvider(const nsACString& tableName, nsACString& _retval) = 0;

  /* ACString getProtocolVersion (in ACString provider); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProtocolVersion(const nsACString& provider, nsACString& _retval) = 0;

  /* ACString convertThreatTypeToListNames (in uint32_t threatType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertThreatTypeToListNames(uint32_t threatType, nsACString& _retval) = 0;

  /* uint32_t convertListNameToThreatType (in ACString listName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertListNameToThreatType(const nsACString& listName, uint32_t *_retval) = 0;

  /* ACString makeUpdateRequestV4 (in Array<ACString> aListNames, in Array<ACString> aStatesBase64); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MakeUpdateRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aStatesBase64, nsACString& _retval) = 0;

  /* ACString makeFindFullHashRequestV4 (in Array<ACString> aListNames, in Array<ACString> aListStatesBase64, in Array<ACString> aPrefixes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MakeFindFullHashRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aListStatesBase64, const nsTArray<nsCString >& aPrefixes, nsACString& _retval) = 0;

  /* ACString makeThreatHitReport (in nsIChannel aChannel, in ACString aListName, in ACString aHashBase64); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MakeThreatHitReport(nsIChannel *aChannel, const nsACString& aListName, const nsACString& aHashBase64, nsACString& _retval) = 0;

  /* void parseFindFullHashResponseV4 (in ACString aResponse, in nsIUrlClassifierParseFindFullHashCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseFindFullHashResponseV4(const nsACString& aResponse, nsIUrlClassifierParseFindFullHashCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierUtils, NS_IURLCLASSIFIERUTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERUTILS \
  NS_IMETHOD GetKeyForURI(nsIURI *uri, nsACString& _retval) override; \
  NS_IMETHOD GetProvider(const nsACString& tableName, nsACString& _retval) override; \
  NS_IMETHOD GetTelemetryProvider(const nsACString& tableName, nsACString& _retval) override; \
  NS_IMETHOD GetProtocolVersion(const nsACString& provider, nsACString& _retval) override; \
  NS_IMETHOD ConvertThreatTypeToListNames(uint32_t threatType, nsACString& _retval) override; \
  NS_IMETHOD ConvertListNameToThreatType(const nsACString& listName, uint32_t *_retval) override; \
  NS_IMETHOD MakeUpdateRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aStatesBase64, nsACString& _retval) override; \
  NS_IMETHOD MakeFindFullHashRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aListStatesBase64, const nsTArray<nsCString >& aPrefixes, nsACString& _retval) override; \
  NS_IMETHOD MakeThreatHitReport(nsIChannel *aChannel, const nsACString& aListName, const nsACString& aHashBase64, nsACString& _retval) override; \
  NS_IMETHOD ParseFindFullHashResponseV4(const nsACString& aResponse, nsIUrlClassifierParseFindFullHashCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERUTILS \
  nsresult GetKeyForURI(nsIURI *uri, nsACString& _retval); \
  nsresult GetProvider(const nsACString& tableName, nsACString& _retval); \
  nsresult GetTelemetryProvider(const nsACString& tableName, nsACString& _retval); \
  nsresult GetProtocolVersion(const nsACString& provider, nsACString& _retval); \
  nsresult ConvertThreatTypeToListNames(uint32_t threatType, nsACString& _retval); \
  nsresult ConvertListNameToThreatType(const nsACString& listName, uint32_t *_retval); \
  nsresult MakeUpdateRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aStatesBase64, nsACString& _retval); \
  nsresult MakeFindFullHashRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aListStatesBase64, const nsTArray<nsCString >& aPrefixes, nsACString& _retval); \
  nsresult MakeThreatHitReport(nsIChannel *aChannel, const nsACString& aListName, const nsACString& aHashBase64, nsACString& _retval); \
  nsresult ParseFindFullHashResponseV4(const nsACString& aResponse, nsIUrlClassifierParseFindFullHashCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERUTILS(_to) \
  NS_IMETHOD GetKeyForURI(nsIURI *uri, nsACString& _retval) override { return _to GetKeyForURI(uri, _retval); } \
  NS_IMETHOD GetProvider(const nsACString& tableName, nsACString& _retval) override { return _to GetProvider(tableName, _retval); } \
  NS_IMETHOD GetTelemetryProvider(const nsACString& tableName, nsACString& _retval) override { return _to GetTelemetryProvider(tableName, _retval); } \
  NS_IMETHOD GetProtocolVersion(const nsACString& provider, nsACString& _retval) override { return _to GetProtocolVersion(provider, _retval); } \
  NS_IMETHOD ConvertThreatTypeToListNames(uint32_t threatType, nsACString& _retval) override { return _to ConvertThreatTypeToListNames(threatType, _retval); } \
  NS_IMETHOD ConvertListNameToThreatType(const nsACString& listName, uint32_t *_retval) override { return _to ConvertListNameToThreatType(listName, _retval); } \
  NS_IMETHOD MakeUpdateRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aStatesBase64, nsACString& _retval) override { return _to MakeUpdateRequestV4(aListNames, aStatesBase64, _retval); } \
  NS_IMETHOD MakeFindFullHashRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aListStatesBase64, const nsTArray<nsCString >& aPrefixes, nsACString& _retval) override { return _to MakeFindFullHashRequestV4(aListNames, aListStatesBase64, aPrefixes, _retval); } \
  NS_IMETHOD MakeThreatHitReport(nsIChannel *aChannel, const nsACString& aListName, const nsACString& aHashBase64, nsACString& _retval) override { return _to MakeThreatHitReport(aChannel, aListName, aHashBase64, _retval); } \
  NS_IMETHOD ParseFindFullHashResponseV4(const nsACString& aResponse, nsIUrlClassifierParseFindFullHashCallback *aCallback) override { return _to ParseFindFullHashResponseV4(aResponse, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERUTILS(_to) \
  NS_IMETHOD GetKeyForURI(nsIURI *uri, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetKeyForURI(uri, _retval); } \
  NS_IMETHOD GetProvider(const nsACString& tableName, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProvider(tableName, _retval); } \
  NS_IMETHOD GetTelemetryProvider(const nsACString& tableName, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTelemetryProvider(tableName, _retval); } \
  NS_IMETHOD GetProtocolVersion(const nsACString& provider, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProtocolVersion(provider, _retval); } \
  NS_IMETHOD ConvertThreatTypeToListNames(uint32_t threatType, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertThreatTypeToListNames(threatType, _retval); } \
  NS_IMETHOD ConvertListNameToThreatType(const nsACString& listName, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertListNameToThreatType(listName, _retval); } \
  NS_IMETHOD MakeUpdateRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aStatesBase64, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeUpdateRequestV4(aListNames, aStatesBase64, _retval); } \
  NS_IMETHOD MakeFindFullHashRequestV4(const nsTArray<nsCString >& aListNames, const nsTArray<nsCString >& aListStatesBase64, const nsTArray<nsCString >& aPrefixes, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeFindFullHashRequestV4(aListNames, aListStatesBase64, aPrefixes, _retval); } \
  NS_IMETHOD MakeThreatHitReport(nsIChannel *aChannel, const nsACString& aListName, const nsACString& aHashBase64, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MakeThreatHitReport(aChannel, aListName, aHashBase64, _retval); } \
  NS_IMETHOD ParseFindFullHashResponseV4(const nsACString& aResponse, nsIUrlClassifierParseFindFullHashCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseFindFullHashResponseV4(aResponse, aCallback); } 


#endif /* __gen_nsIUrlClassifierUtils_h__ */
