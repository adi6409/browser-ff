/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIByteRangeRequest.idl
 */

#ifndef __gen_nsIByteRangeRequest_h__
#define __gen_nsIByteRangeRequest_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIByteRangeRequest */
#define NS_IBYTERANGEREQUEST_IID_STR "c1b1f426-7e83-4759-9f88-0e1b17f49366"

#define NS_IBYTERANGEREQUEST_IID \
  {0xc1b1f426, 0x7e83, 0x4759, \
    { 0x9f, 0x88, 0x0e, 0x1b, 0x17, 0xf4, 0x93, 0x66 }}

class NS_NO_VTABLE nsIByteRangeRequest : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBYTERANGEREQUEST_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIByteRangeRequest;

  /* readonly attribute boolean isByteRangeRequest; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsByteRangeRequest(bool *aIsByteRangeRequest) = 0;

  /* readonly attribute long long startRange; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStartRange(int64_t *aStartRange) = 0;

  /* readonly attribute long long endRange; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEndRange(int64_t *aEndRange) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIByteRangeRequest, NS_IBYTERANGEREQUEST_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBYTERANGEREQUEST \
  NS_IMETHOD GetIsByteRangeRequest(bool *aIsByteRangeRequest) override; \
  NS_IMETHOD GetStartRange(int64_t *aStartRange) override; \
  NS_IMETHOD GetEndRange(int64_t *aEndRange) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBYTERANGEREQUEST \
  nsresult GetIsByteRangeRequest(bool *aIsByteRangeRequest); \
  nsresult GetStartRange(int64_t *aStartRange); \
  nsresult GetEndRange(int64_t *aEndRange); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBYTERANGEREQUEST(_to) \
  NS_IMETHOD GetIsByteRangeRequest(bool *aIsByteRangeRequest) override { return _to GetIsByteRangeRequest(aIsByteRangeRequest); } \
  NS_IMETHOD GetStartRange(int64_t *aStartRange) override { return _to GetStartRange(aStartRange); } \
  NS_IMETHOD GetEndRange(int64_t *aEndRange) override { return _to GetEndRange(aEndRange); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBYTERANGEREQUEST(_to) \
  NS_IMETHOD GetIsByteRangeRequest(bool *aIsByteRangeRequest) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsByteRangeRequest(aIsByteRangeRequest); } \
  NS_IMETHOD GetStartRange(int64_t *aStartRange) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStartRange(aStartRange); } \
  NS_IMETHOD GetEndRange(int64_t *aEndRange) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEndRange(aEndRange); } 


#endif /* __gen_nsIByteRangeRequest_h__ */
