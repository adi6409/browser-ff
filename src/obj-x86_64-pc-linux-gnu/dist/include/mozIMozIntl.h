/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/mozintl/mozIMozIntl.idl
 */

#ifndef __gen_mozIMozIntl_h__
#define __gen_mozIMozIntl_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIMozIntl */
#define MOZIMOZINTL_IID_STR "7f63279a-1a29-4ae6-9e7a-dc9684a23530"

#define MOZIMOZINTL_IID \
  {0x7f63279a, 0x1a29, 0x4ae6, \
    { 0x9e, 0x7a, 0xdc, 0x96, 0x84, 0xa2, 0x35, 0x30 }}

class NS_NO_VTABLE mozIMozIntl : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZIMOZINTL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIMozIntl;

  /* jsval getCalendarInfo ([optional] in jsval locales); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCalendarInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) = 0;

  /* jsval getDisplayNames ([optional] in jsval locales, [optional] in jsval options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDisplayNames(JS::HandleValue locales, JS::HandleValue options, JS::MutableHandleValue _retval) = 0;

  /* jsval getLocaleInfo ([optional] in jsval locales); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocaleInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) = 0;

  /* jsval getAvailableLocaleDisplayNames (in jsval type); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAvailableLocaleDisplayNames(JS::HandleValue type, JS::MutableHandleValue _retval) = 0;

  /* jsval getLanguageDisplayNames (in jsval locales, in jsval langCodes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLanguageDisplayNames(JS::HandleValue locales, JS::HandleValue langCodes, JS::MutableHandleValue _retval) = 0;

  /* jsval getRegionDisplayNames (in jsval locales, in jsval regionCodes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRegionDisplayNames(JS::HandleValue locales, JS::HandleValue regionCodes, JS::MutableHandleValue _retval) = 0;

  /* jsval getLocaleDisplayNames (in jsval locales, in jsval localeCodes); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocaleDisplayNames(JS::HandleValue locales, JS::HandleValue localeCodes, JS::MutableHandleValue _retval) = 0;

  /* readonly attribute jsval Collator; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCollator(JS::MutableHandleValue aCollator) = 0;

  /* readonly attribute jsval DateTimeFormat; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDateTimeFormat(JS::MutableHandleValue aDateTimeFormat) = 0;

  /* readonly attribute jsval ListFormat; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListFormat(JS::MutableHandleValue aListFormat) = 0;

  /* readonly attribute jsval Locale; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLocale(JS::MutableHandleValue aLocale) = 0;

  /* readonly attribute jsval NumberFormat; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetNumberFormat(JS::MutableHandleValue aNumberFormat) = 0;

  /* readonly attribute jsval PluralRules; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPluralRules(JS::MutableHandleValue aPluralRules) = 0;

  /* readonly attribute jsval RelativeTimeFormat; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRelativeTimeFormat(JS::MutableHandleValue aRelativeTimeFormat) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIMozIntl, MOZIMOZINTL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZIMOZINTL \
  NS_IMETHOD GetCalendarInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetDisplayNames(JS::HandleValue locales, JS::HandleValue options, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetLocaleInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetAvailableLocaleDisplayNames(JS::HandleValue type, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetLanguageDisplayNames(JS::HandleValue locales, JS::HandleValue langCodes, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetRegionDisplayNames(JS::HandleValue locales, JS::HandleValue regionCodes, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetLocaleDisplayNames(JS::HandleValue locales, JS::HandleValue localeCodes, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD GetCollator(JS::MutableHandleValue aCollator) override; \
  NS_IMETHOD GetDateTimeFormat(JS::MutableHandleValue aDateTimeFormat) override; \
  NS_IMETHOD GetListFormat(JS::MutableHandleValue aListFormat) override; \
  NS_IMETHOD GetLocale(JS::MutableHandleValue aLocale) override; \
  NS_IMETHOD GetNumberFormat(JS::MutableHandleValue aNumberFormat) override; \
  NS_IMETHOD GetPluralRules(JS::MutableHandleValue aPluralRules) override; \
  NS_IMETHOD GetRelativeTimeFormat(JS::MutableHandleValue aRelativeTimeFormat) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZIMOZINTL \
  nsresult GetCalendarInfo(JS::HandleValue locales, JS::MutableHandleValue _retval); \
  nsresult GetDisplayNames(JS::HandleValue locales, JS::HandleValue options, JS::MutableHandleValue _retval); \
  nsresult GetLocaleInfo(JS::HandleValue locales, JS::MutableHandleValue _retval); \
  nsresult GetAvailableLocaleDisplayNames(JS::HandleValue type, JS::MutableHandleValue _retval); \
  nsresult GetLanguageDisplayNames(JS::HandleValue locales, JS::HandleValue langCodes, JS::MutableHandleValue _retval); \
  nsresult GetRegionDisplayNames(JS::HandleValue locales, JS::HandleValue regionCodes, JS::MutableHandleValue _retval); \
  nsresult GetLocaleDisplayNames(JS::HandleValue locales, JS::HandleValue localeCodes, JS::MutableHandleValue _retval); \
  nsresult GetCollator(JS::MutableHandleValue aCollator); \
  nsresult GetDateTimeFormat(JS::MutableHandleValue aDateTimeFormat); \
  nsresult GetListFormat(JS::MutableHandleValue aListFormat); \
  nsresult GetLocale(JS::MutableHandleValue aLocale); \
  nsresult GetNumberFormat(JS::MutableHandleValue aNumberFormat); \
  nsresult GetPluralRules(JS::MutableHandleValue aPluralRules); \
  nsresult GetRelativeTimeFormat(JS::MutableHandleValue aRelativeTimeFormat); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZIMOZINTL(_to) \
  NS_IMETHOD GetCalendarInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override { return _to GetCalendarInfo(locales, _retval); } \
  NS_IMETHOD GetDisplayNames(JS::HandleValue locales, JS::HandleValue options, JS::MutableHandleValue _retval) override { return _to GetDisplayNames(locales, options, _retval); } \
  NS_IMETHOD GetLocaleInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override { return _to GetLocaleInfo(locales, _retval); } \
  NS_IMETHOD GetAvailableLocaleDisplayNames(JS::HandleValue type, JS::MutableHandleValue _retval) override { return _to GetAvailableLocaleDisplayNames(type, _retval); } \
  NS_IMETHOD GetLanguageDisplayNames(JS::HandleValue locales, JS::HandleValue langCodes, JS::MutableHandleValue _retval) override { return _to GetLanguageDisplayNames(locales, langCodes, _retval); } \
  NS_IMETHOD GetRegionDisplayNames(JS::HandleValue locales, JS::HandleValue regionCodes, JS::MutableHandleValue _retval) override { return _to GetRegionDisplayNames(locales, regionCodes, _retval); } \
  NS_IMETHOD GetLocaleDisplayNames(JS::HandleValue locales, JS::HandleValue localeCodes, JS::MutableHandleValue _retval) override { return _to GetLocaleDisplayNames(locales, localeCodes, _retval); } \
  NS_IMETHOD GetCollator(JS::MutableHandleValue aCollator) override { return _to GetCollator(aCollator); } \
  NS_IMETHOD GetDateTimeFormat(JS::MutableHandleValue aDateTimeFormat) override { return _to GetDateTimeFormat(aDateTimeFormat); } \
  NS_IMETHOD GetListFormat(JS::MutableHandleValue aListFormat) override { return _to GetListFormat(aListFormat); } \
  NS_IMETHOD GetLocale(JS::MutableHandleValue aLocale) override { return _to GetLocale(aLocale); } \
  NS_IMETHOD GetNumberFormat(JS::MutableHandleValue aNumberFormat) override { return _to GetNumberFormat(aNumberFormat); } \
  NS_IMETHOD GetPluralRules(JS::MutableHandleValue aPluralRules) override { return _to GetPluralRules(aPluralRules); } \
  NS_IMETHOD GetRelativeTimeFormat(JS::MutableHandleValue aRelativeTimeFormat) override { return _to GetRelativeTimeFormat(aRelativeTimeFormat); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZIMOZINTL(_to) \
  NS_IMETHOD GetCalendarInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCalendarInfo(locales, _retval); } \
  NS_IMETHOD GetDisplayNames(JS::HandleValue locales, JS::HandleValue options, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDisplayNames(locales, options, _retval); } \
  NS_IMETHOD GetLocaleInfo(JS::HandleValue locales, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocaleInfo(locales, _retval); } \
  NS_IMETHOD GetAvailableLocaleDisplayNames(JS::HandleValue type, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAvailableLocaleDisplayNames(type, _retval); } \
  NS_IMETHOD GetLanguageDisplayNames(JS::HandleValue locales, JS::HandleValue langCodes, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLanguageDisplayNames(locales, langCodes, _retval); } \
  NS_IMETHOD GetRegionDisplayNames(JS::HandleValue locales, JS::HandleValue regionCodes, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRegionDisplayNames(locales, regionCodes, _retval); } \
  NS_IMETHOD GetLocaleDisplayNames(JS::HandleValue locales, JS::HandleValue localeCodes, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocaleDisplayNames(locales, localeCodes, _retval); } \
  NS_IMETHOD GetCollator(JS::MutableHandleValue aCollator) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCollator(aCollator); } \
  NS_IMETHOD GetDateTimeFormat(JS::MutableHandleValue aDateTimeFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDateTimeFormat(aDateTimeFormat); } \
  NS_IMETHOD GetListFormat(JS::MutableHandleValue aListFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListFormat(aListFormat); } \
  NS_IMETHOD GetLocale(JS::MutableHandleValue aLocale) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLocale(aLocale); } \
  NS_IMETHOD GetNumberFormat(JS::MutableHandleValue aNumberFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNumberFormat(aNumberFormat); } \
  NS_IMETHOD GetPluralRules(JS::MutableHandleValue aPluralRules) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPluralRules(aPluralRules); } \
  NS_IMETHOD GetRelativeTimeFormat(JS::MutableHandleValue aRelativeTimeFormat) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelativeTimeFormat(aRelativeTimeFormat); } 


#endif /* __gen_mozIMozIntl_h__ */
