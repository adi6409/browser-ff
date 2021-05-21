/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChildChannel.idl
 */

#ifndef __gen_nsIChildChannel_h__
#define __gen_nsIChildChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIStreamListener; /* forward declaration */


/* starting interface:    nsIChildChannel */
#define NS_ICHILDCHANNEL_IID_STR "c45b92ae-4f07-41dd-b0ef-aa044eeabb1e"

#define NS_ICHILDCHANNEL_IID \
  {0xc45b92ae, 0x4f07, 0x41dd, \
    { 0xb0, 0xef, 0xaa, 0x04, 0x4e, 0xea, 0xbb, 0x1e }}

class NS_NO_VTABLE nsIChildChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICHILDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIChildChannel;

  /* void connectParent (in uint32_t registrarId); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ConnectParent(uint32_t registrarId) = 0;

  /* void completeRedirectSetup (in nsIStreamListener aListener); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteRedirectSetup(nsIStreamListener *aListener) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIChildChannel, NS_ICHILDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICHILDCHANNEL \
  NS_IMETHOD ConnectParent(uint32_t registrarId) override; \
  NS_IMETHOD CompleteRedirectSetup(nsIStreamListener *aListener) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICHILDCHANNEL \
  nsresult ConnectParent(uint32_t registrarId); \
  nsresult CompleteRedirectSetup(nsIStreamListener *aListener); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICHILDCHANNEL(_to) \
  NS_IMETHOD ConnectParent(uint32_t registrarId) override { return _to ConnectParent(registrarId); } \
  NS_IMETHOD CompleteRedirectSetup(nsIStreamListener *aListener) override { return _to CompleteRedirectSetup(aListener); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICHILDCHANNEL(_to) \
  NS_IMETHOD ConnectParent(uint32_t registrarId) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ConnectParent(registrarId); } \
  NS_IMETHOD CompleteRedirectSetup(nsIStreamListener *aListener) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompleteRedirectSetup(aListener); } 


#endif /* __gen_nsIChildChannel_h__ */
