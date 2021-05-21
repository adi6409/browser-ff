/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamPriority.idl
 */

#ifndef __gen_nsIInputStreamPriority_h__
#define __gen_nsIInputStreamPriority_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#ifndef __gen_nsIRunnable_h__
#include "nsIRunnable.h"
#endif

#include "js/GCAnnotations.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif

/* starting interface:    nsIInputStreamPriority */
#define NS_IINPUTSTREAMPRIORITY_IID_STR "daa45b24-98ee-4eb2-9cec-aad0bc023e9d"

#define NS_IINPUTSTREAMPRIORITY_IID \
  {0xdaa45b24, 0x98ee, 0x4eb2, \
    { 0x9c, 0xec, 0xaa, 0xd0, 0xbc, 0x02, 0x3e, 0x9d }}

class NS_NO_VTABLE nsIInputStreamPriority : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_IINPUTSTREAMPRIORITY_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsIInputStreamPriority;

  /* attribute unsigned long priority; */
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD GetPriority(uint32_t *aPriority) = 0;
  JS_HAZ_CAN_RUN_SCRIPT NS_IMETHOD SetPriority(uint32_t aPriority) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsIInputStreamPriority, NS_IINPUTSTREAMPRIORITY_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSIINPUTSTREAMPRIORITY \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override; \
  NS_IMETHOD SetPriority(uint32_t aPriority) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSIINPUTSTREAMPRIORITY \
  nsresult GetPriority(uint32_t *aPriority); \
  nsresult SetPriority(uint32_t aPriority); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSIINPUTSTREAMPRIORITY(_to) \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override { return _to GetPriority(aPriority); } \
  NS_IMETHOD SetPriority(uint32_t aPriority) override { return _to SetPriority(aPriority); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSIINPUTSTREAMPRIORITY(_to) \
  NS_IMETHOD GetPriority(uint32_t *aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetPriority(aPriority); } \
  NS_IMETHOD SetPriority(uint32_t aPriority) override { return !_to ? NS_ERROR_NULL_POINTER : _to->SetPriority(aPriority); } 


#endif /* __gen_nsIInputStreamPriority_h__ */
