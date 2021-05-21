/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIVersionComparator.idl
 */

#ifndef __gen_nsIVersionComparator_h__
#define __gen_nsIVersionComparator_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIVersionComparator */
#define NS_IVERSIONCOMPARATOR_IID_STR "e6cd620a-edbb-41d2-9e42-9a2ffc8107f3"

#define NS_IVERSIONCOMPARATOR_IID \
  {0xe6cd620a, 0xedbb, 0x41d2, \
    { 0x9e, 0x42, 0x9a, 0x2f, 0xfc, 0x81, 0x07, 0xf3 }}

class NS_NO_VTABLE nsIVersionComparator : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IVERSIONCOMPARATOR_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIVersionComparator;

  /* long compare (in ACString A, in ACString B); */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD Compare(const nsACString& A, const nsACString& B, int32_t *_retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIVersionComparator, NS_IVERSIONCOMPARATOR_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIVERSIONCOMPARATOR \
  NS_IMETHOD Compare(const nsACString& A, const nsACString& B, int32_t *_retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIVERSIONCOMPARATOR \
  nsresult Compare(const nsACString& A, const nsACString& B, int32_t *_retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIVERSIONCOMPARATOR(_to) \
  NS_IMETHOD Compare(const nsACString& A, const nsACString& B, int32_t *_retval) override { return _to Compare(A, B, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIVERSIONCOMPARATOR(_to) \
  NS_IMETHOD Compare(const nsACString& A, const nsACString& B, int32_t *_retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Compare(A, B, _retval); } 


#endif /* __gen_nsIVersionComparator_h__ */
