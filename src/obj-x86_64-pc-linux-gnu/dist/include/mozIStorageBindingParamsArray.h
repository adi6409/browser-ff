/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBindingParamsArray.idl
 */

#ifndef __gen_mozIStorageBindingParamsArray_h__
#define __gen_mozIStorageBindingParamsArray_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class mozIStorageBindingParams; /* forward declaration */


/* starting interface:    mozIStorageBindingParamsArray */
#define MOZISTORAGEBINDINGPARAMSARRAY_IID_STR "67eea5c3-4881-41ff-b0fe-09f2356aeadb"

#define MOZISTORAGEBINDINGPARAMSARRAY_IID \
  {0x67eea5c3, 0x4881, 0x41ff, \
    { 0xb0, 0xfe, 0x09, 0xf2, 0x35, 0x6a, 0xea, 0xdb }}

class NS_NO_VTABLE mozIStorageBindingParamsArray : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGEBINDINGPARAMSARRAY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageBindingParamsArray;

  /* mozIStorageBindingParams newBindingParams (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NewBindingParams(mozIStorageBindingParams **_retval) = 0;

  /* void addParams (in mozIStorageBindingParams aParameters); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddParams(mozIStorageBindingParams *aParameters) = 0;

  /* readonly attribute unsigned long length; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLength(uint32_t *aLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageBindingParamsArray, MOZISTORAGEBINDINGPARAMSARRAY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGEBINDINGPARAMSARRAY \
  NS_IMETHOD NewBindingParams(mozIStorageBindingParams **_retval) override; \
  NS_IMETHOD AddParams(mozIStorageBindingParams *aParameters) override; \
  NS_IMETHOD GetLength(uint32_t *aLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGEBINDINGPARAMSARRAY \
  nsresult NewBindingParams(mozIStorageBindingParams **_retval); \
  nsresult AddParams(mozIStorageBindingParams *aParameters); \
  nsresult GetLength(uint32_t *aLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGEBINDINGPARAMSARRAY(_to) \
  NS_IMETHOD NewBindingParams(mozIStorageBindingParams **_retval) override { return _to NewBindingParams(_retval); } \
  NS_IMETHOD AddParams(mozIStorageBindingParams *aParameters) override { return _to AddParams(aParameters); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return _to GetLength(aLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGEBINDINGPARAMSARRAY(_to) \
  NS_IMETHOD NewBindingParams(mozIStorageBindingParams **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NewBindingParams(_retval); } \
  NS_IMETHOD AddParams(mozIStorageBindingParams *aParameters) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddParams(aParameters); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } 


#endif /* __gen_mozIStorageBindingParamsArray_h__ */
