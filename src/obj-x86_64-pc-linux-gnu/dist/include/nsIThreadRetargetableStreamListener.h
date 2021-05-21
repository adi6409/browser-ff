/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThreadRetargetableStreamListener.idl
 */

#ifndef __gen_nsIThreadRetargetableStreamListener_h__
#define __gen_nsIThreadRetargetableStreamListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIThreadRetargetableStreamListener */
#define NS_ITHREADRETARGETABLESTREAMLISTENER_IID_STR "fb2304b8-f82f-4433-af68-d874a2ebbdc1"

#define NS_ITHREADRETARGETABLESTREAMLISTENER_IID \
  {0xfb2304b8, 0xf82f, 0x4433, \
    { 0xaf, 0x68, 0xd8, 0x74, 0xa2, 0xeb, 0xbd, 0xc1 }}

class NS_NO_VTABLE nsIThreadRetargetableStreamListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITHREADRETARGETABLESTREAMLISTENER_IID)

  /* void checkListenerChain (); */
  NS_IMETHOD CheckListenerChain(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIThreadRetargetableStreamListener, NS_ITHREADRETARGETABLESTREAMLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITHREADRETARGETABLESTREAMLISTENER \
  NS_IMETHOD CheckListenerChain(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITHREADRETARGETABLESTREAMLISTENER \
  nsresult CheckListenerChain(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITHREADRETARGETABLESTREAMLISTENER(_to) \
  NS_IMETHOD CheckListenerChain(void) override { return _to CheckListenerChain(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITHREADRETARGETABLESTREAMLISTENER(_to) \
  NS_IMETHOD CheckListenerChain(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CheckListenerChain(); } 


#endif /* __gen_nsIThreadRetargetableStreamListener_h__ */
