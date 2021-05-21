/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISlowScriptDebug.idl
 */

#ifndef __gen_nsISlowScriptDebug_h__
#define __gen_nsISlowScriptDebug_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMWindow; /* forward declaration */

namespace mozilla {
namespace dom {
class EventTarget; /* webidl EventTarget */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsISlowScriptDebugCallback */
#define NS_ISLOWSCRIPTDEBUGCALLBACK_IID_STR "f7dbb80c-5d1e-4fd9-b55c-a9ffda4a75b1"

#define NS_ISLOWSCRIPTDEBUGCALLBACK_IID \
  {0xf7dbb80c, 0x5d1e, 0x4fd9, \
    { 0xb5, 0x5c, 0xa9, 0xff, 0xda, 0x4a, 0x75, 0xb1 }}

class NS_NO_VTABLE nsISlowScriptDebugCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISLOWSCRIPTDEBUGCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISlowScriptDebugCallback;

  /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSlowScriptDebug(nsIDOMWindow *aWindow) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISlowScriptDebugCallback, NS_ISLOWSCRIPTDEBUGCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISLOWSCRIPTDEBUGCALLBACK \
  NS_IMETHOD HandleSlowScriptDebug(nsIDOMWindow *aWindow) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISLOWSCRIPTDEBUGCALLBACK \
  nsresult HandleSlowScriptDebug(nsIDOMWindow *aWindow); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISLOWSCRIPTDEBUGCALLBACK(_to) \
  NS_IMETHOD HandleSlowScriptDebug(nsIDOMWindow *aWindow) override { return _to HandleSlowScriptDebug(aWindow); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISLOWSCRIPTDEBUGCALLBACK(_to) \
  NS_IMETHOD HandleSlowScriptDebug(nsIDOMWindow *aWindow) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSlowScriptDebug(aWindow); } 


/* starting interface:    nsISlowScriptDebuggerStartupCallback */
#define NS_ISLOWSCRIPTDEBUGGERSTARTUPCALLBACK_IID_STR "b1c6ecd0-8fa4-11e4-b4a9-0800200c9a66"

#define NS_ISLOWSCRIPTDEBUGGERSTARTUPCALLBACK_IID \
  {0xb1c6ecd0, 0x8fa4, 0x11e4, \
    { 0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsISlowScriptDebuggerStartupCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISLOWSCRIPTDEBUGGERSTARTUPCALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISlowScriptDebuggerStartupCallback;

  /* void finishDebuggerStartup (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD FinishDebuggerStartup(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISlowScriptDebuggerStartupCallback, NS_ISLOWSCRIPTDEBUGGERSTARTUPCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISLOWSCRIPTDEBUGGERSTARTUPCALLBACK \
  NS_IMETHOD FinishDebuggerStartup(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISLOWSCRIPTDEBUGGERSTARTUPCALLBACK \
  nsresult FinishDebuggerStartup(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISLOWSCRIPTDEBUGGERSTARTUPCALLBACK(_to) \
  NS_IMETHOD FinishDebuggerStartup(void) override { return _to FinishDebuggerStartup(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISLOWSCRIPTDEBUGGERSTARTUPCALLBACK(_to) \
  NS_IMETHOD FinishDebuggerStartup(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FinishDebuggerStartup(); } 


/* starting interface:    nsISlowScriptDebugRemoteCallback */
#define NS_ISLOWSCRIPTDEBUGREMOTECALLBACK_IID_STR "dbee14b0-8fa0-11e4-b4a9-0800200c9a66"

#define NS_ISLOWSCRIPTDEBUGREMOTECALLBACK_IID \
  {0xdbee14b0, 0x8fa0, 0x11e4, \
    { 0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsISlowScriptDebugRemoteCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISLOWSCRIPTDEBUGREMOTECALLBACK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISlowScriptDebugRemoteCallback;

  /* void handleSlowScriptDebug (in EventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HandleSlowScriptDebug(mozilla::dom::EventTarget *aBrowser, nsISlowScriptDebuggerStartupCallback *aCallback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISlowScriptDebugRemoteCallback, NS_ISLOWSCRIPTDEBUGREMOTECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISLOWSCRIPTDEBUGREMOTECALLBACK \
  NS_IMETHOD HandleSlowScriptDebug(mozilla::dom::EventTarget *aBrowser, nsISlowScriptDebuggerStartupCallback *aCallback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISLOWSCRIPTDEBUGREMOTECALLBACK \
  nsresult HandleSlowScriptDebug(mozilla::dom::EventTarget *aBrowser, nsISlowScriptDebuggerStartupCallback *aCallback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISLOWSCRIPTDEBUGREMOTECALLBACK(_to) \
  NS_IMETHOD HandleSlowScriptDebug(mozilla::dom::EventTarget *aBrowser, nsISlowScriptDebuggerStartupCallback *aCallback) override { return _to HandleSlowScriptDebug(aBrowser, aCallback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISLOWSCRIPTDEBUGREMOTECALLBACK(_to) \
  NS_IMETHOD HandleSlowScriptDebug(mozilla::dom::EventTarget *aBrowser, nsISlowScriptDebuggerStartupCallback *aCallback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleSlowScriptDebug(aBrowser, aCallback); } 


/* starting interface:    nsISlowScriptDebug */
#define NS_ISLOWSCRIPTDEBUG_IID_STR "f75d4164-3aa7-4395-ba44-a5f95b2e8427"

#define NS_ISLOWSCRIPTDEBUG_IID \
  {0xf75d4164, 0x3aa7, 0x4395, \
    { 0xba, 0x44, 0xa5, 0xf9, 0x5b, 0x2e, 0x84, 0x27 }}

class NS_NO_VTABLE nsISlowScriptDebug : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISLOWSCRIPTDEBUG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISlowScriptDebug;

  /* attribute nsISlowScriptDebugCallback activationHandler; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetActivationHandler(nsISlowScriptDebugCallback **aActivationHandler) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetActivationHandler(nsISlowScriptDebugCallback *aActivationHandler) = 0;

  /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback **aRemoteActivationHandler) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback *aRemoteActivationHandler) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISlowScriptDebug, NS_ISLOWSCRIPTDEBUG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISLOWSCRIPTDEBUG \
  NS_IMETHOD GetActivationHandler(nsISlowScriptDebugCallback **aActivationHandler) override; \
  NS_IMETHOD SetActivationHandler(nsISlowScriptDebugCallback *aActivationHandler) override; \
  NS_IMETHOD GetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback **aRemoteActivationHandler) override; \
  NS_IMETHOD SetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback *aRemoteActivationHandler) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISLOWSCRIPTDEBUG \
  nsresult GetActivationHandler(nsISlowScriptDebugCallback **aActivationHandler); \
  nsresult SetActivationHandler(nsISlowScriptDebugCallback *aActivationHandler); \
  nsresult GetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback **aRemoteActivationHandler); \
  nsresult SetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback *aRemoteActivationHandler); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISLOWSCRIPTDEBUG(_to) \
  NS_IMETHOD GetActivationHandler(nsISlowScriptDebugCallback **aActivationHandler) override { return _to GetActivationHandler(aActivationHandler); } \
  NS_IMETHOD SetActivationHandler(nsISlowScriptDebugCallback *aActivationHandler) override { return _to SetActivationHandler(aActivationHandler); } \
  NS_IMETHOD GetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback **aRemoteActivationHandler) override { return _to GetRemoteActivationHandler(aRemoteActivationHandler); } \
  NS_IMETHOD SetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback *aRemoteActivationHandler) override { return _to SetRemoteActivationHandler(aRemoteActivationHandler); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISLOWSCRIPTDEBUG(_to) \
  NS_IMETHOD GetActivationHandler(nsISlowScriptDebugCallback **aActivationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetActivationHandler(aActivationHandler); } \
  NS_IMETHOD SetActivationHandler(nsISlowScriptDebugCallback *aActivationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetActivationHandler(aActivationHandler); } \
  NS_IMETHOD GetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback **aRemoteActivationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRemoteActivationHandler(aRemoteActivationHandler); } \
  NS_IMETHOD SetRemoteActivationHandler(nsISlowScriptDebugRemoteCallback *aRemoteActivationHandler) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetRemoteActivationHandler(aRemoteActivationHandler); } 


#endif /* __gen_nsISlowScriptDebug_h__ */
