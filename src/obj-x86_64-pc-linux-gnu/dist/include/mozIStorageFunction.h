/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageFunction.idl
 */

#ifndef __gen_mozIStorageFunction_h__
#define __gen_mozIStorageFunction_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_mozIStorageValueArray_h__
#include "mozIStorageValueArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageConnection; /* forward declaration */

class nsIArray; /* forward declaration */

class nsIVariant; /* forward declaration */


/* starting interface:    mozIStorageFunction */
#define MOZISTORAGEFUNCTION_IID_STR "9ff02465-21cb-49f3-b975-7d5b38ceec73"

#define MOZISTORAGEFUNCTION_IID \
  {0x9ff02465, 0x21cb, 0x49f3, \
    { 0xb9, 0x75, 0x7d, 0x5b, 0x38, 0xce, 0xec, 0x73 }}

class NS_NO_VTABLE mozIStorageFunction : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEFUNCTION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageFunction;

  /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnFunctionCall(mozIStorageValueArray *aFunctionArguments, nsIVariant **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageFunction, MOZISTORAGEFUNCTION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEFUNCTION \
  NS_IMETHOD OnFunctionCall(mozIStorageValueArray *aFunctionArguments, nsIVariant **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEFUNCTION \
  nsresult OnFunctionCall(mozIStorageValueArray *aFunctionArguments, nsIVariant **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEFUNCTION(_to) \
  NS_IMETHOD OnFunctionCall(mozIStorageValueArray *aFunctionArguments, nsIVariant **_retval) override { return _to OnFunctionCall(aFunctionArguments, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEFUNCTION(_to) \
  NS_IMETHOD OnFunctionCall(mozIStorageValueArray *aFunctionArguments, nsIVariant **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnFunctionCall(aFunctionArguments, _retval); } 


#endif /* __gen_mozIStorageFunction_h__ */
