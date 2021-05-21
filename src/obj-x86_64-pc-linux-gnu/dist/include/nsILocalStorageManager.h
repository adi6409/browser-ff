/*
 * DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/localstorage/nsILocalStorageManager.idl
 */

#ifndef __gen_nsILocalStorageManager_h__
#define __gen_nsILocalStorageManager_h__


#ifndef __gen_nsISupports_h__
#include "nsISupports.h"
#endif

#include "js/Value.h"

/* For IDL files that don't want to include root IDL files. */
#ifndef NS_NO_VTABLE
#define NS_NO_VTABLE
#endif
class nsIPrincipal; /* forward declaration */


/* starting interface:    nsILocalStorageManager */
#define NS_ILOCALSTORAGEMANAGER_IID_STR "d4f534da-2744-4db3-8774-8b187c64ade9"

#define NS_ILOCALSTORAGEMANAGER_IID \
  {0xd4f534da, 0x2744, 0x4db3, \
    { 0x87, 0x74, 0x8b, 0x18, 0x7c, 0x64, 0xad, 0xe9 }}

class NS_NO_VTABLE nsILocalStorageManager : public nsISupports {
 public:

  NS_DECLARE_STATIC_IID_ACCESSOR(NS_ILOCALSTORAGEMANAGER_IID)

  /* Used by ToJSValue to check which scriptable interface is implemented. */
  using ScriptableInterfaceType = nsILocalStorageManager;

  /* readonly attribute boolean nextGenLocalStorageEnabled; */
  NS_IMETHOD GetNextGenLocalStorageEnabled(bool *aNextGenLocalStorageEnabled) = 0;

  /* [implicit_jscontext] Promise preload (in nsIPrincipal aPrincipal); */
  NS_IMETHOD Preload(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

  /* [implicit_jscontext] Promise isPreloaded (in nsIPrincipal aPrincipal); */
  NS_IMETHOD IsPreloaded(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) = 0;

};

  NS_DEFINE_STATIC_IID_ACCESSOR(nsILocalStorageManager, NS_ILOCALSTORAGEMANAGER_IID)

/* Use this macro when declaring classes that implement this interface. */
#define NS_DECL_NSILOCALSTORAGEMANAGER \
  NS_IMETHOD GetNextGenLocalStorageEnabled(bool *aNextGenLocalStorageEnabled) override; \
  NS_IMETHOD Preload(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; \
  NS_IMETHOD IsPreloaded(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override; 

/* Use this macro when declaring the members of this interface when the
   class doesn't implement the interface. This is useful for forwarding. */
#define NS_DECL_NON_VIRTUAL_NSILOCALSTORAGEMANAGER \
  nsresult GetNextGenLocalStorageEnabled(bool *aNextGenLocalStorageEnabled); \
  nsresult Preload(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval); \
  nsresult IsPreloaded(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval); 

/* Use this macro to declare functions that forward the behavior of this interface to another object. */
#define NS_FORWARD_NSILOCALSTORAGEMANAGER(_to) \
  NS_IMETHOD GetNextGenLocalStorageEnabled(bool *aNextGenLocalStorageEnabled) override { return _to GetNextGenLocalStorageEnabled(aNextGenLocalStorageEnabled); } \
  NS_IMETHOD Preload(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to Preload(aPrincipal, cx, _retval); } \
  NS_IMETHOD IsPreloaded(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return _to IsPreloaded(aPrincipal, cx, _retval); } 

/* Use this macro to declare functions that forward the behavior of this interface to another object in a safe way. */
#define NS_FORWARD_SAFE_NSILOCALSTORAGEMANAGER(_to) \
  NS_IMETHOD GetNextGenLocalStorageEnabled(bool *aNextGenLocalStorageEnabled) override { return !_to ? NS_ERROR_NULL_POINTER : _to->GetNextGenLocalStorageEnabled(aNextGenLocalStorageEnabled); } \
  NS_IMETHOD Preload(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->Preload(aPrincipal, cx, _retval); } \
  NS_IMETHOD IsPreloaded(nsIPrincipal *aPrincipal, JSContext* cx, ::mozilla::dom::Promise * * _retval) override { return !_to ? NS_ERROR_NULL_POINTER : _to->IsPreloaded(aPrincipal, cx, _retval); } 


#endif /* __gen_nsILocalStorageManager_h__ */
