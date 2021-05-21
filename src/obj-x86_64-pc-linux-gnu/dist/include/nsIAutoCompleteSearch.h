/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSearch.idl
 */

#ifndef __gen_nsIAutoCompleteSearch_h__
#define __gen_nsIAutoCompleteSearch_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAutoCompleteResult; /* forward declaration */

class nsIAutoCompleteObserver; /* forward declaration */

class nsIPropertyBag2; /* forward declaration */


/* starting interface:    nsIAutoCompleteSearch */
#define NS_IAUTOCOMPLETESEARCH_IID_STR "de8db85f-c1de-4d87-94ba-7844890f91fe"

#define NS_IAUTOCOMPLETESEARCH_IID \
  {0xde8db85f, 0xc1de, 0x4d87, \
    { 0x94, 0xba, 0x78, 0x44, 0x89, 0x0f, 0x91, 0xfe }}

class NS_NO_VTABLE nsIAutoCompleteSearch : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETESEARCH_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteSearch;

  /* void startSearch (in AString searchString, in AString searchParam, in nsIAutoCompleteResult previousResult, in nsIAutoCompleteObserver listener, [optional] in nsIPropertyBag2 options); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartSearch(const nsAString& searchString, const nsAString& searchParam, nsIAutoCompleteResult *previousResult, nsIAutoCompleteObserver *listener, nsIPropertyBag2 *options) = 0;

  /* void stopSearch (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StopSearch(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteSearch, NS_IAUTOCOMPLETESEARCH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETESEARCH \
  NS_IMETHOD StartSearch(const nsAString& searchString, const nsAString& searchParam, nsIAutoCompleteResult *previousResult, nsIAutoCompleteObserver *listener, nsIPropertyBag2 *options) override; \
  NS_IMETHOD StopSearch(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETESEARCH \
  nsresult StartSearch(const nsAString& searchString, const nsAString& searchParam, nsIAutoCompleteResult *previousResult, nsIAutoCompleteObserver *listener, nsIPropertyBag2 *options); \
  nsresult StopSearch(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETESEARCH(_to) \
  NS_IMETHOD StartSearch(const nsAString& searchString, const nsAString& searchParam, nsIAutoCompleteResult *previousResult, nsIAutoCompleteObserver *listener, nsIPropertyBag2 *options) override { return _to StartSearch(searchString, searchParam, previousResult, listener, options); } \
  NS_IMETHOD StopSearch(void) override { return _to StopSearch(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETESEARCH(_to) \
  NS_IMETHOD StartSearch(const nsAString& searchString, const nsAString& searchParam, nsIAutoCompleteResult *previousResult, nsIAutoCompleteObserver *listener, nsIPropertyBag2 *options) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartSearch(searchString, searchParam, previousResult, listener, options); } \
  NS_IMETHOD StopSearch(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StopSearch(); } 


/* starting interface:    nsIAutoCompleteObserver */
#define NS_IAUTOCOMPLETEOBSERVER_IID_STR "8bd1dbbc-dcce-4007-9afa-b551eb687b61"

#define NS_IAUTOCOMPLETEOBSERVER_IID \
  {0x8bd1dbbc, 0xdcce, 0x4007, \
    { 0x9a, 0xfa, 0xb5, 0x51, 0xeb, 0x68, 0x7b, 0x61 }}

class NS_NO_VTABLE nsIAutoCompleteObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETEOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteObserver;

  /* [can_run_script] void onSearchResult (in nsIAutoCompleteSearch search, in nsIAutoCompleteResult result); */
  JS_HAZ_CAN_RUN_SCRIPT MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchResult(nsIAutoCompleteSearch *search, nsIAutoCompleteResult *result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteObserver, NS_IAUTOCOMPLETEOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETEOBSERVER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchResult(nsIAutoCompleteSearch *search, nsIAutoCompleteResult *result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETEOBSERVER \
  MOZ_CAN_RUN_SCRIPT nsresult OnSearchResult(nsIAutoCompleteSearch *search, nsIAutoCompleteResult *result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETEOBSERVER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchResult(nsIAutoCompleteSearch *search, nsIAutoCompleteResult *result) override { return _to OnSearchResult(search, result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETEOBSERVER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD OnSearchResult(nsIAutoCompleteSearch *search, nsIAutoCompleteResult *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnSearchResult(search, result); } 


/* starting interface:    nsIAutoCompleteSearchDescriptor */
#define NS_IAUTOCOMPLETESEARCHDESCRIPTOR_IID_STR "4c3e7462-fbfb-4310-8f4b-239238392b75"

#define NS_IAUTOCOMPLETESEARCHDESCRIPTOR_IID \
  {0x4c3e7462, 0xfbfb, 0x4310, \
    { 0x8f, 0x4b, 0x23, 0x92, 0x38, 0x39, 0x2b, 0x75 }}

class NS_NO_VTABLE nsIAutoCompleteSearchDescriptor : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETESEARCHDESCRIPTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteSearchDescriptor;

  enum {
    SEARCH_TYPE_DELAYED = 0U,
    SEARCH_TYPE_IMMEDIATE = 1U
  };

  /* readonly attribute unsigned short searchType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetSearchType(uint16_t *aSearchType) = 0;

  /* readonly attribute boolean clearingAutoFillSearchesAgain; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClearingAutoFillSearchesAgain(bool *aClearingAutoFillSearchesAgain) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteSearchDescriptor, NS_IAUTOCOMPLETESEARCHDESCRIPTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETESEARCHDESCRIPTOR \
  NS_IMETHOD GetSearchType(uint16_t *aSearchType) override; \
  NS_IMETHOD GetClearingAutoFillSearchesAgain(bool *aClearingAutoFillSearchesAgain) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETESEARCHDESCRIPTOR \
  nsresult GetSearchType(uint16_t *aSearchType); \
  nsresult GetClearingAutoFillSearchesAgain(bool *aClearingAutoFillSearchesAgain); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETESEARCHDESCRIPTOR(_to) \
  NS_IMETHOD GetSearchType(uint16_t *aSearchType) override { return _to GetSearchType(aSearchType); } \
  NS_IMETHOD GetClearingAutoFillSearchesAgain(bool *aClearingAutoFillSearchesAgain) override { return _to GetClearingAutoFillSearchesAgain(aClearingAutoFillSearchesAgain); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETESEARCHDESCRIPTOR(_to) \
  NS_IMETHOD GetSearchType(uint16_t *aSearchType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSearchType(aSearchType); } \
  NS_IMETHOD GetClearingAutoFillSearchesAgain(bool *aClearingAutoFillSearchesAgain) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClearingAutoFillSearchesAgain(aClearingAutoFillSearchesAgain); } 


#endif /* __gen_nsIAutoCompleteSearch_h__ */
