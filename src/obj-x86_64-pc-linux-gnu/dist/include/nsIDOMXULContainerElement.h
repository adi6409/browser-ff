/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULContainerElement.idl
 */

#ifndef __gen_nsIDOMXULContainerElement_h__
#define __gen_nsIDOMXULContainerElement_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIDOMXULContainerElement; /* forward declaration */

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMXULContainerItemElement */
#define NS_IDOMXULCONTAINERITEMELEMENT_IID_STR "800a68c7-b854-4597-a436-3055ce5c5c96"

#define NS_IDOMXULCONTAINERITEMELEMENT_IID \
  {0x800a68c7, 0xb854, 0x4597, \
    { 0xa4, 0x36, 0x30, 0x55, 0xce, 0x5c, 0x5c, 0x96 }}

class NS_NO_VTABLE nsIDOMXULContainerItemElement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULCONTAINERITEMELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULContainerItemElement;

  /* readonly attribute Element parentContainer; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetParentContainer(mozilla::dom::Element **aParentContainer) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULContainerItemElement, NS_IDOMXULCONTAINERITEMELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULCONTAINERITEMELEMENT \
  NS_IMETHOD GetParentContainer(mozilla::dom::Element **aParentContainer) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULCONTAINERITEMELEMENT \
  nsresult GetParentContainer(mozilla::dom::Element **aParentContainer); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULCONTAINERITEMELEMENT(_to) \
  NS_IMETHOD GetParentContainer(mozilla::dom::Element **aParentContainer) override { return _to GetParentContainer(aParentContainer); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULCONTAINERITEMELEMENT(_to) \
  NS_IMETHOD GetParentContainer(mozilla::dom::Element **aParentContainer) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentContainer(aParentContainer); } 


/* starting interface:    nsIDOMXULContainerElement */
#define NS_IDOMXULCONTAINERELEMENT_IID_STR "b2bc96b8-31fc-42f4-937a-bd27291af40b"

#define NS_IDOMXULCONTAINERELEMENT_IID \
  {0xb2bc96b8, 0x31fc, 0x42f4, \
    { 0x93, 0x7a, 0xbd, 0x27, 0x29, 0x1a, 0xf4, 0x0b }}

class NS_NO_VTABLE nsIDOMXULContainerElement : public nsIDOMXULContainerItemElement {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULCONTAINERELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULContainerElement;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULContainerElement, NS_IDOMXULCONTAINERELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULCONTAINERELEMENT \
  /* no methods! */

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULCONTAINERELEMENT \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULCONTAINERELEMENT(_to) \
  /* no methods! */

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULCONTAINERELEMENT(_to) \
  /* no methods! */


#endif /* __gen_nsIDOMXULContainerElement_h__ */
