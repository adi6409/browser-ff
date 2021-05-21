/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIURIClassifier.idl
 */

#ifndef __gen_nsIURIClassifier_h__
#define __gen_nsIURIClassifier_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIUrlClassifierFeature_h__
#include "nsIUrlClassifierFeature.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#include "nsStringFwd.h"
#include "nsTArrayForwardDeclare.h"
class nsIChannel; /* forward declaration */

class nsISerialEventTarget; /* forward declaration */

class nsIPrincipal; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIUrlClassifierFeatureCallback; /* forward declaration */


/* starting interface:    nsIURIClassifierCallback */
#define NS_IURICLASSIFIERCALLBACK_IID_STR "8face46e-0c96-470f-af40-0037dcd797bd"

#define NS_IURICLASSIFIERCALLBACK_IID \
  {0x8face46e, 0x0c96, 0x470f, \
    { 0xaf, 0x40, 0x00, 0x37, 0xdc, 0xd7, 0x97, 0xbd }}

class NS_NO_VTABLE nsIURIClassifierCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURICLASSIFIERCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIClassifierCallback;

  /* void onClassifyComplete (in nsresult aErrorCode, in ACString aList, in ACString aProvider, in ACString aFullHash); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnClassifyComplete(nsresult aErrorCode, const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIClassifierCallback, NS_IURICLASSIFIERCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURICLASSIFIERCALLBACK \
  NS_IMETHOD OnClassifyComplete(nsresult aErrorCode, const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURICLASSIFIERCALLBACK \
  nsresult OnClassifyComplete(nsresult aErrorCode, const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURICLASSIFIERCALLBACK(_to) \
  NS_IMETHOD OnClassifyComplete(nsresult aErrorCode, const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return _to OnClassifyComplete(aErrorCode, aList, aProvider, aFullHash); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURICLASSIFIERCALLBACK(_to) \
  NS_IMETHOD OnClassifyComplete(nsresult aErrorCode, const nsACString& aList, const nsACString& aProvider, const nsACString& aFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnClassifyComplete(aErrorCode, aList, aProvider, aFullHash); } 


/* starting interface:    nsIURIClassifier */
#define NS_IURICLASSIFIER_IID_STR "596620cc-76e3-4133-9d90-360e59a794cf"

#define NS_IURICLASSIFIER_IID \
  {0x596620cc, 0x76e3, 0x4133, \
    { 0x9d, 0x90, 0x36, 0x0e, 0x59, 0xa7, 0x94, 0xcf }}

class NS_NO_VTABLE nsIURIClassifier : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURICLASSIFIER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIURIClassifier;

  /* boolean classify (in nsIPrincipal aPrincipal, in nsISerialEventTarget aEventTarget, in nsIURIClassifierCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Classify(nsIPrincipal *aPrincipal, nsISerialEventTarget *aEventTarget, nsIURIClassifierCallback *aCallback, bool *_retval) = 0;

  /* void asyncClassifyLocalWithFeatures (in nsIURI aURI, in Array<nsIUrlClassifierFeature> aFeatures, in nsIUrlClassifierFeature_listType aListType, in nsIUrlClassifierFeatureCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncClassifyLocalWithFeatures(nsIURI *aURI, const nsTArray<RefPtr<nsIUrlClassifierFeature>>& aFeatures, nsIUrlClassifierFeature::listType aListType, nsIUrlClassifierFeatureCallback *aCallback) = 0;

  /* nsIUrlClassifierFeature getFeatureByName (in ACString aFeatureName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFeatureByName(const nsACString& aFeatureName, nsIUrlClassifierFeature **_retval) = 0;

  /* Array<ACString> getFeatureNames (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFeatureNames(nsTArray<nsCString >& _retval) = 0;

  /* nsIUrlClassifierFeature createFeatureWithTables (in ACString aName, in Array<ACString> aBlocklistTables, in Array<ACString> aEntitylistTables); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CreateFeatureWithTables(const nsACString& aName, const nsTArray<nsCString >& aBlocklistTables, const nsTArray<nsCString >& aEntitylistTables, nsIUrlClassifierFeature **_retval) = 0;

  /* void sendThreatHitReport (in nsIChannel aChannel, in ACString aProvider, in ACString aList, in ACString aFullHash); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendThreatHitReport(nsIChannel *aChannel, const nsACString& aProvider, const nsACString& aList, const nsACString& aFullHash) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIURIClassifier, NS_IURICLASSIFIER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURICLASSIFIER \
  NS_IMETHOD Classify(nsIPrincipal *aPrincipal, nsISerialEventTarget *aEventTarget, nsIURIClassifierCallback *aCallback, bool *_retval) override; \
  NS_IMETHOD AsyncClassifyLocalWithFeatures(nsIURI *aURI, const nsTArray<RefPtr<nsIUrlClassifierFeature>>& aFeatures, nsIUrlClassifierFeature::listType aListType, nsIUrlClassifierFeatureCallback *aCallback) override; \
  NS_IMETHOD GetFeatureByName(const nsACString& aFeatureName, nsIUrlClassifierFeature **_retval) override; \
  NS_IMETHOD GetFeatureNames(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD CreateFeatureWithTables(const nsACString& aName, const nsTArray<nsCString >& aBlocklistTables, const nsTArray<nsCString >& aEntitylistTables, nsIUrlClassifierFeature **_retval) override; \
  NS_IMETHOD SendThreatHitReport(nsIChannel *aChannel, const nsACString& aProvider, const nsACString& aList, const nsACString& aFullHash) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURICLASSIFIER \
  nsresult Classify(nsIPrincipal *aPrincipal, nsISerialEventTarget *aEventTarget, nsIURIClassifierCallback *aCallback, bool *_retval); \
  nsresult AsyncClassifyLocalWithFeatures(nsIURI *aURI, const nsTArray<RefPtr<nsIUrlClassifierFeature>>& aFeatures, nsIUrlClassifierFeature::listType aListType, nsIUrlClassifierFeatureCallback *aCallback); \
  nsresult GetFeatureByName(const nsACString& aFeatureName, nsIUrlClassifierFeature **_retval); \
  nsresult GetFeatureNames(nsTArray<nsCString >& _retval); \
  nsresult CreateFeatureWithTables(const nsACString& aName, const nsTArray<nsCString >& aBlocklistTables, const nsTArray<nsCString >& aEntitylistTables, nsIUrlClassifierFeature **_retval); \
  nsresult SendThreatHitReport(nsIChannel *aChannel, const nsACString& aProvider, const nsACString& aList, const nsACString& aFullHash); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURICLASSIFIER(_to) \
  NS_IMETHOD Classify(nsIPrincipal *aPrincipal, nsISerialEventTarget *aEventTarget, nsIURIClassifierCallback *aCallback, bool *_retval) override { return _to Classify(aPrincipal, aEventTarget, aCallback, _retval); } \
  NS_IMETHOD AsyncClassifyLocalWithFeatures(nsIURI *aURI, const nsTArray<RefPtr<nsIUrlClassifierFeature>>& aFeatures, nsIUrlClassifierFeature::listType aListType, nsIUrlClassifierFeatureCallback *aCallback) override { return _to AsyncClassifyLocalWithFeatures(aURI, aFeatures, aListType, aCallback); } \
  NS_IMETHOD GetFeatureByName(const nsACString& aFeatureName, nsIUrlClassifierFeature **_retval) override { return _to GetFeatureByName(aFeatureName, _retval); } \
  NS_IMETHOD GetFeatureNames(nsTArray<nsCString >& _retval) override { return _to GetFeatureNames(_retval); } \
  NS_IMETHOD CreateFeatureWithTables(const nsACString& aName, const nsTArray<nsCString >& aBlocklistTables, const nsTArray<nsCString >& aEntitylistTables, nsIUrlClassifierFeature **_retval) override { return _to CreateFeatureWithTables(aName, aBlocklistTables, aEntitylistTables, _retval); } \
  NS_IMETHOD SendThreatHitReport(nsIChannel *aChannel, const nsACString& aProvider, const nsACString& aList, const nsACString& aFullHash) override { return _to SendThreatHitReport(aChannel, aProvider, aList, aFullHash); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURICLASSIFIER(_to) \
  NS_IMETHOD Classify(nsIPrincipal *aPrincipal, nsISerialEventTarget *aEventTarget, nsIURIClassifierCallback *aCallback, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Classify(aPrincipal, aEventTarget, aCallback, _retval); } \
  NS_IMETHOD AsyncClassifyLocalWithFeatures(nsIURI *aURI, const nsTArray<RefPtr<nsIUrlClassifierFeature>>& aFeatures, nsIUrlClassifierFeature::listType aListType, nsIUrlClassifierFeatureCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncClassifyLocalWithFeatures(aURI, aFeatures, aListType, aCallback); } \
  NS_IMETHOD GetFeatureByName(const nsACString& aFeatureName, nsIUrlClassifierFeature **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatureByName(aFeatureName, _retval); } \
  NS_IMETHOD GetFeatureNames(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeatureNames(_retval); } \
  NS_IMETHOD CreateFeatureWithTables(const nsACString& aName, const nsTArray<nsCString >& aBlocklistTables, const nsTArray<nsCString >& aEntitylistTables, nsIUrlClassifierFeature **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CreateFeatureWithTables(aName, aBlocklistTables, aEntitylistTables, _retval); } \
  NS_IMETHOD SendThreatHitReport(nsIChannel *aChannel, const nsACString& aProvider, const nsACString& aList, const nsACString& aFullHash) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendThreatHitReport(aChannel, aProvider, aList, aFullHash); } 


#endif /* __gen_nsIURIClassifier_h__ */
