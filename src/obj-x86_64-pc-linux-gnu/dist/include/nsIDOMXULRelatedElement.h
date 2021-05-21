/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/xul/nsIDOMXULRelatedElement.idl
 */

#ifndef __gen_nsIDOMXULRelatedElement_h__
#define __gen_nsIDOMXULRelatedElement_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
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

class nsINode; /* webidl Node */


/* starting interface:    nsIDOMXULRelatedElement */
#define NS_IDOMXULRELATEDELEMENT_IID_STR "9fbac05a-fb27-470d-8e5f-028b2dc54ad0"

#define NS_IDOMXULRELATEDELEMENT_IID \
  {0x9fbac05a, 0xfb27, 0x470d, \
    { 0x8e, 0x5f, 0x02, 0x8b, 0x2d, 0xc5, 0x4a, 0xd0 }}

class NS_NO_VTABLE nsIDOMXULRelatedElement : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMXULRELATEDELEMENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIDOMXULRelatedElement;

  /* Element getRelatedElement (in Node aElement); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetRelatedElement(nsINode *aElement, mozilla::dom::Element **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMXULRelatedElement, NS_IDOMXULRELATEDELEMENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMXULRELATEDELEMENT \
  NS_IMETHOD GetRelatedElement(nsINode *aElement, mozilla::dom::Element **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMXULRELATEDELEMENT \
  nsresult GetRelatedElement(nsINode *aElement, mozilla::dom::Element **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMXULRELATEDELEMENT(_to) \
  NS_IMETHOD GetRelatedElement(nsINode *aElement, mozilla::dom::Element **_retval) override { return _to GetRelatedElement(aElement, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMXULRELATEDELEMENT(_to) \
  NS_IMETHOD GetRelatedElement(nsINode *aElement, mozilla::dom::Element **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelatedElement(aElement, _retval); } 


#endif /* __gen_nsIDOMXULRelatedElement_h__ */
