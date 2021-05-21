/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChannelEventSink.idl
 */

#ifndef __gen_nsIChannelEventSink_h__
#define __gen_nsIChannelEventSink_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */

class nsIAsyncVerifyRedirectCallback; /* forward declaration */


/* starting interface:    nsIChannelEventSink */
#define NS_ICHANNELEVENTSINK_IID_STR "0197720d-37ed-4e75-8956-d0d296e4d8a6"

#define NS_ICHANNELEVENTSINK_IID \
  {0x0197720d, 0x37ed, 0x4e75, \
    { 0x89, 0x56, 0xd0, 0xd2, 0x96, 0xe4, 0xd8, 0xa6 }}

class NS_NO_VTABLE nsIChannelEventSink : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICHANNELEVENTSINK_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIChannelEventSink;

  enum {
    REDIRECT_TEMPORARY = 1U,
    REDIRECT_PERMANENT = 2U,
    REDIRECT_INTERNAL = 4U,
    REDIRECT_STS_UPGRADE = 8U
  };

  /* void asyncOnChannelRedirect (in nsIChannel oldChannel, in nsIChannel newChannel, in unsigned long flags, in nsIAsyncVerifyRedirectCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AsyncOnChannelRedirect(nsIChannel *oldChannel, nsIChannel *newChannel, uint32_t flags, nsIAsyncVerifyRedirectCallback *callback) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIChannelEventSink, NS_ICHANNELEVENTSINK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICHANNELEVENTSINK \
  NS_IMETHOD AsyncOnChannelRedirect(nsIChannel *oldChannel, nsIChannel *newChannel, uint32_t flags, nsIAsyncVerifyRedirectCallback *callback) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICHANNELEVENTSINK \
  nsresult AsyncOnChannelRedirect(nsIChannel *oldChannel, nsIChannel *newChannel, uint32_t flags, nsIAsyncVerifyRedirectCallback *callback); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICHANNELEVENTSINK(_to) \
  NS_IMETHOD AsyncOnChannelRedirect(nsIChannel *oldChannel, nsIChannel *newChannel, uint32_t flags, nsIAsyncVerifyRedirectCallback *callback) override { return _to AsyncOnChannelRedirect(oldChannel, newChannel, flags, callback); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICHANNELEVENTSINK(_to) \
  NS_IMETHOD AsyncOnChannelRedirect(nsIChannel *oldChannel, nsIChannel *newChannel, uint32_t flags, nsIAsyncVerifyRedirectCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AsyncOnChannelRedirect(oldChannel, newChannel, flags, callback); } 


#endif /* __gen_nsIChannelEventSink_h__ */
