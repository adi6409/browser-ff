/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipListener.idl
 */

#ifndef __gen_nsITooltipListener_h__
#define __gen_nsITooltipListener_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsITooltipListener */
#define NS_ITOOLTIPLISTENER_IID_STR "44b78386-1dd2-11b2-9ad2-e4eee2ca1916"

#define NS_ITOOLTIPLISTENER_IID \
  {0x44b78386, 0x1dd2, 0x11b2, \
    { 0x9a, 0xd2, 0xe4, 0xee, 0xe2, 0xca, 0x19, 0x16 }}

class NS_NO_VTABLE nsITooltipListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ITOOLTIPLISTENER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsITooltipListener;

  /* void onShowTooltip (in long aXCoords, in long aYCoords, in AString aTipText, in AString aTipDir); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnShowTooltip(int32_t aXCoords, int32_t aYCoords, const nsAString& aTipText, const nsAString& aTipDir) = 0;

  /* void onHideTooltip (); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD OnHideTooltip(void) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsITooltipListener, NS_ITOOLTIPLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSITOOLTIPLISTENER \
  NS_IMETHOD OnShowTooltip(int32_t aXCoords, int32_t aYCoords, const nsAString& aTipText, const nsAString& aTipDir) override; \
  NS_IMETHOD OnHideTooltip(void) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSITOOLTIPLISTENER \
  nsresult OnShowTooltip(int32_t aXCoords, int32_t aYCoords, const nsAString& aTipText, const nsAString& aTipDir); \
  nsresult OnHideTooltip(void); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSITOOLTIPLISTENER(_to) \
  NS_IMETHOD OnShowTooltip(int32_t aXCoords, int32_t aYCoords, const nsAString& aTipText, const nsAString& aTipDir) override { return _to OnShowTooltip(aXCoords, aYCoords, aTipText, aTipDir); } \
  NS_IMETHOD OnHideTooltip(void) override { return _to OnHideTooltip(); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSITOOLTIPLISTENER(_to) \
  NS_IMETHOD OnShowTooltip(int32_t aXCoords, int32_t aYCoords, const nsAString& aTipText, const nsAString& aTipDir) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnShowTooltip(aXCoords, aYCoords, aTipText, aTipDir); } \
  NS_IMETHOD OnHideTooltip(void) override { return !_to ? NS_ERROR_NULL_POINTER : _to->OnHideTooltip(); } 


#endif /* __gen_nsITooltipListener_h__ */
