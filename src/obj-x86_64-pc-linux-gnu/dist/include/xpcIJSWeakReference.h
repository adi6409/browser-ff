/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/xpcIJSWeakReference.idl
 */

#ifndef __gen_xpcIJSWeakReference_h__
#define __gen_xpcIJSWeakReference_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    xpcIJSWeakReference */
#define XPCIJSWEAKREFERENCE_IID_STR "75767928-ecb1-4e6c-9f55-c118b297fcef"

#define XPCIJSWEAKREFERENCE_IID \
  {0x75767928, 0xecb1, 0x4e6c, \
    { 0x9f, 0x55, 0xc1, 0x18, 0xb2, 0x97, 0xfc, 0xef }}

class NS_NO_VTABLE xpcIJSWeakReference : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(XPCIJSWEAKREFERENCE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = xpcIJSWeakReference;

  /* [implicit_jscontext] jsval get (); */
  NS_IMETHOD Get(JSContext* cx, JS::MutableHandleValue _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(xpcIJSWeakReference, XPCIJSWEAKREFERENCE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_XPCIJSWEAKREFERENCE \
  NS_IMETHOD Get(JSContext* cx, JS::MutableHandleValue _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_XPCIJSWEAKREFERENCE \
  nsresult Get(JSContext* cx, JS::MutableHandleValue _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_XPCIJSWEAKREFERENCE(_to) \
  NS_IMETHOD Get(JSContext* cx, JS::MutableHandleValue _retval) override { return _to Get(cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_XPCIJSWEAKREFERENCE(_to) \
  NS_IMETHOD Get(JSContext* cx, JS::MutableHandleValue _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Get(cx, _retval); } 


#endif /* __gen_xpcIJSWeakReference_h__ */
