/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/l10n/mozILocalization.idl
 */

#ifndef __gen_mozILocalization_h__
#define __gen_mozILocalization_h__


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

/* starting interface:    mozILocalization */
#define MOZILOCALIZATION_IID_STR "7d468600-551f-4fe0-98c9-92a53b63ec8d"

#define MOZILOCALIZATION_IID \
  {0x7d468600, 0x551f, 0x4fe0, \
    { 0x98, 0xc9, 0x92, 0xa5, 0x3b, 0x63, 0xec, 0x8d }}

class NS_NO_VTABLE mozILocalization : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZILOCALIZATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozILocalization;

  /* jsval generateBundles (in Array<AString> aResourceIds, in bool aIsSync, in bool eager, in jsval aGenerateBundles, in jsval aGenerateBundlesSync); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GenerateBundles(const nsTArray<nsString >& aResourceIds, bool aIsSync, bool eager, JS::HandleValue aGenerateBundles, JS::HandleValue aGenerateBundlesSync, JS::MutableHandleValue _retval) = 0;

  /* Promise formatMessages (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatMessages(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise formatValues (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatValues(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) = 0;

  /* Promise formatValue (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatValue(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, ::mozilla::dom::Promise * * _retval) = 0;

  /* AUTF8String formatValueSync (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatValueSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, nsACString& _retval) = 0;

  /* Array<AUTF8String> formatValuesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatValuesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<nsCString >& _retval) = 0;

  /* Array<jsval> formatMessagesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FormatMessagesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<JS::Value >& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozILocalization, MOZILOCALIZATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZILOCALIZATION \
  NS_IMETHOD GenerateBundles(const nsTArray<nsString >& aResourceIds, bool aIsSync, bool eager, JS::HandleValue aGenerateBundles, JS::HandleValue aGenerateBundlesSync, JS::MutableHandleValue _retval) override; \
  NS_IMETHOD FormatMessages(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD FormatValues(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD FormatValue(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD FormatValueSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, nsACString& _retval) override; \
  NS_IMETHOD FormatValuesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD FormatMessagesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<JS::Value >& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZILOCALIZATION \
  nsresult GenerateBundles(const nsTArray<nsString >& aResourceIds, bool aIsSync, bool eager, JS::HandleValue aGenerateBundles, JS::HandleValue aGenerateBundlesSync, JS::MutableHandleValue _retval); \
  nsresult FormatMessages(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval); \
  nsresult FormatValues(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval); \
  nsresult FormatValue(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, ::mozilla::dom::Promise * * _retval); \
  nsresult FormatValueSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, nsACString& _retval); \
  nsresult FormatValuesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<nsCString >& _retval); \
  nsresult FormatMessagesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<JS::Value >& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZILOCALIZATION(_to) \
  NS_IMETHOD GenerateBundles(const nsTArray<nsString >& aResourceIds, bool aIsSync, bool eager, JS::HandleValue aGenerateBundles, JS::HandleValue aGenerateBundlesSync, JS::MutableHandleValue _retval) override { return _to GenerateBundles(aResourceIds, aIsSync, eager, aGenerateBundles, aGenerateBundlesSync, _retval); } \
  NS_IMETHOD FormatMessages(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override { return _to FormatMessages(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatValues(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override { return _to FormatValues(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatValue(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, ::mozilla::dom::Promise * * _retval) override { return _to FormatValue(aResourceIds, aBundles, aId, aArgs, _retval); } \
  NS_IMETHOD FormatValueSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, nsACString& _retval) override { return _to FormatValueSync(aResourceIds, aBundles, aId, aArgs, _retval); } \
  NS_IMETHOD FormatValuesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<nsCString >& _retval) override { return _to FormatValuesSync(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatMessagesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<JS::Value >& _retval) override { return _to FormatMessagesSync(aResourceIds, aBundles, aKeys, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZILOCALIZATION(_to) \
  NS_IMETHOD GenerateBundles(const nsTArray<nsString >& aResourceIds, bool aIsSync, bool eager, JS::HandleValue aGenerateBundles, JS::HandleValue aGenerateBundlesSync, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateBundles(aResourceIds, aIsSync, eager, aGenerateBundles, aGenerateBundlesSync, _retval); } \
  NS_IMETHOD FormatMessages(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatMessages(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatValues(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatValues(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatValue(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatValue(aResourceIds, aBundles, aId, aArgs, _retval); } \
  NS_IMETHOD FormatValueSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsACString& aId, JS::HandleValue aArgs, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatValueSync(aResourceIds, aBundles, aId, aArgs, _retval); } \
  NS_IMETHOD FormatValuesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatValuesSync(aResourceIds, aBundles, aKeys, _retval); } \
  NS_IMETHOD FormatMessagesSync(const nsTArray<nsString >& aResourceIds, JS::HandleValue aBundles, const nsTArray<JS::Value >& aKeys, nsTArray<JS::Value >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FormatMessagesSync(aResourceIds, aBundles, aKeys, _retval); } 


#endif /* __gen_mozILocalization_h__ */
