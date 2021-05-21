/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleVirtualCursorChangeEvent.idl
 */

#ifndef __gen_nsIAccessibleVirtualCursorChangeEvent_h__
#define __gen_nsIAccessibleVirtualCursorChangeEvent_h__


#ifndef __gen_nsIAccessibleEvent_h__
#include "nsIAccessibleEvent.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleVirtualCursorChangeEvent */
#define NS_IACCESSIBLEVIRTUALCURSORCHANGEEVENT_IID_STR "a58693b1-009e-4cc9-ae93-9c7d8f85cfdf"

#define NS_IACCESSIBLEVIRTUALCURSORCHANGEEVENT_IID \
  {0xa58693b1, 0x009e, 0x4cc9, \
    { 0xae, 0x93, 0x9c, 0x7d, 0x8f, 0x85, 0xcf, 0xdf }}

class NS_NO_VTABLE nsIAccessibleVirtualCursorChangeEvent : public nsIAccessibleEvent {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEVIRTUALCURSORCHANGEEVENT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleVirtualCursorChangeEvent;

  /* readonly attribute nsIAccessible oldAccessible; */
  NS_IMETHOD GetOldAccessible(nsIAccessible **aOldAccessible) = 0;

  /* readonly attribute long oldStartOffset; */
  NS_IMETHOD GetOldStartOffset(int32_t *aOldStartOffset) = 0;

  /* readonly attribute long oldEndOffset; */
  NS_IMETHOD GetOldEndOffset(int32_t *aOldEndOffset) = 0;

  /* readonly attribute nsIAccessible newAccessible; */
  NS_IMETHOD GetNewAccessible(nsIAccessible **aNewAccessible) = 0;

  /* readonly attribute long newStartOffset; */
  NS_IMETHOD GetNewStartOffset(int32_t *aNewStartOffset) = 0;

  /* readonly attribute long newEndOffset; */
  NS_IMETHOD GetNewEndOffset(int32_t *aNewEndOffset) = 0;

  /* readonly attribute short reason; */
  NS_IMETHOD GetReason(int16_t *aReason) = 0;

  /* readonly attribute short boundaryType; */
  NS_IMETHOD GetBoundaryType(int16_t *aBoundaryType) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleVirtualCursorChangeEvent, NS_IACCESSIBLEVIRTUALCURSORCHANGEEVENT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEVIRTUALCURSORCHANGEEVENT \
  NS_IMETHOD GetOldAccessible(nsIAccessible **aOldAccessible) override; \
  NS_IMETHOD GetOldStartOffset(int32_t *aOldStartOffset) override; \
  NS_IMETHOD GetOldEndOffset(int32_t *aOldEndOffset) override; \
  NS_IMETHOD GetNewAccessible(nsIAccessible **aNewAccessible) override; \
  NS_IMETHOD GetNewStartOffset(int32_t *aNewStartOffset) override; \
  NS_IMETHOD GetNewEndOffset(int32_t *aNewEndOffset) override; \
  NS_IMETHOD GetReason(int16_t *aReason) override; \
  NS_IMETHOD GetBoundaryType(int16_t *aBoundaryType) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEVIRTUALCURSORCHANGEEVENT \
  nsresult GetOldAccessible(nsIAccessible **aOldAccessible); \
  nsresult GetOldStartOffset(int32_t *aOldStartOffset); \
  nsresult GetOldEndOffset(int32_t *aOldEndOffset); \
  nsresult GetNewAccessible(nsIAccessible **aNewAccessible); \
  nsresult GetNewStartOffset(int32_t *aNewStartOffset); \
  nsresult GetNewEndOffset(int32_t *aNewEndOffset); \
  nsresult GetReason(int16_t *aReason); \
  nsresult GetBoundaryType(int16_t *aBoundaryType); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEVIRTUALCURSORCHANGEEVENT(_to) \
  NS_IMETHOD GetOldAccessible(nsIAccessible **aOldAccessible) override { return _to GetOldAccessible(aOldAccessible); } \
  NS_IMETHOD GetOldStartOffset(int32_t *aOldStartOffset) override { return _to GetOldStartOffset(aOldStartOffset); } \
  NS_IMETHOD GetOldEndOffset(int32_t *aOldEndOffset) override { return _to GetOldEndOffset(aOldEndOffset); } \
  NS_IMETHOD GetNewAccessible(nsIAccessible **aNewAccessible) override { return _to GetNewAccessible(aNewAccessible); } \
  NS_IMETHOD GetNewStartOffset(int32_t *aNewStartOffset) override { return _to GetNewStartOffset(aNewStartOffset); } \
  NS_IMETHOD GetNewEndOffset(int32_t *aNewEndOffset) override { return _to GetNewEndOffset(aNewEndOffset); } \
  NS_IMETHOD GetReason(int16_t *aReason) override { return _to GetReason(aReason); } \
  NS_IMETHOD GetBoundaryType(int16_t *aBoundaryType) override { return _to GetBoundaryType(aBoundaryType); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEVIRTUALCURSORCHANGEEVENT(_to) \
  NS_IMETHOD GetOldAccessible(nsIAccessible **aOldAccessible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOldAccessible(aOldAccessible); } \
  NS_IMETHOD GetOldStartOffset(int32_t *aOldStartOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOldStartOffset(aOldStartOffset); } \
  NS_IMETHOD GetOldEndOffset(int32_t *aOldEndOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetOldEndOffset(aOldEndOffset); } \
  NS_IMETHOD GetNewAccessible(nsIAccessible **aNewAccessible) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewAccessible(aNewAccessible); } \
  NS_IMETHOD GetNewStartOffset(int32_t *aNewStartOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewStartOffset(aNewStartOffset); } \
  NS_IMETHOD GetNewEndOffset(int32_t *aNewEndOffset) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNewEndOffset(aNewEndOffset); } \
  NS_IMETHOD GetReason(int16_t *aReason) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetReason(aReason); } \
  NS_IMETHOD GetBoundaryType(int16_t *aBoundaryType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetBoundaryType(aBoundaryType); } 


#endif /* __gen_nsIAccessibleVirtualCursorChangeEvent_h__ */
