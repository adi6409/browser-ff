/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFormPOSTActionChannel.idl
 */

#ifndef __gen_nsIFormPOSTActionChannel_h__
#define __gen_nsIFormPOSTActionChannel_h__


#ifndef __gen_nsIUploadChannel_h__
#include "nsIUploadChannel.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFormPOSTActionChannel */
#define NS_IFORMPOSTACTIONCHANNEL_IID_STR "fc826b53-0db8-42b4-aa6a-5dd2cfca52a4"

#define NS_IFORMPOSTACTIONCHANNEL_IID \
  {0xfc826b53, 0x0db8, 0x42b4, \
    { 0xaa, 0x6a, 0x5d, 0xd2, 0xcf, 0xca, 0x52, 0xa4 }}

class NS_NO_VTABLE nsIFormPOSTActionChannel : public nsIUploadChannel {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMPOSTACTIONCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormPOSTActionChannel;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormPOSTActionChannel, NS_IFORMPOSTACTIONCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMPOSTACTIONCHANNEL \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMPOSTACTIONCHANNEL \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMPOSTACTIONCHANNEL(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMPOSTACTIONCHANNEL(_to) \
  /* no methods! */


#endif /* __gen_nsIFormPOSTActionChannel_h__ */
