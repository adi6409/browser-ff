/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/glean/xpcom/nsIFOG.idl
 */

#ifndef __gen_nsIFOG_h__
#define __gen_nsIFOG_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIFOG */
#define NS_IFOG_IID_STR "98d0e975-9cad-4ce3-ae2f-f878b8be6307"

#define NS_IFOG_IID \
  {0x98d0e975, 0x9cad, 0x4ce3, \
    { 0xae, 0x2f, 0xf8, 0x78, 0xb8, 0xbe, 0x63, 0x07 }}

class NS_NO_VTABLE nsIFOG : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFOG_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFOG;

  /* void initializeFOG (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD InitializeFOG(void) = 0;

  /* void setLogPings (in boolean aEnableLogPings); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetLogPings(bool aEnableLogPings) = 0;

  /* void setTagPings (in ACString aDebugTag); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetTagPings(const nsACString& aDebugTag) = 0;

  /* void sendPing (in ACString aPingName); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SendPing(const nsACString& aPingName) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFOG, NS_IFOG_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFOG \
  NS_IMETHOD InitializeFOG(void) override; \
  NS_IMETHOD SetLogPings(bool aEnableLogPings) override; \
  NS_IMETHOD SetTagPings(const nsACString& aDebugTag) override; \
  NS_IMETHOD SendPing(const nsACString& aPingName) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFOG \
  nsresult InitializeFOG(void); \
  nsresult SetLogPings(bool aEnableLogPings); \
  nsresult SetTagPings(const nsACString& aDebugTag); \
  nsresult SendPing(const nsACString& aPingName); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFOG(_to) \
  NS_IMETHOD InitializeFOG(void) override { return _to InitializeFOG(); } \
  NS_IMETHOD SetLogPings(bool aEnableLogPings) override { return _to SetLogPings(aEnableLogPings); } \
  NS_IMETHOD SetTagPings(const nsACString& aDebugTag) override { return _to SetTagPings(aDebugTag); } \
  NS_IMETHOD SendPing(const nsACString& aPingName) override { return _to SendPing(aPingName); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFOG(_to) \
  NS_IMETHOD InitializeFOG(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->InitializeFOG(); } \
  NS_IMETHOD SetLogPings(bool aEnableLogPings) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetLogPings(aEnableLogPings); } \
  NS_IMETHOD SetTagPings(const nsACString& aDebugTag) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetTagPings(aDebugTag); } \
  NS_IMETHOD SendPing(const nsACString& aPingName) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SendPing(aPingName); } 


#endif /* __gen_nsIFOG_h__ */
