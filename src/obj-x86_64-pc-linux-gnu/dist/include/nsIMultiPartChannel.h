/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMultiPartChannel.idl
 */

#ifndef __gen_nsIMultiPartChannel_h__
#define __gen_nsIMultiPartChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */


/* starting interface:    nsIMultiPartChannel */
#define NS_IMULTIPARTCHANNEL_IID_STR "4fefb490-5567-11e5-a837-0800200c9a66"

#define NS_IMULTIPARTCHANNEL_IID \
  {0x4fefb490, 0x5567, 0x11e5, \
    { 0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66 }}

class NS_NO_VTABLE nsIMultiPartChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMULTIPARTCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMultiPartChannel;

  /* readonly attribute nsIChannel baseChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetBaseChannel(nsIChannel **aBaseChannel) = 0;

  /* readonly attribute uint32_t partID; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPartID(uint32_t *aPartID) = 0;

  /* readonly attribute boolean isLastPart; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetIsLastPart(bool *aIsLastPart) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMultiPartChannel, NS_IMULTIPARTCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMULTIPARTCHANNEL \
  NS_IMETHOD GetBaseChannel(nsIChannel **aBaseChannel) override; \
  NS_IMETHOD GetPartID(uint32_t *aPartID) override; \
  NS_IMETHOD GetIsLastPart(bool *aIsLastPart) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMULTIPARTCHANNEL \
  nsresult GetBaseChannel(nsIChannel **aBaseChannel); \
  nsresult GetPartID(uint32_t *aPartID); \
  nsresult GetIsLastPart(bool *aIsLastPart); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMULTIPARTCHANNEL(_to) \
  NS_IMETHOD GetBaseChannel(nsIChannel **aBaseChannel) override { return _to GetBaseChannel(aBaseChannel); } \
  NS_IMETHOD GetPartID(uint32_t *aPartID) override { return _to GetPartID(aPartID); } \
  NS_IMETHOD GetIsLastPart(bool *aIsLastPart) override { return _to GetIsLastPart(aIsLastPart); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMULTIPARTCHANNEL(_to) \
  NS_IMETHOD GetBaseChannel(nsIChannel **aBaseChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBaseChannel(aBaseChannel); } \
  NS_IMETHOD GetPartID(uint32_t *aPartID) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPartID(aPartID); } \
  NS_IMETHOD GetIsLastPart(bool *aIsLastPart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsLastPart(aIsLastPart); } 


/* starting interface:    nsIMultiPartChannelListener */
#define NS_IMULTIPARTCHANNELLISTENER_IID_STR "b084959a-4fb9-41a5-88a0-d0f045ce75cf"

#define NS_IMULTIPARTCHANNELLISTENER_IID \
  {0xb084959a, 0x4fb9, 0x41a5, \
    { 0x88, 0xa0, 0xd0, 0xf0, 0x45, 0xce, 0x75, 0xcf }}

class NS_NO_VTABLE nsIMultiPartChannelListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IMULTIPARTCHANNELLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIMultiPartChannelListener;

  /* void onAfterLastPart (in nsresult status); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnAfterLastPart(nsresult status) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIMultiPartChannelListener, NS_IMULTIPARTCHANNELLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIMULTIPARTCHANNELLISTENER \
  NS_IMETHOD OnAfterLastPart(nsresult status) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIMULTIPARTCHANNELLISTENER \
  nsresult OnAfterLastPart(nsresult status); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIMULTIPARTCHANNELLISTENER(_to) \
  NS_IMETHOD OnAfterLastPart(nsresult status) override { return _to OnAfterLastPart(status); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIMULTIPARTCHANNELLISTENER(_to) \
  NS_IMETHOD OnAfterLastPart(nsresult status) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnAfterLastPart(status); } 


#endif /* __gen_nsIMultiPartChannel_h__ */
