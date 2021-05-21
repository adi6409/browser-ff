/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHyperText.idl
 */

#ifndef __gen_nsIAccessibleHyperText_h__
#define __gen_nsIAccessibleHyperText_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIAccessibleHyperLink_h__
#include "nsIAccessibleHyperLink.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIAccessibleHyperText */
#define NS_IACCESSIBLEHYPERTEXT_IID_STR "b33684e2-090c-4e1d-a3d9-f4b46f4237b9"

#define NS_IACCESSIBLEHYPERTEXT_IID \
  {0xb33684e2, 0x090c, 0x4e1d, \
    { 0xa3, 0xd9, 0xf4, 0xb4, 0x6f, 0x42, 0x37, 0xb9 }}

class NS_NO_VTABLE nsIAccessibleHyperText : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IACCESSIBLEHYPERTEXT_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIAccessibleHyperText;

  /* readonly attribute long linkCount; */
  NS_IMETHOD GetLinkCount(int32_t *aLinkCount) = 0;

  /* nsIAccessibleHyperLink getLinkAt (in long index); */
  NS_IMETHOD GetLinkAt(int32_t index, nsIAccessibleHyperLink **_retval) = 0;

  /* long getLinkIndex (in nsIAccessibleHyperLink link); */
  NS_IMETHOD GetLinkIndex(nsIAccessibleHyperLink *link, int32_t *_retval) = 0;

  /* long getLinkIndexAtOffset (in long offset); */
  NS_IMETHOD GetLinkIndexAtOffset(int32_t offset, int32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIAccessibleHyperText, NS_IACCESSIBLEHYPERTEXT_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIACCESSIBLEHYPERTEXT \
  NS_IMETHOD GetLinkCount(int32_t *aLinkCount) override; \
  NS_IMETHOD GetLinkAt(int32_t index, nsIAccessibleHyperLink **_retval) override; \
  NS_IMETHOD GetLinkIndex(nsIAccessibleHyperLink *link, int32_t *_retval) override; \
  NS_IMETHOD GetLinkIndexAtOffset(int32_t offset, int32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIACCESSIBLEHYPERTEXT \
  nsresult GetLinkCount(int32_t *aLinkCount); \
  nsresult GetLinkAt(int32_t index, nsIAccessibleHyperLink **_retval); \
  nsresult GetLinkIndex(nsIAccessibleHyperLink *link, int32_t *_retval); \
  nsresult GetLinkIndexAtOffset(int32_t offset, int32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIACCESSIBLEHYPERTEXT(_to) \
  NS_IMETHOD GetLinkCount(int32_t *aLinkCount) override { return _to GetLinkCount(aLinkCount); } \
  NS_IMETHOD GetLinkAt(int32_t index, nsIAccessibleHyperLink **_retval) override { return _to GetLinkAt(index, _retval); } \
  NS_IMETHOD GetLinkIndex(nsIAccessibleHyperLink *link, int32_t *_retval) override { return _to GetLinkIndex(link, _retval); } \
  NS_IMETHOD GetLinkIndexAtOffset(int32_t offset, int32_t *_retval) override { return _to GetLinkIndexAtOffset(offset, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIACCESSIBLEHYPERTEXT(_to) \
  NS_IMETHOD GetLinkCount(int32_t *aLinkCount) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkCount(aLinkCount); } \
  NS_IMETHOD GetLinkAt(int32_t index, nsIAccessibleHyperLink **_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkAt(index, _retval); } \
  NS_IMETHOD GetLinkIndex(nsIAccessibleHyperLink *link, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkIndex(link, _retval); } \
  NS_IMETHOD GetLinkIndexAtOffset(int32_t offset, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetLinkIndexAtOffset(offset, _retval); } 


#endif /* __gen_nsIAccessibleHyperText_h__ */
