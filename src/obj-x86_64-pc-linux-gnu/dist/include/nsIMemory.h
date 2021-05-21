/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIMemory.idl
 */

#ifndef __gen_nsIMemory_h__
#define __gen_nsIMemory_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIMemory */
#define NS_IMEMORY_IID_STR "1e004834-6d8f-425a-bc9c-a2812ed43bb7"

#define NS_IMEMORY_IID \
  {0x1e004834, 0x6d8f, 0x425a, \
    { 0xbc, 0x9c, 0xa2, 0x81, 0x2e, 0xd4, 0x3b, 0xb7 }}

class NS_NO_VTABLE nsIMemory : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMEMORY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMemory;

  /* void heapMinimize (in boolean immediate); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD HeapMinimize(bool immediate) = 0;

  /* boolean isLowMemoryPlatform (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD IsLowMemoryPlatform(bool *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMemory, NS_IMEMORY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMEMORY \
  NS_IMETHOD HeapMinimize(bool immediate) override; \
  NS_IMETHOD IsLowMemoryPlatform(bool *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMEMORY \
  nsresult HeapMinimize(bool immediate); \
  nsresult IsLowMemoryPlatform(bool *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMEMORY(_to) \
  NS_IMETHOD HeapMinimize(bool immediate) override { return _to HeapMinimize(immediate); } \
  NS_IMETHOD IsLowMemoryPlatform(bool *_retval) override { return _to IsLowMemoryPlatform(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMEMORY(_to) \
  NS_IMETHOD HeapMinimize(bool immediate) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HeapMinimize(immediate); } \
  NS_IMETHOD IsLowMemoryPlatform(bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsLowMemoryPlatform(_retval); } 


#endif /* __gen_nsIMemory_h__ */
