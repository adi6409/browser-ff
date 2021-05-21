/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSimpleSearch.idl
 */

#ifndef __gen_nsIAutoCompleteSimpleSearch_h__
#define __gen_nsIAutoCompleteSimpleSearch_h__


#ifndef __gen_nsIAutoCompleteSearch_h__
#include "nsIAutoCompleteSearch.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAutoCompleteSimpleSearch */
#define NS_IAUTOCOMPLETESIMPLESEARCH_IID_STR "dc185a77-ba88-4caa-8f16-465253f7599a"

#define NS_IAUTOCOMPLETESIMPLESEARCH_IID \
  {0xdc185a77, 0xba88, 0x4caa, \
    { 0x8f, 0x16, 0x46, 0x52, 0x53, 0xf7, 0x59, 0x9a }}

class NS_NO_VTABLE nsIAutoCompleteSimpleSearch : public nsIAutoCompleteSearch {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IAUTOCOMPLETESIMPLESEARCH_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAutoCompleteSimpleSearch;

  /* void overrideNextResult (in nsIAutoCompleteResult values); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OverrideNextResult(nsIAutoCompleteResult *values) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAutoCompleteSimpleSearch, NS_IAUTOCOMPLETESIMPLESEARCH_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIAUTOCOMPLETESIMPLESEARCH \
  NS_IMETHOD OverrideNextResult(nsIAutoCompleteResult *values) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIAUTOCOMPLETESIMPLESEARCH \
  nsresult OverrideNextResult(nsIAutoCompleteResult *values); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIAUTOCOMPLETESIMPLESEARCH(_to) \
  NS_IMETHOD OverrideNextResult(nsIAutoCompleteResult *values) override { return _to OverrideNextResult(values); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIAUTOCOMPLETESIMPLESEARCH(_to) \
  NS_IMETHOD OverrideNextResult(nsIAutoCompleteResult *values) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OverrideNextResult(values); } 


#endif /* __gen_nsIAutoCompleteSimpleSearch_h__ */
