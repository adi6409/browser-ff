/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/search/nsISearchService.idl
 */

#ifndef __gen_nsISearchService_h__
#define __gen_nsISearchService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIURI; /* forward declaration */

class nsIInputStream; /* forward declaration */


/* starting interface:    nsISearchSubmission */
#define NS_ISEARCHSUBMISSION_IID_STR "5799251f-5b55-4df7-a9e7-0c27812c469a"

#define NS_ISEARCHSUBMISSION_IID \
  {0x5799251f, 0x5b55, 0x4df7, \
    { 0xa9, 0xe7, 0x0c, 0x27, 0x81, 0x2c, 0x46, 0x9a }}

class NS_NO_VTABLE nsISearchSubmission : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEARCHSUBMISSION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISearchSubmission;

  /* readonly attribute nsIInputStream postData; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPostData(nsIInputStream **aPostData) = 0;

  /* readonly attribute nsIURI uri; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetUri(nsIURI **aUri) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISearchSubmission, NS_ISEARCHSUBMISSION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEARCHSUBMISSION \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override; \
  NS_IMETHOD GetUri(nsIURI **aUri) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEARCHSUBMISSION \
  nsresult GetPostData(nsIInputStream **aPostData); \
  nsresult GetUri(nsIURI **aUri); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEARCHSUBMISSION(_to) \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return _to GetPostData(aPostData); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return _to GetUri(aUri); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEARCHSUBMISSION(_to) \
  NS_IMETHOD GetPostData(nsIInputStream **aPostData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPostData(aPostData); } \
  NS_IMETHOD GetUri(nsIURI **aUri) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetUri(aUri); } 


/* starting interface:    nsISearchEngine */
#define NS_ISEARCHENGINE_IID_STR "620bd920-0491-48c8-99a8-d6047e64802d"

#define NS_ISEARCHENGINE_IID \
  {0x620bd920, 0x0491, 0x48c8, \
    { 0x99, 0xa8, 0xd6, 0x04, 0x7e, 0x64, 0x80, 0x2d }}

class NS_NO_VTABLE nsISearchEngine : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEARCHENGINE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISearchEngine;

  /* nsISearchSubmission getSubmission (in AString data, [optional] in AString responseType, [optional] in AString purpose); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSubmission(const nsAString& data, const nsAString& responseType, const nsAString& purpose, nsISearchSubmission **_retval) = 0;

  /* readonly attribute AString searchUrlQueryParamName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchUrlQueryParamName(nsAString& aSearchUrlQueryParamName) = 0;

  /* readonly attribute AString searchUrlPublicSuffix; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchUrlPublicSuffix(nsAString& aSearchUrlPublicSuffix) = 0;

  /* boolean supportsResponseType (in AString responseType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SupportsResponseType(const nsAString& responseType, bool *_retval) = 0;

  /* AString getIconURLBySize (in long width, in long height); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIconURLBySize(int32_t width, int32_t height, nsAString& _retval) = 0;

  /* jsval getIcons (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIcons(JS::MutableHandleValue _retval) = 0;

  /* void speculativeConnect (in jsval options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SpeculativeConnect(JS::HandleValue options) = 0;

  /* attribute AString alias; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAlias(nsAString& aAlias) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAlias(const nsAString& aAlias) = 0;

  /* readonly attribute Array<AString> aliases; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAliases(nsTArray<nsString >& aAliases) = 0;

  /* readonly attribute AString description; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDescription(nsAString& aDescription) = 0;

  /* attribute boolean hidden; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetHidden(bool *aHidden) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetHidden(bool aHidden) = 0;

  /* readonly attribute nsIURI iconURI; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIconURI(nsIURI **aIconURI) = 0;

  /* readonly attribute AString name; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetName(nsAString& aName) = 0;

  /* readonly attribute AString searchForm; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchForm(nsAString& aSearchForm) = 0;

  /* readonly attribute boolean sendAttributionRequest; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSendAttributionRequest(bool *aSendAttributionRequest) = 0;

  /* readonly attribute AString telemetryId; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTelemetryId(nsAString& aTelemetryId) = 0;

  /* readonly attribute AString identifier; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIdentifier(nsAString& aIdentifier) = 0;

  /* readonly attribute boolean isAppProvided; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsAppProvided(bool *aIsAppProvided) = 0;

  /* AString getResultDomain ([optional] in AString responseType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetResultDomain(const nsAString& responseType, nsAString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISearchEngine, NS_ISEARCHENGINE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEARCHENGINE \
  NS_IMETHOD GetSubmission(const nsAString& data, const nsAString& responseType, const nsAString& purpose, nsISearchSubmission **_retval) override; \
  NS_IMETHOD GetSearchUrlQueryParamName(nsAString& aSearchUrlQueryParamName) override; \
  NS_IMETHOD GetSearchUrlPublicSuffix(nsAString& aSearchUrlPublicSuffix) override; \
  NS_IMETHOD SupportsResponseType(const nsAString& responseType, bool *_retval) override; \
  NS_IMETHOD GetIconURLBySize(int32_t width, int32_t height, nsAString& _retval) override; \
  NS_IMETHOD GetIcons(JS::MutableHandleValue _retval) override; \
  NS_IMETHOD SpeculativeConnect(JS::HandleValue options) override; \
  NS_IMETHOD GetAlias(nsAString& aAlias) override; \
  NS_IMETHOD SetAlias(const nsAString& aAlias) override; \
  NS_IMETHOD GetAliases(nsTArray<nsString >& aAliases) override; \
  NS_IMETHOD GetDescription(nsAString& aDescription) override; \
  NS_IMETHOD GetHidden(bool *aHidden) override; \
  NS_IMETHOD SetHidden(bool aHidden) override; \
  NS_IMETHOD GetIconURI(nsIURI **aIconURI) override; \
  NS_IMETHOD GetName(nsAString& aName) override; \
  NS_IMETHOD GetSearchForm(nsAString& aSearchForm) override; \
  NS_IMETHOD GetSendAttributionRequest(bool *aSendAttributionRequest) override; \
  NS_IMETHOD GetTelemetryId(nsAString& aTelemetryId) override; \
  NS_IMETHOD GetIdentifier(nsAString& aIdentifier) override; \
  NS_IMETHOD GetIsAppProvided(bool *aIsAppProvided) override; \
  NS_IMETHOD GetResultDomain(const nsAString& responseType, nsAString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEARCHENGINE \
  nsresult GetSubmission(const nsAString& data, const nsAString& responseType, const nsAString& purpose, nsISearchSubmission **_retval); \
  nsresult GetSearchUrlQueryParamName(nsAString& aSearchUrlQueryParamName); \
  nsresult GetSearchUrlPublicSuffix(nsAString& aSearchUrlPublicSuffix); \
  nsresult SupportsResponseType(const nsAString& responseType, bool *_retval); \
  nsresult GetIconURLBySize(int32_t width, int32_t height, nsAString& _retval); \
  nsresult GetIcons(JS::MutableHandleValue _retval); \
  nsresult SpeculativeConnect(JS::HandleValue options); \
  nsresult GetAlias(nsAString& aAlias); \
  nsresult SetAlias(const nsAString& aAlias); \
  nsresult GetAliases(nsTArray<nsString >& aAliases); \
  nsresult GetDescription(nsAString& aDescription); \
  nsresult GetHidden(bool *aHidden); \
  nsresult SetHidden(bool aHidden); \
  nsresult GetIconURI(nsIURI **aIconURI); \
  nsresult GetName(nsAString& aName); \
  nsresult GetSearchForm(nsAString& aSearchForm); \
  nsresult GetSendAttributionRequest(bool *aSendAttributionRequest); \
  nsresult GetTelemetryId(nsAString& aTelemetryId); \
  nsresult GetIdentifier(nsAString& aIdentifier); \
  nsresult GetIsAppProvided(bool *aIsAppProvided); \
  nsresult GetResultDomain(const nsAString& responseType, nsAString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEARCHENGINE(_to) \
  NS_IMETHOD GetSubmission(const nsAString& data, const nsAString& responseType, const nsAString& purpose, nsISearchSubmission **_retval) override { return _to GetSubmission(data, responseType, purpose, _retval); } \
  NS_IMETHOD GetSearchUrlQueryParamName(nsAString& aSearchUrlQueryParamName) override { return _to GetSearchUrlQueryParamName(aSearchUrlQueryParamName); } \
  NS_IMETHOD GetSearchUrlPublicSuffix(nsAString& aSearchUrlPublicSuffix) override { return _to GetSearchUrlPublicSuffix(aSearchUrlPublicSuffix); } \
  NS_IMETHOD SupportsResponseType(const nsAString& responseType, bool *_retval) override { return _to SupportsResponseType(responseType, _retval); } \
  NS_IMETHOD GetIconURLBySize(int32_t width, int32_t height, nsAString& _retval) override { return _to GetIconURLBySize(width, height, _retval); } \
  NS_IMETHOD GetIcons(JS::MutableHandleValue _retval) override { return _to GetIcons(_retval); } \
  NS_IMETHOD SpeculativeConnect(JS::HandleValue options) override { return _to SpeculativeConnect(options); } \
  NS_IMETHOD GetAlias(nsAString& aAlias) override { return _to GetAlias(aAlias); } \
  NS_IMETHOD SetAlias(const nsAString& aAlias) override { return _to SetAlias(aAlias); } \
  NS_IMETHOD GetAliases(nsTArray<nsString >& aAliases) override { return _to GetAliases(aAliases); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return _to GetDescription(aDescription); } \
  NS_IMETHOD GetHidden(bool *aHidden) override { return _to GetHidden(aHidden); } \
  NS_IMETHOD SetHidden(bool aHidden) override { return _to SetHidden(aHidden); } \
  NS_IMETHOD GetIconURI(nsIURI **aIconURI) override { return _to GetIconURI(aIconURI); } \
  NS_IMETHOD GetName(nsAString& aName) override { return _to GetName(aName); } \
  NS_IMETHOD GetSearchForm(nsAString& aSearchForm) override { return _to GetSearchForm(aSearchForm); } \
  NS_IMETHOD GetSendAttributionRequest(bool *aSendAttributionRequest) override { return _to GetSendAttributionRequest(aSendAttributionRequest); } \
  NS_IMETHOD GetTelemetryId(nsAString& aTelemetryId) override { return _to GetTelemetryId(aTelemetryId); } \
  NS_IMETHOD GetIdentifier(nsAString& aIdentifier) override { return _to GetIdentifier(aIdentifier); } \
  NS_IMETHOD GetIsAppProvided(bool *aIsAppProvided) override { return _to GetIsAppProvided(aIsAppProvided); } \
  NS_IMETHOD GetResultDomain(const nsAString& responseType, nsAString& _retval) override { return _to GetResultDomain(responseType, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEARCHENGINE(_to) \
  NS_IMETHOD GetSubmission(const nsAString& data, const nsAString& responseType, const nsAString& purpose, nsISearchSubmission **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSubmission(data, responseType, purpose, _retval); } \
  NS_IMETHOD GetSearchUrlQueryParamName(nsAString& aSearchUrlQueryParamName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchUrlQueryParamName(aSearchUrlQueryParamName); } \
  NS_IMETHOD GetSearchUrlPublicSuffix(nsAString& aSearchUrlPublicSuffix) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchUrlPublicSuffix(aSearchUrlPublicSuffix); } \
  NS_IMETHOD SupportsResponseType(const nsAString& responseType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SupportsResponseType(responseType, _retval); } \
  NS_IMETHOD GetIconURLBySize(int32_t width, int32_t height, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIconURLBySize(width, height, _retval); } \
  NS_IMETHOD GetIcons(JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIcons(_retval); } \
  NS_IMETHOD SpeculativeConnect(JS::HandleValue options) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SpeculativeConnect(options); } \
  NS_IMETHOD GetAlias(nsAString& aAlias) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAlias(aAlias); } \
  NS_IMETHOD SetAlias(const nsAString& aAlias) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAlias(aAlias); } \
  NS_IMETHOD GetAliases(nsTArray<nsString >& aAliases) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAliases(aAliases); } \
  NS_IMETHOD GetDescription(nsAString& aDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDescription(aDescription); } \
  NS_IMETHOD GetHidden(bool *aHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetHidden(aHidden); } \
  NS_IMETHOD SetHidden(bool aHidden) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetHidden(aHidden); } \
  NS_IMETHOD GetIconURI(nsIURI **aIconURI) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIconURI(aIconURI); } \
  NS_IMETHOD GetName(nsAString& aName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetName(aName); } \
  NS_IMETHOD GetSearchForm(nsAString& aSearchForm) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchForm(aSearchForm); } \
  NS_IMETHOD GetSendAttributionRequest(bool *aSendAttributionRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSendAttributionRequest(aSendAttributionRequest); } \
  NS_IMETHOD GetTelemetryId(nsAString& aTelemetryId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTelemetryId(aTelemetryId); } \
  NS_IMETHOD GetIdentifier(nsAString& aIdentifier) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIdentifier(aIdentifier); } \
  NS_IMETHOD GetIsAppProvided(bool *aIsAppProvided) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAppProvided(aIsAppProvided); } \
  NS_IMETHOD GetResultDomain(const nsAString& responseType, nsAString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResultDomain(responseType, _retval); } 


/* starting interface:    nsISearchParseSubmissionResult */
#define NS_ISEARCHPARSESUBMISSIONRESULT_IID_STR "0dc93e51-a7bf-4a16-862d-4b3469ff6206"

#define NS_ISEARCHPARSESUBMISSIONRESULT_IID \
  {0x0dc93e51, 0xa7bf, 0x4a16, \
    { 0x86, 0x2d, 0x4b, 0x34, 0x69, 0xff, 0x62, 0x06 }}

class NS_NO_VTABLE nsISearchParseSubmissionResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEARCHPARSESUBMISSIONRESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISearchParseSubmissionResult;

  /* readonly attribute nsISearchEngine engine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEngine(nsISearchEngine **aEngine) = 0;

  /* readonly attribute AString terms; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTerms(nsAString& aTerms) = 0;

  /* readonly attribute AString termsParameterName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTermsParameterName(nsAString& aTermsParameterName) = 0;

  /* readonly attribute long termsOffset; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTermsOffset(int32_t *aTermsOffset) = 0;

  /* readonly attribute long termsLength; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetTermsLength(int32_t *aTermsLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISearchParseSubmissionResult, NS_ISEARCHPARSESUBMISSIONRESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEARCHPARSESUBMISSIONRESULT \
  NS_IMETHOD GetEngine(nsISearchEngine **aEngine) override; \
  NS_IMETHOD GetTerms(nsAString& aTerms) override; \
  NS_IMETHOD GetTermsParameterName(nsAString& aTermsParameterName) override; \
  NS_IMETHOD GetTermsOffset(int32_t *aTermsOffset) override; \
  NS_IMETHOD GetTermsLength(int32_t *aTermsLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEARCHPARSESUBMISSIONRESULT \
  nsresult GetEngine(nsISearchEngine **aEngine); \
  nsresult GetTerms(nsAString& aTerms); \
  nsresult GetTermsParameterName(nsAString& aTermsParameterName); \
  nsresult GetTermsOffset(int32_t *aTermsOffset); \
  nsresult GetTermsLength(int32_t *aTermsLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEARCHPARSESUBMISSIONRESULT(_to) \
  NS_IMETHOD GetEngine(nsISearchEngine **aEngine) override { return _to GetEngine(aEngine); } \
  NS_IMETHOD GetTerms(nsAString& aTerms) override { return _to GetTerms(aTerms); } \
  NS_IMETHOD GetTermsParameterName(nsAString& aTermsParameterName) override { return _to GetTermsParameterName(aTermsParameterName); } \
  NS_IMETHOD GetTermsOffset(int32_t *aTermsOffset) override { return _to GetTermsOffset(aTermsOffset); } \
  NS_IMETHOD GetTermsLength(int32_t *aTermsLength) override { return _to GetTermsLength(aTermsLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEARCHPARSESUBMISSIONRESULT(_to) \
  NS_IMETHOD GetEngine(nsISearchEngine **aEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEngine(aEngine); } \
  NS_IMETHOD GetTerms(nsAString& aTerms) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTerms(aTerms); } \
  NS_IMETHOD GetTermsParameterName(nsAString& aTermsParameterName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTermsParameterName(aTermsParameterName); } \
  NS_IMETHOD GetTermsOffset(int32_t *aTermsOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTermsOffset(aTermsOffset); } \
  NS_IMETHOD GetTermsLength(int32_t *aTermsLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTermsLength(aTermsLength); } 


/* starting interface:    nsISearchService */
#define NS_ISEARCHSERVICE_IID_STR "0301834b-2630-440e-8b98-db8dc55f34b9"

#define NS_ISEARCHSERVICE_IID \
  {0x0301834b, 0x2630, 0x440e, \
    { 0x8b, 0x98, 0xdb, 0x8d, 0xc5, 0x5f, 0x34, 0xb9 }}

class NS_NO_VTABLE nsISearchService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISEARCHSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISearchService;

  enum {
    ERROR_DOWNLOAD_FAILURE = 1U,
    ERROR_DUPLICATE_ENGINE = 2U,
    ERROR_ENGINE_CORRUPTED = 3U
  };

  /* Promise init (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute bool isInitialized; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsInitialized(bool *aIsInitialized) = 0;

  /* Promise runBackgroundChecks (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RunBackgroundChecks(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise resetToOriginalDefaultEngine (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetToOriginalDefaultEngine(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise addOpenSearchEngine (in AString engineURL, in AString iconURL); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddOpenSearchEngine(const nsAString& engineURL, const nsAString& iconURL, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise addPolicyEngine (in jsval details); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddPolicyEngine(JS::HandleValue details, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise addUserEngine (in AString name, in AString url, [optional] in AString alias); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddUserEngine(const nsAString& name, const nsAString& url, const nsAString& alias, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise addEngineWithDetails (in AString name, in jsval details); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEngineWithDetails(const nsAString& name, JS::HandleValue details, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise addEnginesFromExtension (in jsval extension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddEnginesFromExtension(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) = 0;

  /* void restoreDefaultEngines (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RestoreDefaultEngines(void) = 0;

  /* Promise getEngineByAlias (in AString alias); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEngineByAlias(const nsAString& alias, ::mozilla::dom::Promise * * _retval) = 0;

  /* nsISearchEngine getEngineByName (in AString aEngineName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEngineByName(const nsAString& aEngineName, nsISearchEngine **_retval) = 0;

  /* Promise getEngines (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEngines(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getVisibleEngines (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetVisibleEngines(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getAppProvidedEngines (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppProvidedEngines(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getEnginesByExtensionID (in AString extensionID); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEnginesByExtensionID(const nsAString& extensionID, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise moveEngine (in nsISearchEngine engine, in long newIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MoveEngine(nsISearchEngine *engine, int32_t newIndex, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise removeEngine (in nsISearchEngine engine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveEngine(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise removeWebExtensionEngine (in AString id); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveWebExtensionEngine(const nsAString& id, ::mozilla::dom::Promise * * _retval) = 0;

  /* readonly attribute nsISearchEngine originalDefaultEngine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOriginalDefaultEngine(nsISearchEngine **aOriginalDefaultEngine) = 0;

  /* readonly attribute nsISearchEngine originalPrivateDefaultEngine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOriginalPrivateDefaultEngine(nsISearchEngine **aOriginalPrivateDefaultEngine) = 0;

  /* attribute nsISearchEngine defaultEngine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultEngine(nsISearchEngine **aDefaultEngine) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultEngine(nsISearchEngine *aDefaultEngine) = 0;

  /* Promise getDefault (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefault(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise setDefault (in nsISearchEngine engine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefault(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) = 0;

  /* attribute nsISearchEngine defaultPrivateEngine; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultPrivateEngine(nsISearchEngine **aDefaultPrivateEngine) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultPrivateEngine(nsISearchEngine *aDefaultPrivateEngine) = 0;

  /* Promise getDefaultPrivate (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultPrivate(::mozilla::dom::Promise * * _retval) = 0;

  /* Promise setDefaultPrivate (in nsISearchEngine engine); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultPrivate(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise maybeSetAndOverrideDefault (in jsval extension); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD MaybeSetAndOverrideDefault(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise getDefaultEngineInfo (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultEngineInfo(::mozilla::dom::Promise * * _retval) = 0;

  /* nsISearchParseSubmissionResult parseSubmissionURL (in AString url); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ParseSubmissionURL(const nsAString& url, nsISearchParseSubmissionResult **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISearchService, NS_ISEARCHSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISEARCHSERVICE \
  NS_IMETHOD Init(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override; \
  NS_IMETHOD RunBackgroundChecks(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ResetToOriginalDefaultEngine(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddOpenSearchEngine(const nsAString& engineURL, const nsAString& iconURL, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddPolicyEngine(JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddUserEngine(const nsAString& name, const nsAString& url, const nsAString& alias, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddEngineWithDetails(const nsAString& name, JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD AddEnginesFromExtension(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD RestoreDefaultEngines(void) override; \
  NS_IMETHOD GetEngineByAlias(const nsAString& alias, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetEngineByName(const nsAString& aEngineName, nsISearchEngine **_retval) override; \
  NS_IMETHOD GetEngines(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetVisibleEngines(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetAppProvidedEngines(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetEnginesByExtensionID(const nsAString& extensionID, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD MoveEngine(nsISearchEngine *engine, int32_t newIndex, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD RemoveEngine(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD RemoveWebExtensionEngine(const nsAString& id, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetOriginalDefaultEngine(nsISearchEngine **aOriginalDefaultEngine) override; \
  NS_IMETHOD GetOriginalPrivateDefaultEngine(nsISearchEngine **aOriginalPrivateDefaultEngine) override; \
  NS_IMETHOD GetDefaultEngine(nsISearchEngine **aDefaultEngine) override; \
  NS_IMETHOD SetDefaultEngine(nsISearchEngine *aDefaultEngine) override; \
  NS_IMETHOD GetDefault(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetDefault(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetDefaultPrivateEngine(nsISearchEngine **aDefaultPrivateEngine) override; \
  NS_IMETHOD SetDefaultPrivateEngine(nsISearchEngine *aDefaultPrivateEngine) override; \
  NS_IMETHOD GetDefaultPrivate(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD SetDefaultPrivate(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD MaybeSetAndOverrideDefault(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD GetDefaultEngineInfo(::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD ParseSubmissionURL(const nsAString& url, nsISearchParseSubmissionResult **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISEARCHSERVICE \
  nsresult Init(::mozilla::dom::Promise * * _retval); \
  nsresult GetIsInitialized(bool *aIsInitialized); \
  nsresult RunBackgroundChecks(::mozilla::dom::Promise * * _retval); \
  nsresult ResetToOriginalDefaultEngine(::mozilla::dom::Promise * * _retval); \
  nsresult AddOpenSearchEngine(const nsAString& engineURL, const nsAString& iconURL, ::mozilla::dom::Promise * * _retval); \
  nsresult AddPolicyEngine(JS::HandleValue details, ::mozilla::dom::Promise * * _retval); \
  nsresult AddUserEngine(const nsAString& name, const nsAString& url, const nsAString& alias, ::mozilla::dom::Promise * * _retval); \
  nsresult AddEngineWithDetails(const nsAString& name, JS::HandleValue details, ::mozilla::dom::Promise * * _retval); \
  nsresult AddEnginesFromExtension(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval); \
  nsresult RestoreDefaultEngines(void); \
  nsresult GetEngineByAlias(const nsAString& alias, ::mozilla::dom::Promise * * _retval); \
  nsresult GetEngineByName(const nsAString& aEngineName, nsISearchEngine **_retval); \
  nsresult GetEngines(::mozilla::dom::Promise * * _retval); \
  nsresult GetVisibleEngines(::mozilla::dom::Promise * * _retval); \
  nsresult GetAppProvidedEngines(::mozilla::dom::Promise * * _retval); \
  nsresult GetEnginesByExtensionID(const nsAString& extensionID, ::mozilla::dom::Promise * * _retval); \
  nsresult MoveEngine(nsISearchEngine *engine, int32_t newIndex, ::mozilla::dom::Promise * * _retval); \
  nsresult RemoveEngine(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval); \
  nsresult RemoveWebExtensionEngine(const nsAString& id, ::mozilla::dom::Promise * * _retval); \
  nsresult GetOriginalDefaultEngine(nsISearchEngine **aOriginalDefaultEngine); \
  nsresult GetOriginalPrivateDefaultEngine(nsISearchEngine **aOriginalPrivateDefaultEngine); \
  nsresult GetDefaultEngine(nsISearchEngine **aDefaultEngine); \
  nsresult SetDefaultEngine(nsISearchEngine *aDefaultEngine); \
  nsresult GetDefault(::mozilla::dom::Promise * * _retval); \
  nsresult SetDefault(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval); \
  nsresult GetDefaultPrivateEngine(nsISearchEngine **aDefaultPrivateEngine); \
  nsresult SetDefaultPrivateEngine(nsISearchEngine *aDefaultPrivateEngine); \
  nsresult GetDefaultPrivate(::mozilla::dom::Promise * * _retval); \
  nsresult SetDefaultPrivate(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval); \
  nsresult MaybeSetAndOverrideDefault(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval); \
  nsresult GetDefaultEngineInfo(::mozilla::dom::Promise * * _retval); \
  nsresult ParseSubmissionURL(const nsAString& url, nsISearchParseSubmissionResult **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISEARCHSERVICE(_to) \
  NS_IMETHOD Init(::mozilla::dom::Promise * * _retval) override { return _to Init(_retval); } \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override { return _to GetIsInitialized(aIsInitialized); } \
  NS_IMETHOD RunBackgroundChecks(::mozilla::dom::Promise * * _retval) override { return _to RunBackgroundChecks(_retval); } \
  NS_IMETHOD ResetToOriginalDefaultEngine(::mozilla::dom::Promise * * _retval) override { return _to ResetToOriginalDefaultEngine(_retval); } \
  NS_IMETHOD AddOpenSearchEngine(const nsAString& engineURL, const nsAString& iconURL, ::mozilla::dom::Promise * * _retval) override { return _to AddOpenSearchEngine(engineURL, iconURL, _retval); } \
  NS_IMETHOD AddPolicyEngine(JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override { return _to AddPolicyEngine(details, _retval); } \
  NS_IMETHOD AddUserEngine(const nsAString& name, const nsAString& url, const nsAString& alias, ::mozilla::dom::Promise * * _retval) override { return _to AddUserEngine(name, url, alias, _retval); } \
  NS_IMETHOD AddEngineWithDetails(const nsAString& name, JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override { return _to AddEngineWithDetails(name, details, _retval); } \
  NS_IMETHOD AddEnginesFromExtension(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override { return _to AddEnginesFromExtension(extension, _retval); } \
  NS_IMETHOD RestoreDefaultEngines(void) override { return _to RestoreDefaultEngines(); } \
  NS_IMETHOD GetEngineByAlias(const nsAString& alias, ::mozilla::dom::Promise * * _retval) override { return _to GetEngineByAlias(alias, _retval); } \
  NS_IMETHOD GetEngineByName(const nsAString& aEngineName, nsISearchEngine **_retval) override { return _to GetEngineByName(aEngineName, _retval); } \
  NS_IMETHOD GetEngines(::mozilla::dom::Promise * * _retval) override { return _to GetEngines(_retval); } \
  NS_IMETHOD GetVisibleEngines(::mozilla::dom::Promise * * _retval) override { return _to GetVisibleEngines(_retval); } \
  NS_IMETHOD GetAppProvidedEngines(::mozilla::dom::Promise * * _retval) override { return _to GetAppProvidedEngines(_retval); } \
  NS_IMETHOD GetEnginesByExtensionID(const nsAString& extensionID, ::mozilla::dom::Promise * * _retval) override { return _to GetEnginesByExtensionID(extensionID, _retval); } \
  NS_IMETHOD MoveEngine(nsISearchEngine *engine, int32_t newIndex, ::mozilla::dom::Promise * * _retval) override { return _to MoveEngine(engine, newIndex, _retval); } \
  NS_IMETHOD RemoveEngine(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return _to RemoveEngine(engine, _retval); } \
  NS_IMETHOD RemoveWebExtensionEngine(const nsAString& id, ::mozilla::dom::Promise * * _retval) override { return _to RemoveWebExtensionEngine(id, _retval); } \
  NS_IMETHOD GetOriginalDefaultEngine(nsISearchEngine **aOriginalDefaultEngine) override { return _to GetOriginalDefaultEngine(aOriginalDefaultEngine); } \
  NS_IMETHOD GetOriginalPrivateDefaultEngine(nsISearchEngine **aOriginalPrivateDefaultEngine) override { return _to GetOriginalPrivateDefaultEngine(aOriginalPrivateDefaultEngine); } \
  NS_IMETHOD GetDefaultEngine(nsISearchEngine **aDefaultEngine) override { return _to GetDefaultEngine(aDefaultEngine); } \
  NS_IMETHOD SetDefaultEngine(nsISearchEngine *aDefaultEngine) override { return _to SetDefaultEngine(aDefaultEngine); } \
  NS_IMETHOD GetDefault(::mozilla::dom::Promise * * _retval) override { return _to GetDefault(_retval); } \
  NS_IMETHOD SetDefault(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return _to SetDefault(engine, _retval); } \
  NS_IMETHOD GetDefaultPrivateEngine(nsISearchEngine **aDefaultPrivateEngine) override { return _to GetDefaultPrivateEngine(aDefaultPrivateEngine); } \
  NS_IMETHOD SetDefaultPrivateEngine(nsISearchEngine *aDefaultPrivateEngine) override { return _to SetDefaultPrivateEngine(aDefaultPrivateEngine); } \
  NS_IMETHOD GetDefaultPrivate(::mozilla::dom::Promise * * _retval) override { return _to GetDefaultPrivate(_retval); } \
  NS_IMETHOD SetDefaultPrivate(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return _to SetDefaultPrivate(engine, _retval); } \
  NS_IMETHOD MaybeSetAndOverrideDefault(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override { return _to MaybeSetAndOverrideDefault(extension, _retval); } \
  NS_IMETHOD GetDefaultEngineInfo(::mozilla::dom::Promise * * _retval) override { return _to GetDefaultEngineInfo(_retval); } \
  NS_IMETHOD ParseSubmissionURL(const nsAString& url, nsISearchParseSubmissionResult **_retval) override { return _to ParseSubmissionURL(url, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISEARCHSERVICE(_to) \
  NS_IMETHOD Init(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(_retval); } \
  NS_IMETHOD GetIsInitialized(bool *aIsInitialized) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInitialized(aIsInitialized); } \
  NS_IMETHOD RunBackgroundChecks(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RunBackgroundChecks(_retval); } \
  NS_IMETHOD ResetToOriginalDefaultEngine(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetToOriginalDefaultEngine(_retval); } \
  NS_IMETHOD AddOpenSearchEngine(const nsAString& engineURL, const nsAString& iconURL, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddOpenSearchEngine(engineURL, iconURL, _retval); } \
  NS_IMETHOD AddPolicyEngine(JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddPolicyEngine(details, _retval); } \
  NS_IMETHOD AddUserEngine(const nsAString& name, const nsAString& url, const nsAString& alias, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddUserEngine(name, url, alias, _retval); } \
  NS_IMETHOD AddEngineWithDetails(const nsAString& name, JS::HandleValue details, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEngineWithDetails(name, details, _retval); } \
  NS_IMETHOD AddEnginesFromExtension(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddEnginesFromExtension(extension, _retval); } \
  NS_IMETHOD RestoreDefaultEngines(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RestoreDefaultEngines(); } \
  NS_IMETHOD GetEngineByAlias(const nsAString& alias, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEngineByAlias(alias, _retval); } \
  NS_IMETHOD GetEngineByName(const nsAString& aEngineName, nsISearchEngine **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEngineByName(aEngineName, _retval); } \
  NS_IMETHOD GetEngines(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEngines(_retval); } \
  NS_IMETHOD GetVisibleEngines(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetVisibleEngines(_retval); } \
  NS_IMETHOD GetAppProvidedEngines(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppProvidedEngines(_retval); } \
  NS_IMETHOD GetEnginesByExtensionID(const nsAString& extensionID, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEnginesByExtensionID(extensionID, _retval); } \
  NS_IMETHOD MoveEngine(nsISearchEngine *engine, int32_t newIndex, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MoveEngine(engine, newIndex, _retval); } \
  NS_IMETHOD RemoveEngine(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveEngine(engine, _retval); } \
  NS_IMETHOD RemoveWebExtensionEngine(const nsAString& id, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveWebExtensionEngine(id, _retval); } \
  NS_IMETHOD GetOriginalDefaultEngine(nsISearchEngine **aOriginalDefaultEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalDefaultEngine(aOriginalDefaultEngine); } \
  NS_IMETHOD GetOriginalPrivateDefaultEngine(nsISearchEngine **aOriginalPrivateDefaultEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOriginalPrivateDefaultEngine(aOriginalPrivateDefaultEngine); } \
  NS_IMETHOD GetDefaultEngine(nsISearchEngine **aDefaultEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultEngine(aDefaultEngine); } \
  NS_IMETHOD SetDefaultEngine(nsISearchEngine *aDefaultEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultEngine(aDefaultEngine); } \
  NS_IMETHOD GetDefault(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefault(_retval); } \
  NS_IMETHOD SetDefault(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefault(engine, _retval); } \
  NS_IMETHOD GetDefaultPrivateEngine(nsISearchEngine **aDefaultPrivateEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultPrivateEngine(aDefaultPrivateEngine); } \
  NS_IMETHOD SetDefaultPrivateEngine(nsISearchEngine *aDefaultPrivateEngine) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultPrivateEngine(aDefaultPrivateEngine); } \
  NS_IMETHOD GetDefaultPrivate(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultPrivate(_retval); } \
  NS_IMETHOD SetDefaultPrivate(nsISearchEngine *engine, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultPrivate(engine, _retval); } \
  NS_IMETHOD MaybeSetAndOverrideDefault(JS::HandleValue extension, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeSetAndOverrideDefault(extension, _retval); } \
  NS_IMETHOD GetDefaultEngineInfo(::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultEngineInfo(_retval); } \
  NS_IMETHOD ParseSubmissionURL(const nsAString& url, nsISearchParseSubmissionResult **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ParseSubmissionURL(url, _retval); } 


#endif /* __gen_nsISearchService_h__ */
