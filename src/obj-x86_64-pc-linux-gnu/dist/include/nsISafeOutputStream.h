/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsISafeOutputStream.idl
 */

#ifndef __gen_nsISafeOutputStream_h__
#define __gen_nsISafeOutputStream_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsISafeOutputStream */
#define NS_ISAFEOUTPUTSTREAM_IID_STR "5f914307-5c34-4e1f-8e32-ec749d25b27a"

#define NS_ISAFEOUTPUTSTREAM_IID \
  {0x5f914307, 0x5c34, 0x4e1f, \
    { 0x8e, 0x32, 0xec, 0x74, 0x9d, 0x25, 0xb2, 0x7a }}

class NS_NO_VTABLE nsISafeOutputStream : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISAFEOUTPUTSTREAM_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISafeOutputStream;

  /* void finish (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Finish(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISafeOutputStream, NS_ISAFEOUTPUTSTREAM_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISAFEOUTPUTSTREAM \
  NS_IMETHOD Finish(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISAFEOUTPUTSTREAM \
  nsresult Finish(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISAFEOUTPUTSTREAM(_to) \
  NS_IMETHOD Finish(void) override { return _to Finish(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISAFEOUTPUTSTREAM(_to) \
  NS_IMETHOD Finish(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Finish(); } 


#endif /* __gen_nsISafeOutputStream_h__ */
