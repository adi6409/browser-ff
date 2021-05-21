/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListener.idl
 */

#ifndef __gen_nsIStreamListener_h__
#define __gen_nsIStreamListener_h__


#ifndef __gen_nsIRequestObserver_h__
#include "nsIRequestObserver.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIInputStream; /* forward declaration */


/* starting interface:    nsIStreamListener */
#define NS_ISTREAMLISTENER_IID_STR "3b4c8a77-76ba-4610-b316-678c73a3b88c"

#define NS_ISTREAMLISTENER_IID \
  {0x3b4c8a77, 0x76ba, 0x4610, \
    { 0xb3, 0x16, 0x67, 0x8c, 0x73, 0xa3, 0xb8, 0x8c }}

class NS_NO_VTABLE nsIStreamListener : public nsIRequestObserver {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISTREAMLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIStreamListener;

  /* void onDataAvailable (in nsIRequest aRequest, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnDataAvailable(nsIRequest *aRequest, nsIInputStream *aInputStream, uint64_t aOffset, uint32_t aCount) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIStreamListener, NS_ISTREAMLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISTREAMLISTENER \
  NS_IMETHOD OnDataAvailable(nsIRequest *aRequest, nsIInputStream *aInputStream, uint64_t aOffset, uint32_t aCount) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISTREAMLISTENER \
  nsresult OnDataAvailable(nsIRequest *aRequest, nsIInputStream *aInputStream, uint64_t aOffset, uint32_t aCount); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISTREAMLISTENER(_to) \
  NS_IMETHOD OnDataAvailable(nsIRequest *aRequest, nsIInputStream *aInputStream, uint64_t aOffset, uint32_t aCount) override { return _to OnDataAvailable(aRequest, aInputStream, aOffset, aCount); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISTREAMLISTENER(_to) \
  NS_IMETHOD OnDataAvailable(nsIRequest *aRequest, nsIInputStream *aInputStream, uint64_t aOffset, uint32_t aCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnDataAvailable(aRequest, aInputStream, aOffset, aCount); } 


#endif /* __gen_nsIStreamListener_h__ */
