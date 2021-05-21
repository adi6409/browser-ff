/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/satchel/nsIFormAutoComplete.idl
 */

#ifndef __gen_nsIFormAutoComplete_h__
#define __gen_nsIFormAutoComplete_h__


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

class nsIPropertyBag2; /* forward declaration */

namespace mozilla {
namespace dom {
class HTMLInputElement; /* webidl HTMLInputElement */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIFormAutoComplete */
#define NS_IFORMAUTOCOMPLETE_IID_STR "bfd9b82b-0ab3-4b6b-9e54-aa961ff4b732"

#define NS_IFORMAUTOCOMPLETE_IID \
  {0xbfd9b82b, 0x0ab3, 0x4b6b, \
    { 0x9e, 0x54, 0xaa, 0x96, 0x1f, 0xf4, 0xb7, 0x32 }}

class NS_NO_VTABLE nsIFormAutoComplete : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMAUTOCOMPLETE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormAutoComplete;

  /* void autoCompleteSearchAsync (in AString aInputName, in AString aSearchString, in HTMLInputElement aField, in nsIAutoCompleteResult aPreviousResult, in nsIAutoCompleteResult aDatalistResult, in nsIFormAutoCompleteObserver aListener, [optional] in nsIPropertyBag2 options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AutoCompleteSearchAsync(const nsAString& aInputName, const nsAString& aSearchString, mozilla::dom::HTMLInputElement *aField, nsIAutoCompleteResult *aPreviousResult, nsIAutoCompleteResult *aDatalistResult, nsIFormAutoCompleteObserver *aListener, nsIPropertyBag2 *options) = 0;

  /* void stopAutoCompleteSearch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopAutoCompleteSearch(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormAutoComplete, NS_IFORMAUTOCOMPLETE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMAUTOCOMPLETE \
  NS_IMETHOD AutoCompleteSearchAsync(const nsAString& aInputName, const nsAString& aSearchString, mozilla::dom::HTMLInputElement *aField, nsIAutoCompleteResult *aPreviousResult, nsIAutoCompleteResult *aDatalistResult, nsIFormAutoCompleteObserver *aListener, nsIPropertyBag2 *options) override; \
  NS_IMETHOD StopAutoCompleteSearch(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMAUTOCOMPLETE \
  nsresult AutoCompleteSearchAsync(const nsAString& aInputName, const nsAString& aSearchString, mozilla::dom::HTMLInputElement *aField, nsIAutoCompleteResult *aPreviousResult, nsIAutoCompleteResult *aDatalistResult, nsIFormAutoCompleteObserver *aListener, nsIPropertyBag2 *options); \
  nsresult StopAutoCompleteSearch(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMAUTOCOMPLETE(_to) \
  NS_IMETHOD AutoCompleteSearchAsync(const nsAString& aInputName, const nsAString& aSearchString, mozilla::dom::HTMLInputElement *aField, nsIAutoCompleteResult *aPreviousResult, nsIAutoCompleteResult *aDatalistResult, nsIFormAutoCompleteObserver *aListener, nsIPropertyBag2 *options) override { return _to AutoCompleteSearchAsync(aInputName, aSearchString, aField, aPreviousResult, aDatalistResult, aListener, options); } \
  NS_IMETHOD StopAutoCompleteSearch(void) override { return _to StopAutoCompleteSearch(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMAUTOCOMPLETE(_to) \
  NS_IMETHOD AutoCompleteSearchAsync(const nsAString& aInputName, const nsAString& aSearchString, mozilla::dom::HTMLInputElement *aField, nsIAutoCompleteResult *aPreviousResult, nsIAutoCompleteResult *aDatalistResult, nsIFormAutoCompleteObserver *aListener, nsIPropertyBag2 *options) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AutoCompleteSearchAsync(aInputName, aSearchString, aField, aPreviousResult, aDatalistResult, aListener, options); } \
  NS_IMETHOD StopAutoCompleteSearch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopAutoCompleteSearch(); } 


/* starting interface:    nsIFormAutoCompleteObserver */
#define NS_IFORMAUTOCOMPLETEOBSERVER_IID_STR "604419ab-55a0-4831-9eca-1b9e67cc4751"

#define NS_IFORMAUTOCOMPLETEOBSERVER_IID \
  {0x604419ab, 0x55a0, 0x4831, \
    { 0x9e, 0xca, 0x1b, 0x9e, 0x67, 0xcc, 0x47, 0x51 }}

class NS_NO_VTABLE nsIFormAutoCompleteObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMAUTOCOMPLETEOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormAutoCompleteObserver;

  /* [can_run_script] void onSearchCompletion (in nsIAutoCompleteResult result); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchCompletion(nsIAutoCompleteResult *result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormAutoCompleteObserver, NS_IFORMAUTOCOMPLETEOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMAUTOCOMPLETEOBSERVER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchCompletion(nsIAutoCompleteResult *result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMAUTOCOMPLETEOBSERVER \
  MOZ_CAN_RUN_SCRIPT nsresult OnSearchCompletion(nsIAutoCompleteResult *result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMAUTOCOMPLETEOBSERVER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchCompletion(nsIAutoCompleteResult *result) override { return _to OnSearchCompletion(result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMAUTOCOMPLETEOBSERVER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchCompletion(nsIAutoCompleteResult *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSearchCompletion(result); } 


#endif /* __gen_nsIFormAutoComplete_h__ */
