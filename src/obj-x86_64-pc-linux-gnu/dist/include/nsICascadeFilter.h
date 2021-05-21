/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/cascade_bloom_filter/nsICascadeFilter.idl
 */

#ifndef __gen_nsICascadeFilter_h__
#define __gen_nsICascadeFilter_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICascadeFilter */
#define NS_ICASCADEFILTER_IID_STR "c8d0b0b3-17f8-458b-9264-7b67b288fe79"

#define NS_ICASCADEFILTER_IID \
  {0xc8d0b0b3, 0x17f8, 0x458b, \
    { 0x92, 0x64, 0x7b, 0x67, 0xb2, 0x88, 0xfe, 0x79 }}

class NS_NO_VTABLE nsICascadeFilter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICASCADEFILTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICascadeFilter;

  /* void setFilterData (in Array<octet> data); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFilterData(const nsTArray<uint8_t >& data) = 0;

  /* boolean has (in ACString key); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Has(const nsACString& key, bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICascadeFilter, NS_ICASCADEFILTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICASCADEFILTER \
  NS_IMETHOD SetFilterData(const nsTArray<uint8_t >& data) override; \
  NS_IMETHOD Has(const nsACString& key, bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICASCADEFILTER \
  nsresult SetFilterData(const nsTArray<uint8_t >& data); \
  nsresult Has(const nsACString& key, bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICASCADEFILTER(_to) \
  NS_IMETHOD SetFilterData(const nsTArray<uint8_t >& data) override { return _to SetFilterData(data); } \
  NS_IMETHOD Has(const nsACString& key, bool *_retval) override { return _to Has(key, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICASCADEFILTER(_to) \
  NS_IMETHOD SetFilterData(const nsTArray<uint8_t >& data) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFilterData(data); } \
  NS_IMETHOD Has(const nsACString& key, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Has(key, _retval); } 


#endif /* __gen_nsICascadeFilter_h__ */
