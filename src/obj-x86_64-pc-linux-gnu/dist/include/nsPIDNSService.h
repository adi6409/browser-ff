/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsPIDNSService.idl
 */

#ifndef __gen_nsPIDNSService_h__
#define __gen_nsPIDNSService_h__


#ifndef __gen_nsIDNSService_h__
#include "nsIDNSService.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsPIDNSService */
#define NS_PIDNSSERVICE_IID_STR "24e598fd-7b1a-436c-9154-14d8b38df8a5"

#define NS_PIDNSSERVICE_IID \
  {0x24e598fd, 0x7b1a, 0x436c, \
    { 0x91, 0x54, 0x14, 0xd8, 0xb3, 0x8d, 0xf8, 0xa5 }}

class NS_NO_VTABLE nsPIDNSService : public nsIDNSService {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_PIDNSSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsPIDNSService;

  /* void init (); */
  NS_IMETHOD Init(void) = 0;

  /* void shutdown (); */
  NS_IMETHOD Shutdown(void) = 0;

  /* attribute boolean prefetchEnabled; */
  NS_IMETHOD GetPrefetchEnabled(bool *aPrefetchEnabled) = 0;
  NS_IMETHOD SetPrefetchEnabled(bool aPrefetchEnabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsPIDNSService, NS_PIDNSSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSPIDNSSERVICE \
  NS_IMETHOD Init(void) override; \
  NS_IMETHOD Shutdown(void) override; \
  NS_IMETHOD GetPrefetchEnabled(bool *aPrefetchEnabled) override; \
  NS_IMETHOD SetPrefetchEnabled(bool aPrefetchEnabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSPIDNSSERVICE \
  nsresult Init(void); \
  nsresult Shutdown(void); \
  nsresult GetPrefetchEnabled(bool *aPrefetchEnabled); \
  nsresult SetPrefetchEnabled(bool aPrefetchEnabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSPIDNSSERVICE(_to) \
  NS_IMETHOD Init(void) override { return _to Init(); } \
  NS_IMETHOD Shutdown(void) override { return _to Shutdown(); } \
  NS_IMETHOD GetPrefetchEnabled(bool *aPrefetchEnabled) override { return _to GetPrefetchEnabled(aPrefetchEnabled); } \
  NS_IMETHOD SetPrefetchEnabled(bool aPrefetchEnabled) override { return _to SetPrefetchEnabled(aPrefetchEnabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSPIDNSSERVICE(_to) \
  NS_IMETHOD Init(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Init(); } \
  NS_IMETHOD Shutdown(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Shutdown(); } \
  NS_IMETHOD GetPrefetchEnabled(bool *aPrefetchEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPrefetchEnabled(aPrefetchEnabled); } \
  NS_IMETHOD SetPrefetchEnabled(bool aPrefetchEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPrefetchEnabled(aPrefetchEnabled); } 


#endif /* __gen_nsPIDNSService_h__ */
