/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadGroupChild.idl
 */

#ifndef __gen_nsILoadGroupChild_h__
#define __gen_nsILoadGroupChild_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsILoadGroup; /* forward declaration */


/* starting interface:    nsILoadGroupChild */
#define NS_ILOADGROUPCHILD_IID_STR "02efe8e2-fbbc-4718-a299-b8a09c60bf6b"

#define NS_ILOADGROUPCHILD_IID \
  {0x02efe8e2, 0xfbbc, 0x4718, \
    { 0xa2, 0x99, 0xb8, 0xa0, 0x9c, 0x60, 0xbf, 0x6b }}

class NS_NO_VTABLE nsILoadGroupChild : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOADGROUPCHILD_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILoadGroupChild;

  /* attribute nsILoadGroup parentLoadGroup; */
  NS_IMETHOD GetParentLoadGroup(nsILoadGroup **aParentLoadGroup) = 0;
  NS_IMETHOD SetParentLoadGroup(nsILoadGroup *aParentLoadGroup) = 0;

  /* readonly attribute nsILoadGroup childLoadGroup; */
  NS_IMETHOD GetChildLoadGroup(nsILoadGroup **aChildLoadGroup) = 0;

  /* readonly attribute nsILoadGroup rootLoadGroup; */
  NS_IMETHOD GetRootLoadGroup(nsILoadGroup **aRootLoadGroup) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILoadGroupChild, NS_ILOADGROUPCHILD_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOADGROUPCHILD \
  NS_IMETHOD GetParentLoadGroup(nsILoadGroup **aParentLoadGroup) override; \
  NS_IMETHOD SetParentLoadGroup(nsILoadGroup *aParentLoadGroup) override; \
  NS_IMETHOD GetChildLoadGroup(nsILoadGroup **aChildLoadGroup) override; \
  NS_IMETHOD GetRootLoadGroup(nsILoadGroup **aRootLoadGroup) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOADGROUPCHILD \
  nsresult GetParentLoadGroup(nsILoadGroup **aParentLoadGroup); \
  nsresult SetParentLoadGroup(nsILoadGroup *aParentLoadGroup); \
  nsresult GetChildLoadGroup(nsILoadGroup **aChildLoadGroup); \
  nsresult GetRootLoadGroup(nsILoadGroup **aRootLoadGroup); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOADGROUPCHILD(_to) \
  NS_IMETHOD GetParentLoadGroup(nsILoadGroup **aParentLoadGroup) override { return _to GetParentLoadGroup(aParentLoadGroup); } \
  NS_IMETHOD SetParentLoadGroup(nsILoadGroup *aParentLoadGroup) override { return _to SetParentLoadGroup(aParentLoadGroup); } \
  NS_IMETHOD GetChildLoadGroup(nsILoadGroup **aChildLoadGroup) override { return _to GetChildLoadGroup(aChildLoadGroup); } \
  NS_IMETHOD GetRootLoadGroup(nsILoadGroup **aRootLoadGroup) override { return _to GetRootLoadGroup(aRootLoadGroup); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOADGROUPCHILD(_to) \
  NS_IMETHOD GetParentLoadGroup(nsILoadGroup **aParentLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetParentLoadGroup(aParentLoadGroup); } \
  NS_IMETHOD SetParentLoadGroup(nsILoadGroup *aParentLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetParentLoadGroup(aParentLoadGroup); } \
  NS_IMETHOD GetChildLoadGroup(nsILoadGroup **aChildLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetChildLoadGroup(aChildLoadGroup); } \
  NS_IMETHOD GetRootLoadGroup(nsILoadGroup **aRootLoadGroup) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetRootLoadGroup(aRootLoadGroup); } 


#endif /* __gen_nsILoadGroupChild_h__ */
