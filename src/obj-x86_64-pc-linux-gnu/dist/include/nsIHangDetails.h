/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/backgroundhangmonitor/nsIHangDetails.idl
 */

#ifndef __gen_nsIHangDetails_h__
#define __gen_nsIHangDetails_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
class HangDetails;
}

/* starting interface:    nsIHangDetails */
#define NS_IHANGDETAILS_IID_STR "23d63fff-38d6-4003-9c57-2c90aca1180a"

#define NS_IHANGDETAILS_IID \
  {0x23d63fff, 0x38d6, 0x4003, \
    { 0x9c, 0x57, 0x2c, 0x90, 0xac, 0xa1, 0x18, 0x0a }}

class NS_NO_VTABLE nsIHangDetails : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IHANGDETAILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIHangDetails;

  /* readonly attribute bool wasPersisted; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetWasPersisted(bool *aWasPersisted) = 0;

  /* readonly attribute double duration; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetDuration(double *aDuration) = 0;

  /* readonly attribute ACString thread; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetThread(nsACString& aThread) = 0;

  /* readonly attribute ACString runnableName; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRunnableName(nsACString& aRunnableName) = 0;

  /* readonly attribute ACString process; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetProcess(nsACString& aProcess) = 0;

  /* readonly attribute AUTF8String remoteType; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteType(nsACString& aRemoteType) = 0;

  /* [implicit_jscontext] readonly attribute jsval stack; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetStack(JSContext* cx, JS::MutableHandleValue aStack) = 0;

  /* [implicit_jscontext] readonly attribute jsval modules; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetModules(JSContext* cx, JS::MutableHandleValue aModules) = 0;

  /* [implicit_jscontext] readonly attribute jsval annotations; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetAnnotations(JSContext* cx, JS::MutableHandleValue aAnnotations) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIHangDetails, NS_IHANGDETAILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIHANGDETAILS \
  NS_IMETHOD GetWasPersisted(bool *aWasPersisted) override; \
  NS_IMETHOD GetDuration(double *aDuration) override; \
  NS_IMETHOD GetThread(nsACString& aThread) override; \
  NS_IMETHOD GetRunnableName(nsACString& aRunnableName) override; \
  NS_IMETHOD GetProcess(nsACString& aProcess) override; \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override; \
  NS_IMETHOD GetStack(JSContext* cx, JS::MutableHandleValue aStack) override; \
  NS_IMETHOD GetModules(JSContext* cx, JS::MutableHandleValue aModules) override; \
  NS_IMETHOD GetAnnotations(JSContext* cx, JS::MutableHandleValue aAnnotations) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIHANGDETAILS \
  nsresult GetWasPersisted(bool *aWasPersisted); \
  nsresult GetDuration(double *aDuration); \
  nsresult GetThread(nsACString& aThread); \
  nsresult GetRunnableName(nsACString& aRunnableName); \
  nsresult GetProcess(nsACString& aProcess); \
  nsresult GetRemoteType(nsACString& aRemoteType); \
  nsresult GetStack(JSContext* cx, JS::MutableHandleValue aStack); \
  nsresult GetModules(JSContext* cx, JS::MutableHandleValue aModules); \
  nsresult GetAnnotations(JSContext* cx, JS::MutableHandleValue aAnnotations); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIHANGDETAILS(_to) \
  NS_IMETHOD GetWasPersisted(bool *aWasPersisted) override { return _to GetWasPersisted(aWasPersisted); } \
  NS_IMETHOD GetDuration(double *aDuration) override { return _to GetDuration(aDuration); } \
  NS_IMETHOD GetThread(nsACString& aThread) override { return _to GetThread(aThread); } \
  NS_IMETHOD GetRunnableName(nsACString& aRunnableName) override { return _to GetRunnableName(aRunnableName); } \
  NS_IMETHOD GetProcess(nsACString& aProcess) override { return _to GetProcess(aProcess); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return _to GetRemoteType(aRemoteType); } \
  NS_IMETHOD GetStack(JSContext* cx, JS::MutableHandleValue aStack) override { return _to GetStack(cx, aStack); } \
  NS_IMETHOD GetModules(JSContext* cx, JS::MutableHandleValue aModules) override { return _to GetModules(cx, aModules); } \
  NS_IMETHOD GetAnnotations(JSContext* cx, JS::MutableHandleValue aAnnotations) override { return _to GetAnnotations(cx, aAnnotations); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIHANGDETAILS(_to) \
  NS_IMETHOD GetWasPersisted(bool *aWasPersisted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetWasPersisted(aWasPersisted); } \
  NS_IMETHOD GetDuration(double *aDuration) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDuration(aDuration); } \
  NS_IMETHOD GetThread(nsACString& aThread) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetThread(aThread); } \
  NS_IMETHOD GetRunnableName(nsACString& aRunnableName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRunnableName(aRunnableName); } \
  NS_IMETHOD GetProcess(nsACString& aProcess) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetProcess(aProcess); } \
  NS_IMETHOD GetRemoteType(nsACString& aRemoteType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteType(aRemoteType); } \
  NS_IMETHOD GetStack(JSContext* cx, JS::MutableHandleValue aStack) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStack(cx, aStack); } \
  NS_IMETHOD GetModules(JSContext* cx, JS::MutableHandleValue aModules) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModules(cx, aModules); } \
  NS_IMETHOD GetAnnotations(JSContext* cx, JS::MutableHandleValue aAnnotations) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetAnnotations(cx, aAnnotations); } 


#endif /* __gen_nsIHangDetails_h__ */
