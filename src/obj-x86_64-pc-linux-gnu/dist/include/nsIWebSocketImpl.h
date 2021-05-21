/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsIWebSocketImpl.idl
 */

#ifndef __gen_nsIWebSocketImpl_h__
#define __gen_nsIWebSocketImpl_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIWebSocketImpl */
#define NS_IWEBSOCKETIMPL_IID_STR "db1f4e2b-3cff-4615-a03c-341fda66c53d"

#define NS_IWEBSOCKETIMPL_IID \
  {0xdb1f4e2b, 0x3cff, 0x4615, \
    { 0xa0, 0x3c, 0x34, 0x1f, 0xda, 0x66, 0xc5, 0x3d }}

class NS_NO_VTABLE nsIWebSocketImpl : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWEBSOCKETIMPL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWebSocketImpl;

  /* [must_use] void sendMessage (in AString aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD SendMessage(const nsAString& aMessage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWebSocketImpl, NS_IWEBSOCKETIMPL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWEBSOCKETIMPL \
  [[nodiscard]] NS_IMETHOD SendMessage(const nsAString& aMessage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWEBSOCKETIMPL \
  [[nodiscard]] nsresult SendMessage(const nsAString& aMessage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWEBSOCKETIMPL(_to) \
  [[nodiscard]] NS_IMETHOD SendMessage(const nsAString& aMessage) override { return _to SendMessage(aMessage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWEBSOCKETIMPL(_to) \
  [[nodiscard]] NS_IMETHOD SendMessage(const nsAString& aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendMessage(aMessage); } 


#endif /* __gen_nsIWebSocketImpl_h__ */
