/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTableChangeEvent.idl
 */

#ifndef __gen_nsIAccessibleTableChangeEvent_h__
#define __gen_nsIAccessibleTableChangeEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleTableChangeEvent */
#define NS_IACCESSIBLETABLECHANGEEVENT_IID_STR "9fb3a8a4-d254-43d3-80a5-20e171d52b21"

#define NS_IACCESSIBLETABLECHANGEEVENT_IID \
  {0x9fb3a8a4, 0xd254, 0x43d3, \
    { 0x80, 0xa5, 0x20, 0xe1, 0x71, 0xd5, 0x2b, 0x21 }}

class NS_NO_VTABLE nsIAccessibleTableChangeEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLETABLECHANGEEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleTableChangeEvent;

  /* readonly attribute long rowOrColIndex; */
  NS_IMETHOD GetRowOrColIndex(int32_t *aRowOrColIndex) = 0;

  /* readonly attribute long RowsOrColsCount; */
  NS_IMETHOD GetRowsOrColsCount(int32_t *aRowsOrColsCount) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleTableChangeEvent, NS_IACCESSIBLETABLECHANGEEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLETABLECHANGEEVENT \
  NS_IMETHOD GetRowOrColIndex(int32_t *aRowOrColIndex) override; \
  NS_IMETHOD GetRowsOrColsCount(int32_t *aRowsOrColsCount) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLETABLECHANGEEVENT \
  nsresult GetRowOrColIndex(int32_t *aRowOrColIndex); \
  nsresult GetRowsOrColsCount(int32_t *aRowsOrColsCount); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLETABLECHANGEEVENT(_to) \
  NS_IMETHOD GetRowOrColIndex(int32_t *aRowOrColIndex) override { return _to GetRowOrColIndex(aRowOrColIndex); } \
  NS_IMETHOD GetRowsOrColsCount(int32_t *aRowsOrColsCount) override { return _to GetRowsOrColsCount(aRowsOrColsCount); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLETABLECHANGEEVENT(_to) \
  NS_IMETHOD GetRowOrColIndex(int32_t *aRowOrColIndex) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowOrColIndex(aRowOrColIndex); } \
  NS_IMETHOD GetRowsOrColsCount(int32_t *aRowsOrColsCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRowsOrColsCount(aRowsOrColsCount); } 


#endif /* __gen_nsIAccessibleTableChangeEvent_h__ */
