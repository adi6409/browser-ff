/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIParentRedirectingChannel.idl
 */

#ifndef __gen_nsIParentRedirectingChannel_h__
#define __gen_nsIParentRedirectingChannel_h__


#ifndef __gen_nsIParentChannel_h__
#include "nsIParentChannel.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIRemoteTab; /* forward declaration */

class nsIChannel; /* forward declaration */

class nsIAsyncVerifyRedirectCallback; /* forward declaration */


/* starting interface:    nsIAsyncVerifyRedirectReadyCallback */
#define NS_IASYNCVERIFYREDIRECTREADYCALLBACK_IID_STR "01987690-48cf-45de-bae3-e143c2adc2a8"

#define NS_IASYNCVERIFYREDIRECTREADYCALLBACK_IID \
  {0x01987690, 0x48cf, 0x45de, \
    { 0xba, 0xe3, 0xe1, 0x43, 0xc2, 0xad, 0xc2, 0xa8 }}

class NS_NO_VTABLE nsIAsyncVerifyRedirectReadyCallback : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IASYNCVERIFYREDIRECTREADYCALLBACK_IID)

  /* void readyToVerify (in nsresult result); */
  NS_IMETHOD ReadyToVerify(nsresult result) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAsyncVerifyRedirectReadyCallback, NS_IASYNCVERIFYREDIRECTREADYCALLBACK_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIASYNCVERIFYREDIRECTREADYCALLBACK \
  NS_IMETHOD ReadyToVerify(nsresult result) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIASYNCVERIFYREDIRECTREADYCALLBACK \
  nsresult ReadyToVerify(nsresult result); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIASYNCVERIFYREDIRECTREADYCALLBACK(_to) \
  NS_IMETHOD ReadyToVerify(nsresult result) override { return _to ReadyToVerify(result); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIASYNCVERIFYREDIRECTREADYCALLBACK(_to) \
  NS_IMETHOD ReadyToVerify(nsresult result) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ReadyToVerify(result); } 


/* starting interface:    nsIParentRedirectingChannel */
#define NS_IPARENTREDIRECTINGCHANNEL_IID_STR "3ed1d288-5324-46ee-8a98-33ac37d1080b"

#define NS_IPARENTREDIRECTINGCHANNEL_IID \
  {0x3ed1d288, 0x5324, 0x46ee, \
    { 0x8a, 0x98, 0x33, 0xac, 0x37, 0xd1, 0x08, 0x0b }}

class NS_NO_VTABLE nsIParentRedirectingChannel : public nsIParentChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPARENTREDIRECTINGCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIParentRedirectingChannel;

  /* void startRedirect (in nsIChannel newChannel, in uint32_t redirectFlags, in nsIAsyncVerifyRedirectCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD StartRedirect(nsIChannel *newChannel, uint32_t redirectFlags, nsIAsyncVerifyRedirectCallback *callback) = 0;

  /* void continueVerification (in nsIAsyncVerifyRedirectReadyCallback callback); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ContinueVerification(nsIAsyncVerifyRedirectReadyCallback *callback) = 0;

  /* void completeRedirect (in boolean succeeded); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD CompleteRedirect(bool succeeded) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIParentRedirectingChannel, NS_IPARENTREDIRECTINGCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPARENTREDIRECTINGCHANNEL \
  NS_IMETHOD StartRedirect(nsIChannel *newChannel, uint32_t redirectFlags, nsIAsyncVerifyRedirectCallback *callback) override; \
  NS_IMETHOD ContinueVerification(nsIAsyncVerifyRedirectReadyCallback *callback) override; \
  NS_IMETHOD CompleteRedirect(bool succeeded) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPARENTREDIRECTINGCHANNEL \
  nsresult StartRedirect(nsIChannel *newChannel, uint32_t redirectFlags, nsIAsyncVerifyRedirectCallback *callback); \
  nsresult ContinueVerification(nsIAsyncVerifyRedirectReadyCallback *callback); \
  nsresult CompleteRedirect(bool succeeded); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPARENTREDIRECTINGCHANNEL(_to) \
  NS_IMETHOD StartRedirect(nsIChannel *newChannel, uint32_t redirectFlags, nsIAsyncVerifyRedirectCallback *callback) override { return _to StartRedirect(newChannel, redirectFlags, callback); } \
  NS_IMETHOD ContinueVerification(nsIAsyncVerifyRedirectReadyCallback *callback) override { return _to ContinueVerification(callback); } \
  NS_IMETHOD CompleteRedirect(bool succeeded) override { return _to CompleteRedirect(succeeded); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPARENTREDIRECTINGCHANNEL(_to) \
  NS_IMETHOD StartRedirect(nsIChannel *newChannel, uint32_t redirectFlags, nsIAsyncVerifyRedirectCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->StartRedirect(newChannel, redirectFlags, callback); } \
  NS_IMETHOD ContinueVerification(nsIAsyncVerifyRedirectReadyCallback *callback) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ContinueVerification(callback); } \
  NS_IMETHOD CompleteRedirect(bool succeeded) override { return !_to ? NS_ERROR_NULL_POINTER : _to->CompleteRedirect(succeeded); } 


#endif /* __gen_nsIParentRedirectingChannel_h__ */
