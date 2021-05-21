/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheTesting.idl
 */

#ifndef __gen_nsICacheTesting_h__
#define __gen_nsICacheTesting_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIObserver; /* forward declaration */


/* starting interface:    nsICacheTesting */
#define NS_ICACHETESTING_IID_STR "4e8ba935-92e1-4a74-944b-b1a2f02a7480"

#define NS_ICACHETESTING_IID \
  {0x4e8ba935, 0x92e1, 0x4a74, \
    { 0x94, 0x4b, 0xb1, 0xa2, 0xf0, 0x2a, 0x74, 0x80 }}

class NS_NO_VTABLE nsICacheTesting : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICACHETESTING_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsICacheTesting;

  /* void suspendCacheIOThread (in uint32_t aLevel); */
  NS_IMETHOD SuspendCacheIOThread(uint32_t aLevel) = 0;

  /* void resumeCacheIOThread (); */
  NS_IMETHOD ResumeCacheIOThread(void) = 0;

  /* void flush (in nsIObserver aObserver); */
  NS_IMETHOD Flush(nsIObserver *aObserver) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsICacheTesting, NS_ICACHETESTING_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICACHETESTING \
  NS_IMETHOD SuspendCacheIOThread(uint32_t aLevel) override; \
  NS_IMETHOD ResumeCacheIOThread(void) override; \
  NS_IMETHOD Flush(nsIObserver *aObserver) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICACHETESTING \
  nsresult SuspendCacheIOThread(uint32_t aLevel); \
  nsresult ResumeCacheIOThread(void); \
  nsresult Flush(nsIObserver *aObserver); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICACHETESTING(_to) \
  NS_IMETHOD SuspendCacheIOThread(uint32_t aLevel) override { return _to SuspendCacheIOThread(aLevel); } \
  NS_IMETHOD ResumeCacheIOThread(void) override { return _to ResumeCacheIOThread(); } \
  NS_IMETHOD Flush(nsIObserver *aObserver) override { return _to Flush(aObserver); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICACHETESTING(_to) \
  NS_IMETHOD SuspendCacheIOThread(uint32_t aLevel) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SuspendCacheIOThread(aLevel); } \
  NS_IMETHOD ResumeCacheIOThread(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ResumeCacheIOThread(); } \
  NS_IMETHOD Flush(nsIObserver *aObserver) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Flush(aObserver); } 


#endif /* __gen_nsICacheTesting_h__ */
