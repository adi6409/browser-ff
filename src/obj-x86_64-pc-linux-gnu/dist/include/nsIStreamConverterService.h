/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIStreamConverterService.idl
 */

#ifndef __gen_nsIStreamConverterService_h__
#define __gen_nsIStreamConverterService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIInputStream; /* forward declaration */

class nsIStreamListener; /* forward declaration */

#define NS_ISTREAMCONVERTER_KEY         "@mozilla.org/streamconv;1"

/* starting interface:    nsIStreamConverterService */
#define NS_ISTREAMCONVERTERSERVICE_IID_STR "f2b1ab53-f0bd-4adb-9365-e59b1701a258"

#define NS_ISTREAMCONVERTERSERVICE_IID \
  {0xf2b1ab53, 0xf0bd, 0x4adb, \
    { 0x93, 0x65, 0xe5, 0x9b, 0x17, 0x01, 0xa2, 0x58 }}

class NS_NO_VTABLE nsIStreamConverterService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMCONVERTERSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamConverterService;

  /* boolean canConvert (in string aFromType, in string aToType); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CanConvert(const char * aFromType, const char * aToType, bool *_retval) = 0;

  /* ACString convertedType (in ACString aFromType, in nsIChannel aChannel); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertedType(const nsACString& aFromType, nsIChannel *aChannel, nsACString& _retval) = 0;

  /* nsIInputStream convert (in nsIInputStream aFromStream, in string aFromType, in string aToType, in nsISupports aContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Convert(nsIInputStream *aFromStream, const char * aFromType, const char * aToType, nsISupports *aContext, nsIInputStream **_retval) = 0;

  /* nsIStreamListener asyncConvertData (in string aFromType, in string aToType, in nsIStreamListener aListener, in nsISupports aContext); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncConvertData(const char * aFromType, const char * aToType, nsIStreamListener *aListener, nsISupports *aContext, nsIStreamListener **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamConverterService, NS_ISTREAMCONVERTERSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMCONVERTERSERVICE \
  NS_IMETHOD CanConvert(const char * aFromType, const char * aToType, bool *_retval) override; \
  NS_IMETHOD ConvertedType(const nsACString& aFromType, nsIChannel *aChannel, nsACString& _retval) override; \
  NS_IMETHOD Convert(nsIInputStream *aFromStream, const char * aFromType, const char * aToType, nsISupports *aContext, nsIInputStream **_retval) override; \
  NS_IMETHOD AsyncConvertData(const char * aFromType, const char * aToType, nsIStreamListener *aListener, nsISupports *aContext, nsIStreamListener **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMCONVERTERSERVICE \
  nsresult CanConvert(const char * aFromType, const char * aToType, bool *_retval); \
  nsresult ConvertedType(const nsACString& aFromType, nsIChannel *aChannel, nsACString& _retval); \
  nsresult Convert(nsIInputStream *aFromStream, const char * aFromType, const char * aToType, nsISupports *aContext, nsIInputStream **_retval); \
  nsresult AsyncConvertData(const char * aFromType, const char * aToType, nsIStreamListener *aListener, nsISupports *aContext, nsIStreamListener **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMCONVERTERSERVICE(_to) \
  NS_IMETHOD CanConvert(const char * aFromType, const char * aToType, bool *_retval) override { return _to CanConvert(aFromType, aToType, _retval); } \
  NS_IMETHOD ConvertedType(const nsACString& aFromType, nsIChannel *aChannel, nsACString& _retval) override { return _to ConvertedType(aFromType, aChannel, _retval); } \
  NS_IMETHOD Convert(nsIInputStream *aFromStream, const char * aFromType, const char * aToType, nsISupports *aContext, nsIInputStream **_retval) override { return _to Convert(aFromStream, aFromType, aToType, aContext, _retval); } \
  NS_IMETHOD AsyncConvertData(const char * aFromType, const char * aToType, nsIStreamListener *aListener, nsISupports *aContext, nsIStreamListener **_retval) override { return _to AsyncConvertData(aFromType, aToType, aListener, aContext, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMCONVERTERSERVICE(_to) \
  NS_IMETHOD CanConvert(const char * aFromType, const char * aToType, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CanConvert(aFromType, aToType, _retval); } \
  NS_IMETHOD ConvertedType(const nsACString& aFromType, nsIChannel *aChannel, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertedType(aFromType, aChannel, _retval); } \
  NS_IMETHOD Convert(nsIInputStream *aFromStream, const char * aFromType, const char * aToType, nsISupports *aContext, nsIInputStream **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Convert(aFromStream, aFromType, aToType, aContext, _retval); } \
  NS_IMETHOD AsyncConvertData(const char * aFromType, const char * aToType, nsIStreamListener *aListener, nsISupports *aContext, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncConvertData(aFromType, aToType, aListener, aContext, _retval); } 


#endif /* __gen_nsIStreamConverterService_h__ */
