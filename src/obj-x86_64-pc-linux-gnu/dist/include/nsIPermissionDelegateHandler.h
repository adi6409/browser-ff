/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIPermissionDelegateHandler.idl
 */

#ifndef __gen_nsIPermissionDelegateHandler_h__
#define __gen_nsIPermissionDelegateHandler_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsTArray_h__
#include "nsTArray.h"
#endif

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsIPermissionDelegateHandler */
#define NS_IPERMISSIONDELEGATEHANDLER_IID_STR "07611dc6-bf4d-4d8a-a64b-f3a5904dddc7"

#define NS_IPERMISSIONDELEGATEHANDLER_IID \
  {0x07611dc6, 0xbf4d, 0x4d8a, \
    { 0xa6, 0x4b, 0xf3, 0xa5, 0x90, 0x4d, 0xdd, 0xc7 }}

class NS_NO_VTABLE nsIPermissionDelegateHandler : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IPERMISSIONDELEGATEHANDLER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIPermissionDelegateHandler;

  /* boolean maybeUnsafePermissionDelegate (in Array<ACString> aTypes); */
  NS_IMETHOD MaybeUnsafePermissionDelegate(const nsTArray<nsCString >& aTypes, bool *_retval) = 0;

  /* readonly attribute boolean permissionDelegateFPEnabled; */
  NS_IMETHOD GetPermissionDelegateFPEnabled(bool *aPermissionDelegateFPEnabled) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIPermissionDelegateHandler, NS_IPERMISSIONDELEGATEHANDLER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIPERMISSIONDELEGATEHANDLER \
  NS_IMETHOD MaybeUnsafePermissionDelegate(const nsTArray<nsCString >& aTypes, bool *_retval) override; \
  NS_IMETHOD GetPermissionDelegateFPEnabled(bool *aPermissionDelegateFPEnabled) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIPERMISSIONDELEGATEHANDLER \
  nsresult MaybeUnsafePermissionDelegate(const nsTArray<nsCString >& aTypes, bool *_retval); \
  nsresult GetPermissionDelegateFPEnabled(bool *aPermissionDelegateFPEnabled); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIPERMISSIONDELEGATEHANDLER(_to) \
  NS_IMETHOD MaybeUnsafePermissionDelegate(const nsTArray<nsCString >& aTypes, bool *_retval) override { return _to MaybeUnsafePermissionDelegate(aTypes, _retval); } \
  NS_IMETHOD GetPermissionDelegateFPEnabled(bool *aPermissionDelegateFPEnabled) override { return _to GetPermissionDelegateFPEnabled(aPermissionDelegateFPEnabled); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIPERMISSIONDELEGATEHANDLER(_to) \
  NS_IMETHOD MaybeUnsafePermissionDelegate(const nsTArray<nsCString >& aTypes, bool *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->MaybeUnsafePermissionDelegate(aTypes, _retval); } \
  NS_IMETHOD GetPermissionDelegateFPEnabled(bool *aPermissionDelegateFPEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPermissionDelegateFPEnabled(aPermissionDelegateFPEnabled); } 


#endif /* __gen_nsIPermissionDelegateHandler_h__ */
