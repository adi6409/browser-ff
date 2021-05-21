/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleRelation.idl
 */

#ifndef __gen_nsIAccessibleRelation_h__
#define __gen_nsIAccessibleRelation_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIArray_h__
#include "nsIArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIAccessible; /* forward declaration */


/* starting interface:    nsIAccessibleRelation */
#define NS_IACCESSIBLERELATION_IID_STR "55b308c4-2ae4-46bc-b4cd-4d4370e0a660"

#define NS_IACCESSIBLERELATION_IID \
  {0x55b308c4, 0x2ae4, 0x46bc, \
    { 0xb4, 0xcd, 0x4d, 0x43, 0x70, 0xe0, 0xa6, 0x60 }}

class NS_NO_VTABLE nsIAccessibleRelation : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLERELATION_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleRelation;

  enum {
    RELATION_LABELLED_BY = 0U,
    RELATION_LABEL_FOR = 1U,
    RELATION_DESCRIBED_BY = 2U,
    RELATION_DESCRIPTION_FOR = 3U,
    RELATION_NODE_CHILD_OF = 4U,
    RELATION_NODE_PARENT_OF = 5U,
    RELATION_CONTROLLED_BY = 6U,
    RELATION_CONTROLLER_FOR = 7U,
    RELATION_FLOWS_TO = 8U,
    RELATION_FLOWS_FROM = 9U,
    RELATION_MEMBER_OF = 10U,
    RELATION_SUBWINDOW_OF = 11U,
    RELATION_EMBEDS = 12U,
    RELATION_EMBEDDED_BY = 13U,
    RELATION_POPUP_FOR = 14U,
    RELATION_PARENT_WINDOW_OF = 15U,
    RELATION_DEFAULT_BUTTON = 16U,
    RELATION_CONTAINING_DOCUMENT = 17U,
    RELATION_CONTAINING_TAB_PANE = 18U,
    RELATION_CONTAINING_WINDOW = 19U,
    RELATION_CONTAINING_APPLICATION = 20U,
    RELATION_DETAILS = 21U,
    RELATION_DETAILS_FOR = 22U,
    RELATION_ERRORMSG = 23U,
    RELATION_ERRORMSG_FOR = 24U
  };

  /* readonly attribute unsigned long relationType; */
  NS_IMETHOD GetRelationType(uint32_t *aRelationType) = 0;

  /* readonly attribute unsigned long targetsCount; */
  NS_IMETHOD GetTargetsCount(uint32_t *aTargetsCount) = 0;

  /* nsIAccessible getTarget (in unsigned long index); */
  NS_IMETHOD GetTarget(uint32_t index, nsIAccessible **_retval) = 0;

  /* nsIArray getTargets (); */
  NS_IMETHOD GetTargets(nsIArray **_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleRelation, NS_IACCESSIBLERELATION_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLERELATION \
  NS_IMETHOD GetRelationType(uint32_t *aRelationType) override; \
  NS_IMETHOD GetTargetsCount(uint32_t *aTargetsCount) override; \
  NS_IMETHOD GetTarget(uint32_t index, nsIAccessible **_retval) override; \
  NS_IMETHOD GetTargets(nsIArray **_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLERELATION \
  nsresult GetRelationType(uint32_t *aRelationType); \
  nsresult GetTargetsCount(uint32_t *aTargetsCount); \
  nsresult GetTarget(uint32_t index, nsIAccessible **_retval); \
  nsresult GetTargets(nsIArray **_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLERELATION(_to) \
  NS_IMETHOD GetRelationType(uint32_t *aRelationType) override { return _to GetRelationType(aRelationType); } \
  NS_IMETHOD GetTargetsCount(uint32_t *aTargetsCount) override { return _to GetTargetsCount(aTargetsCount); } \
  NS_IMETHOD GetTarget(uint32_t index, nsIAccessible **_retval) override { return _to GetTarget(index, _retval); } \
  NS_IMETHOD GetTargets(nsIArray **_retval) override { return _to GetTargets(_retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLERELATION(_to) \
  NS_IMETHOD GetRelationType(uint32_t *aRelationType) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRelationType(aRelationType); } \
  NS_IMETHOD GetTargetsCount(uint32_t *aTargetsCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargetsCount(aTargetsCount); } \
  NS_IMETHOD GetTarget(uint32_t index, nsIAccessible **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTarget(index, _retval); } \
  NS_IMETHOD GetTargets(nsIArray **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetTargets(_retval); } 


#endif /* __gen_nsIAccessibleRelation_h__ */
