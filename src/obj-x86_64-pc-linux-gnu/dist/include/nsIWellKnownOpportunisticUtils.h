/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIWellKnownOpportunisticUtils.idl
 */

#ifndef __gen_nsIWellKnownOpportunisticUtils_h__
#define __gen_nsIWellKnownOpportunisticUtils_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
#define NS_WELLKNOWNOPPORTUNISTICUTILS_CONTRACTID "@mozilla.org/network/well-known-opportunistic-utils;1"

/* starting interface:    nsIWellKnownOpportunisticUtils */
#define NS_IWELLKNOWNOPPORTUNISTICUTILS_IID_STR "b4f96c89-5238-450c-8bda-e12c26f1d150"

#define NS_IWELLKNOWNOPPORTUNISTICUTILS_IID \
  {0xb4f96c89, 0x5238, 0x450c, \
    { 0x8b, 0xda, 0xe1, 0x2c, 0x26, 0xf1, 0xd1, 0x50 }}

class NS_NO_VTABLE nsIWellKnownOpportunisticUtils : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IWELLKNOWNOPPORTUNISTICUTILS_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIWellKnownOpportunisticUtils;

  /* [must_use] void verify (in ACString aJSON, in ACString aOrigin); */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD Verify(const nsACString& aJSON, const nsACString& aOrigin) = 0;

  /* [must_use] readonly attribute bool valid; */
  JS_HAZ_CAN_RUN_SCRIPT [[nodiscard]] NS_IMETHOD GetValid(bool *aValid) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIWellKnownOpportunisticUtils, NS_IWELLKNOWNOPPORTUNISTICUTILS_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIWELLKNOWNOPPORTUNISTICUTILS \
  [[nodiscard]] NS_IMETHOD Verify(const nsACString& aJSON, const nsACString& aOrigin) override; \
  [[nodiscard]] NS_IMETHOD GetValid(bool *aValid) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIWELLKNOWNOPPORTUNISTICUTILS \
  [[nodiscard]] nsresult Verify(const nsACString& aJSON, const nsACString& aOrigin); \
  [[nodiscard]] nsresult GetValid(bool *aValid); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIWELLKNOWNOPPORTUNISTICUTILS(_to) \
  [[nodiscard]] NS_IMETHOD Verify(const nsACString& aJSON, const nsACString& aOrigin) override { return _to Verify(aJSON, aOrigin); } \
  [[nodiscard]] NS_IMETHOD GetValid(bool *aValid) override { return _to GetValid(aValid); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIWELLKNOWNOPPORTUNISTICUTILS(_to) \
  [[nodiscard]] NS_IMETHOD Verify(const nsACString& aJSON, const nsACString& aOrigin) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Verify(aJSON, aOrigin); } \
  [[nodiscard]] NS_IMETHOD GetValid(bool *aValid) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetValid(aValid); } 


#endif /* __gen_nsIWellKnownOpportunisticUtils_h__ */
