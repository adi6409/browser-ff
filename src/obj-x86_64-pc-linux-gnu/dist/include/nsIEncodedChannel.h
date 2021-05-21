/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIEncodedChannel.idl
 */

#ifndef __gen_nsIEncodedChannel_h__
#define __gen_nsIEncodedChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIUTF8StringEnumerator; /* forward declaration */

class nsIStreamListener; /* forward declaration */

class nsISupports; /* forward declaration */


/* starting interface:    nsIEncodedChannel */
#define NS_IENCODEDCHANNEL_IID_STR "29c29ce6-8ce4-45e6-8d60-36c8fa3e255b"

#define NS_IENCODEDCHANNEL_IID \
  {0x29c29ce6, 0x8ce4, 0x45e6, \
    { 0x8d, 0x60, 0x36, 0xc8, 0xfa, 0x3e, 0x25, 0x5b }}

class NS_NO_VTABLE nsIEncodedChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IENCODEDCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIEncodedChannel;

  /* readonly attribute nsIUTF8StringEnumerator contentEncodings; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetContentEncodings(nsIUTF8StringEnumerator **aContentEncodings) = 0;

  /* attribute boolean applyConversion; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetApplyConversion(bool *aApplyConversion) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetApplyConversion(bool aApplyConversion) = 0;

  /* void doApplyContentConversions (in nsIStreamListener aNextListener, out nsIStreamListener aNewNextListener, in nsISupports aCtxt); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD DoApplyContentConversions(nsIStreamListener *aNextListener, nsIStreamListener **aNewNextListener, nsISupports *aCtxt) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIEncodedChannel, NS_IENCODEDCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIENCODEDCHANNEL \
  NS_IMETHOD GetContentEncodings(nsIUTF8StringEnumerator **aContentEncodings) override; \
  NS_IMETHOD GetApplyConversion(bool *aApplyConversion) override; \
  NS_IMETHOD SetApplyConversion(bool aApplyConversion) override; \
  NS_IMETHOD DoApplyContentConversions(nsIStreamListener *aNextListener, nsIStreamListener **aNewNextListener, nsISupports *aCtxt) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIENCODEDCHANNEL \
  nsresult GetContentEncodings(nsIUTF8StringEnumerator **aContentEncodings); \
  nsresult GetApplyConversion(bool *aApplyConversion); \
  nsresult SetApplyConversion(bool aApplyConversion); \
  nsresult DoApplyContentConversions(nsIStreamListener *aNextListener, nsIStreamListener **aNewNextListener, nsISupports *aCtxt); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIENCODEDCHANNEL(_to) \
  NS_IMETHOD GetContentEncodings(nsIUTF8StringEnumerator **aContentEncodings) override { return _to GetContentEncodings(aContentEncodings); } \
  NS_IMETHOD GetApplyConversion(bool *aApplyConversion) override { return _to GetApplyConversion(aApplyConversion); } \
  NS_IMETHOD SetApplyConversion(bool aApplyConversion) override { return _to SetApplyConversion(aApplyConversion); } \
  NS_IMETHOD DoApplyContentConversions(nsIStreamListener *aNextListener, nsIStreamListener **aNewNextListener, nsISupports *aCtxt) override { return _to DoApplyContentConversions(aNextListener, aNewNextListener, aCtxt); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIENCODEDCHANNEL(_to) \
  NS_IMETHOD GetContentEncodings(nsIUTF8StringEnumerator **aContentEncodings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetContentEncodings(aContentEncodings); } \
  NS_IMETHOD GetApplyConversion(bool *aApplyConversion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetApplyConversion(aApplyConversion); } \
  NS_IMETHOD SetApplyConversion(bool aApplyConversion) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetApplyConversion(aApplyConversion); } \
  NS_IMETHOD DoApplyContentConversions(nsIStreamListener *aNextListener, nsIStreamListener **aNewNextListener, nsISupports *aCtxt) override { return !_to ? NS_ERROR_NULL_POINTER : _to->DoApplyContentConversions(aNextListener, aNewNextListener, aCtxt); } 


#endif /* __gen_nsIEncodedChannel_h__ */
