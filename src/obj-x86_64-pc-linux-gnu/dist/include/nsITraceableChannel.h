/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsITraceableChannel.idl
 */

#ifndef __gen_nsITraceableChannel_h__
#define __gen_nsITraceableChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIStreamListener; /* forward declaration */


/* starting interface:    nsITraceableChannel */
#define NS_ITRACEABLECHANNEL_IID_STR "68167b0b-ef34-4d79-a09a-8045f7c5140e"

#define NS_ITRACEABLECHANNEL_IID \
  {0x68167b0b, 0xef34, 0x4d79, \
    { 0xa0, 0x9a, 0x80, 0x45, 0xf7, 0xc5, 0x14, 0x0e }}

class NS_NO_VTABLE nsITraceableChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITRACEABLECHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITraceableChannel;

  /* nsIStreamListener setNewListener (in nsIStreamListener aListener, [optional] in boolean aMustApplyContentConversion); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetNewListener(nsIStreamListener *aListener, bool aMustApplyContentConversion, nsIStreamListener **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITraceableChannel, NS_ITRACEABLECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITRACEABLECHANNEL \
  NS_IMETHOD SetNewListener(nsIStreamListener *aListener, bool aMustApplyContentConversion, nsIStreamListener **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITRACEABLECHANNEL \
  nsresult SetNewListener(nsIStreamListener *aListener, bool aMustApplyContentConversion, nsIStreamListener **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITRACEABLECHANNEL(_to) \
  NS_IMETHOD SetNewListener(nsIStreamListener *aListener, bool aMustApplyContentConversion, nsIStreamListener **_retval) override { return _to SetNewListener(aListener, aMustApplyContentConversion, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITRACEABLECHANNEL(_to) \
  NS_IMETHOD SetNewListener(nsIStreamListener *aListener, bool aMustApplyContentConversion, nsIStreamListener **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetNewListener(aListener, aMustApplyContentConversion, _retval); } 


#endif /* __gen_nsITraceableChannel_h__ */
