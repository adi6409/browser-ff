/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextSelectionChangeEvent.idl
 */

#ifndef __gen_nsIAccessibleTextSelectionChangeEvent_h__
#define __gen_nsIAccessibleTextSelectionChangeEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIArray; /* forward declaration */


/* starting interface:    nsIAccessibleTextSelectionChangeEvent */
#define NS_IACCESSIBLETEXTSELECTIONCHANGEEVENT_IID_STR "011f98e2-2beb-4ec3-97a5-f154f624e112"

#define NS_IACCESSIBLETEXTSELECTIONCHANGEEVENT_IID \
  {0x011f98e2, 0x2beb, 0x4ec3, \
    { 0x97, 0xa5, 0xf1, 0x54, 0xf6, 0x24, 0xe1, 0x12 }}

class NS_NO_VTABLE nsIAccessibleTextSelectionChangeEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETEXTSELECTIONCHANGEEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTextSelectionChangeEvent;

  /* readonly attribute nsIArray selectionRanges; */
  NS_IMETHOD GetSelectionRanges(nsIArray **aSelectionRanges) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTextSelectionChangeEvent, NS_IACCESSIBLETEXTSELECTIONCHANGEEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETEXTSELECTIONCHANGEEVENT \
  NS_IMETHOD GetSelectionRanges(nsIArray **aSelectionRanges) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETEXTSELECTIONCHANGEEVENT \
  nsresult GetSelectionRanges(nsIArray **aSelectionRanges); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETEXTSELECTIONCHANGEEVENT(_to) \
  NS_IMETHOD GetSelectionRanges(nsIArray **aSelectionRanges) override { return _to GetSelectionRanges(aSelectionRanges); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETEXTSELECTIONCHANGEEVENT(_to) \
  NS_IMETHOD GetSelectionRanges(nsIArray **aSelectionRanges) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetSelectionRanges(aSelectionRanges); } 


#endif /* __gen_nsIAccessibleTextSelectionChangeEvent_h__ */
