/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/ductwork/debugger/IJSDebugger.idl
 */

#ifndef __gen_IJSDebugger_h__
#define __gen_IJSDebugger_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    IJSDebugger */
#define IJSDEBUGGER_IID_STR "a36fa816-31da-4b23-bc97-6412771f0867"

#define IJSDEBUGGER_IID \
  {0xa36fa816, 0x31da, 0x4b23, \
    { 0xbc, 0x97, 0x64, 0x12, 0x77, 0x1f, 0x08, 0x67 }}

class NS_NO_VTABLE IJSDebugger : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(IJSDEBUGGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = IJSDebugger;

  /* [implicit_jscontext] void addClass (in jsval global); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddClass(JS::HandleValue global, JSContext* cx) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(IJSDebugger, IJSDEBUGGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_IJSDEBUGGER \
  NS_IMETHOD AddClass(JS::HandleValue global, JSContext* cx) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_IJSDEBUGGER \
  nsresult AddClass(JS::HandleValue global, JSContext* cx); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_IJSDEBUGGER(_to) \
  NS_IMETHOD AddClass(JS::HandleValue global, JSContext* cx) override { return _to AddClass(global, cx); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_IJSDEBUGGER(_to) \
  NS_IMETHOD AddClass(JS::HandleValue global, JSContext* cx) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddClass(global, cx); } 


#endif /* __gen_IJSDebugger_h__ */
