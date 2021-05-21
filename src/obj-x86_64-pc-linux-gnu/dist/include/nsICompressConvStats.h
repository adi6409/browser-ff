/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/converters/nsICompressConvStats.idl
 */

#ifndef __gen_nsICompressConvStats_h__
#define __gen_nsICompressConvStats_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsICompressConvStats */
#define NS_ICOMPRESSCONVSTATS_IID_STR "58172ad0-46a9-4893-8fde-cd909c10792a"

#define NS_ICOMPRESSCONVSTATS_IID \
  {0x58172ad0, 0x46a9, 0x4893, \
    { 0x8f, 0xde, 0xcd, 0x90, 0x9c, 0x10, 0x79, 0x2a }}

class NS_NO_VTABLE nsICompressConvStats : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICOMPRESSCONVSTATS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICompressConvStats;

  /* readonly attribute uint64_t decodedDataLength; */
  NS_IMETHOD GetDecodedDataLength(uint64_t *aDecodedDataLength) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICompressConvStats, NS_ICOMPRESSCONVSTATS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICOMPRESSCONVSTATS \
  NS_IMETHOD GetDecodedDataLength(uint64_t *aDecodedDataLength) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICOMPRESSCONVSTATS \
  nsresult GetDecodedDataLength(uint64_t *aDecodedDataLength); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICOMPRESSCONVSTATS(_to) \
  NS_IMETHOD GetDecodedDataLength(uint64_t *aDecodedDataLength) override { return _to GetDecodedDataLength(aDecodedDataLength); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICOMPRESSCONVSTATS(_to) \
  NS_IMETHOD GetDecodedDataLength(uint64_t *aDecodedDataLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetDecodedDataLength(aDecodedDataLength); } 


#endif /* __gen_nsICompressConvStats_h__ */
