/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleScrollingEvent.idl
 */

#ifndef __gen_nsIAccessibleScrollingEvent_h__
#define __gen_nsIAccessibleScrollingEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleScrollingEvent */
#define NS_IACCESSIBLESCROLLINGEVENT_IID_STR "f75f0b32-5342-4d60-b1a5-b7bd6888eef5"

#define NS_IACCESSIBLESCROLLINGEVENT_IID \
  {0xf75f0b32, 0x5342, 0x4d60, \
    { 0xb1, 0xa5, 0xb7, 0xbd, 0x68, 0x88, 0xee, 0xf5 }}

class NS_NO_VTABLE nsIAccessibleScrollingEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLESCROLLINGEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleScrollingEvent;

  /* readonly attribute unsigned long scrollX; */
  NS_IMETHOD GetScrollX(uint32_t *aScrollX) = 0;

  /* readonly attribute unsigned long scrollY; */
  NS_IMETHOD GetScrollY(uint32_t *aScrollY) = 0;

  /* readonly attribute unsigned long maxScrollX; */
  NS_IMETHOD GetMaxScrollX(uint32_t *aMaxScrollX) = 0;

  /* readonly attribute unsigned long maxScrollY; */
  NS_IMETHOD GetMaxScrollY(uint32_t *aMaxScrollY) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleScrollingEvent, NS_IACCESSIBLESCROLLINGEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLESCROLLINGEVENT \
  NS_IMETHOD GetScrollX(uint32_t *aScrollX) override; \
  NS_IMETHOD GetScrollY(uint32_t *aScrollY) override; \
  NS_IMETHOD GetMaxScrollX(uint32_t *aMaxScrollX) override; \
  NS_IMETHOD GetMaxScrollY(uint32_t *aMaxScrollY) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLESCROLLINGEVENT \
  nsresult GetScrollX(uint32_t *aScrollX); \
  nsresult GetScrollY(uint32_t *aScrollY); \
  nsresult GetMaxScrollX(uint32_t *aMaxScrollX); \
  nsresult GetMaxScrollY(uint32_t *aMaxScrollY); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLESCROLLINGEVENT(_to) \
  NS_IMETHOD GetScrollX(uint32_t *aScrollX) override { return _to GetScrollX(aScrollX); } \
  NS_IMETHOD GetScrollY(uint32_t *aScrollY) override { return _to GetScrollY(aScrollY); } \
  NS_IMETHOD GetMaxScrollX(uint32_t *aMaxScrollX) override { return _to GetMaxScrollX(aMaxScrollX); } \
  NS_IMETHOD GetMaxScrollY(uint32_t *aMaxScrollY) override { return _to GetMaxScrollY(aMaxScrollY); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLESCROLLINGEVENT(_to) \
  NS_IMETHOD GetScrollX(uint32_t *aScrollX) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollX(aScrollX); } \
  NS_IMETHOD GetScrollY(uint32_t *aScrollY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetScrollY(aScrollY); } \
  NS_IMETHOD GetMaxScrollX(uint32_t *aMaxScrollX) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxScrollX(aMaxScrollX); } \
  NS_IMETHOD GetMaxScrollY(uint32_t *aMaxScrollY) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetMaxScrollY(aMaxScrollY); } 


#endif /* __gen_nsIAccessibleScrollingEvent_h__ */
