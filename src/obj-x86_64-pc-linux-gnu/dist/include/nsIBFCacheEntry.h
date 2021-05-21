/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsIBFCacheEntry.idl
 */

#ifndef __gen_nsIBFCacheEntry_h__
#define __gen_nsIBFCacheEntry_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIBFCacheEntry */
#define NS_IBFCACHEENTRY_IID_STR "a576060e-c7df-4d81-aa8c-5b52bd6ad25d"

#define NS_IBFCACHEENTRY_IID \
  {0xa576060e, 0xc7df, 0x4d81, \
    { 0xaa, 0x8c, 0x5b, 0x52, 0xbd, 0x6a, 0xd2, 0x5d }}

class NS_NO_VTABLE nsIBFCacheEntry : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBFCACHEENTRY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBFCacheEntry;

  /* void RemoveFromBFCacheSync (); */
  NS_IMETHOD RemoveFromBFCacheSync(void) = 0;

  /* void RemoveFromBFCacheAsync (); */
  NS_IMETHOD RemoveFromBFCacheAsync(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBFCacheEntry, NS_IBFCACHEENTRY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBFCACHEENTRY \
  NS_IMETHOD RemoveFromBFCacheSync(void) override; \
  NS_IMETHOD RemoveFromBFCacheAsync(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBFCACHEENTRY \
  nsresult RemoveFromBFCacheSync(void); \
  nsresult RemoveFromBFCacheAsync(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBFCACHEENTRY(_to) \
  NS_IMETHOD RemoveFromBFCacheSync(void) override { return _to RemoveFromBFCacheSync(); } \
  NS_IMETHOD RemoveFromBFCacheAsync(void) override { return _to RemoveFromBFCacheAsync(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBFCACHEENTRY(_to) \
  NS_IMETHOD RemoveFromBFCacheSync(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveFromBFCacheSync(); } \
  NS_IMETHOD RemoveFromBFCacheAsync(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveFromBFCacheAsync(); } 


#endif /* __gen_nsIBFCacheEntry_h__ */
