/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/devtools/platform/nsIJSInspector.idl
 */

#ifndef __gen_nsIJSInspector_h__
#define __gen_nsIJSInspector_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIJSInspector */
#define NS_IJSINSPECTOR_IID_STR "6758d0d7-e96a-4c5c-bca8-3bcbe5a15943"

#define NS_IJSINSPECTOR_IID \
  {0x6758d0d7, 0xe96a, 0x4c5c, \
    { 0xbc, 0xa8, 0x3b, 0xcb, 0xe5, 0xa1, 0x59, 0x43 }}

class NS_NO_VTABLE nsIJSInspector : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IJSINSPECTOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIJSInspector;

  /* unsigned long enterNestedEventLoop (in jsval requestor); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnterNestedEventLoop(JS::HandleValue requestor, uint32_t *_retval) = 0;

  /* unsigned long exitNestedEventLoop (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ExitNestedEventLoop(uint32_t *_retval) = 0;

  /* readonly attribute unsigned long eventLoopNestLevel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetEventLoopNestLevel(uint32_t *aEventLoopNestLevel) = 0;

  /* readonly attribute jsval lastNestRequestor; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetLastNestRequestor(JS::MutableHandleValue aLastNestRequestor) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIJSInspector, NS_IJSINSPECTOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIJSINSPECTOR \
  NS_IMETHOD EnterNestedEventLoop(JS::HandleValue requestor, uint32_t *_retval) override; \
  NS_IMETHOD ExitNestedEventLoop(uint32_t *_retval) override; \
  NS_IMETHOD GetEventLoopNestLevel(uint32_t *aEventLoopNestLevel) override; \
  NS_IMETHOD GetLastNestRequestor(JS::MutableHandleValue aLastNestRequestor) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIJSINSPECTOR \
  nsresult EnterNestedEventLoop(JS::HandleValue requestor, uint32_t *_retval); \
  nsresult ExitNestedEventLoop(uint32_t *_retval); \
  nsresult GetEventLoopNestLevel(uint32_t *aEventLoopNestLevel); \
  nsresult GetLastNestRequestor(JS::MutableHandleValue aLastNestRequestor); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIJSINSPECTOR(_to) \
  NS_IMETHOD EnterNestedEventLoop(JS::HandleValue requestor, uint32_t *_retval) override { return _to EnterNestedEventLoop(requestor, _retval); } \
  NS_IMETHOD ExitNestedEventLoop(uint32_t *_retval) override { return _to ExitNestedEventLoop(_retval); } \
  NS_IMETHOD GetEventLoopNestLevel(uint32_t *aEventLoopNestLevel) override { return _to GetEventLoopNestLevel(aEventLoopNestLevel); } \
  NS_IMETHOD GetLastNestRequestor(JS::MutableHandleValue aLastNestRequestor) override { return _to GetLastNestRequestor(aLastNestRequestor); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIJSINSPECTOR(_to) \
  NS_IMETHOD EnterNestedEventLoop(JS::HandleValue requestor, uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnterNestedEventLoop(requestor, _retval); } \
  NS_IMETHOD ExitNestedEventLoop(uint32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ExitNestedEventLoop(_retval); } \
  NS_IMETHOD GetEventLoopNestLevel(uint32_t *aEventLoopNestLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetEventLoopNestLevel(aEventLoopNestLevel); } \
  NS_IMETHOD GetLastNestRequestor(JS::MutableHandleValue aLastNestRequestor) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLastNestRequestor(aLastNestRequestor); } 


#endif /* __gen_nsIJSInspector_h__ */
