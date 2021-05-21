/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISystemStatusBar.idl
 */

#ifndef __gen_nsISystemStatusBar_h__
#define __gen_nsISystemStatusBar_h__


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


/* starting interface:    nsISystemStatusBar */
#define NS_ISYSTEMSTATUSBAR_IID_STR "24493180-ee81-4b7c-8b17-9e69480b7b8a"

#define NS_ISYSTEMSTATUSBAR_IID \
  {0x24493180, 0xee81, 0x4b7c, \
    { 0x8b, 0x17, 0x9e, 0x69, 0x48, 0x0b, 0x7b, 0x8a }}

class NS_NO_VTABLE nsISystemStatusBar : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ISYSTEMSTATUSBAR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsISystemStatusBar;

  /* void addItem (in Element aMenuElement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD AddItem(mozilla::dom::Element *aMenuElement) = 0;

  /* void removeItem (in Element aMenuElement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD RemoveItem(mozilla::dom::Element *aMenuElement) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsISystemStatusBar, NS_ISYSTEMSTATUSBAR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSISYSTEMSTATUSBAR \
  NS_IMETHOD AddItem(mozilla::dom::Element *aMenuElement) override; \
  NS_IMETHOD RemoveItem(mozilla::dom::Element *aMenuElement) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSISYSTEMSTATUSBAR \
  nsresult AddItem(mozilla::dom::Element *aMenuElement); \
  nsresult RemoveItem(mozilla::dom::Element *aMenuElement); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSISYSTEMSTATUSBAR(_to) \
  NS_IMETHOD AddItem(mozilla::dom::Element *aMenuElement) override { return _to AddItem(aMenuElement); } \
  NS_IMETHOD RemoveItem(mozilla::dom::Element *aMenuElement) override { return _to RemoveItem(aMenuElement); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSISYSTEMSTATUSBAR(_to) \
  NS_IMETHOD AddItem(mozilla::dom::Element *aMenuElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->AddItem(aMenuElement); } \
  NS_IMETHOD RemoveItem(mozilla::dom::Element *aMenuElement) override { return !_to ? NS_ERROR_NULL_POINTER : _to->RemoveItem(aMenuElement); } 


#endif /* __gen_nsISystemStatusBar_h__ */
