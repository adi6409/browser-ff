/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/find/nsIWebBrowserFind.idl
 */

#ifndef __gen_nsIWebBrowserFind_h__
#define __gen_nsIWebBrowserFind_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindowProxy; /* forward declaration */


/* starting interface:    nsIWebBrowserFind */
#define NS_IWEBBROWSERFIND_IID_STR "e4920136-b3e0-49e0-b1cd-6c783d2591a8"

#define NS_IWEBBROWSERFIND_IID \
  {0xe4920136, 0xb3e0, 0x49e0, \
    { 0xb1, 0xcd, 0x6c, 0x78, 0x3d, 0x25, 0x91, 0xa8 }}

class NS_NO_VTABLE nsIWebBrowserFind : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERFIND_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserFind;

  /* boolean findNext (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FindNext(bool *_retval) = 0;

  /* attribute AString searchString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchString(nsAString& aSearchString) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchString(const nsAString& aSearchString) = 0;

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

  /* attribute boolean searchFrames; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchFrames(bool *aSearchFrames) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchFrames(bool aSearchFrames) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserFind, NS_IWEBBROWSERFIND_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERFIND \
  NS_IMETHOD FindNext(bool *_retval) override; \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override; \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override; \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override; \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override; \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override; \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override; \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override; \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override; \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override; \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override; \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override; \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override; \
  NS_IMETHOD GetSearchFrames(bool *aSearchFrames) override; \
  NS_IMETHOD SetSearchFrames(bool aSearchFrames) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERFIND \
  nsresult FindNext(bool *_retval); \
  nsresult GetSearchString(nsAString& aSearchString); \
  nsresult SetSearchString(const nsAString& aSearchString); \
  nsresult GetFindBackwards(bool *aFindBackwards); \
  nsresult SetFindBackwards(bool aFindBackwards); \
  nsresult GetWrapFind(bool *aWrapFind); \
  nsresult SetWrapFind(bool aWrapFind); \
  nsresult GetEntireWord(bool *aEntireWord); \
  nsresult SetEntireWord(bool aEntireWord); \
  nsresult GetMatchCase(bool *aMatchCase); \
  nsresult SetMatchCase(bool aMatchCase); \
  nsresult GetMatchDiacritics(bool *aMatchDiacritics); \
  nsresult SetMatchDiacritics(bool aMatchDiacritics); \
  nsresult GetSearchFrames(bool *aSearchFrames); \
  nsresult SetSearchFrames(bool aSearchFrames); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERFIND(_to) \
  NS_IMETHOD FindNext(bool *_retval) override { return _to FindNext(_retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return _to GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return _to SetSearchString(aSearchString); } \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return _to GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return _to SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override { return _to GetWrapFind(aWrapFind); } \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override { return _to SetWrapFind(aWrapFind); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return _to GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return _to SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override { return _to GetMatchCase(aMatchCase); } \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override { return _to SetMatchCase(aMatchCase); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return _to GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return _to SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD GetSearchFrames(bool *aSearchFrames) override { return _to GetSearchFrames(aSearchFrames); } \
  NS_IMETHOD SetSearchFrames(bool aSearchFrames) override { return _to SetSearchFrames(aSearchFrames); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERFIND(_to) \
  NS_IMETHOD FindNext(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FindNext(_retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchString(aSearchString); } \
  NS_IMETHOD SetSearchString(const nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchString(aSearchString); } \
  NS_IMETHOD GetFindBackwards(bool *aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFindBackwards(aFindBackwards); } \
  NS_IMETHOD SetFindBackwards(bool aFindBackwards) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFindBackwards(aFindBackwards); } \
  NS_IMETHOD GetWrapFind(bool *aWrapFind) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWrapFind(aWrapFind); } \
  NS_IMETHOD SetWrapFind(bool aWrapFind) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetWrapFind(aWrapFind); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetMatchCase(bool *aMatchCase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchCase(aMatchCase); } \
  NS_IMETHOD SetMatchCase(bool aMatchCase) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchCase(aMatchCase); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD GetSearchFrames(bool *aSearchFrames) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchFrames(aSearchFrames); } \
  NS_IMETHOD SetSearchFrames(bool aSearchFrames) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchFrames(aSearchFrames); } 


/* starting interface:    nsIWebBrowserFindInFrames */
#define NS_IWEBBROWSERFINDINFRAMES_IID_STR "e0f5d182-34bc-11d5-be5b-b760676c6ebc"

#define NS_IWEBBROWSERFINDINFRAMES_IID \
  {0xe0f5d182, 0x34bc, 0x11d5, \
    { 0xbe, 0x5b, 0xb7, 0x60, 0x67, 0x6c, 0x6e, 0xbc }}

class NS_NO_VTABLE nsIWebBrowserFindInFrames : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBBROWSERFINDINFRAMES_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebBrowserFindInFrames;

  /* attribute mozIDOMWindowProxy currentSearchFrame; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentSearchFrame(mozIDOMWindowProxy **aCurrentSearchFrame) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCurrentSearchFrame(mozIDOMWindowProxy *aCurrentSearchFrame) = 0;

  /* attribute mozIDOMWindowProxy rootSearchFrame; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRootSearchFrame(mozIDOMWindowProxy **aRootSearchFrame) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRootSearchFrame(mozIDOMWindowProxy *aRootSearchFrame) = 0;

  /* attribute boolean searchSubframes; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchSubframes(bool *aSearchSubframes) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchSubframes(bool aSearchSubframes) = 0;

  /* attribute boolean searchParentFrames; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchParentFrames(bool *aSearchParentFrames) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSearchParentFrames(bool aSearchParentFrames) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebBrowserFindInFrames, NS_IWEBBROWSERFINDINFRAMES_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBBROWSERFINDINFRAMES \
  NS_IMETHOD GetCurrentSearchFrame(mozIDOMWindowProxy **aCurrentSearchFrame) override; \
  NS_IMETHOD SetCurrentSearchFrame(mozIDOMWindowProxy *aCurrentSearchFrame) override; \
  NS_IMETHOD GetRootSearchFrame(mozIDOMWindowProxy **aRootSearchFrame) override; \
  NS_IMETHOD SetRootSearchFrame(mozIDOMWindowProxy *aRootSearchFrame) override; \
  NS_IMETHOD GetSearchSubframes(bool *aSearchSubframes) override; \
  NS_IMETHOD SetSearchSubframes(bool aSearchSubframes) override; \
  NS_IMETHOD GetSearchParentFrames(bool *aSearchParentFrames) override; \
  NS_IMETHOD SetSearchParentFrames(bool aSearchParentFrames) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBBROWSERFINDINFRAMES \
  nsresult GetCurrentSearchFrame(mozIDOMWindowProxy **aCurrentSearchFrame); \
  nsresult SetCurrentSearchFrame(mozIDOMWindowProxy *aCurrentSearchFrame); \
  nsresult GetRootSearchFrame(mozIDOMWindowProxy **aRootSearchFrame); \
  nsresult SetRootSearchFrame(mozIDOMWindowProxy *aRootSearchFrame); \
  nsresult GetSearchSubframes(bool *aSearchSubframes); \
  nsresult SetSearchSubframes(bool aSearchSubframes); \
  nsresult GetSearchParentFrames(bool *aSearchParentFrames); \
  nsresult SetSearchParentFrames(bool aSearchParentFrames); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBBROWSERFINDINFRAMES(_to) \
  NS_IMETHOD GetCurrentSearchFrame(mozIDOMWindowProxy **aCurrentSearchFrame) override { return _to GetCurrentSearchFrame(aCurrentSearchFrame); } \
  NS_IMETHOD SetCurrentSearchFrame(mozIDOMWindowProxy *aCurrentSearchFrame) override { return _to SetCurrentSearchFrame(aCurrentSearchFrame); } \
  NS_IMETHOD GetRootSearchFrame(mozIDOMWindowProxy **aRootSearchFrame) override { return _to GetRootSearchFrame(aRootSearchFrame); } \
  NS_IMETHOD SetRootSearchFrame(mozIDOMWindowProxy *aRootSearchFrame) override { return _to SetRootSearchFrame(aRootSearchFrame); } \
  NS_IMETHOD GetSearchSubframes(bool *aSearchSubframes) override { return _to GetSearchSubframes(aSearchSubframes); } \
  NS_IMETHOD SetSearchSubframes(bool aSearchSubframes) override { return _to SetSearchSubframes(aSearchSubframes); } \
  NS_IMETHOD GetSearchParentFrames(bool *aSearchParentFrames) override { return _to GetSearchParentFrames(aSearchParentFrames); } \
  NS_IMETHOD SetSearchParentFrames(bool aSearchParentFrames) override { return _to SetSearchParentFrames(aSearchParentFrames); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBBROWSERFINDINFRAMES(_to) \
  NS_IMETHOD GetCurrentSearchFrame(mozIDOMWindowProxy **aCurrentSearchFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentSearchFrame(aCurrentSearchFrame); } \
  NS_IMETHOD SetCurrentSearchFrame(mozIDOMWindowProxy *aCurrentSearchFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCurrentSearchFrame(aCurrentSearchFrame); } \
  NS_IMETHOD GetRootSearchFrame(mozIDOMWindowProxy **aRootSearchFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootSearchFrame(aRootSearchFrame); } \
  NS_IMETHOD SetRootSearchFrame(mozIDOMWindowProxy *aRootSearchFrame) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRootSearchFrame(aRootSearchFrame); } \
  NS_IMETHOD GetSearchSubframes(bool *aSearchSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchSubframes(aSearchSubframes); } \
  NS_IMETHOD SetSearchSubframes(bool aSearchSubframes) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchSubframes(aSearchSubframes); } \
  NS_IMETHOD GetSearchParentFrames(bool *aSearchParentFrames) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchParentFrames(aSearchParentFrames); } \
  NS_IMETHOD SetSearchParentFrames(bool aSearchParentFrames) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSearchParentFrames(aSearchParentFrames); } 


#endif /* __gen_nsIWebBrowserFind_h__ */
