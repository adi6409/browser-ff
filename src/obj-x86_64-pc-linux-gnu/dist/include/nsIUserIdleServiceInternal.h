/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIUserIdleServiceInternal.idl
 */

#ifndef __gen_nsIUserIdleServiceInternal_h__
#define __gen_nsIUserIdleServiceInternal_h__


#ifndef __gen_nsIUserIdleService_h__
#include "nsIUserIdleService.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIUserIdleServiceInternal */
#define NS_IUSERIDLESERVICEINTERNAL_IID_STR "7b89a2e7-ed12-42e0-b86d-4984239abd7b"

#define NS_IUSERIDLESERVICEINTERNAL_IID \
  {0x7b89a2e7, 0xed12, 0x42e0, \
    { 0xb8, 0x6d, 0x49, 0x84, 0x23, 0x9a, 0xbd, 0x7b }}

class NS_NO_VTABLE nsIUserIdleServiceInternal : public nsIUserIdleService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IUSERIDLESERVICEINTERNAL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIUserIdleServiceInternal;

  /* void resetIdleTimeOut (in unsigned long idleDeltaInMS); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ResetIdleTimeOut(uint32_t idleDeltaInMS) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIUserIdleServiceInternal, NS_IUSERIDLESERVICEINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIUSERIDLESERVICEINTERNAL \
  NS_IMETHOD ResetIdleTimeOut(uint32_t idleDeltaInMS) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIUSERIDLESERVICEINTERNAL \
  nsresult ResetIdleTimeOut(uint32_t idleDeltaInMS); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIUSERIDLESERVICEINTERNAL(_to) \
  NS_IMETHOD ResetIdleTimeOut(uint32_t idleDeltaInMS) override { return _to ResetIdleTimeOut(idleDeltaInMS); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIUSERIDLESERVICEINTERNAL(_to) \
  NS_IMETHOD ResetIdleTimeOut(uint32_t idleDeltaInMS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResetIdleTimeOut(idleDeltaInMS); } 


#endif /* __gen_nsIUserIdleServiceInternal_h__ */
