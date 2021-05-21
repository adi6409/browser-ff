/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIIDNService.idl
 */

#ifndef __gen_nsIIDNService_h__
#define __gen_nsIIDNService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIIDNService */
#define NS_IIDNSERVICE_IID_STR "a592a60e-3621-4f19-a318-2bf233cfad3e"

#define NS_IIDNSERVICE_IID \
  {0xa592a60e, 0x3621, 0x4f19, \
    { 0xa3, 0x18, 0x2b, 0xf2, 0x33, 0xcf, 0xad, 0x3e }}

class NS_NO_VTABLE nsIIDNService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IIDNSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIIDNService;

  /* ACString convertUTF8toACE (in AUTF8String input); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertUTF8toACE(const nsACString& input, nsACString& _retval) = 0;

  /* AUTF8String convertACEtoUTF8 (in ACString input); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertACEtoUTF8(const nsACString& input, nsACString& _retval) = 0;

  /* boolean isACE (in ACString input); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsACE(const nsACString& input, bool *_retval) = 0;

  /* AUTF8String normalize (in AUTF8String input); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Normalize(const nsACString& input, nsACString& _retval) = 0;

  /* AUTF8String convertToDisplayIDN (in AUTF8String input, out boolean isASCII); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConvertToDisplayIDN(const nsACString& input, bool *isASCII, nsACString& _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIIDNService, NS_IIDNSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIIDNSERVICE \
  NS_IMETHOD ConvertUTF8toACE(const nsACString& input, nsACString& _retval) override; \
  NS_IMETHOD ConvertACEtoUTF8(const nsACString& input, nsACString& _retval) override; \
  NS_IMETHOD IsACE(const nsACString& input, bool *_retval) override; \
  NS_IMETHOD Normalize(const nsACString& input, nsACString& _retval) override; \
  NS_IMETHOD ConvertToDisplayIDN(const nsACString& input, bool *isASCII, nsACString& _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIIDNSERVICE \
  nsresult ConvertUTF8toACE(const nsACString& input, nsACString& _retval); \
  nsresult ConvertACEtoUTF8(const nsACString& input, nsACString& _retval); \
  nsresult IsACE(const nsACString& input, bool *_retval); \
  nsresult Normalize(const nsACString& input, nsACString& _retval); \
  nsresult ConvertToDisplayIDN(const nsACString& input, bool *isASCII, nsACString& _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIIDNSERVICE(_to) \
  NS_IMETHOD ConvertUTF8toACE(const nsACString& input, nsACString& _retval) override { return _to ConvertUTF8toACE(input, _retval); } \
  NS_IMETHOD ConvertACEtoUTF8(const nsACString& input, nsACString& _retval) override { return _to ConvertACEtoUTF8(input, _retval); } \
  NS_IMETHOD IsACE(const nsACString& input, bool *_retval) override { return _to IsACE(input, _retval); } \
  NS_IMETHOD Normalize(const nsACString& input, nsACString& _retval) override { return _to Normalize(input, _retval); } \
  NS_IMETHOD ConvertToDisplayIDN(const nsACString& input, bool *isASCII, nsACString& _retval) override { return _to ConvertToDisplayIDN(input, isASCII, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIIDNSERVICE(_to) \
  NS_IMETHOD ConvertUTF8toACE(const nsACString& input, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertUTF8toACE(input, _retval); } \
  NS_IMETHOD ConvertACEtoUTF8(const nsACString& input, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertACEtoUTF8(input, _retval); } \
  NS_IMETHOD IsACE(const nsACString& input, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsACE(input, _retval); } \
  NS_IMETHOD Normalize(const nsACString& input, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Normalize(input, _retval); } \
  NS_IMETHOD ConvertToDisplayIDN(const nsACString& input, bool *isASCII, nsACString& _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConvertToDisplayIDN(input, isASCII, _retval); } 


#endif /* __gen_nsIIDNService_h__ */
