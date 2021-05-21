/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozIOSPreferences.idl
 */

#ifndef __gen_mozIOSPreferences_h__
#define __gen_mozIOSPreferences_h__


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
#define MOZ_OSPREFERENCES_CID \
  { 0x65944815, 0xe9ae, 0x48bd, { 0xa2, 0xbf, 0xf1, 0x10, 0x87, 0x20, 0x95, 0x0c } }
#define MOZ_OSPREFERENCES_CONTRACTID "@mozilla.org/intl/ospreferences;1"

/* starting interface:    mozIOSPreferences */
#define MOZIOSPREFERENCES_IID_STR "65944815-e9ae-48bd-a2bf-f1108720950c"

#define MOZIOSPREFERENCES_IID \
  {0x65944815, 0xe9ae, 0x48bd, \
    { 0xa2, 0xbf, 0xf1, 0x10, 0x87, 0x20, 0x95, 0x0c }}

class NS_NO_VTABLE mozIOSPreferences : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIOSPREFERENCES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIOSPreferences;

  enum {
    dateTimeFormatStyleNone = 0,
    dateTimeFormatStyleShort = 1,
    dateTimeFormatStyleMedium = 2,
    dateTimeFormatStyleLong = 3,
    dateTimeFormatStyleFull = 4
  };

  /* readonly attribute Array<ACString> systemLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSystemLocales(nsTArray<nsCString >& aSystemLocales) = 0;

  /* readonly attribute Array<ACString> regionalPrefsLocales; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) = 0;

  /* readonly attribute ACString systemLocale; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSystemLocale(nsACString& aSystemLocale) = 0;

  /* AUTF8String getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDateTimePattern(int32_t timeFormatStyle, int32_t dateFormatStyle, const nsACString& locale, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIOSPreferences, MOZIOSPREFERENCES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIOSPREFERENCES \
  NS_IMETHOD GetSystemLocales(nsTArray<nsCString >& aSystemLocales) override; \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override; \
  NS_IMETHOD GetSystemLocale(nsACString& aSystemLocale) override; \
  NS_IMETHOD GetDateTimePattern(int32_t timeFormatStyle, int32_t dateFormatStyle, const nsACString& locale, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIOSPREFERENCES \
  nsresult GetSystemLocales(nsTArray<nsCString >& aSystemLocales); \
  nsresult GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales); \
  nsresult GetSystemLocale(nsACString& aSystemLocale); \
  nsresult GetDateTimePattern(int32_t timeFormatStyle, int32_t dateFormatStyle, const nsACString& locale, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIOSPREFERENCES(_to) \
  NS_IMETHOD GetSystemLocales(nsTArray<nsCString >& aSystemLocales) override { return _to GetSystemLocales(aSystemLocales); } \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override { return _to GetRegionalPrefsLocales(aRegionalPrefsLocales); } \
  NS_IMETHOD GetSystemLocale(nsACString& aSystemLocale) override { return _to GetSystemLocale(aSystemLocale); } \
  NS_IMETHOD GetDateTimePattern(int32_t timeFormatStyle, int32_t dateFormatStyle, const nsACString& locale, nsACString& _retval) override { return _to GetDateTimePattern(timeFormatStyle, dateFormatStyle, locale, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIOSPREFERENCES(_to) \
  NS_IMETHOD GetSystemLocales(nsTArray<nsCString >& aSystemLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemLocales(aSystemLocales); } \
  NS_IMETHOD GetRegionalPrefsLocales(nsTArray<nsCString >& aRegionalPrefsLocales) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegionalPrefsLocales(aRegionalPrefsLocales); } \
  NS_IMETHOD GetSystemLocale(nsACString& aSystemLocale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSystemLocale(aSystemLocale); } \
  NS_IMETHOD GetDateTimePattern(int32_t timeFormatStyle, int32_t dateFormatStyle, const nsACString& locale, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDateTimePattern(timeFormatStyle, dateFormatStyle, locale, _retval); } 


#endif /* __gen_mozIOSPreferences_h__ */
