/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextChangeEvent.idl
 */

#ifndef __gen_nsIAccessibleTextChangeEvent_h__
#define __gen_nsIAccessibleTextChangeEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleTextChangeEvent */
#define NS_IACCESSIBLETEXTCHANGEEVENT_IID_STR "1fcc0dfa-93e6-48f4-bbd4-f80eb1d9f2e6"

#define NS_IACCESSIBLETEXTCHANGEEVENT_IID \
  {0x1fcc0dfa, 0x93e6, 0x48f4, \
    { 0xbb, 0xd4, 0xf8, 0x0e, 0xb1, 0xd9, 0xf2, 0xe6 }}

class NS_NO_VTABLE nsIAccessibleTextChangeEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETEXTCHANGEEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTextChangeEvent;

  /* readonly attribute long start; */
  NS_IMETHOD GetStart(int32_t *aStart) = 0;

  /* readonly attribute unsigned long length; */
  NS_IMETHOD GetLength(uint32_t *aLength) = 0;

  /* readonly attribute boolean isInserted; */
  NS_IMETHOD GetIsInserted(bool *aIsInserted) = 0;

  /* readonly attribute AString modifiedText; */
  NS_IMETHOD GetModifiedText(nsAString& aModifiedText) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTextChangeEvent, NS_IACCESSIBLETEXTCHANGEEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETEXTCHANGEEVENT \
  NS_IMETHOD GetStart(int32_t *aStart) override; \
  NS_IMETHOD GetLength(uint32_t *aLength) override; \
  NS_IMETHOD GetIsInserted(bool *aIsInserted) override; \
  NS_IMETHOD GetModifiedText(nsAString& aModifiedText) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETEXTCHANGEEVENT \
  nsresult GetStart(int32_t *aStart); \
  nsresult GetLength(uint32_t *aLength); \
  nsresult GetIsInserted(bool *aIsInserted); \
  nsresult GetModifiedText(nsAString& aModifiedText); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETEXTCHANGEEVENT(_to) \
  NS_IMETHOD GetStart(int32_t *aStart) override { return _to GetStart(aStart); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return _to GetLength(aLength); } \
  NS_IMETHOD GetIsInserted(bool *aIsInserted) override { return _to GetIsInserted(aIsInserted); } \
  NS_IMETHOD GetModifiedText(nsAString& aModifiedText) override { return _to GetModifiedText(aModifiedText); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETEXTCHANGEEVENT(_to) \
  NS_IMETHOD GetStart(int32_t *aStart) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetStart(aStart); } \
  NS_IMETHOD GetLength(uint32_t *aLength) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLength(aLength); } \
  NS_IMETHOD GetIsInserted(bool *aIsInserted) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetIsInserted(aIsInserted); } \
  NS_IMETHOD GetModifiedText(nsAString& aModifiedText) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetModifiedText(aModifiedText); } 


#endif /* __gen_nsIAccessibleTextChangeEvent_h__ */
