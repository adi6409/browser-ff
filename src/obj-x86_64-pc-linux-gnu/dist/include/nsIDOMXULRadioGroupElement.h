/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRadioGroupElement.idl
 */

#ifndef __gen_nsIDOMXULRadioGroupElement_h__
#define __gen_nsIDOMXULRadioGroupElement_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULRadioGroupElement */
#define NS_IDOMXULRADIOGROUPELEMENT_IID_STR "2cc1d24b-ec9f-4e18-aa34-a298a9007f23"

#define NS_IDOMXULRADIOGROUPELEMENT_IID \
  {0x2cc1d24b, 0xec9f, 0x4e18, \
    { 0xaa, 0x34, 0xa2, 0x98, 0xa9, 0x00, 0x7f, 0x23 }}

class NS_NO_VTABLE nsIDOMXULRadioGroupElement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULRADIOGROUPELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULRadioGroupElement;

  /* attribute Element focusedItem; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetFocusedItem(mozilla::dom::Element **aFocusedItem) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetFocusedItem(mozilla::dom::Element *aFocusedItem) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULRadioGroupElement, NS_IDOMXULRADIOGROUPELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULRADIOGROUPELEMENT \
  NS_IMETHOD GetFocusedItem(mozilla::dom::Element **aFocusedItem) override; \
  NS_IMETHOD SetFocusedItem(mozilla::dom::Element *aFocusedItem) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULRADIOGROUPELEMENT \
  nsresult GetFocusedItem(mozilla::dom::Element **aFocusedItem); \
  nsresult SetFocusedItem(mozilla::dom::Element *aFocusedItem); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULRADIOGROUPELEMENT(_to) \
  NS_IMETHOD GetFocusedItem(mozilla::dom::Element **aFocusedItem) override { return _to GetFocusedItem(aFocusedItem); } \
  NS_IMETHOD SetFocusedItem(mozilla::dom::Element *aFocusedItem) override { return _to SetFocusedItem(aFocusedItem); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULRADIOGROUPELEMENT(_to) \
  NS_IMETHOD GetFocusedItem(mozilla::dom::Element **aFocusedItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetFocusedItem(aFocusedItem); } \
  NS_IMETHOD SetFocusedItem(mozilla::dom::Element *aFocusedItem) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetFocusedItem(aFocusedItem); } 


#endif /* __gen_nsIDOMXULRadioGroupElement_h__ */
