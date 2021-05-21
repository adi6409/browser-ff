/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleObjectAttributeChangedEvent.idl
 */

#ifndef __gen_nsIAccessibleObjectAttributeChangedEvent_h__
#define __gen_nsIAccessibleObjectAttributeChangedEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleObjectAttributeChangedEvent */
#define NS_IACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT_IID_STR "ce41add2-096e-4606-b1ca-7408c6d5b4c3"

#define NS_IACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT_IID \
  {0xce41add2, 0x096e, 0x4606, \
    { 0xb1, 0xca, 0x74, 0x08, 0xc6, 0xd5, 0xb4, 0xc3 }}

class NS_NO_VTABLE nsIAccessibleObjectAttributeChangedEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleObjectAttributeChangedEvent;

  /* readonly attribute AString changedAttribute; */
  NS_IMETHOD GetChangedAttribute(nsAString& aChangedAttribute) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleObjectAttributeChangedEvent, NS_IACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT \
  NS_IMETHOD GetChangedAttribute(nsAString& aChangedAttribute) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT \
  nsresult GetChangedAttribute(nsAString& aChangedAttribute); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT(_to) \
  NS_IMETHOD GetChangedAttribute(nsAString& aChangedAttribute) override { return _to GetChangedAttribute(aChangedAttribute); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEOBJECTATTRIBUTECHANGEDEVENT(_to) \
  NS_IMETHOD GetChangedAttribute(nsAString& aChangedAttribute) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChangedAttribute(aChangedAttribute); } 


#endif /* __gen_nsIAccessibleObjectAttributeChangedEvent_h__ */
