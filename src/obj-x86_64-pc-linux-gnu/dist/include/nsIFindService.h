/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIFindService.idl
 */

#ifndef __gen_nsIFindService_h__
#define __gen_nsIFindService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFindService */
#define NS_IFINDSERVICE_IID_STR "5060b801-340e-11d5-be5b-b3e063ec6a3c"

#define NS_IFINDSERVICE_IID \
  {0x5060b801, 0x340e, 0x11d5, \
    { 0xbe, 0x5b, 0xb3, 0xe0, 0x63, 0xec, 0x6a, 0x3c }}

class NS_NO_VTABLE nsIFindService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFINDSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFindService;

  /* attribute AString searchString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchString(nsAString& aSearchString) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchString(const nsAString& aSearchString) = 0;

  /* attribute AString replaceString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetReplaceString(nsAString& aReplaceString) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetReplaceString(const nsAString& aReplaceString) = 0;

  /* attribute boolean findBackwards; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFindBackwards(bool *aFindBackwards) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFindBackwards(bool aFindBackwards) = 0;

  /* attribute boolean wrapFind; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWrapFind(bool *aWrapFind) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetWrapFind(bool aWrapFind) = 0;

  /* attribute boolean entireWord; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntireWord(bool *aEntireWord) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEntireWord(bool aEntireWord) = 0;

  /* attribute boolean matchCase; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchCase(bool *aMatchCase) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMatchCase(bool aMatchCase) = 0;

  /* attribute boolean matchDiacritics; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFindService, NS_IFINDSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFINDSERVICE \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override; \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override; \
  NS_IMETHOD GetReplaceString(nsAString& aReplaceString) override; \
  NS_IMETHOD SetReplaceString(const nsAString& aReplaceString) override; \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override; \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override; \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override; \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override; \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override; \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override; \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override; \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override; \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override; \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFINDSERVICE \
  nsresult GetSearchString(nsAString& aSearchString); \
  nsresult SetSearchString(const nsAString& aSearchString); \
  nsresult GetReplaceString(nsAString& aReplaceString); \
  nsresult SetReplaceString(const nsAString& aReplaceString); \
  nsresult GetFindBackwards(bool *aFindBackwards); \
  nsresult SetFindBackwards(bool aFindBackwards); \
  nsresult GetWrapFind(bool *aWrapFind); \
  nsresult SetWrapFind(bool aWrapFind); \
  nsresult GetEntireWord(bool *aEntireWord); \
  nsresult SetEntireWord(bool aEntireWord); \
  nsresult GetMatchCase(bool *aMatchCase); \
  nsresult SetMatchCase(bool aMatchCase); \
  nsresult GetMatchDiacritics(bool *aMatchDiacritics); \
  nsresult SetMatchDiacritics(bool aMatchDiacritics); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFINDSERVICE(_to) \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return _to GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return _to SetSearchString(aSearchString); } \
  NS_IMETHOD GetReplaceString(nsAString& aReplaceString) override { return _to GetReplaceString(aReplaceString); } \
  NS_IMETHOD SetReplaceString(const nsAString& aReplaceString) override { return _to SetReplaceString(aReplaceString); } \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return _to GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return _to SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override { return _to GetWrapFind(aWrapFind); } \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override { return _to SetWrapFind(aWrapFind); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return _to GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return _to SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override { return _to GetMatchCase(aMatchCase); } \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override { return _to SetMatchCase(aMatchCase); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return _to GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return _to SetMatchDiacritics(aMatchDiacritics); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFINDSERVICE(_to) \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchString(aSearchString); } \
  NS_IMETHOD GetReplaceString(nsAString& aReplaceString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReplaceString(aReplaceString); } \
  NS_IMETHOD SetReplaceString(const nsAString& aReplaceString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetReplaceString(aReplaceString); } \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWrapFind(aWrapFind); } \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWrapFind(aWrapFind); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchCase(aMatchCase); } \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchCase(aMatchCase); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchDiacritics(aMatchDiacritics); } 


#endif /* __gen_nsIFindService_h__ */
