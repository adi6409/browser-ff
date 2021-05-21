/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/reputationservice/nsIApplicationReputation.idl
 */

#ifndef __gen_nsIApplicationReputation_h__
#define __gen_nsIApplicationReputation_h__


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
class nsIApplicationReputationCallback; /* forward declaration */

class nsIApplicationReputationQuery; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIReferrerInfo; /* forward declaration */


/* starting interface:    nsIApplicationReputationService */
#define NS_IAPPLICATIONREPUTATIONSERVICE_IID_STR "c9f03479-fd68-4393-acb2-c88d4f563174"

#define NS_IAPPLICATIONREPUTATIONSERVICE_IID \
  {0xc9f03479, 0xfd68, 0x4393, \
    { 0xac, 0xb2, 0xc8, 0x8d, 0x4f, 0x56, 0x31, 0x74 }}

class NS_NO_VTABLE nsIApplicationReputationService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONREPUTATIONSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationReputationService;

  enum {
    VERDICT_SAFE = 0U,
    VERDICT_DANGEROUS = 1U,
    VERDICT_UNCOMMON = 2U,
    VERDICT_POTENTIALLY_UNWANTED = 3U,
    VERDICT_DANGEROUS_HOST = 4U
  };

  /* void queryReputation (in nsIApplicationReputationQuery aQuery, in nsIApplicationReputationCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD QueryReputation(nsIApplicationReputationQuery *aQuery, nsIApplicationReputationCallback *aCallback) = 0;

  /* bool isBinary (in AUTF8String aFilename); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsBinary(const nsACString& aFilename, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationReputationService, NS_IAPPLICATIONREPUTATIONSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONREPUTATIONSERVICE \
  NS_IMETHOD QueryReputation(nsIApplicationReputationQuery *aQuery, nsIApplicationReputationCallback *aCallback) override; \
  NS_IMETHOD IsBinary(const nsACString& aFilename, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONREPUTATIONSERVICE \
  nsresult QueryReputation(nsIApplicationReputationQuery *aQuery, nsIApplicationReputationCallback *aCallback); \
  nsresult IsBinary(const nsACString& aFilename, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONREPUTATIONSERVICE(_to) \
  NS_IMETHOD QueryReputation(nsIApplicationReputationQuery *aQuery, nsIApplicationReputationCallback *aCallback) override { return _to QueryReputation(aQuery, aCallback); } \
  NS_IMETHOD IsBinary(const nsACString& aFilename, bool *_retval) override { return _to IsBinary(aFilename, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONREPUTATIONSERVICE(_to) \
  NS_IMETHOD QueryReputation(nsIApplicationReputationQuery *aQuery, nsIApplicationReputationCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->QueryReputation(aQuery, aCallback); } \
  NS_IMETHOD IsBinary(const nsACString& aFilename, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsBinary(aFilename, _retval); } 


/* starting interface:    nsIApplicationReputationQuery */
#define NS_IAPPLICATIONREPUTATIONQUERY_IID_STR "812d7509-a9a3-446e-a66f-3ed8cc91ebd0"

#define NS_IAPPLICATIONREPUTATIONQUERY_IID \
  {0x812d7509, 0xa9a3, 0x446e, \
    { 0xa6, 0x6f, 0x3e, 0xd8, 0xcc, 0x91, 0xeb, 0xd0 }}

class NS_NO_VTABLE nsIApplicationReputationQuery : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONREPUTATIONQUERY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationReputationQuery;

  /* readonly attribute nsIURI sourceURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSourceURI(nsIURI **aSourceURI) = 0;

  /* readonly attribute nsIReferrerInfo referrerInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) = 0;

  /* readonly attribute AUTF8String suggestedFileName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSuggestedFileName(nsACString& aSuggestedFileName) = 0;

  /* readonly attribute unsigned long fileSize; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFileSize(uint32_t *aFileSize) = 0;

  /* readonly attribute ACString sha256Hash; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) = 0;

  /* readonly attribute Array<Array<Array<uint8_t>>> signatureInfo; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) = 0;

  /* readonly attribute nsIArray redirects; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRedirects(nsIArray **aRedirects) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationReputationQuery, NS_IAPPLICATIONREPUTATIONQUERY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONREPUTATIONQUERY \
  NS_IMETHOD GetSourceURI(nsIURI **aSourceURI) override; \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override; \
  NS_IMETHOD GetSuggestedFileName(nsACString& aSuggestedFileName) override; \
  NS_IMETHOD GetFileSize(uint32_t *aFileSize) override; \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override; \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override; \
  NS_IMETHOD GetRedirects(nsIArray **aRedirects) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONREPUTATIONQUERY \
  nsresult GetSourceURI(nsIURI **aSourceURI); \
  nsresult GetReferrerInfo(nsIReferrerInfo **aReferrerInfo); \
  nsresult GetSuggestedFileName(nsACString& aSuggestedFileName); \
  nsresult GetFileSize(uint32_t *aFileSize); \
  nsresult GetSha256Hash(nsACString& aSha256Hash); \
  nsresult GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo); \
  nsresult GetRedirects(nsIArray **aRedirects); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONREPUTATIONQUERY(_to) \
  NS_IMETHOD GetSourceURI(nsIURI **aSourceURI) override { return _to GetSourceURI(aSourceURI); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return _to GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetSuggestedFileName(nsACString& aSuggestedFileName) override { return _to GetSuggestedFileName(aSuggestedFileName); } \
  NS_IMETHOD GetFileSize(uint32_t *aFileSize) override { return _to GetFileSize(aFileSize); } \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override { return _to GetSha256Hash(aSha256Hash); } \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return _to GetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD GetRedirects(nsIArray **aRedirects) override { return _to GetRedirects(aRedirects); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONREPUTATIONQUERY(_to) \
  NS_IMETHOD GetSourceURI(nsIURI **aSourceURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSourceURI(aSourceURI); } \
  NS_IMETHOD GetReferrerInfo(nsIReferrerInfo **aReferrerInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReferrerInfo(aReferrerInfo); } \
  NS_IMETHOD GetSuggestedFileName(nsACString& aSuggestedFileName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSuggestedFileName(aSuggestedFileName); } \
  NS_IMETHOD GetFileSize(uint32_t *aFileSize) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFileSize(aFileSize); } \
  NS_IMETHOD GetSha256Hash(nsACString& aSha256Hash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSha256Hash(aSha256Hash); } \
  NS_IMETHOD GetSignatureInfo(nsTArray<nsTArray<nsTArray<uint8_t >>>& aSignatureInfo) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSignatureInfo(aSignatureInfo); } \
  NS_IMETHOD GetRedirects(nsIArray **aRedirects) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRedirects(aRedirects); } 


/* starting interface:    nsIApplicationReputationCallback */
#define NS_IAPPLICATIONREPUTATIONCALLBACK_IID_STR "9a228470-cfe5-11e2-8b8b-0800200c9a66"

#define NS_IAPPLICATIONREPUTATIONCALLBACK_IID \
  {0x9a228470, 0xcfe5, 0x11e2, \
    { 0x8b, 0x8b, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIApplicationReputationCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAPPLICATIONREPUTATIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIApplicationReputationCallback;

  /* void onComplete (in bool aShouldBlock, in nsresult aStatus, in unsigned long aVerdict); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnComplete(bool aShouldBlock, nsresult aStatus, uint32_t aVerdict) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIApplicationReputationCallback, NS_IAPPLICATIONREPUTATIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAPPLICATIONREPUTATIONCALLBACK \
  NS_IMETHOD OnComplete(bool aShouldBlock, nsresult aStatus, uint32_t aVerdict) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAPPLICATIONREPUTATIONCALLBACK \
  nsresult OnComplete(bool aShouldBlock, nsresult aStatus, uint32_t aVerdict); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAPPLICATIONREPUTATIONCALLBACK(_to) \
  NS_IMETHOD OnComplete(bool aShouldBlock, nsresult aStatus, uint32_t aVerdict) override { return _to OnComplete(aShouldBlock, aStatus, aVerdict); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAPPLICATIONREPUTATIONCALLBACK(_to) \
  NS_IMETHOD OnComplete(bool aShouldBlock, nsresult aStatus, uint32_t aVerdict) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnComplete(aShouldBlock, aStatus, aVerdict); } 


#endif /* __gen_nsIApplicationReputation_h__ */
