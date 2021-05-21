/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIDOMWakeLockListener.idl
 */

#ifndef __gen_nsIDOMWakeLockListener_h__
#define __gen_nsIDOMWakeLockListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIDOMMozWakeLockListener */
#define NS_IDOMMOZWAKELOCKLISTENER_IID_STR "4e258af8-cffb-47bc-b16d-e8241243426e"

#define NS_IDOMMOZWAKELOCKLISTENER_IID \
  {0x4e258af8, 0xcffb, 0x47bc, \
    { 0xb1, 0x6d, 0xe8, 0x24, 0x12, 0x43, 0x42, 0x6e }}

class NS_NO_VTABLE nsIDOMMozWakeLockListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMMOZWAKELOCKLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMMozWakeLockListener;

  /* void callback (in AString aTopic, in AString aState); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Callback(const nsAString& aTopic, const nsAString& aState) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMMozWakeLockListener, NS_IDOMMOZWAKELOCKLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMMOZWAKELOCKLISTENER \
  NS_IMETHOD Callback(const nsAString& aTopic, const nsAString& aState) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMMOZWAKELOCKLISTENER \
  nsresult Callback(const nsAString& aTopic, const nsAString& aState); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMMOZWAKELOCKLISTENER(_to) \
  NS_IMETHOD Callback(const nsAString& aTopic, const nsAString& aState) override { return _to Callback(aTopic, aState); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMMOZWAKELOCKLISTENER(_to) \
  NS_IMETHOD Callback(const nsAString& aTopic, const nsAString& aState) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Callback(aTopic, aState); } 


#endif /* __gen_nsIDOMWakeLockListener_h__ */
