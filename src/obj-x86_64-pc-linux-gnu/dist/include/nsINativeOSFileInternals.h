/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/osfile/nsINativeOSFileInternals.idl
 */

#ifndef __gen_nsINativeOSFileInternals_h__
#define __gen_nsINativeOSFileInternals_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINativeOSFileResult */
#define NS_INATIVEOSFILERESULT_IID_STR "08b4cf29-3d65-4e79-b522-a694c322ed07"

#define NS_INATIVEOSFILERESULT_IID \
  {0x08b4cf29, 0x3d65, 0x4e79, \
    { 0xb5, 0x22, 0xa6, 0x94, 0xc3, 0x22, 0xed, 0x07 }}

class NS_NO_VTABLE nsINativeOSFileResult : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEOSFILERESULT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeOSFileResult;

  /* [implicit_jscontext] readonly attribute jsval result; */
  NS_IMETHOD GetResult(JSContext* cx, JS::MutableHandleValue aResult) = 0;

  /* readonly attribute double dispatchDurationMS; */
  NS_IMETHOD GetDispatchDurationMS(double *aDispatchDurationMS) = 0;

  /* readonly attribute double executionDurationMS; */
  NS_IMETHOD GetExecutionDurationMS(double *aExecutionDurationMS) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeOSFileResult, NS_INATIVEOSFILERESULT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEOSFILERESULT \
  NS_IMETHOD GetResult(JSContext* cx, JS::MutableHandleValue aResult) override; \
  NS_IMETHOD GetDispatchDurationMS(double *aDispatchDurationMS) override; \
  NS_IMETHOD GetExecutionDurationMS(double *aExecutionDurationMS) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEOSFILERESULT \
  nsresult GetResult(JSContext* cx, JS::MutableHandleValue aResult); \
  nsresult GetDispatchDurationMS(double *aDispatchDurationMS); \
  nsresult GetExecutionDurationMS(double *aExecutionDurationMS); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEOSFILERESULT(_to) \
  NS_IMETHOD GetResult(JSContext* cx, JS::MutableHandleValue aResult) override { return _to GetResult(cx, aResult); } \
  NS_IMETHOD GetDispatchDurationMS(double *aDispatchDurationMS) override { return _to GetDispatchDurationMS(aDispatchDurationMS); } \
  NS_IMETHOD GetExecutionDurationMS(double *aExecutionDurationMS) override { return _to GetExecutionDurationMS(aExecutionDurationMS); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEOSFILERESULT(_to) \
  NS_IMETHOD GetResult(JSContext* cx, JS::MutableHandleValue aResult) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetResult(cx, aResult); } \
  NS_IMETHOD GetDispatchDurationMS(double *aDispatchDurationMS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDispatchDurationMS(aDispatchDurationMS); } \
  NS_IMETHOD GetExecutionDurationMS(double *aExecutionDurationMS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetExecutionDurationMS(aExecutionDurationMS); } 


/* starting interface:    nsINativeOSFileSuccessCallback */
#define NS_INATIVEOSFILESUCCESSCALLBACK_IID_STR "2c1922ca-ca1b-4099-8b61-ec23cff49412"

#define NS_INATIVEOSFILESUCCESSCALLBACK_IID \
  {0x2c1922ca, 0xca1b, 0x4099, \
    { 0x8b, 0x61, 0xec, 0x23, 0xcf, 0xf4, 0x94, 0x12 }}

class NS_NO_VTABLE nsINativeOSFileSuccessCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEOSFILESUCCESSCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeOSFileSuccessCallback;

  /* void complete (in nsINativeOSFileResult result); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(nsINativeOSFileResult *result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeOSFileSuccessCallback, NS_INATIVEOSFILESUCCESSCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEOSFILESUCCESSCALLBACK \
  NS_IMETHOD Complete(nsINativeOSFileResult *result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEOSFILESUCCESSCALLBACK \
  nsresult Complete(nsINativeOSFileResult *result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEOSFILESUCCESSCALLBACK(_to) \
  NS_IMETHOD Complete(nsINativeOSFileResult *result) override { return _to Complete(result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEOSFILESUCCESSCALLBACK(_to) \
  NS_IMETHOD Complete(nsINativeOSFileResult *result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(result); } 


/* starting interface:    nsINativeOSFileErrorCallback */
#define NS_INATIVEOSFILEERRORCALLBACK_IID_STR "f612e0fc-6736-4d24-aa50-fd661b3b40b6"

#define NS_INATIVEOSFILEERRORCALLBACK_IID \
  {0xf612e0fc, 0x6736, 0x4d24, \
    { 0xaa, 0x50, 0xfd, 0x66, 0x1b, 0x3b, 0x40, 0xb6 }}

class NS_NO_VTABLE nsINativeOSFileErrorCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEOSFILEERRORCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeOSFileErrorCallback;

  /* void complete (in ACString operation, in long OSstatus); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(const nsACString& operation, int32_t OSstatus) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeOSFileErrorCallback, NS_INATIVEOSFILEERRORCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEOSFILEERRORCALLBACK \
  NS_IMETHOD Complete(const nsACString& operation, int32_t OSstatus) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEOSFILEERRORCALLBACK \
  nsresult Complete(const nsACString& operation, int32_t OSstatus); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEOSFILEERRORCALLBACK(_to) \
  NS_IMETHOD Complete(const nsACString& operation, int32_t OSstatus) override { return _to Complete(operation, OSstatus); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEOSFILEERRORCALLBACK(_to) \
  NS_IMETHOD Complete(const nsACString& operation, int32_t OSstatus) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(operation, OSstatus); } 


/* starting interface:    nsINativeOSFileInternalsService */
#define NS_INATIVEOSFILEINTERNALSSERVICE_IID_STR "913362ad-1526-4623-9e6b-a2eb08afbbb9"

#define NS_INATIVEOSFILEINTERNALSSERVICE_IID \
  {0x913362ad, 0x1526, 0x4623, \
    { 0x9e, 0x6b, 0xa2, 0xeb, 0x08, 0xaf, 0xbb, 0xb9 }}

class NS_NO_VTABLE nsINativeOSFileInternalsService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEOSFILEINTERNALSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeOSFileInternalsService;

  /* [implicit_jscontext] void read (in AString path, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */
  NS_IMETHOD Read(const nsAString& path, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) = 0;

  /* [implicit_jscontext] void writeAtomic (in AString path, in jsval buffer, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */
  NS_IMETHOD WriteAtomic(const nsAString& path, JS::HandleValue buffer, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeOSFileInternalsService, NS_INATIVEOSFILEINTERNALSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEOSFILEINTERNALSSERVICE \
  NS_IMETHOD Read(const nsAString& path, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override; \
  NS_IMETHOD WriteAtomic(const nsAString& path, JS::HandleValue buffer, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEOSFILEINTERNALSSERVICE \
  nsresult Read(const nsAString& path, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx); \
  nsresult WriteAtomic(const nsAString& path, JS::HandleValue buffer, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEOSFILEINTERNALSSERVICE(_to) \
  NS_IMETHOD Read(const nsAString& path, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override { return _to Read(path, options, onSuccess, onError, cx); } \
  NS_IMETHOD WriteAtomic(const nsAString& path, JS::HandleValue buffer, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override { return _to WriteAtomic(path, buffer, options, onSuccess, onError, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEOSFILEINTERNALSSERVICE(_to) \
  NS_IMETHOD Read(const nsAString& path, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Read(path, options, onSuccess, onError, cx); } \
  NS_IMETHOD WriteAtomic(const nsAString& path, JS::HandleValue buffer, JS::HandleValue options, nsINativeOSFileSuccessCallback *onSuccess, nsINativeOSFileErrorCallback *onError, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->WriteAtomic(path, buffer, options, onSuccess, onError, cx); } 


#define NATIVE_OSFILE_INTERNALS_SERVICE_CID {0x63A69303,0x8A64,0x45A9,{0x84, 0x8C, 0xD4, 0xE2, 0x79, 0x27, 0x94, 0xE6}}
#define NATIVE_OSFILE_INTERNALS_SERVICE_CONTRACTID "@mozilla.org/toolkit/osfile/native-internals;1"

#endif /* __gen_nsINativeOSFileInternals_h__ */
