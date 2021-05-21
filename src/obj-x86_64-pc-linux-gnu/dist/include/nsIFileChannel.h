/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/file/nsIFileChannel.idl
 */

#ifndef __gen_nsIFileChannel_h__
#define __gen_nsIFileChannel_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIFile; /* forward declaration */


/* starting interface:    nsIFileChannel */
#define NS_IFILECHANNEL_IID_STR "06169120-136d-45a5-b535-498f1f755ab7"

#define NS_IFILECHANNEL_IID \
  {0x06169120, 0x136d, 0x45a5, \
    { 0xb5, 0x35, 0x49, 0x8f, 0x1f, 0x75, 0x5a, 0xb7 }}

class NS_NO_VTABLE nsIFileChannel : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFILECHANNEL_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFileChannel;

  /* readonly attribute nsIFile file; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFile(nsIFile **aFile) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFileChannel, NS_IFILECHANNEL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFILECHANNEL \
  NS_IMETHOD GetFile(nsIFile **aFile) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFILECHANNEL \
  nsresult GetFile(nsIFile **aFile); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFILECHANNEL(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return _to GetFile(aFile); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFILECHANNEL(_to) \
  NS_IMETHOD GetFile(nsIFile **aFile) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFile(aFile); } 


#endif /* __gen_nsIFileChannel_h__ */
