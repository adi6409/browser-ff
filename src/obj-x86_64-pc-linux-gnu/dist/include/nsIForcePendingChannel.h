/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIForcePendingChannel.idl
 */

#ifndef __gen_nsIForcePendingChannel_h__
#define __gen_nsIForcePendingChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIForcePendingChannel */
#define NS_IFORCEPENDINGCHANNEL_IID_STR "2ac3e1ca-049f-44c3-a519-f0681f51e9b1"

#define NS_IFORCEPENDINGCHANNEL_IID \
  {0x2ac3e1ca, 0x049f, 0x44c3, \
    { 0xa5, 0x19, 0xf0, 0x68, 0x1f, 0x51, 0xe9, 0xb1 }}

class NS_NO_VTABLE nsIForcePendingChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORCEPENDINGCHANNEL_IID)

  /* void forcePending (in boolean aForcePending); */
  NS_IMETHOD ForcePending(bool aForcePending) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIForcePendingChannel, NS_IFORCEPENDINGCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORCEPENDINGCHANNEL \
  NS_IMETHOD ForcePending(bool aForcePending) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORCEPENDINGCHANNEL \
  nsresult ForcePending(bool aForcePending); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORCEPENDINGCHANNEL(_to) \
  NS_IMETHOD ForcePending(bool aForcePending) override { return _to ForcePending(aForcePending); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORCEPENDINGCHANNEL(_to) \
  NS_IMETHOD ForcePending(bool aForcePending) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ForcePending(aForcePending); } 


#endif /* __gen_nsIForcePendingChannel_h__ */
