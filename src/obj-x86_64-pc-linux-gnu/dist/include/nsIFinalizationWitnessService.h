/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/finalizationwitness/nsIFinalizationWitnessService.idl
 */

#ifndef __gen_nsIFinalizationWitnessService_h__
#define __gen_nsIFinalizationWitnessService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFinalizationWitnessService */
#define NS_IFINALIZATIONWITNESSSERVICE_IID_STR "15686f9d-483e-4361-98cd-37f1e8f1e61d"

#define NS_IFINALIZATIONWITNESSSERVICE_IID \
  {0x15686f9d, 0x483e, 0x4361, \
    { 0x98, 0xcd, 0x37, 0xf1, 0xe8, 0xf1, 0xe6, 0x1d }}

class NS_NO_VTABLE nsIFinalizationWitnessService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFINALIZATIONWITNESSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFinalizationWitnessService;

  /* [implicit_jscontext] jsval make (in string aTopic, in wstring aString); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Make(const char * aTopic, const char16_t * aString, JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFinalizationWitnessService, NS_IFINALIZATIONWITNESSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFINALIZATIONWITNESSSERVICE \
  NS_IMETHOD Make(const char * aTopic, const char16_t * aString, JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFINALIZATIONWITNESSSERVICE \
  nsresult Make(const char * aTopic, const char16_t * aString, JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFINALIZATIONWITNESSSERVICE(_to) \
  NS_IMETHOD Make(const char * aTopic, const char16_t * aString, JSContext* cx, JS::MutableHandleValue _retval) override { return _to Make(aTopic, aString, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFINALIZATIONWITNESSSERVICE(_to) \
  NS_IMETHOD Make(const char * aTopic, const char16_t * aString, JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Make(aTopic, aString, cx, _retval); } 


#define FINALIZATIONWITNESSSERVICE_CID {0x15686f9d,0x483e,0x4361,{0x98,0xcd,0x37,0xf1,0xe8,0xf1,0xe6,0x1d}}
#define FINALIZATIONWITNESSSERVICE_CONTRACTID "@mozilla.org/toolkit/finalizationwitness;1"

#endif /* __gen_nsIFinalizationWitnessService_h__ */
