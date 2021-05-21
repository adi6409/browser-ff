/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIUUIDGenerator.idl
 */

#ifndef __gen_nsIUUIDGenerator_h__
#define __gen_nsIUUIDGenerator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUUIDGenerator */
#define NS_IUUIDGENERATOR_IID_STR "138ad1b2-c694-41cc-b201-333ce936d8b8"

#define NS_IUUIDGENERATOR_IID \
  {0x138ad1b2, 0xc694, 0x41cc, \
    { 0xb2, 0x01, 0x33, 0x3c, 0xe9, 0x36, 0xd8, 0xb8 }}

class NS_NO_VTABLE nsIUUIDGenerator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUUIDGENERATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUUIDGenerator;

  /* nsIDPtr generateUUID (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GenerateUUID(nsID * * _retval) = 0;

  /* [noscript] void generateUUIDInPlace (in nsNonConstIDPtr id); */
  NS_IMETHOD GenerateUUIDInPlace(nsID * id) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUUIDGenerator, NS_IUUIDGENERATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUUIDGENERATOR \
  NS_IMETHOD GenerateUUID(nsID * * _retval) override; \
  NS_IMETHOD GenerateUUIDInPlace(nsID * id) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUUIDGENERATOR \
  nsresult GenerateUUID(nsID * * _retval); \
  nsresult GenerateUUIDInPlace(nsID * id); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUUIDGENERATOR(_to) \
  NS_IMETHOD GenerateUUID(nsID * * _retval) override { return _to GenerateUUID(_retval); } \
  NS_IMETHOD GenerateUUIDInPlace(nsID * id) override { return _to GenerateUUIDInPlace(id); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUUIDGENERATOR(_to) \
  NS_IMETHOD GenerateUUID(nsID * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateUUID(_retval); } \
  NS_IMETHOD GenerateUUIDInPlace(nsID * id) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GenerateUUIDInPlace(id); } 


#endif /* __gen_nsIUUIDGenerator_h__ */
