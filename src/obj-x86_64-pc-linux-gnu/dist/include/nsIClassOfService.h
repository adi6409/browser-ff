/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIClassOfService.idl
 */

#ifndef __gen_nsIClassOfService_h__
#define __gen_nsIClassOfService_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIClassOfService */
#define NS_ICLASSOFSERVICE_IID_STR "1ccb58ec-5e07-4cf9-a30d-ac5490d23b41"

#define NS_ICLASSOFSERVICE_IID \
  {0x1ccb58ec, 0x5e07, 0x4cf9, \
    { 0xa3, 0x0d, 0xac, 0x54, 0x90, 0xd2, 0x3b, 0x41 }}

class NS_NO_VTABLE nsIClassOfService : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ICLASSOFSERVICE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIClassOfService;

  /* attribute unsigned long classFlags; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetClassFlags(uint32_t *aClassFlags) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetClassFlags(uint32_t aClassFlags) = 0;

  /* void clearClassFlags (in unsigned long flags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD ClearClassFlags(uint32_t flags) = 0;

  /* void addClassFlags (in unsigned long flags); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddClassFlags(uint32_t flags) = 0;

  enum {
    Leader = 1U,
    Follower = 2U,
    Speculative = 4U,
    Background = 8U,
    Unblocked = 16U,
    Throttleable = 32U,
    UrgentStart = 64U,
    DontThrottle = 128U,
    Tail = 256U,
    TailAllowed = 512U,
    TailForbidden = 1024U
  };

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIClassOfService, NS_ICLASSOFSERVICE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSICLASSOFSERVICE \
  NS_IMETHOD GetClassFlags(uint32_t *aClassFlags) override; \
  NS_IMETHOD SetClassFlags(uint32_t aClassFlags) override; \
  NS_IMETHOD ClearClassFlags(uint32_t flags) override; \
  NS_IMETHOD AddClassFlags(uint32_t flags) override; \

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSICLASSOFSERVICE \
  nsresult GetClassFlags(uint32_t *aClassFlags); \
  nsresult SetClassFlags(uint32_t aClassFlags); \
  nsresult ClearClassFlags(uint32_t flags); \
  nsresult AddClassFlags(uint32_t flags); \

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSICLASSOFSERVICE(_to) \
  NS_IMETHOD GetClassFlags(uint32_t *aClassFlags) override { return _to GetClassFlags(aClassFlags); } \
  NS_IMETHOD SetClassFlags(uint32_t aClassFlags) override { return _to SetClassFlags(aClassFlags); } \
  NS_IMETHOD ClearClassFlags(uint32_t flags) override { return _to ClearClassFlags(flags); } \
  NS_IMETHOD AddClassFlags(uint32_t flags) override { return _to AddClassFlags(flags); } \

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSICLASSOFSERVICE(_to) \
  NS_IMETHOD GetClassFlags(uint32_t *aClassFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetClassFlags(aClassFlags); } \
  NS_IMETHOD SetClassFlags(uint32_t aClassFlags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetClassFlags(aClassFlags); } \
  NS_IMETHOD ClearClassFlags(uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearClassFlags(flags); } \
  NS_IMETHOD AddClassFlags(uint32_t flags) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddClassFlags(flags); } \


#endif /* __gen_nsIClassOfService_h__ */
