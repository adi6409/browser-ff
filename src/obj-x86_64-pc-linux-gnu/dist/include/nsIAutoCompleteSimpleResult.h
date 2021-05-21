/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSimpleResult.idl
 */

#ifndef __gen_nsIAutoCompleteSimpleResult_h__
#define __gen_nsIAutoCompleteSimpleResult_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIAutoCompleteResult_h__
#include "nsIAutoCompleteResult.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompleteSimpleResultListener; /* forward declaration */


/* starting interface:    nsIAutoCompleteSimpleResult */
#define NS_IAUTOCOMPLETESIMPLERESULT_IID_STR "23de9c96-becb-4d0d-a9bb-1d131ce361b5"

#define NS_IAUTOCOMPLETESIMPLERESULT_IID \
  {0x23de9c96, 0xbecb, 0x4d0d, \
    { 0xa9, 0xbb, 0x1d, 0x13, 0x1c, 0xe3, 0x61, 0xb5 }}

class NS_NO_VTABLE nsIAutoCompleteSimpleResult : public nsIAutoCompleteResult {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETESIMPLERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteSimpleResult;

  /* void setSearchString (in AString aSearchString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchString(const nsAString& aSearchString) = 0;

  /* void setErrorDescription (in AString aErrorDescription); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetErrorDescription(const nsAString& aErrorDescription) = 0;

  /* void setDefaultIndex (in long aDefaultIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDefaultIndex(int32_t aDefaultIndex) = 0;

  /* void setSearchResult (in unsigned short aSearchResult); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchResult(uint16_t aSearchResult) = 0;

  /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InsertMatchAt(int32_t aIndex, const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) = 0;

  /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AppendMatch(const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) = 0;

  /* void removeMatchAt (in long aIndex); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveMatchAt(int32_t aIndex) = 0;

  /* nsIAutoCompleteSimpleResultListener getListener (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetListener(nsIAutoCompleteSimpleResultListener **_retval) = 0;

  /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetListener(nsIAutoCompleteSimpleResultListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteSimpleResult, NS_IAUTOCOMPLETESIMPLERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETESIMPLERESULT \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override; \
  NS_IMETHOD SetErrorDescription(const nsAString& aErrorDescription) override; \
  NS_IMETHOD SetDefaultIndex(int32_t aDefaultIndex) override; \
  NS_IMETHOD SetSearchResult(uint16_t aSearchResult) override; \
  NS_IMETHOD InsertMatchAt(int32_t aIndex, const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override; \
  NS_IMETHOD AppendMatch(const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override; \
  NS_IMETHOD RemoveMatchAt(int32_t aIndex) override; \
  NS_IMETHOD GetListener(nsIAutoCompleteSimpleResultListener **_retval) override; \
  NS_IMETHOD SetListener(nsIAutoCompleteSimpleResultListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETESIMPLERESULT \
  nsresult SetSearchString(const nsAString& aSearchString); \
  nsresult SetErrorDescription(const nsAString& aErrorDescription); \
  nsresult SetDefaultIndex(int32_t aDefaultIndex); \
  nsresult SetSearchResult(uint16_t aSearchResult); \
  nsresult InsertMatchAt(int32_t aIndex, const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel); \
  nsresult AppendMatch(const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel); \
  nsresult RemoveMatchAt(int32_t aIndex); \
  nsresult GetListener(nsIAutoCompleteSimpleResultListener **_retval); \
  nsresult SetListener(nsIAutoCompleteSimpleResultListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETESIMPLERESULT(_to) \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return _to SetSearchString(aSearchString); } \
  NS_IMETHOD SetErrorDescription(const nsAString& aErrorDescription) override { return _to SetErrorDescription(aErrorDescription); } \
  NS_IMETHOD SetDefaultIndex(int32_t aDefaultIndex) override { return _to SetDefaultIndex(aDefaultIndex); } \
  NS_IMETHOD SetSearchResult(uint16_t aSearchResult) override { return _to SetSearchResult(aSearchResult); } \
  NS_IMETHOD InsertMatchAt(int32_t aIndex, const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override { return _to InsertMatchAt(aIndex, aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel); } \
  NS_IMETHOD AppendMatch(const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override { return _to AppendMatch(aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel); } \
  NS_IMETHOD RemoveMatchAt(int32_t aIndex) override { return _to RemoveMatchAt(aIndex); } \
  NS_IMETHOD GetListener(nsIAutoCompleteSimpleResultListener **_retval) override { return _to GetListener(_retval); } \
  NS_IMETHOD SetListener(nsIAutoCompleteSimpleResultListener *aListener) override { return _to SetListener(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETESIMPLERESULT(_to) \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchString(aSearchString); } \
  NS_IMETHOD SetErrorDescription(const nsAString& aErrorDescription) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetErrorDescription(aErrorDescription); } \
  NS_IMETHOD SetDefaultIndex(int32_t aDefaultIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDefaultIndex(aDefaultIndex); } \
  NS_IMETHOD SetSearchResult(uint16_t aSearchResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchResult(aSearchResult); } \
  NS_IMETHOD InsertMatchAt(int32_t aIndex, const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InsertMatchAt(aIndex, aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel); } \
  NS_IMETHOD AppendMatch(const nsAString& aValue, const nsAString& aComment, const nsAString& aImage, const nsAString& aStyle, const nsAString& aFinalCompleteValue, const nsAString& aLabel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AppendMatch(aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel); } \
  NS_IMETHOD RemoveMatchAt(int32_t aIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveMatchAt(aIndex); } \
  NS_IMETHOD GetListener(nsIAutoCompleteSimpleResultListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetListener(_retval); } \
  NS_IMETHOD SetListener(nsIAutoCompleteSimpleResultListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetListener(aListener); } 


/* starting interface:    nsIAutoCompleteSimpleResultListener */
#define NS_IAUTOCOMPLETESIMPLERESULTLISTENER_IID_STR "004efdc5-1989-4874-8a7a-345bf2fa33af"

#define NS_IAUTOCOMPLETESIMPLERESULTLISTENER_IID \
  {0x004efdc5, 0x1989, 0x4874, \
    { 0x8a, 0x7a, 0x34, 0x5b, 0xf2, 0xfa, 0x33, 0xaf }}

class NS_NO_VTABLE nsIAutoCompleteSimpleResultListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETESIMPLERESULTLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteSimpleResultListener;

  /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnValueRemoved(nsIAutoCompleteSimpleResult *aResult, const nsAString& aValue) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteSimpleResultListener, NS_IAUTOCOMPLETESIMPLERESULTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETESIMPLERESULTLISTENER \
  NS_IMETHOD OnValueRemoved(nsIAutoCompleteSimpleResult *aResult, const nsAString& aValue) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETESIMPLERESULTLISTENER \
  nsresult OnValueRemoved(nsIAutoCompleteSimpleResult *aResult, const nsAString& aValue); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETESIMPLERESULTLISTENER(_to) \
  NS_IMETHOD OnValueRemoved(nsIAutoCompleteSimpleResult *aResult, const nsAString& aValue) override { return _to OnValueRemoved(aResult, aValue); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETESIMPLERESULTLISTENER(_to) \
  NS_IMETHOD OnValueRemoved(nsIAutoCompleteSimpleResult *aResult, const nsAString& aValue) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnValueRemoved(aResult, aValue); } 


#endif /* __gen_nsIAutoCompleteSimpleResult_h__ */
