/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSResolverInfo.idl
 */

#ifndef __gen_nsIDNSResolverInfo_h__
#define __gen_nsIDNSResolverInfo_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDNSResolverInfo */
#define NS_IDNSRESOLVERINFO_IID_STR "74db2955-6298-4d82-a3b9-7f9e8ba9e854"

#define NS_IDNSRESOLVERINFO_IID \
  {0x74db2955, 0x6298, 0x4d82, \
    { 0xa3, 0xb9, 0x7f, 0x9e, 0x8b, 0xa9, 0xe8, 0x54 }}

class NS_NO_VTABLE nsIDNSResolverInfo : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDNSRESOLVERINFO_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDNSResolverInfo;

  /* readonly attribute ACString URL; */
  NS_IMETHOD GetURL(nsACString& aURL) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDNSResolverInfo, NS_IDNSRESOLVERINFO_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDNSRESOLVERINFO \
  NS_IMETHOD GetURL(nsACString& aURL) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDNSRESOLVERINFO \
  nsresult GetURL(nsACString& aURL); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDNSRESOLVERINFO(_to) \
  NS_IMETHOD GetURL(nsACString& aURL) override { return _to GetURL(aURL); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDNSRESOLVERINFO(_to) \
  NS_IMETHOD GetURL(nsACString& aURL) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetURL(aURL); } 


#endif /* __gen_nsIDNSResolverInfo_h__ */
