/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecCheckWrapChannel.idl
 */

#ifndef __gen_nsISecCheckWrapChannel_h__
#define __gen_nsISecCheckWrapChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIChannel; /* forward declaration */


/* starting interface:    nsISecCheckWrapChannel */
#define NS_ISECCHECKWRAPCHANNEL_IID_STR "9446c5d5-c9fb-4a6e-acf9-ca4fc666efe0"

#define NS_ISECCHECKWRAPCHANNEL_IID \
  {0x9446c5d5, 0xc9fb, 0x4a6e, \
    { 0xac, 0xf9, 0xca, 0x4f, 0xc6, 0x66, 0xef, 0xe0 }}

class NS_NO_VTABLE nsISecCheckWrapChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISECCHECKWRAPCHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISecCheckWrapChannel;

  /* readonly attribute nsIChannel innerChannel; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetInnerChannel(nsIChannel **aInnerChannel) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISecCheckWrapChannel, NS_ISECCHECKWRAPCHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISECCHECKWRAPCHANNEL \
  NS_IMETHOD GetInnerChannel(nsIChannel **aInnerChannel) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISECCHECKWRAPCHANNEL \
  nsresult GetInnerChannel(nsIChannel **aInnerChannel); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISECCHECKWRAPCHANNEL(_to) \
  NS_IMETHOD GetInnerChannel(nsIChannel **aInnerChannel) override { return _to GetInnerChannel(aInnerChannel); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISECCHECKWRAPCHANNEL(_to) \
  NS_IMETHOD GetInnerChannel(nsIChannel **aInnerChannel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetInnerChannel(aInnerChannel); } 


#endif /* __gen_nsISecCheckWrapChannel_h__ */
