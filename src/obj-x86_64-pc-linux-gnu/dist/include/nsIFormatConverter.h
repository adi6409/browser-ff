/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIFormatConverter.idl
 */

#ifndef __gen_nsIFormatConverter_h__
#define __gen_nsIFormatConverter_h__


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

/* starting interface:    nsIFormatConverter */
#define NS_IFORMATCONVERTER_IID_STR "948a0023-e3a7-11d2-96cf-0060b0fb9956"

#define NS_IFORMATCONVERTER_IID \
  {0x948a0023, 0xe3a7, 0x11d2, \
    { 0x96, 0xcf, 0x00, 0x60, 0xb0, 0xfb, 0x99, 0x56 }}

class NS_NO_VTABLE nsIFormatConverter : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMATCONVERTER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormatConverter;

  /* Array<ACString> getInputDataFlavors (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInputDataFlavors(nsTArray<nsCString >& _retval) = 0;

  /* Array<ACString> getOutputDataFlavors (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetOutputDataFlavors(nsTArray<nsCString >& _retval) = 0;

  /* boolean canConvert (in string aFromDataFlavor, in string aToDataFlavor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanConvert(const char * aFromDataFlavor, const char * aToDataFlavor, bool *_retval) = 0;

  /* void convert (in string aFromDataFlavor, in nsISupports aFromData, in string aToDataFlavor, out nsISupports aToData); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Convert(const char * aFromDataFlavor, nsISupports *aFromData, const char * aToDataFlavor, nsISupports **aToData) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormatConverter, NS_IFORMATCONVERTER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMATCONVERTER \
  NS_IMETHOD GetInputDataFlavors(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD GetOutputDataFlavors(nsTArray<nsCString >& _retval) override; \
  NS_IMETHOD CanConvert(const char * aFromDataFlavor, const char * aToDataFlavor, bool *_retval) override; \
  NS_IMETHOD Convert(const char * aFromDataFlavor, nsISupports *aFromData, const char * aToDataFlavor, nsISupports **aToData) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMATCONVERTER \
  nsresult GetInputDataFlavors(nsTArray<nsCString >& _retval); \
  nsresult GetOutputDataFlavors(nsTArray<nsCString >& _retval); \
  nsresult CanConvert(const char * aFromDataFlavor, const char * aToDataFlavor, bool *_retval); \
  nsresult Convert(const char * aFromDataFlavor, nsISupports *aFromData, const char * aToDataFlavor, nsISupports **aToData); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMATCONVERTER(_to) \
  NS_IMETHOD GetInputDataFlavors(nsTArray<nsCString >& _retval) override { return _to GetInputDataFlavors(_retval); } \
  NS_IMETHOD GetOutputDataFlavors(nsTArray<nsCString >& _retval) override { return _to GetOutputDataFlavors(_retval); } \
  NS_IMETHOD CanConvert(const char * aFromDataFlavor, const char * aToDataFlavor, bool *_retval) override { return _to CanConvert(aFromDataFlavor, aToDataFlavor, _retval); } \
  NS_IMETHOD Convert(const char * aFromDataFlavor, nsISupports *aFromData, const char * aToDataFlavor, nsISupports **aToData) override { return _to Convert(aFromDataFlavor, aFromData, aToDataFlavor, aToData); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMATCONVERTER(_to) \
  NS_IMETHOD GetInputDataFlavors(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInputDataFlavors(_retval); } \
  NS_IMETHOD GetOutputDataFlavors(nsTArray<nsCString >& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOutputDataFlavors(_retval); } \
  NS_IMETHOD CanConvert(const char * aFromDataFlavor, const char * aToDataFlavor, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanConvert(aFromDataFlavor, aToDataFlavor, _retval); } \
  NS_IMETHOD Convert(const char * aFromDataFlavor, nsISupports *aFromData, const char * aToDataFlavor, nsISupports **aToData) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Convert(aFromDataFlavor, aFromData, aToDataFlavor, aToData); } 



#endif /* __gen_nsIFormatConverter_h__ */
