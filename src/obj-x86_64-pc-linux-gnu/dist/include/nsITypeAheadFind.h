/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/typeaheadfind/nsITypeAheadFind.idl
 */

#ifndef __gen_nsITypeAheadFind_h__
#define __gen_nsITypeAheadFind_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIDOMWindow; /* forward declaration */

class nsIDocShell; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla

class nsRange; /* webidl Range */


/* starting interface:    nsITypeAheadFind */
#define NS_ITYPEAHEADFIND_IID_STR "ae501e28-c57f-4692-ac74-410e1bed98b7"

#define NS_ITYPEAHEADFIND_IID \
  {0xae501e28, 0xc57f, 0x4692, \
    { 0xac, 0x74, 0x41, 0x0e, 0x1b, 0xed, 0x98, 0xb7 }}

class NS_NO_VTABLE nsITypeAheadFind : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITYPEAHEADFIND_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITypeAheadFind;

  /* void init (in nsIDocShell aDocShell); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIDocShell *aDocShell) = 0;

  /* unsigned short find (in AString aSearchString, in boolean aLinksOnly, in unsigned long aMode, in boolean aDontIterateFrames); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Find(const nsAString& aSearchString, bool aLinksOnly, uint32_t aMode, bool aDontIterateFrames, uint16_t *_retval) = 0;

  /* Range getFoundRange (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFoundRange(nsRange **_retval) = 0;

  /* void setDocShell (in nsIDocShell aDocShell); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetDocShell(nsIDocShell *aDocShell) = 0;

  /* void setSelectionModeAndRepaint (in short toggle); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetSelectionModeAndRepaint(int16_t toggle) = 0;

  /* void collapseSelection (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CollapseSelection(void) = 0;

  /* boolean isRangeVisible (in Range aRange, in boolean aMustBeInViewPort); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsRangeVisible(nsRange *aRange, bool aMustBeInViewPort, bool *_retval) = 0;

  /* boolean isRangeRendered (in Range aRange); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsRangeRendered(nsRange *aRange, bool *_retval) = 0;

  /* readonly attribute AString searchString; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchString(nsAString& aSearchString) = 0;

  /* attribute boolean caseSensitive; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) = 0;

  /* attribute boolean matchDiacritics; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) = 0;

  /* attribute boolean entireWord; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEntireWord(bool *aEntireWord) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetEntireWord(bool aEntireWord) = 0;

  /* readonly attribute Element foundLink; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFoundLink(mozilla::dom::Element **aFoundLink) = 0;

  /* readonly attribute Element foundEditable; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFoundEditable(mozilla::dom::Element **aFoundEditable) = 0;

  /* readonly attribute mozIDOMWindow currentWindow; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetCurrentWindow(mozIDOMWindow **aCurrentWindow) = 0;

  enum {
    FIND_INITIAL = 0U,
    FIND_NEXT = 1U,
    FIND_PREVIOUS = 2U,
    FIND_FIRST = 3U,
    FIND_LAST = 4U,
    FIND_FOUND = 0U,
    FIND_NOTFOUND = 1U,
    FIND_WRAPPED = 2U,
    FIND_PENDING = 3U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITypeAheadFind, NS_ITYPEAHEADFIND_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITYPEAHEADFIND \
  NS_IMETHOD Init(nsIDocShell *aDocShell) override; \
  NS_IMETHOD Find(const nsAString& aSearchString, bool aLinksOnly, uint32_t aMode, bool aDontIterateFrames, uint16_t *_retval) override; \
  NS_IMETHOD GetFoundRange(nsRange **_retval) override; \
  NS_IMETHOD SetDocShell(nsIDocShell *aDocShell) override; \
  NS_IMETHOD SetSelectionModeAndRepaint(int16_t toggle) override; \
  NS_IMETHOD CollapseSelection(void) override; \
  NS_IMETHOD IsRangeVisible(nsRange *aRange, bool aMustBeInViewPort, bool *_retval) override; \
  NS_IMETHOD IsRangeRendered(nsRange *aRange, bool *_retval) override; \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override; \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override; \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override; \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override; \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override; \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override; \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override; \
  NS_IMETHOD GetFoundLink(mozilla::dom::Element **aFoundLink) override; \
  NS_IMETHOD GetFoundEditable(mozilla::dom::Element **aFoundEditable) override; \
  NS_IMETHOD GetCurrentWindow(mozIDOMWindow **aCurrentWindow) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITYPEAHEADFIND \
  nsresult Init(nsIDocShell *aDocShell); \
  nsresult Find(const nsAString& aSearchString, bool aLinksOnly, uint32_t aMode, bool aDontIterateFrames, uint16_t *_retval); \
  nsresult GetFoundRange(nsRange **_retval); \
  nsresult SetDocShell(nsIDocShell *aDocShell); \
  nsresult SetSelectionModeAndRepaint(int16_t toggle); \
  nsresult CollapseSelection(void); \
  nsresult IsRangeVisible(nsRange *aRange, bool aMustBeInViewPort, bool *_retval); \
  nsresult IsRangeRendered(nsRange *aRange, bool *_retval); \
  nsresult GetSearchString(nsAString& aSearchString); \
  nsresult GetCaseSensitive(bool *aCaseSensitive); \
  nsresult SetCaseSensitive(bool aCaseSensitive); \
  nsresult GetMatchDiacritics(bool *aMatchDiacritics); \
  nsresult SetMatchDiacritics(bool aMatchDiacritics); \
  nsresult GetEntireWord(bool *aEntireWord); \
  nsresult SetEntireWord(bool aEntireWord); \
  nsresult GetFoundLink(mozilla::dom::Element **aFoundLink); \
  nsresult GetFoundEditable(mozilla::dom::Element **aFoundEditable); \
  nsresult GetCurrentWindow(mozIDOMWindow **aCurrentWindow); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITYPEAHEADFIND(_to) \
  NS_IMETHOD Init(nsIDocShell *aDocShell) override { return _to Init(aDocShell); } \
  NS_IMETHOD Find(const nsAString& aSearchString, bool aLinksOnly, uint32_t aMode, bool aDontIterateFrames, uint16_t *_retval) override { return _to Find(aSearchString, aLinksOnly, aMode, aDontIterateFrames, _retval); } \
  NS_IMETHOD GetFoundRange(nsRange **_retval) override { return _to GetFoundRange(_retval); } \
  NS_IMETHOD SetDocShell(nsIDocShell *aDocShell) override { return _to SetDocShell(aDocShell); } \
  NS_IMETHOD SetSelectionModeAndRepaint(int16_t toggle) override { return _to SetSelectionModeAndRepaint(toggle); } \
  NS_IMETHOD CollapseSelection(void) override { return _to CollapseSelection(); } \
  NS_IMETHOD IsRangeVisible(nsRange *aRange, bool aMustBeInViewPort, bool *_retval) override { return _to IsRangeVisible(aRange, aMustBeInViewPort, _retval); } \
  NS_IMETHOD IsRangeRendered(nsRange *aRange, bool *_retval) override { return _to IsRangeRendered(aRange, _retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return _to GetSearchString(aSearchString); } \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override { return _to GetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override { return _to SetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return _to GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return _to SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return _to GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return _to SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetFoundLink(mozilla::dom::Element **aFoundLink) override { return _to GetFoundLink(aFoundLink); } \
  NS_IMETHOD GetFoundEditable(mozilla::dom::Element **aFoundEditable) override { return _to GetFoundEditable(aFoundEditable); } \
  NS_IMETHOD GetCurrentWindow(mozIDOMWindow **aCurrentWindow) override { return _to GetCurrentWindow(aCurrentWindow); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITYPEAHEADFIND(_to) \
  NS_IMETHOD Init(nsIDocShell *aDocShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aDocShell); } \
  NS_IMETHOD Find(const nsAString& aSearchString, bool aLinksOnly, uint32_t aMode, bool aDontIterateFrames, uint16_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Find(aSearchString, aLinksOnly, aMode, aDontIterateFrames, _retval); } \
  NS_IMETHOD GetFoundRange(nsRange **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFoundRange(_retval); } \
  NS_IMETHOD SetDocShell(nsIDocShell *aDocShell) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetDocShell(aDocShell); } \
  NS_IMETHOD SetSelectionModeAndRepaint(int16_t toggle) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetSelectionModeAndRepaint(toggle); } \
  NS_IMETHOD CollapseSelection(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CollapseSelection(); } \
  NS_IMETHOD IsRangeVisible(nsRange *aRange, bool aMustBeInViewPort, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRangeVisible(aRange, aMustBeInViewPort, _retval); } \
  NS_IMETHOD IsRangeRendered(nsRange *aRange, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsRangeRendered(aRange, _retval); } \
  NS_IMETHOD GetSearchString(nsAString& aSearchString) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchString(aSearchString); } \
  NS_IMETHOD GetCaseSensitive(bool *aCaseSensitive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD SetCaseSensitive(bool aCaseSensitive) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCaseSensitive(aCaseSensitive); } \
  NS_IMETHOD GetMatchDiacritics(bool *aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD SetMatchDiacritics(bool aMatchDiacritics) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetMatchDiacritics(aMatchDiacritics); } \
  NS_IMETHOD GetEntireWord(bool *aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEntireWord(aEntireWord); } \
  NS_IMETHOD SetEntireWord(bool aEntireWord) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetEntireWord(aEntireWord); } \
  NS_IMETHOD GetFoundLink(mozilla::dom::Element **aFoundLink) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFoundLink(aFoundLink); } \
  NS_IMETHOD GetFoundEditable(mozilla::dom::Element **aFoundEditable) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFoundEditable(aFoundEditable); } \
  NS_IMETHOD GetCurrentWindow(mozIDOMWindow **aCurrentWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetCurrentWindow(aCurrentWindow); } \


#endif /* __gen_nsITypeAheadFind_h__ */
