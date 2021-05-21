/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/events/nsIDOMEventListener.idl
 */

#ifndef __gen_nsIDOMEventListener_h__
#define __gen_nsIDOMEventListener_h__


#ifndef __gen_domstubs_h__
#include "domstubs.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
namespace mozilla {
namespace dom {
class Event; /* webidl Event */
} // namespace dom
} // namespace mozilla


/* starting interface:    nsIDOMEventListener */
#define NS_IDOMEVENTLISTENER_IID_STR "df31c120-ded6-11d1-bd85-00805f8ae3f4"

#define NS_IDOMEVENTLISTENER_IID \
  {0xdf31c120, 0xded6, 0x11d1, \
    { 0xbd, 0x85, 0x00, 0x80, 0x5f, 0x8a, 0xe3, 0xf4 }}

class NS_NO_VTABLE nsIDOMEventListener : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IDOMEVENTLISTENER_IID)

  /* [can_run_script] void handleEvent (in Event event); */
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(mozilla::dom::Event *event) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIDOMEventListener, NS_IDOMEVENTLISTENER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIDOMEVENTLISTENER \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(mozilla::dom::Event *event) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIDOMEVENTLISTENER \
  MOZ_CAN_RUN_SCRIPT nsresult HandleEvent(mozilla::dom::Event *event); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIDOMEVENTLISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(mozilla::dom::Event *event) override { return _to HandleEvent(event); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIDOMEVENTLISTENER(_to) \
  MOZ_CAN_RUN_SCRIPT NS_IMETHOD HandleEvent(mozilla::dom::Event *event) override { return !_to ? NS_ERROR_NULL_POINTER : _to->HandleEvent(event); } 


#endif /* __gen_nsIDOMEventListener_h__ */
