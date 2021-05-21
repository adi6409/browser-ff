/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleListener.idl
 */

#ifndef __gen_nsIConsoleListener_h__
#define __gen_nsIConsoleListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIConsoleMessage; /* forward declaration */


/* starting interface:    nsIConsoleListener */
#define NS_ICONSOLELISTENER_IID_STR "35c400a4-5792-438c-b915-65e30d58d557"

#define NS_ICONSOLELISTENER_IID \
  {0x35c400a4, 0x5792, 0x438c, \
    { 0xb9, 0x15, 0x65, 0xe3, 0x0d, 0x58, 0xd5, 0x57 }}

class NS_NO_VTABLE nsIConsoleListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICONSOLELISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIConsoleListener;

  /* void observe (in nsIConsoleMessage aMessage); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Observe(nsIConsoleMessage *aMessage) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIConsoleListener, NS_ICONSOLELISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICONSOLELISTENER \
  NS_IMETHOD Observe(nsIConsoleMessage *aMessage) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICONSOLELISTENER \
  nsresult Observe(nsIConsoleMessage *aMessage); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICONSOLELISTENER(_to) \
  NS_IMETHOD Observe(nsIConsoleMessage *aMessage) override { return _to Observe(aMessage); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICONSOLELISTENER(_to) \
  NS_IMETHOD Observe(nsIConsoleMessage *aMessage) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Observe(aMessage); } 


#endif /* __gen_nsIConsoleListener_h__ */
