/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginAutoCompleteSearch.idl
 */

#ifndef __gen_nsILoginAutoCompleteSearch_h__
#define __gen_nsILoginAutoCompleteSearch_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompleteResult; /* forward declaration */

class nsIFormAutoCompleteObserver; /* forward declaration */

namespace mozilla {
namespace dom {
class HTMLInputElement; /* webidl HTMLInputElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsILoginAutoCompleteSearch */
#define NS_ILOGINAUTOCOMPLETESEARCH_IID_STR "2bdac17c-53f1-4896-a521-682ccdeef3a8"

#define NS_ILOGINAUTOCOMPLETESEARCH_IID \
  {0x2bdac17c, 0x53f1, 0x4896, \
    { 0xa5, 0x21, 0x68, 0x2c, 0xcd, 0xee, 0xf3, 0xa8 }}

class NS_NO_VTABLE nsILoginAutoCompleteSearch : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOGINAUTOCOMPLETESEARCH_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoginAutoCompleteSearch;

  /* void startSearch (in AString aSearchString, in nsIAutoCompleteResult aPreviousResult, in HTMLInputElement aElement, in nsIFormAutoCompleteObserver aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& aSearchString, nsIAutoCompleteResult *aPreviousResult, mozilla::dom::HTMLInputElement *aElement, nsIFormAutoCompleteObserver *aListener) = 0;

  /* void stopSearch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoginAutoCompleteSearch, NS_ILOGINAUTOCOMPLETESEARCH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOGINAUTOCOMPLETESEARCH \
  NS_IMETHOD StartSearch(const nsAString& aSearchString, nsIAutoCompleteResult *aPreviousResult, mozilla::dom::HTMLInputElement *aElement, nsIFormAutoCompleteObserver *aListener) override; \
  NS_IMETHOD StopSearch(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOGINAUTOCOMPLETESEARCH \
  nsresult StartSearch(const nsAString& aSearchString, nsIAutoCompleteResult *aPreviousResult, mozilla::dom::HTMLInputElement *aElement, nsIFormAutoCompleteObserver *aListener); \
  nsresult StopSearch(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOGINAUTOCOMPLETESEARCH(_to) \
  NS_IMETHOD StartSearch(const nsAString& aSearchString, nsIAutoCompleteResult *aPreviousResult, mozilla::dom::HTMLInputElement *aElement, nsIFormAutoCompleteObserver *aListener) override { return _to StartSearch(aSearchString, aPreviousResult, aElement, aListener); } \
  NS_IMETHOD StopSearch(void) override { return _to StopSearch(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOGINAUTOCOMPLETESEARCH(_to) \
  NS_IMETHOD StartSearch(const nsAString& aSearchString, nsIAutoCompleteResult *aPreviousResult, mozilla::dom::HTMLInputElement *aElement, nsIFormAutoCompleteObserver *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartSearch(aSearchString, aPreviousResult, aElement, aListener); } \
  NS_IMETHOD StopSearch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopSearch(); } 


#endif /* __gen_nsILoginAutoCompleteSearch_h__ */
