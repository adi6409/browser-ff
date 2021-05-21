/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozILocaleService.idl
 */

#ifndef __gen_mozILocaleService_h__
#define __gen_mozILocaleService_h__


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
// Define Contractid and CID
#define MOZ_LOCALESERVICE_CID \
  { 0x92735ff4, 0x6384, 0x4ad6, { 0x85, 0x08, 0x75, 0x70, 0x10, 0xe1, 0x49, 0xee } }
#define MOZ_LOCALESERVICE_CONTRACTID "@mozilla.org/intl/localeservice;1"

/* starting interface:    mozILocaleService */
#define MOZILOCALESERVICE_IID_STR "c27f8983-b48b-4d1a-92d7-feb8106f212d"

#define MOZILOCALESERVICE_IID \
  {0xc27f8983, 0xb48b, 0x4d1a, \
    { 0x92, 0xd7, 0xfe, 0xb8, 0x10, 0x6f, 0x21, 0x2d }}

class NS_NO_VTABLE mozILocaleService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZILOCALESERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozILocaleService;

  enum {
    langNegStrategyFiltering = 0,
    langNegStrategyMatching = 1,
    langNegStrategyLookup = 2
  };

  /* readonly attribute ACString defaultLocale; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDefaultLocale(nsACString& aDefaultLocale) = 0;

  /* readonly attribute ACString lastFallbackLocale; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastFallbackLocale(nsACString& aLastFallbackLocale) = 0;

  /* readonly attribute Array<ACString> appLocalesAsLangTags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppLocalesAsLangTags(nsTArray<nsCString >& aAppLocalesAsLangTags) = 0;

  /* readonly attribute Array<ACString> appLocalesAsBCP47; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppLocalesAsBCP47(nsTArray<nsCString >& aAppLocalesAsBCP47) = 0;

  /* readonly attribute Array<ACString> regionalPrefsLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) = 0;

  /* readonly attribute Array<ACString> webExposedLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWebExposedLocales(nsTArray<nsCString >& aWebExposedLocales) = 0;

  /* Array<ACString> negotiateLanguages (in Array<AUTF8String> aRequested, in Array<AUTF8String> aAvailable, [optional] in ACString aDefaultLocale, [optional] in long langNegStrategy); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NegotiateLanguages(const nsTArray<nsCString >& aRequested, const nsTArray<nsCString >& aAvailable, const nsACString& aDefaultLocale, int32_t langNegStrategy, nsTArray<nsCString >& _retval) = 0;

  /* readonly attribute ACString appLocaleAsLangTag; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppLocaleAsLangTag(nsACString& aAppLocaleAsLangTag) = 0;

  /* readonly attribute ACString appLocaleAsBCP47; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAppLocaleAsBCP47(nsACString& aAppLocaleAsBCP47) = 0;

  /* attribute Array<ACString> requestedLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequestedLocales(nsTArray<nsCString >& aRequestedLocales) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRequestedLocales(const nsTArray<nsCString >& aRequestedLocales) = 0;

  /* readonly attribute ACString requestedLocale; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRequestedLocale(nsACString& aRequestedLocale) = 0;

  /* attribute Array<ACString> availableLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAvailableLocales(nsTArray<nsCString >& aAvailableLocales) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetAvailableLocales(const nsTArray<nsCString >& aAvailableLocales) = 0;

  /* readonly attribute boolean isAppLocaleRTL; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsAppLocaleRTL(bool *aIsAppLocaleRTL) = 0;

  /* readonly attribute Array<ACString> packagedLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPackagedLocales(nsTArray<nsCString >& aPackagedLocales) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozILocaleService, MOZILOCALESERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZILOCALESERVICE \
  NS_IMETHOD GetDefaultLocale(nsACString& aDefaultLocale) override; \
  NS_IMETHOD GetLastFallbackLocale(nsACString& aLastFallbackLocale) override; \
  NS_IMETHOD GetAppLocalesAsLangTags(nsTArray<nsCString >& aAppLocalesAsLangTags) override; \
  NS_IMETHOD GetAppLocalesAsBCP47(nsTArray<nsCString >& aAppLocalesAsBCP47) override; \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override; \
  NS_IMETHOD GetWebExposedLocales(nsTArray<nsCString >& aWebExposedLocales) override; \
  NS_IMETHOD NegotiateLanguages(const nsTArray<nsCString >& aRequested, const nsTArray<nsCString >& aAvailable, const nsACString& aDefaultLocale, int32_t langNegStrategy, nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetAppLocaleAsLangTag(nsACString& aAppLocaleAsLangTag) override; \
  NS_IMETHOD GetAppLocaleAsBCP47(nsACString& aAppLocaleAsBCP47) override; \
  NS_IMETHOD GetRequestedLocales(nsTArray<nsCString >& aRequestedLocales) override; \
  NS_IMETHOD SetRequestedLocales(const nsTArray<nsCString >& aRequestedLocales) override; \
  NS_IMETHOD GetRequestedLocale(nsACString& aRequestedLocale) override; \
  NS_IMETHOD GetAvailableLocales(nsTArray<nsCString >& aAvailableLocales) override; \
  NS_IMETHOD SetAvailableLocales(const nsTArray<nsCString >& aAvailableLocales) override; \
  NS_IMETHOD GetIsAppLocaleRTL(bool *aIsAppLocaleRTL) override; \
  NS_IMETHOD GetPackagedLocales(nsTArray<nsCString >& aPackagedLocales) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZILOCALESERVICE \
  nsresult GetDefaultLocale(nsACString& aDefaultLocale); \
  nsresult GetLastFallbackLocale(nsACString& aLastFallbackLocale); \
  nsresult GetAppLocalesAsLangTags(nsTArray<nsCString >& aAppLocalesAsLangTags); \
  nsresult GetAppLocalesAsBCP47(nsTArray<nsCString >& aAppLocalesAsBCP47); \
  nsresult GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales); \
  nsresult GetWebExposedLocales(nsTArray<nsCString >& aWebExposedLocales); \
  nsresult NegotiateLanguages(const nsTArray<nsCString >& aRequested, const nsTArray<nsCString >& aAvailable, const nsACString& aDefaultLocale, int32_t langNegStrategy, nsTArray<nsCString >& _retval); \
  nsresult GetAppLocaleAsLangTag(nsACString& aAppLocaleAsLangTag); \
  nsresult GetAppLocaleAsBCP47(nsACString& aAppLocaleAsBCP47); \
  nsresult GetRequestedLocales(nsTArray<nsCString >& aRequestedLocales); \
  nsresult SetRequestedLocales(const nsTArray<nsCString >& aRequestedLocales); \
  nsresult GetRequestedLocale(nsACString& aRequestedLocale); \
  nsresult GetAvailableLocales(nsTArray<nsCString >& aAvailableLocales); \
  nsresult SetAvailableLocales(const nsTArray<nsCString >& aAvailableLocales); \
  nsresult GetIsAppLocaleRTL(bool *aIsAppLocaleRTL); \
  nsresult GetPackagedLocales(nsTArray<nsCString >& aPackagedLocales); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZILOCALESERVICE(_to) \
  NS_IMETHOD GetDefaultLocale(nsACString& aDefaultLocale) override { return _to GetDefaultLocale(aDefaultLocale); } \
  NS_IMETHOD GetLastFallbackLocale(nsACString& aLastFallbackLocale) override { return _to GetLastFallbackLocale(aLastFallbackLocale); } \
  NS_IMETHOD GetAppLocalesAsLangTags(nsTArray<nsCString >& aAppLocalesAsLangTags) override { return _to GetAppLocalesAsLangTags(aAppLocalesAsLangTags); } \
  NS_IMETHOD GetAppLocalesAsBCP47(nsTArray<nsCString >& aAppLocalesAsBCP47) override { return _to GetAppLocalesAsBCP47(aAppLocalesAsBCP47); } \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override { return _to GetRegionalPrefsLocales(aRegionalPrefsLocales); } \
  NS_IMETHOD GetWebExposedLocales(nsTArray<nsCString >& aWebExposedLocales) override { return _to GetWebExposedLocales(aWebExposedLocales); } \
  NS_IMETHOD NegotiateLanguages(const nsTArray<nsCString >& aRequested, const nsTArray<nsCString >& aAvailable, const nsACString& aDefaultLocale, int32_t langNegStrategy, nsTArray<nsCString >& _retval) override { return _to NegotiateLanguages(aRequested, aAvailable, aDefaultLocale, langNegStrategy, _retval); } \
  NS_IMETHOD GetAppLocaleAsLangTag(nsACString& aAppLocaleAsLangTag) override { return _to GetAppLocaleAsLangTag(aAppLocaleAsLangTag); } \
  NS_IMETHOD GetAppLocaleAsBCP47(nsACString& aAppLocaleAsBCP47) override { return _to GetAppLocaleAsBCP47(aAppLocaleAsBCP47); } \
  NS_IMETHOD GetRequestedLocales(nsTArray<nsCString >& aRequestedLocales) override { return _to GetRequestedLocales(aRequestedLocales); } \
  NS_IMETHOD SetRequestedLocales(const nsTArray<nsCString >& aRequestedLocales) override { return _to SetRequestedLocales(aRequestedLocales); } \
  NS_IMETHOD GetRequestedLocale(nsACString& aRequestedLocale) override { return _to GetRequestedLocale(aRequestedLocale); } \
  NS_IMETHOD GetAvailableLocales(nsTArray<nsCString >& aAvailableLocales) override { return _to GetAvailableLocales(aAvailableLocales); } \
  NS_IMETHOD SetAvailableLocales(const nsTArray<nsCString >& aAvailableLocales) override { return _to SetAvailableLocales(aAvailableLocales); } \
  NS_IMETHOD GetIsAppLocaleRTL(bool *aIsAppLocaleRTL) override { return _to GetIsAppLocaleRTL(aIsAppLocaleRTL); } \
  NS_IMETHOD GetPackagedLocales(nsTArray<nsCString >& aPackagedLocales) override { return _to GetPackagedLocales(aPackagedLocales); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZILOCALESERVICE(_to) \
  NS_IMETHOD GetDefaultLocale(nsACString& aDefaultLocale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDefaultLocale(aDefaultLocale); } \
  NS_IMETHOD GetLastFallbackLocale(nsACString& aLastFallbackLocale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastFallbackLocale(aLastFallbackLocale); } \
  NS_IMETHOD GetAppLocalesAsLangTags(nsTArray<nsCString >& aAppLocalesAsLangTags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppLocalesAsLangTags(aAppLocalesAsLangTags); } \
  NS_IMETHOD GetAppLocalesAsBCP47(nsTArray<nsCString >& aAppLocalesAsBCP47) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppLocalesAsBCP47(aAppLocalesAsBCP47); } \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegionalPrefsLocales(aRegionalPrefsLocales); } \
  NS_IMETHOD GetWebExposedLocales(nsTArray<nsCString >& aWebExposedLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWebExposedLocales(aWebExposedLocales); } \
  NS_IMETHOD NegotiateLanguages(const nsTArray<nsCString >& aRequested, const nsTArray<nsCString >& aAvailable, const nsACString& aDefaultLocale, int32_t langNegStrategy, nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NegotiateLanguages(aRequested, aAvailable, aDefaultLocale, langNegStrategy, _retval); } \
  NS_IMETHOD GetAppLocaleAsLangTag(nsACString& aAppLocaleAsLangTag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppLocaleAsLangTag(aAppLocaleAsLangTag); } \
  NS_IMETHOD GetAppLocaleAsBCP47(nsACString& aAppLocaleAsBCP47) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAppLocaleAsBCP47(aAppLocaleAsBCP47); } \
  NS_IMETHOD GetRequestedLocales(nsTArray<nsCString >& aRequestedLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestedLocales(aRequestedLocales); } \
  NS_IMETHOD SetRequestedLocales(const nsTArray<nsCString >& aRequestedLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRequestedLocales(aRequestedLocales); } \
  NS_IMETHOD GetRequestedLocale(nsACString& aRequestedLocale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRequestedLocale(aRequestedLocale); } \
  NS_IMETHOD GetAvailableLocales(nsTArray<nsCString >& aAvailableLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAvailableLocales(aAvailableLocales); } \
  NS_IMETHOD SetAvailableLocales(const nsTArray<nsCString >& aAvailableLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetAvailableLocales(aAvailableLocales); } \
  NS_IMETHOD GetIsAppLocaleRTL(bool *aIsAppLocaleRTL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsAppLocaleRTL(aIsAppLocaleRTL); } \
  NS_IMETHOD GetPackagedLocales(nsTArray<nsCString >& aPackagedLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPackagedLocales(aPackagedLocales); } 


#endif /* __gen_mozILocaleService_h__ */
