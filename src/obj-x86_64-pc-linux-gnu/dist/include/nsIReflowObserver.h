/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIReflowObserver.idl
 */

#ifndef __gen_nsIReflowObserver_h__
#define __gen_nsIReflowObserver_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIReflowObserver */
#define NS_IREFLOWOBSERVER_IID_STR "832e692c-c4a6-11e2-8fd1-dce678957a39"

#define NS_IREFLOWOBSERVER_IID \
  {0x832e692c, 0xc4a6, 0x11e2, \
    { 0x8f, 0xd1, 0xdc, 0xe6, 0x78, 0x95, 0x7a, 0x39 }}

class NS_NO_VTABLE nsIReflowObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IREFLOWOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIReflowObserver;

  /* void reflow (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Reflow(DOMHighResTimeStamp start, DOMHighResTimeStamp end) = 0;

  /* void reflowInterruptible (in DOMHighResTimeStamp start, in DOMHighResTimeStamp end); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ReflowInterruptible(DOMHighResTimeStamp start, DOMHighResTimeStamp end) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIReflowObserver, NS_IREFLOWOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIREFLOWOBSERVER \
  NS_IMETHOD Reflow(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override; \
  NS_IMETHOD ReflowInterruptible(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIREFLOWOBSERVER \
  nsresult Reflow(DOMHighResTimeStamp start, DOMHighResTimeStamp end); \
  nsresult ReflowInterruptible(DOMHighResTimeStamp start, DOMHighResTimeStamp end); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIREFLOWOBSERVER(_to) \
  NS_IMETHOD Reflow(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return _to Reflow(start, end); } \
  NS_IMETHOD ReflowInterruptible(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return _to ReflowInterruptible(start, end); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIREFLOWOBSERVER(_to) \
  NS_IMETHOD Reflow(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Reflow(start, end); } \
  NS_IMETHOD ReflowInterruptible(DOMHighResTimeStamp start, DOMHighResTimeStamp end) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReflowInterruptible(start, end); } 


#endif /* __gen_nsIReflowObserver_h__ */
