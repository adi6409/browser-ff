/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierFeature.idl
 */

#ifndef __gen_nsIUrlClassifierFeature_h__
#define __gen_nsIUrlClassifierFeature_h__


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
#include "nsStringFwd.h"
#include "nsTArrayForwardDeclare.h"
class nsIChannel; /* forward declaration */

class nsIURI; /* forward declaration */


/* starting interface:    nsIUrlClassifierFeature */
#define NS_IURLCLASSIFIERFEATURE_IID_STR "a6c9b24e-b4f1-426e-af58-2c976c3943a8"

#define NS_IURLCLASSIFIERFEATURE_IID \
  {0xa6c9b24e, 0xb4f1, 0x426e, \
    { 0xaf, 0x58, 0x2c, 0x97, 0x6c, 0x39, 0x43, 0xa8 }}

class NS_NO_VTABLE nsIUrlClassifierFeature : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERFEATURE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierFeature;

  enum listType : uint8_t {
    blocklist = 0,
    entitylist = 1,
  };

  enum URIType : uint8_t {
    blocklistURI = 0,
    entitylistURI = 1,
    pairwiseEntitylistURI = 2,
  };

  /* readonly attribute ACString name; */
  NS_IMETHOD GetName(nsACString& aName) = 0;

  /* [noscript] StringArrayRef getTables (in nsIUrlClassifierFeature_listType aListType); */
  NS_IMETHOD GetTables(nsIUrlClassifierFeature::listType aListType, nsTArray<nsCString> & _retval) = 0;

  /* [noscript] boolean hasTable (in ACString aTable, in nsIUrlClassifierFeature_listType aListType); */
  NS_IMETHOD HasTable(const nsACString& aTable, nsIUrlClassifierFeature::listType aListType, bool *_retval) = 0;

  /* [noscript] boolean hasHostInPreferences (in ACString aHost, in nsIUrlClassifierFeature_listType aListType, out ACString aPrefTableName); */
  NS_IMETHOD HasHostInPreferences(const nsACString& aHost, nsIUrlClassifierFeature::listType aListType, nsACString& aPrefTableName, bool *_retval) = 0;

  /* readonly attribute ACString exceptionHostList; */
  NS_IMETHOD GetExceptionHostList(nsACString& aExceptionHostList) = 0;

  /* [noscript] boolean processChannel (in nsIChannel aChannel, in ConstStringArrayRef aList, in ConstStringArrayRef aHashes); */
  NS_IMETHOD ProcessChannel(nsIChannel *aChannel, const nsTArray<nsCString> & aList, const nsTArray<nsCString> & aHashes, bool *_retval) = 0;

  /* [noscript] nsIURI getURIByListType (in nsIChannel channel, in nsIUrlClassifierFeature_listType listType, out nsIUrlClassifierFeature_URIType URIType); */
  NS_IMETHOD GetURIByListType(nsIChannel *channel, nsIUrlClassifierFeature::listType listType, nsIUrlClassifierFeature::URIType *URIType, nsIURI **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierFeature, NS_IURLCLASSIFIERFEATURE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERFEATURE \
  NS_IMETHOD GetName(nsACString& aName) override; \
  NS_IMETHOD GetTables(nsIUrlClassifierFeature::listType aListType, nsTArray<nsCString> & _retval) override; \
  NS_IMETHOD HasTable(const nsACString& aTable, nsIUrlClassifierFeature::listType aListType, bool *_retval) override; \
  NS_IMETHOD HasHostInPreferences(const nsACString& aHost, nsIUrlClassifierFeature::listType aListType, nsACString& aPrefTableName, bool *_retval) override; \
  NS_IMETHOD GetExceptionHostList(nsACString& aExceptionHostList) override; \
  NS_IMETHOD ProcessChannel(nsIChannel *aChannel, const nsTArray<nsCString> & aList, const nsTArray<nsCString> & aHashes, bool *_retval) override; \
  NS_IMETHOD GetURIByListType(nsIChannel *channel, nsIUrlClassifierFeature::listType listType, nsIUrlClassifierFeature::URIType *URIType, nsIURI **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERFEATURE \
  nsresult GetName(nsACString& aName); \
  nsresult GetTables(nsIUrlClassifierFeature::listType aListType, nsTArray<nsCString> & _retval); \
  nsresult HasTable(const nsACString& aTable, nsIUrlClassifierFeature::listType aListType, bool *_retval); \
  nsresult HasHostInPreferences(const nsACString& aHost, nsIUrlClassifierFeature::listType aListType, nsACString& aPrefTableName, bool *_retval); \
  nsresult GetExceptionHostList(nsACString& aExceptionHostList); \
  nsresult ProcessChannel(nsIChannel *aChannel, const nsTArray<nsCString> & aList, const nsTArray<nsCString> & aHashes, bool *_retval); \
  nsresult GetURIByListType(nsIChannel *channel, nsIUrlClassifierFeature::listType listType, nsIUrlClassifierFeature::URIType *URIType, nsIURI **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERFEATURE(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetTables(nsIUrlClassifierFeature::listType aListType, nsTArray<nsCString> & _retval) override { return _to GetTables(aListType, _retval); } \
  NS_IMETHOD HasTable(const nsACString& aTable, nsIUrlClassifierFeature::listType aListType, bool *_retval) override { return _to HasTable(aTable, aListType, _retval); } \
  NS_IMETHOD HasHostInPreferences(const nsACString& aHost, nsIUrlClassifierFeature::listType aListType, nsACString& aPrefTableName, bool *_retval) override { return _to HasHostInPreferences(aHost, aListType, aPrefTableName, _retval); } \
  NS_IMETHOD GetExceptionHostList(nsACString& aExceptionHostList) override { return _to GetExceptionHostList(aExceptionHostList); } \
  NS_IMETHOD ProcessChannel(nsIChannel *aChannel, const nsTArray<nsCString> & aList, const nsTArray<nsCString> & aHashes, bool *_retval) override { return _to ProcessChannel(aChannel, aList, aHashes, _retval); } \
  NS_IMETHOD GetURIByListType(nsIChannel *channel, nsIUrlClassifierFeature::listType listType, nsIUrlClassifierFeature::URIType *URIType, nsIURI **_retval) override { return _to GetURIByListType(channel, listType, URIType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERFEATURE(_to) \
  NS_IMETHOD GetName(nsACString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetTables(nsIUrlClassifierFeature::listType aListType, nsTArray<nsCString> & _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTables(aListType, _retval); } \
  NS_IMETHOD HasTable(const nsACString& aTable, nsIUrlClassifierFeature::listType aListType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasTable(aTable, aListType, _retval); } \
  NS_IMETHOD HasHostInPreferences(const nsACString& aHost, nsIUrlClassifierFeature::listType aListType, nsACString& aPrefTableName, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HasHostInPreferences(aHost, aListType, aPrefTableName, _retval); } \
  NS_IMETHOD GetExceptionHostList(nsACString& aExceptionHostList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExceptionHostList(aExceptionHostList); } \
  NS_IMETHOD ProcessChannel(nsIChannel *aChannel, const nsTArray<nsCString> & aList, const nsTArray<nsCString> & aHashes, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ProcessChannel(aChannel, aList, aHashes, _retval); } \
  NS_IMETHOD GetURIByListType(nsIChannel *channel, nsIUrlClassifierFeature::listType listType, nsIUrlClassifierFeature::URIType *URIType, nsIURI **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURIByListType(channel, listType, URIType, _retval); } 


/* starting interface:    nsIUrlClassifierFeatureResult */
#define NS_IURLCLASSIFIERFEATURERESULT_IID_STR "ccb88140-5d66-4873-9815-a1b98d6cdc92"

#define NS_IURLCLASSIFIERFEATURERESULT_IID \
  {0xccb88140, 0x5d66, 0x4873, \
    { 0x98, 0x15, 0xa1, 0xb9, 0x8d, 0x6c, 0xdc, 0x92 }}

class NS_NO_VTABLE nsIUrlClassifierFeatureResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERFEATURERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierFeatureResult;

  /* readonly attribute nsIURI uri; */
  NS_IMETHOD GetUri(nsIURI **aUri) = 0;

  /* readonly attribute nsIUrlClassifierFeature feature; */
  NS_IMETHOD GetFeature(nsIUrlClassifierFeature **aFeature) = 0;

  /* readonly attribute ACString list; */
  NS_IMETHOD GetList(nsACString& aList) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierFeatureResult, NS_IURLCLASSIFIERFEATURERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERFEATURERESULT \
  NS_IMETHOD GetUri(nsIURI **aUri) override; \
  NS_IMETHOD GetFeature(nsIUrlClassifierFeature **aFeature) override; \
  NS_IMETHOD GetList(nsACString& aList) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERFEATURERESULT \
  nsresult GetUri(nsIURI **aUri); \
  nsresult GetFeature(nsIUrlClassifierFeature **aFeature); \
  nsresult GetList(nsACString& aList); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERFEATURERESULT(_to) \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return _to GetUri(aUri); } \
  NS_IMETHOD GetFeature(nsIUrlClassifierFeature **aFeature) override { return _to GetFeature(aFeature); } \
  NS_IMETHOD GetList(nsACString& aList) override { return _to GetList(aList); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERFEATURERESULT(_to) \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUri(aUri); } \
  NS_IMETHOD GetFeature(nsIUrlClassifierFeature **aFeature) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFeature(aFeature); } \
  NS_IMETHOD GetList(nsACString& aList) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetList(aList); } 


/* starting interface:    nsIUrlClassifierFeatureCallback */
#define NS_IURLCLASSIFIERFEATURECALLBACK_IID_STR "2ea83c26-dfc9-44ed-9cfc-171d4753d78e"

#define NS_IURLCLASSIFIERFEATURECALLBACK_IID \
  {0x2ea83c26, 0xdfc9, 0x44ed, \
    { 0x9c, 0xfc, 0x17, 0x1d, 0x47, 0x53, 0xd7, 0x8e }}

class NS_NO_VTABLE nsIUrlClassifierFeatureCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IURLCLASSIFIERFEATURECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUrlClassifierFeatureCallback;

  /* void onClassifyComplete (in Array<nsIUrlClassifierFeatureResult> aResults); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnClassifyComplete(const nsTArray<RefPtr<nsIUrlClassifierFeatureResult>>& aResults) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUrlClassifierFeatureCallback, NS_IURLCLASSIFIERFEATURECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIURLCLASSIFIERFEATURECALLBACK \
  NS_IMETHOD OnClassifyComplete(const nsTArray<RefPtr<nsIUrlClassifierFeatureResult>>& aResults) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIURLCLASSIFIERFEATURECALLBACK \
  nsresult OnClassifyComplete(const nsTArray<RefPtr<nsIUrlClassifierFeatureResult>>& aResults); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIURLCLASSIFIERFEATURECALLBACK(_to) \
  NS_IMETHOD OnClassifyComplete(const nsTArray<RefPtr<nsIUrlClassifierFeatureResult>>& aResults) override { return _to OnClassifyComplete(aResults); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIURLCLASSIFIERFEATURECALLBACK(_to) \
  NS_IMETHOD OnClassifyComplete(const nsTArray<RefPtr<nsIUrlClassifierFeatureResult>>& aResults) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnClassifyComplete(aResults); } 


#endif /* __gen_nsIUrlClassifierFeature_h__ */
