/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/xul/nsIBrowserController.idl
 */

#ifndef __gen_nsIBrowserController_h__
#define __gen_nsIBrowserController_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIBrowserController */
#define NS_IBROWSERCONTROLLER_IID_STR "5bb3d56b-e733-4a2c-8a53-058123df65e2"

#define NS_IBROWSERCONTROLLER_IID \
  {0x5bb3d56b, 0xe733, 0x4a2c, \
    { 0x8a, 0x53, 0x05, 0x81, 0x23, 0xdf, 0x65, 0xe2 }}

class NS_NO_VTABLE nsIBrowserController : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IBROWSERCONTROLLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIBrowserController;

  /* void enableDisableCommands (in AString action, in Array<ACString> enabledCommands, in Array<ACString> disabledCommands); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD EnableDisableCommands(const nsAString& action, const nsTArray<nsCString >& enabledCommands, const nsTArray<nsCString >& disabledCommands) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIBrowserController, NS_IBROWSERCONTROLLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIBROWSERCONTROLLER \
  NS_IMETHOD EnableDisableCommands(const nsAString& action, const nsTArray<nsCString >& enabledCommands, const nsTArray<nsCString >& disabledCommands) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIBROWSERCONTROLLER \
  nsresult EnableDisableCommands(const nsAString& action, const nsTArray<nsCString >& enabledCommands, const nsTArray<nsCString >& disabledCommands); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIBROWSERCONTROLLER(_to) \
  NS_IMETHOD EnableDisableCommands(const nsAString& action, const nsTArray<nsCString >& enabledCommands, const nsTArray<nsCString >& disabledCommands) override { return _to EnableDisableCommands(action, enabledCommands, disabledCommands); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIBROWSERCONTROLLER(_to) \
  NS_IMETHOD EnableDisableCommands(const nsAString& action, const nsTArray<nsCString >& enabledCommands, const nsTArray<nsCString >& disabledCommands) override { return !_to ? NS_ERROR_NULL_POINTER : _to->EnableDisableCommands(action, enabledCommands, disabledCommands); } 


#endif /* __gen_nsIBrowserController_h__ */
