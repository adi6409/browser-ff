/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassifiedChannel.idl
 */

#ifndef __gen_nsIClassifiedChannel_h__
#define __gen_nsIClassifiedChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "mozilla/AlreadyAddRefed.h"
#include "mozilla/Assertions.h"
#include "mozilla/DebugOnly.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIClassifiedChannel */
#define NS_ICLASSIFIEDCHANNEL_IID_STR "70cf6091-a1de-4aa8-8224-058f8964be31"

#define NS_ICLASSIFIEDCHANNEL_IID \
  {0x70cf6091, 0xa1de, 0x4aa8, \
    { 0x82, 0x24, 0x05, 0x8f, 0x89, 0x64, 0xbe, 0x31 }}

class nsIClassifiedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLASSIFIEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClassifiedChannel;

  /* void setMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash); */
  NS_IMETHOD SetMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) = 0;

  /* readonly attribute ACString matchedList; */
  NS_IMETHOD GetMatchedList(nsACString& aMatchedList) = 0;

  /* readonly attribute ACString matchedProvider; */
  NS_IMETHOD GetMatchedProvider(nsACString& aMatchedProvider) = 0;

  /* readonly attribute ACString matchedFullHash; */
  NS_IMETHOD GetMatchedFullHash(nsACString& aMatchedFullHash) = 0;

  /* void setMatchedTrackingInfo (in Array<ACString> aLists, in Array<ACString> aFullHashes); */
  NS_IMETHOD SetMatchedTrackingInfo(const nsTArray<nsCString >& aLists, const nsTArray<nsCString >& aFullHashes) = 0;

  /* readonly attribute Array<ACString> matchedTrackingLists; */
  NS_IMETHOD GetMatchedTrackingLists(nsTArray<nsCString >& aMatchedTrackingLists) = 0;

  /* readonly attribute Array<ACString> matchedTrackingFullHashes; */
  NS_IMETHOD GetMatchedTrackingFullHashes(nsTArray<nsCString >& aMatchedTrackingFullHashes) = 0;

  /* [infallible] readonly attribute unsigned long firstPartyClassificationFlags; */
  NS_IMETHOD GetFirstPartyClassificationFlags(uint32_t *aFirstPartyClassificationFlags) = 0;
  inline uint32_t  GetFirstPartyClassificationFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetFirstPartyClassificationFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long thirdPartyClassificationFlags; */
  NS_IMETHOD GetThirdPartyClassificationFlags(uint32_t *aThirdPartyClassificationFlags) = 0;
  inline uint32_t  GetThirdPartyClassificationFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetThirdPartyClassificationFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  /* [infallible] readonly attribute unsigned long classificationFlags; */
  NS_IMETHOD GetClassificationFlags(uint32_t *aClassificationFlags) = 0;
  inline uint32_t  GetClassificationFlags()
  {
    uint32_t result;
    mozilla::DebugOnly<nsresult> rv = GetClassificationFlags(&result);
    MOZ_ASSERT(NS_SUCCEEDED(rv));
    return result;
  }

  enum ClassificationFlags : uint32_t {
    CLASSIFIED_FINGERPRINTING = 1,
    CLASSIFIED_FINGERPRINTING_CONTENT = 128,
    CLASSIFIED_CRYPTOMINING = 2,
    CLASSIFIED_CRYPTOMINING_CONTENT = 256,
    CLASSIFIED_TRACKING = 4,
    CLASSIFIED_TRACKING_AD = 8,
    CLASSIFIED_TRACKING_ANALYTICS = 16,
    CLASSIFIED_TRACKING_SOCIAL = 32,
    CLASSIFIED_TRACKING_CONTENT = 64,
    CLASSIFIED_SOCIALTRACKING = 512,
    CLASSIFIED_SOCIALTRACKING_FACEBOOK = 1024,
    CLASSIFIED_SOCIALTRACKING_LINKEDIN = 2048,
    CLASSIFIED_SOCIALTRACKING_TWITTER = 4096,
    CLASSIFIED_ANY_BASIC_TRACKING = 61,
    CLASSIFIED_ANY_STRICT_TRACKING = 253,
    CLASSIFIED_ANY_SOCIAL_TRACKING = 7680,
  };

  /* boolean isThirdPartyTrackingResource (); */
  NS_IMETHOD IsThirdPartyTrackingResource(bool *_retval) = 0;

   inline bool IsThirdPartyTrackingResource()
  {
    bool value = false;
    if (NS_SUCCEEDED(IsThirdPartyTrackingResource(&value)) && value) {
      return true;
    }
    return false;
  }
  /* boolean isThirdPartySocialTrackingResource (); */
  NS_IMETHOD IsThirdPartySocialTrackingResource(bool *_retval) = 0;

   inline bool IsThirdPartySocialTrackingResource()
  {
    bool value = false;
    if (NS_SUCCEEDED(IsThirdPartySocialTrackingResource(&value)) && value) {
      return true;
    }
    return false;
  }
};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClassifiedChannel, NS_ICLASSIFIEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLASSIFIEDCHANNEL \
  NS_IMETHOD SetMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override; \
  NS_IMETHOD GetMatchedList(nsACString& aMatchedList) override; \
  NS_IMETHOD GetMatchedProvider(nsACString& aMatchedProvider) override; \
  NS_IMETHOD GetMatchedFullHash(nsACString& aMatchedFullHash) override; \
  NS_IMETHOD SetMatchedTrackingInfo(const nsTArray<nsCString >& aLists, const nsTArray<nsCString >& aFullHashes) override; \
  NS_IMETHOD GetMatchedTrackingLists(nsTArray<nsCString >& aMatchedTrackingLists) override; \
  NS_IMETHOD GetMatchedTrackingFullHashes(nsTArray<nsCString >& aMatchedTrackingFullHashes) override; \
  using nsIClassifiedChannel::GetFirstPartyClassificationFlags; \
  NS_IMETHOD GetFirstPartyClassificationFlags(uint32_t *aFirstPartyClassificationFlags) override; \
  using nsIClassifiedChannel::GetThirdPartyClassificationFlags; \
  NS_IMETHOD GetThirdPartyClassificationFlags(uint32_t *aThirdPartyClassificationFlags) override; \
  using nsIClassifiedChannel::GetClassificationFlags; \
  NS_IMETHOD GetClassificationFlags(uint32_t *aClassificationFlags) override; \
  NS_IMETHOD IsThirdPartyTrackingResource(bool *_retval) override; \
  NS_IMETHOD IsThirdPartySocialTrackingResource(bool *_retval) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLASSIFIEDCHANNEL \
  nsresult SetMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash); \
  nsresult GetMatchedList(nsACString& aMatchedList); \
  nsresult GetMatchedProvider(nsACString& aMatchedProvider); \
  nsresult GetMatchedFullHash(nsACString& aMatchedFullHash); \
  nsresult SetMatchedTrackingInfo(const nsTArray<nsCString >& aLists, const nsTArray<nsCString >& aFullHashes); \
  nsresult GetMatchedTrackingLists(nsTArray<nsCString >& aMatchedTrackingLists); \
  nsresult GetMatchedTrackingFullHashes(nsTArray<nsCString >& aMatchedTrackingFullHashes); \
  using nsIClassifiedChannel::GetFirstPartyClassificationFlags; \
  nsresult GetFirstPartyClassificationFlags(uint32_t *aFirstPartyClassificationFlags); \
  using nsIClassifiedChannel::GetThirdPartyClassificationFlags; \
  nsresult GetThirdPartyClassificationFlags(uint32_t *aThirdPartyClassificationFlags); \
  using nsIClassifiedChannel::GetClassificationFlags; \
  nsresult GetClassificationFlags(uint32_t *aClassificationFlags); \
  nsresult IsThirdPartyTrackingResource(bool *_retval); \
  nsresult IsThirdPartySocialTrackingResource(bool *_retval); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLASSIFIEDCHANNEL(_to) \
  NS_IMETHOD SetMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return _to SetMatchedInfo(aList, aProvider, aFullHash); } \
  NS_IMETHOD GetMatchedList(nsACString& aMatchedList) override { return _to GetMatchedList(aMatchedList); } \
  NS_IMETHOD GetMatchedProvider(nsACString& aMatchedProvider) override { return _to GetMatchedProvider(aMatchedProvider); } \
  NS_IMETHOD GetMatchedFullHash(nsACString& aMatchedFullHash) override { return _to GetMatchedFullHash(aMatchedFullHash); } \
  NS_IMETHOD SetMatchedTrackingInfo(const nsTArray<nsCString >& aLists, const nsTArray<nsCString >& aFullHashes) override { return _to SetMatchedTrackingInfo(aLists, aFullHashes); } \
  NS_IMETHOD GetMatchedTrackingLists(nsTArray<nsCString >& aMatchedTrackingLists) override { return _to GetMatchedTrackingLists(aMatchedTrackingLists); } \
  NS_IMETHOD GetMatchedTrackingFullHashes(nsTArray<nsCString >& aMatchedTrackingFullHashes) override { return _to GetMatchedTrackingFullHashes(aMatchedTrackingFullHashes); } \
  using nsIClassifiedChannel::GetFirstPartyClassificationFlags; \
  NS_IMETHOD GetFirstPartyClassificationFlags(uint32_t *aFirstPartyClassificationFlags) override { return _to GetFirstPartyClassificationFlags(aFirstPartyClassificationFlags); } \
  using nsIClassifiedChannel::GetThirdPartyClassificationFlags; \
  NS_IMETHOD GetThirdPartyClassificationFlags(uint32_t *aThirdPartyClassificationFlags) override { return _to GetThirdPartyClassificationFlags(aThirdPartyClassificationFlags); } \
  using nsIClassifiedChannel::GetClassificationFlags; \
  NS_IMETHOD GetClassificationFlags(uint32_t *aClassificationFlags) override { return _to GetClassificationFlags(aClassificationFlags); } \
  NS_IMETHOD IsThirdPartyTrackingResource(bool *_retval) override { return _to IsThirdPartyTrackingResource(_retval); } \
  NS_IMETHOD IsThirdPartySocialTrackingResource(bool *_retval) override { return _to IsThirdPartySocialTrackingResource(_retval); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLASSIFIEDCHANNEL(_to) \
  NS_IMETHOD SetMatchedInfo(const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchedInfo(aList, aProvider, aFullHash); } \
  NS_IMETHOD GetMatchedList(nsACString& aMatchedList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchedList(aMatchedList); } \
  NS_IMETHOD GetMatchedProvider(nsACString& aMatchedProvider) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchedProvider(aMatchedProvider); } \
  NS_IMETHOD GetMatchedFullHash(nsACString& aMatchedFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchedFullHash(aMatchedFullHash); } \
  NS_IMETHOD SetMatchedTrackingInfo(const nsTArray<nsCString >& aLists, const nsTArray<nsCString >& aFullHashes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchedTrackingInfo(aLists, aFullHashes); } \
  NS_IMETHOD GetMatchedTrackingLists(nsTArray<nsCString >& aMatchedTrackingLists) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchedTrackingLists(aMatchedTrackingLists); } \
  NS_IMETHOD GetMatchedTrackingFullHashes(nsTArray<nsCString >& aMatchedTrackingFullHashes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchedTrackingFullHashes(aMatchedTrackingFullHashes); } \
  NS_IMETHOD GetFirstPartyClassificationFlags(uint32_t *aFirstPartyClassificationFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFirstPartyClassificationFlags(aFirstPartyClassificationFlags); } \
  NS_IMETHOD GetThirdPartyClassificationFlags(uint32_t *aThirdPartyClassificationFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetThirdPartyClassificationFlags(aThirdPartyClassificationFlags); } \
  NS_IMETHOD GetClassificationFlags(uint32_t *aClassificationFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassificationFlags(aClassificationFlags); } \
  NS_IMETHOD IsThirdPartyTrackingResource(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartyTrackingResource(_retval); } \
  NS_IMETHOD IsThirdPartySocialTrackingResource(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsThirdPartySocialTrackingResource(_retval); } \


#endif /* __gen_nsIClassifiedChannel_h__ */
