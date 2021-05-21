/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISimpleStreamListener.idl
 */

#ifndef __gen_nsISimpleStreamListener_h__
#define __gen_nsISimpleStreamListener_h__


#ifndef __gen_nsIStreamListener_h__
#include "nsIStreamListener.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIOutputStream; /* forward declaration */


/* starting interface:    nsISimpleStreamListener */
#define NS_ISIMPLESTREAMLISTENER_IID_STR "a9b84f6a-0824-4278-bae6-bfca0570a26e"

#define NS_ISIMPLESTREAMLISTENER_IID \
  {0xa9b84f6a, 0x0824, 0x4278, \
    { 0xba, 0xe6, 0xbf, 0xca, 0x05, 0x70, 0xa2, 0x6e }}

class NS_NO_VTABLE nsISimpleStreamListener : public nsIStreamListener {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISIMPLESTREAMLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISimpleStreamListener;

  /* void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Init(nsIOutputStream *aSink, nsIRequestObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISimpleStreamListener, NS_ISIMPLESTREAMLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISIMPLESTREAMLISTENER \
  NS_IMETHOD Init(nsIOutputStream *aSink, nsIRequestObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISIMPLESTREAMLISTENER \
  nsresult Init(nsIOutputStream *aSink, nsIRequestObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISIMPLESTREAMLISTENER(_to) \
  NS_IMETHOD Init(nsIOutputStream *aSink, nsIRequestObserver *aObserver) override { return _to Init(aSink, aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISIMPLESTREAMLISTENER(_to) \
  NS_IMETHOD Init(nsIOutputStream *aSink, nsIRequestObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(aSink, aObserver); } 


#endif /* __gen_nsISimpleStreamListener_h__ */
