/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsINativeDNSResolverOverride.idl
 */

#ifndef __gen_nsINativeDNSResolverOverride_h__
#define __gen_nsINativeDNSResolverOverride_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsINativeDNSResolverOverride */
#define NS_INATIVEDNSRESOLVEROVERRIDE_IID_STR "8e38d536-5501-48c0-a412-6c450040c8c8"

#define NS_INATIVEDNSRESOLVEROVERRIDE_IID \
  {0x8e38d536, 0x5501, 0x48c0, \
    { 0xa4, 0x12, 0x6c, 0x45, 0x00, 0x40, 0xc8, 0xc8 }}

class NS_NO_VTABLE nsINativeDNSResolverOverride : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_INATIVEDNSRESOLVEROVERRIDE_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsINativeDNSResolverOverride;

  /* void addIPOverride (in AUTF8String aHost, in ACString aIPLiteral); */
  NS_IMETHOD AddIPOverride(const nsACString& aHost, const nsACString& aIPLiteral) = 0;

  /* void setCnameOverride (in AUTF8String aHost, in ACString aCNAME); */
  NS_IMETHOD SetCnameOverride(const nsACString& aHost, const nsACString& aCNAME) = 0;

  /* void clearHostOverride (in AUTF8String aHost); */
  NS_IMETHOD ClearHostOverride(const nsACString& aHost) = 0;

  /* void clearOverrides (); */
  NS_IMETHOD ClearOverrides(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsINativeDNSResolverOverride, NS_INATIVEDNSRESOLVEROVERRIDE_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSINATIVEDNSRESOLVEROVERRIDE \
  NS_IMETHOD AddIPOverride(const nsACString& aHost, const nsACString& aIPLiteral) override; \
  NS_IMETHOD SetCnameOverride(const nsACString& aHost, const nsACString& aCNAME) override; \
  NS_IMETHOD ClearHostOverride(const nsACString& aHost) override; \
  NS_IMETHOD ClearOverrides(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSINATIVEDNSRESOLVEROVERRIDE \
  nsresult AddIPOverride(const nsACString& aHost, const nsACString& aIPLiteral); \
  nsresult SetCnameOverride(const nsACString& aHost, const nsACString& aCNAME); \
  nsresult ClearHostOverride(const nsACString& aHost); \
  nsresult ClearOverrides(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSINATIVEDNSRESOLVEROVERRIDE(_to) \
  NS_IMETHOD AddIPOverride(const nsACString& aHost, const nsACString& aIPLiteral) override { return _to AddIPOverride(aHost, aIPLiteral); } \
  NS_IMETHOD SetCnameOverride(const nsACString& aHost, const nsACString& aCNAME) override { return _to SetCnameOverride(aHost, aCNAME); } \
  NS_IMETHOD ClearHostOverride(const nsACString& aHost) override { return _to ClearHostOverride(aHost); } \
  NS_IMETHOD ClearOverrides(void) override { return _to ClearOverrides(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSINATIVEDNSRESOLVEROVERRIDE(_to) \
  NS_IMETHOD AddIPOverride(const nsACString& aHost, const nsACString& aIPLiteral) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddIPOverride(aHost, aIPLiteral); } \
  NS_IMETHOD SetCnameOverride(const nsACString& aHost, const nsACString& aCNAME) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetCnameOverride(aHost, aCNAME); } \
  NS_IMETHOD ClearHostOverride(const nsACString& aHost) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearHostOverride(aHost); } \
  NS_IMETHOD ClearOverrides(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->ClearOverrides(); } 


#endif /* __gen_nsINativeDNSResolverOverride_h__ */
