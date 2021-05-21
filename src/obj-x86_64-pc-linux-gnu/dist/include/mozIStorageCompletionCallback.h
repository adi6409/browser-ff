/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageCompletionCallback.idl
 */

#ifndef __gen_mozIStorageCompletionCallback_h__
#define __gen_mozIStorageCompletionCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    mozIStorageCompletionCallback */
#define MOZISTORAGECOMPLETIONCALLBACK_IID_STR "8cbf2dc2-91e0-44bc-984f-553638412071"

#define MOZISTORAGECOMPLETIONCALLBACK_IID \
  {0x8cbf2dc2, 0x91e0, 0x44bc, \
    { 0x98, 0x4f, 0x55, 0x36, 0x38, 0x41, 0x20, 0x71 }}

class NS_NO_VTABLE mozIStorageCompletionCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(MOZISTORAGECOMPLETIONCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = mozIStorageCompletionCallback;

  /* void complete (in nsresult status, [optional] in nsISupports value); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Complete(nsresult status, nsISupports *value) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(mozIStorageCompletionCallback, MOZISTORAGECOMPLETIONCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_MOZISTORAGECOMPLETIONCALLBACK \
  NS_IMETHOD Complete(nsresult status, nsISupports *value) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_MOZISTORAGECOMPLETIONCALLBACK \
  nsresult Complete(nsresult status, nsISupports *value); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_MOZISTORAGECOMPLETIONCALLBACK(_to) \
  NS_IMETHOD Complete(nsresult status, nsISupports *value) override { return _to Complete(status, value); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_MOZISTORAGECOMPLETIONCALLBACK(_to) \
  NS_IMETHOD Complete(nsresult status, nsISupports *value) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Complete(status, value); } 


#endif /* __gen_mozIStorageCompletionCallback_h__ */
