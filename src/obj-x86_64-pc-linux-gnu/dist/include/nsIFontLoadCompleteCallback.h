/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/gfx/thebes/nsIFontLoadCompleteCallback.idl
 */

#ifndef __gen_nsIFontLoadCompleteCallback_h__
#define __gen_nsIFontLoadCompleteCallback_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFontLoadCompleteCallback */
#define NS_IFONTLOADCOMPLETECALLBACK_IID_STR "302dbf09-079b-4648-8a06-a0486c1749c0"

#define NS_IFONTLOADCOMPLETECALLBACK_IID \
  {0x302dbf09, 0x079b, 0x4648, \
    { 0x8a, 0x06, 0xa0, 0x48, 0x6c, 0x17, 0x49, 0xc0 }}

class NS_NO_VTABLE nsIFontLoadCompleteCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFONTLOADCOMPLETECALLBACK_IID)

  /* void fontLoadComplete (); */
  NS_IMETHOD FontLoadComplete(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFontLoadCompleteCallback, NS_IFONTLOADCOMPLETECALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFONTLOADCOMPLETECALLBACK \
  NS_IMETHOD FontLoadComplete(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFONTLOADCOMPLETECALLBACK \
  nsresult FontLoadComplete(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFONTLOADCOMPLETECALLBACK(_to) \
  NS_IMETHOD FontLoadComplete(void) override { return _to FontLoadComplete(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFONTLOADCOMPLETECALLBACK(_to) \
  NS_IMETHOD FontLoadComplete(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->FontLoadComplete(); } 


#endif /* __gen_nsIFontLoadCompleteCallback_h__ */
