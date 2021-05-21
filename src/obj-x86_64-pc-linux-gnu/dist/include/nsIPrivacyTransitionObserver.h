/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIPrivacyTransitionObserver.idl
 */

#ifndef __gen_nsIPrivacyTransitionObserver_h__
#define __gen_nsIPrivacyTransitionObserver_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIPrivacyTransitionObserver */
#define NS_IPRIVACYTRANSITIONOBSERVER_IID_STR "b4b1449d-0ef0-47f5-b62e-adc57fd49702"

#define NS_IPRIVACYTRANSITIONOBSERVER_IID \
  {0xb4b1449d, 0x0ef0, 0x47f5, \
    { 0xb6, 0x2e, 0xad, 0xc5, 0x7f, 0xd4, 0x97, 0x02 }}

class NS_NO_VTABLE nsIPrivacyTransitionObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPRIVACYTRANSITIONOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPrivacyTransitionObserver;

  /* void privateModeChanged (in bool enabled); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD PrivateModeChanged(bool enabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPrivacyTransitionObserver, NS_IPRIVACYTRANSITIONOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPRIVACYTRANSITIONOBSERVER \
  NS_IMETHOD PrivateModeChanged(bool enabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPRIVACYTRANSITIONOBSERVER \
  nsresult PrivateModeChanged(bool enabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPRIVACYTRANSITIONOBSERVER(_to) \
  NS_IMETHOD PrivateModeChanged(bool enabled) override { return _to PrivateModeChanged(enabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPRIVACYTRANSITIONOBSERVER(_to) \
  NS_IMETHOD PrivateModeChanged(bool enabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->PrivateModeChanged(enabled); } 


#endif /* __gen_nsIPrivacyTransitionObserver_h__ */
