/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannelParentInternal.idl
 */

#ifndef __gen_nsIFTPChannelParentInternal_h__
#define __gen_nsIFTPChannelParentInternal_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFTPChannelParentInternal */
#define NS_IFTPCHANNELPARENTINTERNAL_IID_STR "87b58410-83cb-42a7-b57b-27c07ef828d7"

#define NS_IFTPCHANNELPARENTINTERNAL_IID \
  {0x87b58410, 0x83cb, 0x42a7, \
    { 0xb5, 0x7b, 0x27, 0xc0, 0x7e, 0xf8, 0x28, 0xd7 }}

class NS_NO_VTABLE nsIFTPChannelParentInternal : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFTPCHANNELPARENTINTERNAL_IID)

  /* void setErrorMsg (in string msg, in boolean useUTF8); */
  NS_IMETHOD SetErrorMsg(const char * msg, bool useUTF8) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFTPChannelParentInternal, NS_IFTPCHANNELPARENTINTERNAL_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFTPCHANNELPARENTINTERNAL \
  NS_IMETHOD SetErrorMsg(const char * msg, bool useUTF8) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFTPCHANNELPARENTINTERNAL \
  nsresult SetErrorMsg(const char * msg, bool useUTF8); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFTPCHANNELPARENTINTERNAL(_to) \
  NS_IMETHOD SetErrorMsg(const char * msg, bool useUTF8) override { return _to SetErrorMsg(msg, useUTF8); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFTPCHANNELPARENTINTERNAL(_to) \
  NS_IMETHOD SetErrorMsg(const char * msg, bool useUTF8) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetErrorMsg(msg, useUTF8); } 


#endif /* __gen_nsIFTPChannelParentInternal_h__ */
