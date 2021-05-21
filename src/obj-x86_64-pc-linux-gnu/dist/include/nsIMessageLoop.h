/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMessageLoop.idl
 */

#ifndef __gen_nsIMessageLoop_h__
#define __gen_nsIMessageLoop_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRunnable; /* forward declaration */


/* starting interface:    nsIMessageLoop */
#define NS_IMESSAGELOOP_IID_STR "3e8c58e8-e52c-43e0-8e66-669ca788ff5f"

#define NS_IMESSAGELOOP_IID \
  {0x3e8c58e8, 0xe52c, 0x43e0, \
    { 0x8e, 0x66, 0x66, 0x9c, 0xa7, 0x88, 0xff, 0x5f }}

class NS_NO_VTABLE nsIMessageLoop : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMESSAGELOOP_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMessageLoop;

  /* void postIdleTask (in nsIRunnable task, in uint32_t ensureRunsAfterMS); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PostIdleTask(nsIRunnable *task, uint32_t ensureRunsAfterMS) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMessageLoop, NS_IMESSAGELOOP_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMESSAGELOOP \
  NS_IMETHOD PostIdleTask(nsIRunnable *task, uint32_t ensureRunsAfterMS) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMESSAGELOOP \
  nsresult PostIdleTask(nsIRunnable *task, uint32_t ensureRunsAfterMS); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMESSAGELOOP(_to) \
  NS_IMETHOD PostIdleTask(nsIRunnable *task, uint32_t ensureRunsAfterMS) override { return _to PostIdleTask(task, ensureRunsAfterMS); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMESSAGELOOP(_to) \
  NS_IMETHOD PostIdleTask(nsIRunnable *task, uint32_t ensureRunsAfterMS) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PostIdleTask(task, ensureRunsAfterMS); } 


#endif /* __gen_nsIMessageLoop_h__ */
