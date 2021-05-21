/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/html/nsIFormSubmitObserver.idl
 */

#ifndef __gen_nsIFormSubmitObserver_h__
#define __gen_nsIFormSubmitObserver_h__


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
class mozIDOMWindow; /* forward declaration */

class nsIURI; /* forward declaration */

class nsIArray; /* forward declaration */

namespace mozilla {
namespace dom {
class HTMLFormElement; /* webidl HTMLFormElement */
} // namespace dom
} // namespace mozilla

namespace mozilla {
namespace dom {
class Element; /* webidl Element */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIFormSubmitObserver */
#define NS_IFORMSUBMITOBSERVER_IID_STR "867cb7e7-835d-408b-9788-d2834d284e03"

#define NS_IFORMSUBMITOBSERVER_IID \
  {0x867cb7e7, 0x835d, 0x408b, \
    { 0x97, 0x88, 0xd2, 0x83, 0x4d, 0x28, 0x4e, 0x03 }}

class NS_NO_VTABLE nsIFormSubmitObserver : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IFORMSUBMITOBSERVER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIFormSubmitObserver;

  /* void notifyInvalidSubmit (in HTMLFormElement formNode, in Array<Element> invalidElements); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD NotifyInvalidSubmit(mozilla::dom::HTMLFormElement *formNode, const nsTArray<RefPtr<mozilla::dom::Element>>& invalidElements) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIFormSubmitObserver, NS_IFORMSUBMITOBSERVER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIFORMSUBMITOBSERVER \
  NS_IMETHOD NotifyInvalidSubmit(mozilla::dom::HTMLFormElement *formNode, const nsTArray<RefPtr<mozilla::dom::Element>>& invalidElements) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIFORMSUBMITOBSERVER \
  nsresult NotifyInvalidSubmit(mozilla::dom::HTMLFormElement *formNode, const nsTArray<RefPtr<mozilla::dom::Element>>& invalidElements); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIFORMSUBMITOBSERVER(_to) \
  NS_IMETHOD NotifyInvalidSubmit(mozilla::dom::HTMLFormElement *formNode, const nsTArray<RefPtr<mozilla::dom::Element>>& invalidElements) override { return _to NotifyInvalidSubmit(formNode, invalidElements); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIFORMSUBMITOBSERVER(_to) \
  NS_IMETHOD NotifyInvalidSubmit(mozilla::dom::HTMLFormElement *formNode, const nsTArray<RefPtr<mozilla::dom::Element>>& invalidElements) override { return !_to ? NS_ERROR_NULL_POINTER : _to->NotifyInvalidSubmit(formNode, invalidElements); } 

#define NS_INVALIDFORMSUBMIT_SUBJECT "invalidformsubmit"

#endif /* __gen_nsIFormSubmitObserver_h__ */
